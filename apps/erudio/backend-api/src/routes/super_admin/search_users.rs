use crate::helpers::ctx::SuperAdmin;
use crate::helpers::pagination::Pagination;
use crate::routes::{super_admin, RspcResult};
use prisma_client::prisma;

#[serde_zod::codegen]
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
		.include(super_admin::user_full::include(vec![
			prisma::user_school_relation::school_id::equals(req.school_id),
		]))
		.exec()
		.await
		.map_err(std::convert::Into::into)
}
