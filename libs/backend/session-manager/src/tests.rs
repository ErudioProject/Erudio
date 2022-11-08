use super::*;
use backend_error_handler::ApiResult;
use backend_prisma_client::prisma::{new_client_with_url, pii_data, user, GrammaticalForm};
use rand::prelude::*;
use std::env;

#[tokio::test]
async fn init_load_destroy() {
	dotenvy::dotenv().ok();
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
	let redis = Arc::new(Mutex::new(
		redis_client
			.get_async_connection()
			.await
			.expect("Some redis Error"),
	));

	let mut connection_secret = [0].repeat(512);
	let mut random_data_for_email = [0].repeat(512);
	{
		let mut rng = thread_rng(); // TODO Maybe change
		rng.fill_bytes(&mut connection_secret);
		rng.fill_bytes(&mut random_data_for_email);
	}

	// TODO replace with transaction one supported Also some auto test user creation maybe
	let user = db
		.user()
		.create(vec![], false, GrammaticalForm::Indeterminate, vec![])
		.exec()
		.await
		.expect("Db error user");

	let pii_data = db
		.pii_data()
		.create(user::id::equals(user.id.clone()), vec![pii_data::email::Set(Some(
			hex::encode(random_data_for_email),
		))
		.into()])
		.exec()
		.await
		.expect("Db error pii_data");

	let user = user::Data {
		pii_data: Some(Some(Box::new(pii_data))),
		..user
	};

	let client_secret = init_session(&db, &redis, &user, &connection_secret)
		.await
		.expect("Init  error");

	let user = load_session(&db, &redis, &client_secret)
		.await
		.expect("Load error");

	assert!(user.is_some());

	destroy_session(&db, &redis, &client_secret)
		.await
		.expect("Destruction error");

	let user = load_session(&db, &redis, &client_secret)
		.await
		.expect("Load error");

	assert!(user.is_none());
}
