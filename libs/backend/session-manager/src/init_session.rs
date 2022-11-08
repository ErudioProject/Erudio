use backend_error_handler::{ApiError, ApiResult};
use backend_prisma_client::{
	prisma::{user, PrismaClient},
	serde_json,
};
use chrono::{DateTime, Duration, Utc};
use redis::AsyncCommands;
use std::future::join;
use tokio::sync::Mutex;

pub async fn init_session(
	db: &PrismaClient,
	redis: &Mutex<redis::aio::Connection>,
	user: &user::Data,
	client_secret: &Vec<u8>,
) -> Result<String, ApiError> {
	let json = serde_json::to_string(&user)?;
	let redis_async = init_redis(redis, client_secret, json);
	let prisma_async = init_prisma(db, client_secret, &user.id);
	let result = join!(redis_async, prisma_async).await;
	result.0?;
	result.1?;
	Ok(hex::encode(client_secret))
}

async fn init_redis(redis: &Mutex<redis::aio::Connection>, client_secret: &Vec<u8>, json: String) -> ApiResult<()> {
	let mut conn = redis.lock().await;
	conn.set(client_secret, json).await?;
	Ok(())
}

async fn init_prisma(db: &PrismaClient, client_secret: &Vec<u8>, id: &String) -> ApiResult<()> {
	db.session()
		.create(
			user::id::equals(id.clone()),
			client_secret.clone(),
			DateTime::from(Utc::now() + Duration::days(365)),
			vec![],
		)
		.exec()
		.await?;
	Ok(())
}
