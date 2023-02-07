use crate::helpers::ctx::SuperAdmin;
use crate::routes::RspcResult;
use error_handler::InternalError;
use prisma_client::prisma;

#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct GetSchoolRequest {
	pub id: String,
}

pub async fn get_school(ctx: SuperAdmin, req: GetSchoolRequest) -> RspcResult<prisma::school::Data> {
	ctx.db
		.school()
		.find_unique(prisma::school::UniqueWhereParam::IdEquals(req.id))
		.exec()
		.await?
		.ok_or_else(|| InternalError::IntoRspc(rspc::ErrorCode::NotFound, None))
		.map_err(std::convert::Into::into)
}
