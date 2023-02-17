use crate::helpers::ctx::SuperAdmin;
use crate::helpers::IdempotenceToken;
use crate::routes::RspcResult;
use prisma_client::prisma;
use prisma_client::prisma::SchoolRelationType;

#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct AddUserToSchoolRequest {
	pub idempotence_token: IdempotenceToken,
	pub school_id: String,
	pub user_id: String,
	pub relation_type: SchoolRelationType,
}

pub async fn add_user_to_school(
	ctx: SuperAdmin,
	req: AddUserToSchoolRequest,
) -> RspcResult<prisma::user_school_relation::Data> {
	ctx.db
		.user_school_relation()
		.create(
			req.relation_type,
			prisma::user::UniqueWhereParam::IdEquals(req.user_id),
			prisma::school::UniqueWhereParam::IdEquals(req.school_id),
			vec![],
		)
		.exec()
		.await
		.map_err(std::convert::Into::into)
}
