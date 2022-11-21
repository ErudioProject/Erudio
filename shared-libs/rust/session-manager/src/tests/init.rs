use backend_error_handler::{ApiError, ApiResult};
use backend_prisma_client::prisma::{pii_data, user, GrammaticalForm, PrismaClient};
use eyre::Context;
use rand::{thread_rng, RngCore};
use redis::aio::MultiplexedConnection;
use std::{env, sync::Arc};

pub(crate) async fn init_tests_with_user() -> ApiResult<(Arc<PrismaClient>, MultiplexedConnection, user::Data, Vec<u8>)>
{
	dotenvy::dotenv().ok();
	env_logger::init();
	#[cfg(target_family = "unix")]
	let url = env::var("DATABASE_URL").expect("Set DATABASE_URL env");
	#[cfg(target_family = "windows")]
	let url = env::var("DATABASE_URL_WIN").expect("Set DATABASE_URL_WIN env");

	let db = Arc::new(
		PrismaClient::_builder()
			.with_url(url)
			.build()
			.await
			.context("db connection error")
			.map_err(|err| ApiError::TestError(format!("{:?}", err)))?,
	);

	db._db_push()
		.await
		.context("DB push failed")
		.map_err(|err| ApiError::TestError(format!("{:?}", err)))?;
	let redis_url = env::var("REDIS_URL")
		.context("set REDIS_URL env")
		.map_err(|err| ApiError::TestError(format!("{:?}", err)))?;

	let redis_client = redis::Client::open(redis_url)
		.context("Redis not found")
		.map_err(|err| ApiError::TestError(format!("{:?}", err)))?;
	let redis = redis_client
		.get_multiplexed_async_connection()
		.await
		.context("Some redis Error")
		.map_err(|err| ApiError::TestError(format!("{:?}", err)))?;

	let mut connection_secret = vec![0; 512];
	let mut random_data_for_email = vec![0; 512];
	{
		let mut rng = thread_rng(); // TODO Maybe change rng
		rng.fill_bytes(&mut connection_secret);
		rng.fill_bytes(&mut random_data_for_email);
	}

	// TODO replace with transaction once supported
	// TODO Further refactor user creating
	let user = db
		.user()
		.create(vec![], vec![])
		.exec()
		.await
		.context("Db error user")
		.map_err(|err| ApiError::TestError(format!("{:?}", err)))?;

	let pii_data = db
		.pii_data()
		.create(GrammaticalForm::Indeterminate, user::id::equals(user.id.clone()), vec![
			pii_data::email::Set(Some(hex::encode(random_data_for_email))).into(),
		])
		.exec()
		.await
		.context("Db error pii_data")
		.map_err(|err| ApiError::TestError(format!("{:?}", err)))?;

	let user = user::Data {
		pii_data: Some(Some(Box::new(pii_data))),
		..user
	};

	Ok((db, redis, user, connection_secret))
}
