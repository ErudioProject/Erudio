use crate::{routes::RspcResult, Ctx};
use log::debug;
use rspc::{RouterBuilder, Type};

pub(crate) trait LoginBuilder {
	fn login_query(self, key: &'static str) -> Self;
}

impl LoginBuilder for RouterBuilder<Ctx> {
	fn login_query(self, key: &'static str) -> Self {
		self.query(key, |t| {
			#[derive(Type, serde::Deserialize, Debug)]
			pub struct LoginRequest {
				pub email: String,
				pub password: String,
			}

			#[derive(Type, serde::Serialize, Debug)]
			#[serde(tag = "t", content = "c")]
			pub enum LoginResponse {
				Success,
				TwoFactorAuth(TwoFactorAuthType),
			}

			#[derive(Type, serde::Serialize, Debug)]
			pub enum TwoFactorAuthType {
				GoogleAuth,
				SMS,
				EMail,
			}

			t(|_ctx: Ctx, req: LoginRequest| -> RspcResult<LoginResponse> {
				debug!("Login Request: {:?}", req);
				Ok(LoginResponse::Success)
			})
		})
	}
}
