use backend_error_handler::{ApiError, ApiResult};
use backend_prisma_client::{
	prisma::{user, PrismaClient},
	prisma_client_rust::serde_json,
};
use chrono::{DateTime, Duration, Utc};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::future::join;

pub async fn init_session(
	db: &PrismaClient,
	redis: &mut MultiplexedConnection,
	user: &user::Data,
	client_secret: &Vec<u8>,
	redis_expires_seconds: Option<usize>,
) -> Result<String, ApiError> {
	let json = serde_json::to_string(&user)?;
	let encoded = hex::encode(client_secret);
	let redis_async = init_redis(redis, &encoded, json, redis_expires_seconds);
	let prisma_async = init_prisma(db, client_secret, &user.id);
	let result = join!(redis_async, prisma_async).await;
	result.0?;
	result.1?;
	Ok(encoded)
}

async fn init_redis(
	redis: &mut MultiplexedConnection,
	client_secret: &String,
	json: String,
	expires: Option<usize>,
) -> ApiResult<()> {
	match expires {
		None => redis.set(client_secret, json).await?,
		Some(time) => redis.set_ex(client_secret, json, time).await?,
	};
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
