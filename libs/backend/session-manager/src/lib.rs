#![feature(future_join)]
#![forbid(unsafe_code)]

mod destroy_all_sessions_for_user;
mod init_session;

pub use destroy_all_sessions_for_user::destroy_all_sessions_for_user;
pub use init_session::init_session;

use backend_error_handler::ApiError;
use backend_prisma_client::{
	prisma::{session, session::Data, PrismaClient},
	serde_json, User,
};
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn load_session(
	db: Arc<PrismaClient>,
	redis: Arc<Mutex<redis::aio::Connection>>,
	client_secret: &str,
) -> Result<Option<User>, ApiError> {
	let session_id = hex::decode(client_secret)?;
	let json: Option<String> = {
		let mut conn = redis.lock().await;
		conn.get(&session_id).await?
	};

	match json {
		None => {
			let result = db
				.session()
				.find_unique(session::session_id::equals(session_id.clone()))
				.with(session::user::fetch())
				.exec()
				.await?;
			match result {
				Some(Data { user: Some(user), .. }) => Ok(Some(*user)),
				Some(_) => Err(ApiError::Unreachable),
				None => Ok(None),
			}
		}
		Some(json) => Ok(Some(serde_json::from_str(&json)?)),
	}
}

pub async fn destroy_session(
	db: Arc<PrismaClient>,
	redis: Arc<Mutex<redis::aio::Connection>>,
	client_secret: &str,
) -> Result<(), ApiError> {
	let session_id = hex::decode(client_secret)?;
	let deleted = db
		.session()
		.delete(session::session_id::equals(session_id))
		.exec()
		.await?;
	{
		redis.lock().await.del(deleted.session_id).await?
	}
	Ok(())
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
