use crate::routes::{AuthCtx, RspcResult};
use backend_prisma_client::prisma::user;
use rspc::ErrorCode;

user::select!(user_data {
	two_factor_auth_settings : select {
		previous_data
	}
	pii_data: select {
		grammatical_form
		email
		pesel
		birth_date
		legal_name
		display_name
		phone_prefix
		phone_number
		previous_data
	}
	user_school_relation: select {
		school: select {
			name
			previous_data
			school_settings: select {
				previous_data
			}
		}
	}
});

pub(crate) async fn me(ctx: AuthCtx, _: ()) -> RspcResult<user_data::Data> {
	ctx.db
		.user()
		.find_unique(user::id::equals(ctx.user.id))
		.select(user_data::select())
		.exec()
		.await?
		.ok_or_else(|| rspc::Error::new(ErrorCode::NotFound, "User not found".into()))
}
