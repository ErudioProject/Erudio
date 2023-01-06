use crate::session;
use crate::session::init;
use chrono::{DateTime, Duration, Utc};
use error_handler::{InternalError, InternalResult};
use prisma_client::{
	prisma::{user, PrismaClient},
	User,
};
use redis::{AsyncCommands, JsonAsyncCommands};
use tokio::join;

pub async fn session<R: AsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	user: User,
	client_secret: &[u8],
	redis_expires_seconds: Option<usize>,
) -> Result<String, InternalError> {
	debug_assert!(user.user_school_relation.is_some());
	debug_assert!(user.pii_data.is_some());
	let data = user.into();
	let encoded = hex::encode(client_secret);
	let redis_async = init::redis(redis, &data, &encoded, redis_expires_seconds);
	let prisma_async = init::prisma(db, client_secret, &data.user.id);
	let result = join!(redis_async, prisma_async);
	result.0?;
	result.1?;
	Ok(encoded)
}

pub(crate) async fn redis<R: AsyncCommands + JsonAsyncCommands>(
	redis: &mut R,
	data: &session::Info,
	client_secret: &str,
	expires: Option<usize>,
) -> InternalResult<()> {
	let mut data = data.clone();
	data.user.password_hash = String::new();
	redis.json_set(client_secret, "$", &data).await?;
	if let Some(time) = expires {
		redis.expire(client_secret, time).await?;
	};
	Ok(())
}

async fn prisma(db: &PrismaClient, client_secret: &[u8], id: &str) -> InternalResult<()> {
	db.session()
		.create(
			client_secret.into(),
			DateTime::from(Utc::now() + Duration::days(365)),
			user::id::equals(id.into()),
			vec![],
		)
		.exec()
		.await?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use once_cell::sync::Lazy;
	use prisma_client::prisma_client_rust::serde_json;
	use redis_test::{MockCmd, MockRedisConnection};
	use std::env;
	use uuid::Uuid;

	static CLIENT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| vec![0u8; 32]);
	static USER: Lazy<User> = Lazy::new(|| User {
		id: Uuid::new_v4().to_string(),
		password_hash: "1".repeat(32),
		two_factor_auth_settings_id: None,
		pii_data: Some(None),
		two_factor_auth_settings: None,
		session: None,
		user_school_relation: Some(vec![]),
		user_classes: None,
		mark_gotten: None,
		mark_given: None,
		teached_lesson: None,
		attended_external_lessons: None,
		class_admin: None,
		subject_admin: None,
		subject_class_teacher_relation: None,
	});

	#[tokio::test]
	async fn test_init_redis_no_expire() -> InternalResult<()> {
		let user = USER.clone();
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("JSON.SET")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg("$")
					.arg(
						serde_json::to_string(&crate::session::Info::from(user.clone()))
							.unwrap()
							.replace(&user.password_hash, ""),
					),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);
		let result = init::redis(&mut mock_redis, &user.into(), &hex::encode(CLIENT_SECRET.clone()), None).await;
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
		result
	}

	#[tokio::test]
	async fn test_init_redis_expire() -> InternalResult<()> {
		let expire_seconds = 10;
		let user = USER.clone();
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("JSON.SET")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg("$")
					.arg(
						serde_json::to_string(&crate::session::Info::from(user.clone()))
							.unwrap()
							.replace(&user.password_hash, ""),
					),
				Ok("OK"),
			),
			MockCmd::new(
				redis::cmd("EXPIRE")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg(expire_seconds),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);
		let result = init::redis(
			&mut mock_redis,
			&user.into(),
			&hex::encode(CLIENT_SECRET.clone()),
			Some(expire_seconds),
		)
		.await;
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
		result
	}

	#[tokio::test]
	async fn test_init_prisma_expire() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");

		let db =
			prisma_client::prisma_mocked_client(env::var("DATABASE_URL_TESTS").expect("DATABASE_URL_TESTS not found"))
				.await
				.expect("Test database error");
		db.user()
			.create(USER.password_hash.clone(), vec![user::id::set(USER.id.clone())])
			.exec()
			.await?;

		init::prisma(&db, &CLIENT_SECRET, &USER.id).await
	}

	#[tokio::test]
	async fn test_init() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");
		let db =
			prisma_client::prisma_mocked_client(env::var("DATABASE_URL_TESTS").expect("DATABASE_URL_TESTS not found"))
				.await
				.expect("Test database error");
		db.user()
			.create(USER.password_hash.clone(), vec![user::id::set(USER.id.clone())])
			.exec()
			.await?;
		let user = USER.clone();
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("JSON.SET")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg("$")
					.arg(
						serde_json::to_string(&session::Info::from(user.clone()))
							.unwrap()
							.replace(&user.password_hash, ""),
					),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let result = init::session(&db, &mut mock_redis, user, &CLIENT_SECRET, None).await?;
		assert_eq!(result, hex::encode(&*CLIENT_SECRET));
		Ok(())
	}
}
