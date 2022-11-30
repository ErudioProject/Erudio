use crate::{
	routes::{public::SECRET_SIZE, RspcResult, SESSION_COOKIE_NAME},
	Ctx,
};
use cookie::{Cookie, SameSite};
use log::debug;
use prisma_client::prisma::{pii_data, user};
use rand::RngCore;
use rspc::{ErrorCode, Type};
use session_manager::init_session;

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

pub(crate) async fn login(ctx: Ctx, req: LoginRequest) -> RspcResult<LoginResponse> {
	debug!("Login Request: {:?}", req);
	let mut connection_secret = vec![0; SECRET_SIZE];
	{
		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut connection_secret);
	}
	let user = ctx
		.db
		.user()
		.find_first(vec![user::pii_data::is(vec![pii_data::email::equals(Some(req.email))])]) // IDK why this Some is needed maybe open issue one dat
		.with(user::pii_data::fetch())
		.exec()
		.await?
		.ok_or_else(|| rspc::Error::new(ErrorCode::NotFound, "Email not found".to_string()))?;

	//TODO 2fa handling
	ctx.cookies.add(
		Cookie::build(
			SESSION_COOKIE_NAME,
			init_session(&ctx.db, &mut ctx.redis.clone(), &user, &connection_secret, None).await?,
		)
		.secure(false) // TODO change one we have ssl set up
		.http_only(true)
		.same_site(SameSite::Strict)
		.finish(),
	);

	Ok(LoginResponse::Success)
}
