mod init;

use super::*;
use crate::tests::init::init_tests_with_user;
use backend_error_handler::{ApiError, ApiResult};
use backend_prisma_client::{
	prisma::{user, PrismaClient},
	User,
};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::sync::Arc;

#[tokio::test]
async fn init_load_destroy() -> ApiResult<()> {
	let (db, mut redis, user, connection_secret) = init_tests_with_user().await.expect("Init Failed");
	match init_load_destroy_inner(db.clone(), &mut redis, &user, &connection_secret).await {
		Ok(_) => {
			db.user().delete(user::id::equals(user.id)).exec().await?;
			Ok(())
		}
		Err(err) => {
			let _: Result<(), _> = redis.del(hex::encode(&connection_secret)).await; // Error deliberately ignored
			db.user().delete(user::id::equals(user.id)).exec().await?;
			Err(err)
		}
	}
}

async fn init_load_destroy_inner(
	db: Arc<PrismaClient>,
	redis: &mut MultiplexedConnection,
	user: &User,
	connection_secret: &Vec<u8>,
) -> ApiResult<()> {
	let client_secret = init_session(&db, redis, user, connection_secret, Some(10)).await?;

	let user = load_session(db.clone(), redis, &client_secret, Some(10)).await?;

	user.ok_or_else(|| ApiError::TestError("User is none".into()))?;

	redis.del(&client_secret).await?;

	let user = load_session(db.clone(), redis, &client_secret, Some(10)).await?;

	user.ok_or_else(|| ApiError::TestError("User wasn't successfully recovered".into()))?;

	destroy_session(&db, redis, &client_secret).await?;

	let user = load_session(db, redis, &client_secret, Some(10)).await?;

	if user.is_some() {
		return Err(ApiError::TestError("Session didn't got deleted".into()));
	}
	Ok(())
}
