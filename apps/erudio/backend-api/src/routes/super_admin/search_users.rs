use crate::helpers::ctx::SuperAdmin;
use crate::helpers::pagination::Pagination;
use crate::routes::{super_admin, RspcResult};
use prisma_client::prisma;

#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct SearchUsersRequest {
	pub page: Option<Pagination>,
	pub school_id: String,
	pub query: String,
}

pub async fn search_users(ctx: SuperAdmin, req: SearchUsersRequest) -> RspcResult<Vec<super_admin::user_full::Data>> {
	let (skip, take) = req.page.unwrap_or_default().unpack(&ctx.config);
	ctx.db
		.user()
		.find_many(vec![prisma::user::WhereParam::And(vec![
			prisma::user::user_school_relation::some(vec![prisma::user_school_relation::school_id::equals(
				req.school_id.clone(),
			)]),
			prisma::user::pii_data::is(vec![prisma::pii_data::WhereParam::Or(vec![
				prisma::pii_data::legal_name::contains(req.query.clone()),
				prisma::pii_data::email::contains(req.query),
			])]),
		])])
		.skip(skip)
		.take(take)
		.select(super_admin::user_full::select(vec![
			prisma::user_school_relation::school_id::equals(req.school_id),
		]))
		.exec()
		.await
		.map_err(std::convert::Into::into)
}

#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct SearchUsersAmountRequest {
	pub school_id: String,
	pub query: String,
}

pub async fn search_users_amount(ctx: SuperAdmin, req: SearchUsersAmountRequest) -> RspcResult<i32> {
	ctx.db
		.user()
		.count(vec![prisma::user::WhereParam::And(vec![
			prisma::user::user_school_relation::some(vec![prisma::user_school_relation::school_id::equals(
				req.school_id.clone(),
			)]),
			prisma::user::pii_data::is(vec![prisma::pii_data::WhereParam::Or(vec![
				prisma::pii_data::legal_name::contains(req.query.clone()),
				prisma::pii_data::email::contains(req.query),
			])]),
		])])
		.exec()
		.await
		.map(|res| res.min(i32::MAX.into()).try_into().expect("Unreachable"))
		.map_err(std::convert::Into::into)
}
