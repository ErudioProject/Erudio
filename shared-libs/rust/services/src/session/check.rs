use crate::session::recover::recover;
use error_handler::InternalError;
use prisma_client::prisma::PrismaClient;
use redis::{AsyncCommands, JsonAsyncCommands};

pub async fn check<R: AsyncCommands + JsonAsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	client_secret: &str,
	redis_expires_seconds: Option<usize>,
) -> Result<bool, InternalError> {
	let exists = redis.exists(client_secret).await?;
	Ok(if exists {
		if let Some(expire) = redis_expires_seconds {
			redis.expire(client_secret, expire).await?;
		}
		true
	} else {
		recover(db, redis, client_secret, redis_expires_seconds)
			.await?
			.is_some()
	})
}
