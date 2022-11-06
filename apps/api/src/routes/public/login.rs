use crate::{routes::RspcResult, Ctx};
use log::debug;
use rspc::Type;

#[derive(Type, serde::Deserialize, Debug)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

#[derive(Type, serde::Serialize, Debug)]
#[serde(tag = "t", content = "c")]
#[allow(dead_code)] // TODO
pub enum LoginResponse {
	Success,
	TwoFactorAuth(TwoFactorAuthType),
}

#[derive(Type, serde::Serialize, Debug)]
#[allow(dead_code)] // TODO
pub enum TwoFactorAuthType {
	GoogleAuth,
	Sms,
	EMail,
}

// TODO
pub(crate) async fn login(_ctx: Ctx, req: LoginRequest) -> RspcResult<LoginResponse> {
	debug!("Login Request: {:?}", req);
	Ok(LoginResponse::Success)
}
