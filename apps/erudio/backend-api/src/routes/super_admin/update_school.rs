use crate::helpers::ctx::SuperAdmin;
use crate::routes::RspcResult;
use color_eyre::eyre::eyre;
use error_handler::InternalError;
use prisma_client::prisma;
use serde_json::Value;
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

			if !school.previous_data.is_array() {
				if school.previous_data.is_null() {
					school.previous_data = Value::Array(vec![]);
				}
				return Err(InternalError::ServerError(
					"DB error".to_string(),
					eyre!("previus data filed is not null for {:?}", school),
				)
				.into());
			}
			let mut previous_data_vec: Vec<Value> = school
				.previous_data
				.as_array_mut()
				.expect("Checked above")
				.drain(..)
				.collect();

			let previous_data = serde_json::to_value(&school).map_err(Into::<InternalError>::into)?;
			previous_data_vec.push(previous_data);

			let school = db
				.school()
				.upsert(
					prisma::school::UniqueWhereParam::IdEquals(req.id),
					prisma::school::create(
						req.name.clone().unwrap_or_else(|| school.name.clone()),
						Value::Array(vec![]),
						vec![],
					),
					vec![
						prisma::school::name::set(req.name.unwrap_or(school.name)),
						prisma::school::previous_data::set(Value::Array(previous_data_vec)),
					],
				)
				.exec()
				.await
				.map_err(Into::<InternalError>::into)?;

			Ok(school)
		})
		.await
}
