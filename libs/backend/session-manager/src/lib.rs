use backend_error_handler::ApiError;
use backend_prisma_client::{
	prisma::{user, PrismaClient},
	serde_json, User,
};
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_session(
	db: Arc<PrismaClient>,
	redis: Arc<Mutex<redis::aio::Connection>>,
	user: user::Data,
	client_secret: Vec<u8>,
) -> Result<String, ApiError> {
	let json = serde_json::to_string(&user).map_err(Into::<ApiError>::into)?;
	{
		let mut conn = redis.lock().await;
		conn.set_ex(&client_secret, json, 60 * 60)
			.await
			.map_err(Into::<ApiError>::into)?;
		conn.lpush(user.id, client_secret.clone())
			.await
			.map_err(Into::<ApiError>::into)?; // Reverse record for fast logout
	}
	Ok(hex::encode(client_secret))
}
pub async fn load_session(
	db: Arc<PrismaClient>,
	redis: Arc<Mutex<redis::aio::Connection>>,
	client_secret: &str,
) -> Result<Option<User>, ApiError> {
	let session_id = hex::decode(client_secret).map_err(Into::<ApiError>::into)?;
	let json: Option<String> = {
		let mut conn = redis.lock().await;
		conn.get(session_id).await.map_err(Into::<ApiError>::into)?
	};

	match json {
		None => Ok(None),
		Some(json) => Ok(Some(serde_json::from_str(&json).map_err(Into::<ApiError>::into)?)),
	}
}

pub fn backend_session_manager() -> String {
	"backend_session_manager".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(backend_session_manager(), "backend_session_manager".to_string());
	}
}
