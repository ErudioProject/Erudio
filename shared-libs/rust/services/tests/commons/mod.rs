use config::Config;
use error_handler::{InternalError, InternalResult};
use eyre::Context;
use log::debug;
use prisma_client::{
	prisma::{pii_data, user, GrammaticalForm, PrismaClient},
	prisma_mocked_client, User,
};
use rand::{thread_rng, RngCore};
use redis::aio::Connection;
use std::env;
use tokio::fs;
// TODO FIX REMOVE TEMPORARY

pub(crate) async fn init_tests_with_user() -> InternalResult<(PrismaClient, Connection, User, Vec<u8>)> {
	dotenvy::dotenv().ok();
	env_logger::init();
	// TODO pull over http from server
	let contents = fs::read_to_string("../../../Config.ron")
		.await
		.context("no Config.ron file")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;
	let config: Config = ron::from_str(&contents)
		.context("Config.ron is invalid")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;
	debug!("Config: {:?}", config);

	let db = prisma_mocked_client(config.db_url_test.clone())
		.await
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;

	db._db_push()
		.await
		.context("DB push failed")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;

	let redis_client = redis::Client::open(config.redis_url)
		.context("Redis not found")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;
	let redis = redis_client
		.get_async_connection()
		.await
		.context("Some redis Error")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;

	let mut connection_secret = vec![0; 512];
	let mut random_data_for_email = vec![0; 512];
	{
		let mut rng = thread_rng();
		rng.fill_bytes(&mut connection_secret);
		rng.fill_bytes(&mut random_data_for_email);
	}

	// TODO replace with transaction once supported
	// TODO Further refactor user creating
	let user = db
		.user()
		.create("".into(), vec![])
		.with(user::user_school_relation::fetch(vec![]))
		.exec()
		.await
		.context("Db error user")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;

	let pii_data = db
		.pii_data()
		.create(
			GrammaticalForm::Indeterminate,
			"legal_name".into(),
			"diplay_name".into(),
			user::id::equals(user.id.clone()),
			vec![pii_data::email::Set(Some(hex::encode(random_data_for_email))).into()],
		)
		.exec()
		.await
		.context("Db error pii_data")
		.map_err(|err| InternalError::TestError(format!("{:?}", err)))?;

	let user = user::Data {
		pii_data: Some(Some(Box::new(pii_data))),
		..user
	};

	Ok((db, redis, user, connection_secret))
}
