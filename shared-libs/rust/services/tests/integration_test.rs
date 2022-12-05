mod commons;

use crate::commons::init_tests_with_user;
use error_handler::{InternalError, InternalResult};
use prisma_client::{
	prisma::{user, PrismaClient},
	User,
};
use redis::AsyncCommands;
use services::session;

#[tokio::test]
async fn init_load_destroy() -> InternalResult<()> {
	let (db, mut redis, user, connection_secret) = init_tests_with_user().await.expect("Init Failed");
	match init_load_destroy_inner(&db, &mut redis, user.clone(), &connection_secret).await {
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

async fn init_load_destroy_inner<C: AsyncCommands>(
	db: &PrismaClient,
	redis: &mut C,
	user: User,
	connection_secret: &[u8],
) -> InternalResult<()> {
	let client_secret = session::init(db, redis, user, connection_secret, Some(10)).await?;

	let user = session::load(db, redis, &client_secret, Some(10)).await?;

	user.ok_or_else(|| InternalError::TestError("User is none".into()))?;

	redis.del(&client_secret).await?;

	let user = session::load(db, redis, &client_secret, Some(10)).await?;

	user.ok_or_else(|| InternalError::TestError("User wasn't successfully recovered".into()))?;

	session::destroy(db, redis, &client_secret).await?;

	let user = session::load(db, redis, &client_secret, Some(10)).await?;

	if user.is_some() {
		return Err(InternalError::TestError("Session didn't got deleted".into()));
	}
	Ok(())
}
