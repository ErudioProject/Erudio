use crate::helpers::ctx::SuperAdmin;
use crate::routes::RspcResult;
use error_handler::InternalError;
use prisma_client::prisma;
#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct UpdateSchoolRequest {
	pub idempotence_token: String,
	pub id: String,
	pub name: Option<String>,
}

pub async fn update_school(ctx: SuperAdmin, req: UpdateSchoolRequest) -> RspcResult<prisma::school::Data> {
	ctx.db
		._transaction()
		.run(|db| async move {
			let mut school = db
				.school()
				.find_unique(prisma::school::UniqueWhereParam::IdEquals(req.id.clone()))
				.exec()
				.await?
				.ok_or_else(|| InternalError::IntoRspc(rspc::ErrorCode::NotFound, None))?;

			school.previous_data.clear();
			let previous_data = serde_json::to_value(&school).map_err(Into::<InternalError>::into)?;

			let school = db
				.school()
				.upsert(
					prisma::school::UniqueWhereParam::IdEquals(req.id),
					prisma::school::create(req.name.clone().unwrap_or_else(|| school.name.clone()), vec![]),
					vec![
						prisma::school::name::set(req.name.unwrap_or(school.name)),
						prisma::school::previous_data::push(vec![previous_data]),
					],
				)
				.exec()
				.await
				.map_err(Into::<InternalError>::into)?;

			Ok(school)
		})
		.await
}
