use chrono::{DateTime, Duration, Utc};
use error_handler::{InternalError, InternalResult};
use prisma_client::{
	prisma::{user, PrismaClient},
	prisma_client_rust::serde_json,
	User,
};
use redis::AsyncCommands;
use tokio::join;

pub async fn init_session<R: AsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	user: &User,
	client_secret: &[u8],
	redis_expires_seconds: Option<usize>,
) -> Result<String, InternalError> {
	let encoded = hex::encode(client_secret);
	let redis_async = init_redis(redis, user, &encoded, redis_expires_seconds);
	let prisma_async = init_prisma(db, client_secret, &user.id);
	let result = join!(redis_async, prisma_async);
	result.0?;
	result.1?;
	Ok(encoded)
}

async fn init_redis<R: AsyncCommands, U: serde::Serialize>(
	redis: &mut R,
	user: &U,
	client_secret: &str,
	expires: Option<usize>,
) -> InternalResult<()> {
	let json = serde_json::to_string(&user)?;
	match expires {
		None => redis.set(client_secret, json).await?,
		Some(time) => redis.set_ex(client_secret, json, time).await?,
	};
	Ok(())
}

async fn init_prisma(db: &PrismaClient, client_secret: &[u8], id: &str) -> InternalResult<()> {
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
	use redis_test::{MockCmd, MockRedisConnection};
	use std::env;
	use uuid::Uuid;

	static CLIENT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| vec![0u8; 32]);
	static USER: Lazy<User> = Lazy::new(|| User {
		id: Uuid::new_v4().to_string(),
		password_hash: "1".repeat(32),
		two_factor_auth_settings_id: None,
		pii_data: None,
		two_factor_auth_settings: None,
		session: None,
		user_school_relation: None,
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
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("SET")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg(serde_json::to_string(&*USER).unwrap()),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let result = init_redis(&mut mock_redis, &*USER, &hex::encode(CLIENT_SECRET.clone()), None).await;
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
		result
	}

	#[tokio::test]
	async fn test_init_redis_expire() -> InternalResult<()> {
		let expire_seconds = 10;
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("SETEX")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg(expire_seconds)
					.arg(serde_json::to_string(&*USER).unwrap()),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let result = init_redis(
			&mut mock_redis,
			&*USER,
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

		init_prisma(&db, &CLIENT_SECRET, &USER.id).await
	}
}
