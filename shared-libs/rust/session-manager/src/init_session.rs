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

async fn init_redis<R: AsyncCommands>(
	redis: &mut R,
	user: &User,
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

	static CLIENT_SECRET: Lazy<String> = Lazy::new(|| hex::encode(vec![0u8; 32]));
	static USER: Lazy<User> = Lazy::new(|| User {
		id: "0".repeat(16),
		password_hash: vec![1u8; 32],
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
	async fn test_init_redis_no_expire() {
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("SET")
					.arg(&*CLIENT_SECRET)
					.arg(serde_json::to_string(&*USER).unwrap()),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let result = init_redis(&mut mock_redis, &USER, &CLIENT_SECRET, None).await;
		assert!(result.is_ok());
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
	}

	#[tokio::test]
	async fn test_init_redis_expire() {
		let expire_seconds = 10;
		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("SETEX")
					.arg(&*CLIENT_SECRET)
					.arg(expire_seconds)
					.arg(serde_json::to_string(&*USER).unwrap()),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let result = init_redis(&mut mock_redis, &USER, &CLIENT_SECRET, Some(expire_seconds)).await;
		assert!(result.is_ok());
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
	}
}
