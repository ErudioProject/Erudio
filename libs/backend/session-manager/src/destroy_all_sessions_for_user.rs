use backend_error_handler::{ApiError, ApiResult};
use backend_prisma_client::{
	prisma::{session, PrismaClient},
	User,
};
use redis::AsyncCommands;
use std::{future::join, sync::Arc};
use tokio::sync::Mutex;

pub async fn destroy_all_sessions_for_user(
	db: Arc<PrismaClient>,
	redis: Arc<Mutex<redis::aio::Connection>>,
	user: User,
) -> Result<(), ApiError> {
	let sessions = db
		.session()
		.find_many(vec![session::user_id::equals(user.id.clone())])
		.exec()
		.await?;

	let result = join!(destroy_redis(&redis, sessions), destroy_db(&db, user)).await;
	result.0?;
	result.1?;
	Ok(())
}

async fn destroy_redis(redis: &Mutex<redis::aio::Connection>, sessions: Vec<session::Data>) -> ApiResult<()> {
	let session_ids = sessions
		.iter()
		.map(|s| hex::encode(s.session_id.clone()))
		.collect::<Vec<String>>();
	redis.lock().await.del(session_ids).await?;
	Ok(())
}

async fn destroy_db(db: &PrismaClient, user: User) -> ApiResult<()> {
	db.session()
		.delete_many(vec![session::user_id::equals(user.id)])
		.exec()
		.await?;
	Ok(())
}
