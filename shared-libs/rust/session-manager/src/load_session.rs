use backend_error_handler::ApiError;
use backend_prisma_client::{
	prisma::{session, session::Data, PrismaClient},
	prisma_client_rust::{rspc::ErrorCode, serde_json},
	User,
};
use chrono::Utc;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::sync::Arc;

pub async fn load_session(
	db: Arc<PrismaClient>,
	redis: &mut MultiplexedConnection,
	client_secret: &str,
	redis_expires_seconds: Option<usize>,
) -> Result<Option<User>, ApiError> {
	let json: Option<String> = redis.get(client_secret).await?;
	match json {
		None => {
			let session_id = hex::decode(client_secret)?;
			let result = db
				.session()
				.find_unique(session::session_id::equals(session_id.clone()))
				.with(session::user::fetch())
				.exec()
				.await?;
			match result {
				Some(Data {
					user: Some(user),
					valid_until,
					..
				}) => {
					let mut r = redis.clone();
					let secret = client_secret.to_string();
					let json = serde_json::to_string(&*user)?;
					if valid_until.naive_utc() < Utc::now().naive_utc() {
						db.session()
							.delete(session::session_id::equals(session_id))
							.exec()
							.await?;
						return Err(ApiError::new_rspc(
							ErrorCode::Unauthorized,
							"Session timeout".to_string(),
						));
					};

					match redis_expires_seconds {
						None => r.set(secret, json).await?,
						Some(seconds) => r.set_ex(secret, json, seconds).await?,
					};

					Ok(Some(*user))
				}
				Some(_) => Err(ApiError::Unreachable),
				None => Ok(None),
			}
		}
		Some(json) => Ok(Some(serde_json::from_str(&json)?)),
	}
}
