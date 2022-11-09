use crate::routes::{AuthCtx, RspcResult};
use backend_error_handler::ApiError;
use backend_prisma_client::prisma::{user, GrammaticalForm};
use rspc::{internal::specta::Type, selection, ErrorCode};
use serde::Serialize;

user::select!(user_data {
	two_factor_auth
	grammatical_form
	pii_data: select {
	email
	pesel
	birth_date
	legal_name
	display_name
	phone_prefix
	phone_number
	}
});

pub(crate) async fn me(ctx: AuthCtx, _: ()) -> RspcResult<user_data::Data> {
	let user = ctx
		.db
		.user()
		.find_unique(user::UniqueWhereParam::IdEquals(ctx.user.id))
		.select(user_data::select())
		.exec()
		.await?
		.ok_or_else(|| ApiError::Rspc(rspc::Error::new(ErrorCode::NotFound, "User not found".into())))?;
	Ok(user)
}
