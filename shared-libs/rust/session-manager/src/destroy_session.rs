use error_handler::InternalError;
use prisma_client::prisma::{session, PrismaClient};
use redis::AsyncCommands;

pub async fn destroy_session<R: AsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	client_secret: &str,
) -> Result<(), InternalError> {
	let session_id = hex::decode(client_secret)?;
	let result = tokio::join!(
		redis.del::<&str, ()>(client_secret),
		db.session()
			.delete(session::session_id::equals(session_id))
			.exec()
	);
	result.0?;
	result.1?;
	Ok(())
}
