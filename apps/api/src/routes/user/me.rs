use crate::routes::{AuthCtx, RspcResult};
use backend_error_handler::ApiError;
use backend_prisma_client::prisma::{user, GrammaticalForm};
use prisma_client_rust;
use rspc::{internal::specta::Type, selection, ErrorCode};
use serde::Serialize;

user::include!(user_data { pii_data }); // TODO create issue for select and include

pub(crate) async fn me(ctx: AuthCtx, _: ()) -> RspcResult<impl Type + Serialize> {
	let user = ctx
		.db
		.user()
		.find_unique(user::UniqueWhereParam::IdEquals(ctx.user.id))
		.include(user_data::include())
		.exec()
		.await?
		.ok_or_else(|| ApiError::Rspc(rspc::Error::new(ErrorCode::NotFound, "User not found".into())))?;
	Ok(selection!(user, {two_factor_auth, grammatical_form, pii_data}))
}
