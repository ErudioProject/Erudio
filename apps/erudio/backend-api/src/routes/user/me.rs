use crate::{helpers::ctx::AuthCtx, routes::RspcResult};
use prisma_client::prisma::{user, SchoolRelationType};
use rspc::{ErrorCode, Type};
use serde::Serialize;

user::select!(user_data {
	user_school_relation: select {
		school_relation_type
		school: select {
			name
		}
	}
});

#[derive(Serialize, Type)]
pub struct UserMeResponse {
	display_name: String,
	school_relations: Vec<(SchoolRelationType, String)>,
}

pub(crate) async fn me(ctx: AuthCtx, _: ()) -> RspcResult<UserMeResponse> {
	let user = ctx
		.db
		.user()
		.find_unique(user::id::equals(ctx.session_data.user.id))
		.select(user_data::select())
		.exec()
		.await?
		.ok_or_else(|| rspc::Error::new(ErrorCode::NotFound, "User not found".into()))?;
	Ok(UserMeResponse {
		display_name: ctx
			.session_data
			.user
			.pii_data
			.unwrap()
			.unwrap()
			.display_name, // there is always pii_data in session
		school_relations: user
			.user_school_relation
			.iter()
			.map(|relation| (relation.school_relation_type, relation.school.name.clone()))
			.collect(),
	})
}
