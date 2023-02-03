use crate::cookies::get_cookie;
use crate::helpers::argon::get_argon_config;
use crate::{
	helpers::consts::{SALT_SIZE, SECRET_SIZE},
	routes::{RspcResult, SESSION_COOKIE_NAME},
	Public,
};
use error_handler::InternalError;
use log::debug;
use prisma_client::prisma::{pii_data, user, GrammaticalForm};
use rand::RngCore;
use rspc::{ErrorCode, Type};
use services::session;

#[derive(Type, serde::Deserialize, Debug)]
pub struct RegisterRequest {
	pub idempotence_token: String,
	pub email: String,
	pub password: String,
	pub first_name: String,
	pub middle_name: Option<String>,
	pub last_name: String,
	pub code: (),
}

pub async fn register(ctx: Public, req: RegisterRequest) -> RspcResult<()> {
	debug!("Register Request : {:?}", req);
	if req.password.len() > 1024 {
		return Err(rspc::Error::new(
			ErrorCode::BadRequest,
			"Max password length is 1024 characters".into(),
		));
	}
	let argon_config = get_argon_config(&ctx.argon_secret);
	let mut salt = vec![0; SALT_SIZE];
	let mut connection_secret = vec![0; SECRET_SIZE];
	{
		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut salt);
		rng.fill_bytes(&mut connection_secret);
	}
	// TODO nested create

	let legal_name =
		req.first_name.clone() + " " + &(req.middle_name.map_or_else(String::new, |name| name + " ")) + &req.last_name;
	let display_name = req.first_name + " " + &req.last_name;

	let user = ctx
		.db
		._transaction()
		.run(|db| async move {
			let user = db
				.pii_data()
				.find_many(vec![pii_data::email::equals(Some(req.email.clone()))])
				.exec()
				.await?;

			if !user.is_empty() {
				return Err(InternalError::IntoRspc(
					rspc::ErrorCode::Conflict,
					"E-Mail Already in use".into(),
				));
			}

			let user = db
				.user()
				.create(
					argon2::hash_encoded(req.password.as_bytes(), &salt, &argon_config)
						.map_err(Into::<InternalError>::into)?,
					vec![],
				)
				.with(user::user_school_relation::fetch(vec![]))
				.exec()
				.await?;
			db.pii_data()
				.create(
					GrammaticalForm::Indeterminate,
					legal_name,
					display_name,
					user::id::equals(user.id.clone()),
					vec![pii_data::email::Set(Some(req.email)).into()],
				)
				.exec()
				.await
				.map_err(Into::<InternalError>::into)
				.map(|pii_data| user::Data {
					pii_data: Some(Some(Box::new(pii_data))),
					..user
				})
		})
		.await?;

	ctx.cookies.add(get_cookie(
		SESSION_COOKIE_NAME,
		session::init::session(&ctx.db, &mut ctx.redis.clone(), user, &connection_secret, Some(3600)).await?,
	));

	Ok(())
}
