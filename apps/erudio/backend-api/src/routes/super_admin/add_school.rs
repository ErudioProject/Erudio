use crate::helpers::ctx::SuperAdmin;
use crate::helpers::IdempotenceToken;
use crate::routes::RspcResult;
use prisma_client::prisma;
use serde_json::Value;

#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct AddSchoolRequest {
	pub idempotence_token: IdempotenceToken,
	pub name: String,
}

pub async fn add_school(ctx: SuperAdmin, req: AddSchoolRequest) -> RspcResult<prisma::school::Data> {
	ctx.db
		.school()
		.create(req.name, Value::Array(vec![]), vec![])
		.exec()
		.await
		.map_err(std::convert::Into::into)
}
