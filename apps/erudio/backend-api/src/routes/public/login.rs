use crate::cookies::get_cookie;
use crate::{
	routes::{RspcResult, SESSION_COOKIE_NAME},
	Public,
};
use error_handler::InternalError;
use log::info;
use prisma_client::prisma::{pii_data, user};
use rand::RngCore;
use rspc::{ErrorCode, Type};
use services::session;

#[serde_zod::codegen]
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
	info!("Login Request: {:?}", req);
	let user = ctx
		.db
		.user()
		.find_first(vec![user::pii_data::is(vec![pii_data::email::equals(Some(req.email))])]) // IDK why this Some is needed maybe open issue one dat
		.with(user::pii_data::fetch())
		.with(user::user_school_relation::fetch(vec![]))
		.exec()
		.await?
		.ok_or_else(|| rspc::Error::new(ErrorCode::NotFound, "Email not found".to_string()))?;

	if !argon2::verify_encoded_ext(
		&user.password_hash,
		req.password.as_bytes(),
		&ctx.config.argon2.secret,
		&[],
	)
	.map_err(Into::<InternalError>::into)?
	{
		return Err(rspc::Error::new(ErrorCode::Forbidden, "Wrong password".into()));
	}

	let mut connection_secret = vec![0; ctx.config.secret_size];
	{
		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut connection_secret);
	}

	ctx.cookies.add(get_cookie(
		SESSION_COOKIE_NAME,
		session::init::session(&ctx.db, &mut ctx.redis.clone(), user, &connection_secret, Some(3600)).await?,
	));

	Ok(LoginResponse::Success)
}
