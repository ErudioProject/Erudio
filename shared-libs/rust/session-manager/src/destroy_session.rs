use backend_error_handler::ApiError;
use backend_prisma_client::prisma::{session, PrismaClient};
use redis::{aio::MultiplexedConnection, AsyncCommands};

pub async fn destroy_session(
	db: &PrismaClient,
	redis: &mut MultiplexedConnection,
	client_secret: &str,
) -> Result<(), ApiError> {
	let session_id = hex::decode(client_secret)?;
	let _ = db
		.session()
		.delete(session::session_id::equals(session_id))
		.exec()
		.await?;

	redis.del(client_secret).await?;

	Ok(())
}
