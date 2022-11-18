use super::*;
use backend_error_handler::{ApiError, ApiResult};
use backend_prisma_client::{
	prisma::{new_client_with_url, pii_data, user, GrammaticalForm, PrismaClient},
	User,
};
use rand::prelude::*;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::{env, sync::Arc};

#[tokio::test]
async fn init_load_destroy() -> ApiResult<()> {
	//TODO rewrite complete rewrite
	dotenvy::dotenv().ok();
	env_logger::init();
	#[cfg(target_family = "unix")]
	let url = env::var("DATABASE_URL").expect("set DATABASE_URL env");
	#[cfg(target_family = "windows")]
	let url = env::var("DATABASE_URL_WIN").expect("set DATABASE_URL_WIN env");

	let db: Arc<PrismaClient> = Arc::new(
		new_client_with_url(&url)
			.await
			.expect("db connection error"),
	);

	let redis_url = env::var("REDIS_URL").expect("set REDIS_URL env");

	let redis_client = redis::Client::open(redis_url).expect("Redis not found");
	let redis = redis_client
		.get_multiplexed_async_connection()
		.await
		.expect("Some redis Error");

	let mut connection_secret = vec![0; 512];
	let mut random_data_for_email = vec![0; 512];
	{
		let mut rng = thread_rng(); // TODO Maybe change
		rng.fill_bytes(&mut connection_secret);
		rng.fill_bytes(&mut random_data_for_email);
	}

	// TODO replace with transaction once supported Also some auto test user creation maybe
	let user = db
		.user()
		.create(vec![], vec![])
		.exec()
		.await
		.expect("Db error user");

	let pii_data = db
		.pii_data()
		.create(GrammaticalForm::Indeterminate, user::id::equals(user.id.clone()), vec![
			pii_data::email::Set(Some(hex::encode(random_data_for_email))).into(),
		])
		.exec()
		.await
		.expect("Db error pii_data");

	let user = user::Data {
		pii_data: Some(Some(Box::new(pii_data))),
		..user
	};

	let mut_redis = &mut redis.clone();
	match init_load_destroy_inner(db.clone(), mut_redis, &user, &connection_secret).await {
		Ok(_) => {
			db.user().delete(user::id::equals(user.id)).exec().await?;
			Ok(())
		}
		Err(err) => {
			let _: Result<(), _> = mut_redis.del(hex::encode(&connection_secret)).await; // Error deliberately ignored
			db.user().delete(user::id::equals(user.id)).exec().await?;
			Err(err)
		}
	}
}

async fn init_load_destroy_inner(
	db: Arc<PrismaClient>,
	redis: &mut MultiplexedConnection,
	user: &User,
	connection_secret: &Vec<u8>,
) -> ApiResult<()> {
	let client_secret = init_session(&db, redis, user, connection_secret, Some(10)).await?;

	let user = load_session(db.clone(), redis, &client_secret, Some(10)).await?;

	user.ok_or_else(|| ApiError::TestError("User is none".into()))?;

	redis.del(&client_secret).await?;

	let user = load_session(db.clone(), redis, &client_secret, Some(10)).await?;

	user.ok_or_else(|| ApiError::TestError("User wasn't successfully recovered".into()))?;

	destroy_session(&db, redis, &client_secret).await?;

	let user = load_session(db, redis, &client_secret, Some(10)).await?;

	if user.is_some() {
		return Err(ApiError::TestError("Session didn't got deleted".into()));
	}
	Ok(())
}
