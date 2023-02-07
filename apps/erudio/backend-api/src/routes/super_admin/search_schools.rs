use crate::helpers::ctx::SuperAdmin;
use crate::helpers::pagination::Pagination;
use crate::routes::RspcResult;
use prisma_client::prisma;

#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct SearchSchoolsRequest {
	pub page: Option<Pagination>,
	pub name: String,
}

pub async fn search_schools(ctx: SuperAdmin, req: SearchSchoolsRequest) -> RspcResult<Vec<prisma::school::Data>> {
	let (skip, take) = req.page.unwrap_or_default().unpack(&ctx.config);
	ctx.db
		.school()
		.find_many(vec![prisma::school::name::contains(req.name)])
		.skip(skip)
		.take(take)
		.exec()
		.await
		.map_err(std::convert::Into::into)
}
