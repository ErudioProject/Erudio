use backend_error_handler::{InternalError, InternalResult};
use backend_prisma_client::{
	prisma::{session, PrismaClient},
	User,
};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use tokio::join;

pub async fn destroy_all_sessions_for_user(
	db: &PrismaClient,
	redis: &mut MultiplexedConnection,
	user: &User,
) -> Result<(), InternalError> {
	let sessions = db
		.session()
		.find_many(vec![session::user_id::equals(user.id.clone())])
		.exec()
		.await?;

	let result = join!(destroy_redis(redis, &sessions), destroy_db(db, user));
	result.0?;
	result.1?;
	Ok(())
}

async fn destroy_redis(redis: &mut MultiplexedConnection, sessions: &[session::Data]) -> InternalResult<()> {
	let session_ids = sessions
		.iter()
		.map(|s| hex::encode(s.session_id.clone()))
		.collect::<Vec<String>>();
	redis.del(session_ids).await?;
	Ok(())
}

async fn destroy_db(db: &PrismaClient, user: &User) -> InternalResult<()> {
	db.session()
		.delete_many(vec![session::user_id::equals(user.id.clone())])
		.exec()
		.await?;
	Ok(())
}
