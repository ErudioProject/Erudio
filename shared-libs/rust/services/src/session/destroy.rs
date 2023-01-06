use error_handler::InternalError;
use prisma_client::{
	prisma::{session, PrismaClient},
	prisma_client_rust::rspc::ErrorCode,
};
use redis::AsyncCommands;
use std::sync::Arc;

/// Deletes session if exists, successful response guarantees that session with giver secret no longer exists
pub async fn destroy<R: AsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	client_secret: &str,
) -> Result<(), InternalError> {
	let session_id = hex::decode(client_secret).map_err(|err| {
		InternalError::IntoRspcWithCause(
			ErrorCode::BadRequest,
			"Invalid session string".to_string(),
			Arc::new(err),
		)
	})?;
	let result = tokio::join!(
		redis.del::<&str, ()>(client_secret),
		db.session()
			.delete_many(vec![session::session_id::equals(session_id)])
			.exec()
	);
	result.0?;
	result.1?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::session::init;
	use error_handler::InternalResult;
	use once_cell::sync::Lazy;
	use prisma_client::{prisma::user, prisma_client_rust::serde_json};
	use redis_test::{MockCmd, MockRedisConnection};
	use std::env;
	use uuid::Uuid;

	static CLIENT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| vec![0u8; 32]);
	static USER_ID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());

	#[tokio::test]
	async fn destroy_existing_session() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");
		let db =
			prisma_client::prisma_mocked_client(env::var("DATABASE_URL_TESTS").expect("DATABASE_URL_TESTS not found"))
				.await
				.expect("Test database error");

		let user = db
			.user()
			.create("1".repeat(16), vec![user::id::set(USER_ID.clone())])
			.with(user::pii_data::fetch())
			.with(user::user_school_relation::fetch(vec![]))
			.exec()
			.await?;

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
			MockCmd::new(redis::cmd("DEL").arg(&hex::encode(CLIENT_SECRET.clone())), Ok("OK")),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let secret_string = init::session(&db, &mut mock_redis, user, &CLIENT_SECRET, None).await?;

		destroy(&db, &mut mock_redis, &secret_string).await?;
		Ok(())
	}

	#[tokio::test]
	async fn destroy_invalid_session() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");
		let db =
			prisma_client::prisma_mocked_client(env::var("DATABASE_URL_TESTS").expect("DATABASE_URL_TESTS not found"))
				.await
				.expect("Test database error");

		let mut mock_redis = MockRedisConnection::new(vec![MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK"))]);

		let result = destroy(&db, &mut mock_redis, "NOT A VALID SECRET").await;
		assert!(matches!(
			result,
			Err(InternalError::IntoRspcWithCause(ErrorCode::BadRequest, message, _)) if message == "Invalid session string"
		));
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
		Ok(())
	}

	#[tokio::test]
	async fn destroy_not_existing_session() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");
		let db =
			prisma_client::prisma_mocked_client(env::var("DATABASE_URL_TESTS").expect("DATABASE_URL_TESTS not found"))
				.await
				.expect("Test database error");

		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(redis::cmd("DEL").arg(&hex::encode(CLIENT_SECRET.clone())), Ok("OK")),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);
		let result = destroy(&db, &mut mock_redis, &hex::encode(CLIENT_SECRET.clone())).await; // Not existing session
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
		result
	}
}
