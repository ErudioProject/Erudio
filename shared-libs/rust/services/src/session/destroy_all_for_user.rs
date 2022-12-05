use error_handler::{InternalError, InternalResult};
use prisma_client::prisma::{session, PrismaClient};
use redis::AsyncCommands;
use tokio::join;

pub async fn destroy_all_for_user<R: AsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	user_id: String,
) -> Result<(), InternalError> {
	let sessions = db
		.session()
		.find_many(vec![session::user_id::equals(user_id.clone())])
		.exec()
		.await?;

	let result = join!(destroy_redis(redis, &sessions), destroy_db(db, user_id));
	result.0?;
	result.1?;
	Ok(())
}

async fn destroy_redis<R: AsyncCommands>(redis: &mut R, sessions: &[session::Data]) -> InternalResult<()> {
	let session_ids = sessions
		.iter()
		.map(|s| hex::encode(&s.session_id))
		.collect::<Vec<String>>();
	redis.del(session_ids).await?;
	Ok(())
}

async fn destroy_db(db: &PrismaClient, user_id: String) -> InternalResult<()> {
	db.session()
		.delete_many(vec![session::user_id::equals(user_id)])
		.exec()
		.await?;
	Ok(())
}
