use crate::routes::{AuthCtx, RspcResult};
use backend_error_handler::ApiError;
use backend_prisma_client::prisma::user;
use rspc::ErrorCode;

user::select!(user_data {
	two_factor_auth
	pii_data: select {
		grammatical_form
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
