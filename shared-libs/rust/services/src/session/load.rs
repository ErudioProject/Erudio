use crate::session::{recover::recover, SessionData};
use error_handler::InternalError;
use prisma_client::{prisma::PrismaClient, prisma_client_rust::serde_json};
use redis::{AsyncCommands, JsonAsyncCommands};

pub async fn load<R: AsyncCommands + JsonAsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	client_secret: &str,
	redis_expires_seconds: Option<usize>,
) -> Result<Option<SessionData>, InternalError> {
	let json: Option<String> = redis.json_get(client_secret, "$").await?;
	if let Some(data) = json {
		if let Some(expire) = redis_expires_seconds {
			redis.expire(client_secret, expire).await?;
		}
		let mut data: Vec<SessionData> = serde_json::from_str(&data)?;
		Ok(data.pop())
	} else {
		recover(db, redis, client_secret, redis_expires_seconds).await
	}
}
