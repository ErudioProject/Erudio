use crate::helpers::ctx::SuperAdmin;
use crate::routes::{super_admin, RspcResult};
use error_handler::InternalError;
use prisma_client::prisma;

#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct GetUserRequest {
	pub id: String,
	pub school_id: Option<String>,
}

pub async fn get_user(ctx: SuperAdmin, req: GetUserRequest) -> RspcResult<super_admin::user_full::Data> {
	ctx.db
		.user()
		.find_unique(prisma::user::UniqueWhereParam::IdEquals(req.id))
		.include(super_admin::user_full::include(vec![
			prisma::user_school_relation::school_id::equals(req.school_id.unwrap_or_default()),
		]))
		.exec()
		.await?
		.ok_or_else(|| InternalError::IntoRspc(rspc::ErrorCode::NotFound, None))
		.map_err(std::convert::Into::into)
}
