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
) -> Result<i64, InternalError> {
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
	let deleted = result.1?;
	Ok(deleted)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::session::init;
	use chrono::{DateTime, Duration, DurationRound, Utc};
	use error_handler::InternalResult;
	use init::tests::USER;
	use once_cell::sync::Lazy;
	use prisma_client::prisma;
	use prisma_client::prisma::user;
	use prisma_client::prisma_client_rust::serde_json;
	use redis_test::{MockCmd, MockRedisConnection};
	static CLIENT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| vec![0u8; 32]);
	//static USER_ID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());

	#[tokio::test]
	async fn destroy_existing_session() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");
		let (db, mock) = PrismaClient::_mock();

		mock.expect(
			// TODO maybe refactor
			db.session().create(
				CLIENT_SECRET.clone(),
				DateTime::from(Utc::now().duration_round(Duration::days(1)).unwrap() + Duration::days(365)),
				user::id::equals(USER.id.clone()),
				vec![],
			),
			prisma::session::Data {
				user_id: String::new(),
				session_id: vec![],
				valid_until: chrono::DateTime::default(),
				user: Some(Box::new(USER.clone())),
			},
		)
		.await;
		mock.expect(
			db.session()
				.delete_many(vec![session::session_id::equals(CLIENT_SECRET.clone())]),
			1i64,
		)
		.await;

		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(
				redis::cmd("JSON.SET")
					.arg(&hex::encode(CLIENT_SECRET.clone()))
					.arg("$")
					.arg(
						serde_json::to_string(&crate::session::Info::try_from(USER.clone()).unwrap())
							.unwrap()
							.replace(&USER.password_hash, ""),
					),
				Ok("OK"),
			),
			MockCmd::new(redis::cmd("DEL").arg(&hex::encode(CLIENT_SECRET.clone())), Ok("OK")),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);

		let secret_string = init::session(&db, &mut mock_redis, USER.clone(), &CLIENT_SECRET, None).await?;

		destroy(&db, &mut mock_redis, &secret_string).await?;
		Ok(())
	}

	#[tokio::test]
	async fn destroy_invalid_session() -> InternalResult<()> {
		dotenvy::dotenv().expect(".env file loading error");
		let (db, _) = PrismaClient::_mock();
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
		let (db, mock) = PrismaClient::_mock();

		mock.expect(
			db.session()
				.delete_many(vec![session::session_id::equals(CLIENT_SECRET.clone())]),
			0i64,
		)
		.await;

		let mut mock_redis = MockRedisConnection::new(vec![
			MockCmd::new(redis::cmd("DEL").arg(&hex::encode(CLIENT_SECRET.clone())), Ok("OK")),
			MockCmd::new(redis::cmd("GET").arg("last"), Ok("OK")),
		]);
		let result = destroy(&db, &mut mock_redis, &hex::encode(CLIENT_SECRET.clone())).await; // Not existing session
		assert_eq!(mock_redis.get("last").await, Ok("OK".to_string()));
		assert!(matches!(result, Ok(0)));
		Ok(())
	}
}
