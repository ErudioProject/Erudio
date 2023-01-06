use crate::{helpers::ctx::Auth, routes::RspcResult};
use error_handler::InternalError;
use prisma_client::prisma::{user, SchoolRelationType};
use rspc::{ErrorCode, Type};
use serde::Serialize;

user::select!(user_data {
	id
	pii_data: select {
		display_name
	}
	user_school_relation: select {
		school_relation_type
		school: select {
			name
		}
	}
});

#[derive(Serialize, Type)]
pub struct UserMeResponse {
	id: String,
	display_name: String,
	school_relations: Vec<(SchoolRelationType, String)>,
}

pub async fn me(ctx: Auth, _: ()) -> RspcResult<UserMeResponse> {
	ctx.db
		.user()
		.find_unique(user::id::equals(ctx.session_data.user.id))
		.select(user_data::select())
		.exec()
		.await?
		.map(|user| UserMeResponse {
			id: user.id,
			display_name: ctx
				.session_data
				.user
				.pii_data
				.unwrap() // pii_data was fetched
				.expect("Invalid user in db") // there is always pii_data in user
				.display_name,
			school_relations: user
				.user_school_relation
				.iter()
				.map(|relation| (relation.school_relation_type, relation.school.name.clone()))
				.collect(),
		})
		.ok_or_else(|| InternalError::IntoRspc(ErrorCode::NotFound, "User not found".into()).into())
}
