use crate::{helpers::ctx::AuthCtx, routes::RspcResult};
use prisma_client::prisma::user;
use rspc::ErrorCode;

user::select!(user_data {
	pii_data: select {
		display_name
	}
	user_school_relation: select {
		school_relation_type
		school: select {
			name
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
