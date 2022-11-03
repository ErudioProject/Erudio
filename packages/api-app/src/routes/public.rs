use argon2::{Config, ThreadMode, Variant, Version};
use log::debug;
use prisma_client_rust::QueryError::Deserialize;
use rspc::{Router, RouterBuilder, Type};
use crate::{Ctx, eyre};
use crate::routes::RspcResult;

static ARGON_CONFIG: Config = Config{
    variant: Variant::Argon2i,
    version: Version::Version13,
    mem_cost: 16384,
    time_cost: 3,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32
};

pub fn mount() -> RouterBuilder::<Ctx> {
    Router::<Ctx>::new()
        .query("version", |t| {
            t(|ctx: Ctx, _: ()| -> RspcResult<String> {
                Ok(env!("CARGO_PKG_VERSION").to_string())
            })
        })
        .query("login", |t| {

            #[derive(Type, serde::Deserialize, Debug)]
            pub struct LoginRequest {
                pub email: String,
                pub password: String ,
            }

            t(|ctx: Ctx, req: LoginRequest| -> RspcResult<()> {
                debug!("Login Request : {:?}", req);
                Ok(())
            })
        })
        .query("register", |t| {

            #[derive(Type, serde::Deserialize, Debug)]
            pub struct RegisterRequest {
                pub email: String,
                pub password: String,
                pub code: (),
            }

            #[derive(Type, serde::Serialize, Debug)]
            #[serde(tag = "t", content = "c")]
            pub enum RegisterResponse {
                Success,
                TwoFactorAuth(TwoFactorAuthType),
            }

            #[derive(Type, serde::Serialize, Debug)]
            pub enum TwoFactorAuthType {
                GoogleAuth,
                SMS,
                EMail
            }


            t(|ctx: Ctx, req: RegisterRequest| -> RspcResult<RegisterResponse> {
                debug!("Register Request : {:?}", req);
                let rng = rand::thread_rng();
                Ok(RegisterResponse::Success)
            })
        })
}