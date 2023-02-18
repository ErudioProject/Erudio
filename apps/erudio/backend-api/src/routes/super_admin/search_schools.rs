use crate::helpers::ctx::SuperAdmin;
use crate::helpers::pagination::Pagination;
use crate::routes::RspcResult;
use prisma_client::prisma;

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

#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct SearchSchoolsAmountRequest {
	pub name: String,
}

pub async fn search_schools_amount(ctx: SuperAdmin, req: SearchSchoolsAmountRequest) -> RspcResult<i32> {
	ctx.db
		.school()
		.count(vec![prisma::school::name::contains(req.name)])
		.exec()
		.await
		.map(|res| res.min(i32::MAX.into()).try_into().expect("Unreachable"))
		.map_err(std::convert::Into::into)
}
