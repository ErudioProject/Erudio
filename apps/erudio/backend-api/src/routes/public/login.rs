use crate::{
	helpers::consts::SECRET_SIZE,
	routes::{RspcResult, SESSION_COOKIE_NAME},
	Public,
};
use cookie::{Cookie, SameSite};
use error_handler::InternalError;
use log::debug;
use prisma_client::prisma::{pii_data, user};
use rand::RngCore;
use rspc::{ErrorCode, Type};
use services::session;

#[derive(Type, serde::Deserialize, Debug)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

#[derive(Type, serde::Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum LoginResponse {
	Success,
	#[allow(dead_code)] // TODO
	TwoFactorAuth(TwoFactorAuthType),
}

#[derive(Type, serde::Serialize, Debug)]
#[allow(dead_code)] // TODO
pub enum TwoFactorAuthType {
	GoogleAuth,
	Sms,
	EMail,
}

pub async fn login(ctx: Public, req: LoginRequest) -> RspcResult<LoginResponse> {
	debug!("Login Request: {:?}", req);
	let user = ctx
		.db
		.user()
		.find_first(vec![user::pii_data::is(vec![pii_data::email::equals(Some(req.email))])]) // IDK why this Some is needed maybe open issue one dat
		.with(user::pii_data::fetch())
		.with(user::user_school_relation::fetch(vec![]))
		.exec()
		.await?
		.ok_or_else(|| rspc::Error::new(ErrorCode::NotFound, "Email not found".to_string()))?;

	if !argon2::verify_encoded_ext(&user.password_hash, req.password.as_bytes(), &ctx.argon_secret, &[])
		.map_err(Into::<InternalError>::into)?
	{
		return Err(rspc::Error::new(ErrorCode::Forbidden, "Wrong password".into()));
	}

	let mut connection_secret = vec![0; SECRET_SIZE];
	{
		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut connection_secret);
	}

	//TODO 2fa handling
	ctx.cookies.add(
		Cookie::build(
			SESSION_COOKIE_NAME,
			session::init::session(&ctx.db, &mut ctx.redis.clone(), user, &connection_secret, Some(3600)).await?,
		)
		.secure(false) // TODO change one we have ssl set up
		.http_only(true)
		.same_site(SameSite::Strict)
		.finish(),
	);

	Ok(LoginResponse::Success)
}
