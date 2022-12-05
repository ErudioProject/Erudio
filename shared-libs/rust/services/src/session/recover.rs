use crate::session::{init::init_redis, SessionData};
use chrono::Utc;
use error_handler::InternalError;
use prisma_client::{
	prisma::{session, user, PrismaClient},
	prisma_client_rust::rspc::ErrorCode,
};
use redis::{AsyncCommands, JsonAsyncCommands};

pub(crate) async fn recover<R: AsyncCommands + JsonAsyncCommands>(
	db: &PrismaClient,
	redis: &mut R,
	client_secret: &str,
	redis_expires_seconds: Option<usize>,
) -> Result<Option<SessionData>, InternalError> {
	let session_id = hex::decode(client_secret)?;
	let result = db
		.session()
		.find_unique(session::session_id::equals(session_id.clone()))
		.with(
			session::user::fetch()
				.with(user::pii_data::fetch())
				.with(user::user_school_relation::fetch(vec![])),
		)
		.exec()
		.await?;
	match result {
		Some(session::Data {
			user: Some(user),
			valid_until,
			..
		}) => {
			if valid_until.naive_utc() < Utc::now().naive_utc() {
				db.session()
					.delete(session::session_id::equals(session_id))
					.exec()
					.await?;
				return Err(InternalError::new_rspc(
					ErrorCode::Unauthorized,
					"Session timeout".to_string(),
				));
			};
			let user = *user;
			debug_assert!(user.user_school_relation.is_some());
			debug_assert!(user.pii_data.is_some());
			let data: SessionData = user.into();
			init_redis(redis, &data, client_secret, redis_expires_seconds).await?;
			Ok(Some(data))
		}
		Some(_) => unreachable!(),
		None => Ok(None),
	}
}
