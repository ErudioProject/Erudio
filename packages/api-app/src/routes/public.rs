use argon2::{Config, ThreadMode, Variant, Version};
use log::debug;
use rand::RngCore;
use rspc::{ErrorCode, Router, RouterBuilder, Type};
use crate::{Ctx, GrammaticalForm};
use crate::routes::RspcResult;
use crate::prisma::{user, pii_data};

const ARGON_CONFIG: Config = Config{
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
const SALT_SIZE: usize = 16;

pub fn mount() -> RouterBuilder::<Ctx> {
    Router::<Ctx>::new()
        .query("version", |t| {
            t(|_, _: ()| -> RspcResult<String> {
                Ok(env!("CARGO_PKG_VERSION").to_string())
            })
        })
        .query("login", |t| {

            #[derive(Type, serde::Deserialize, Debug)]
            pub struct LoginRequest {
                pub email: String,
                pub password: String ,
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
                EMail
            }

            t(|_ctx: Ctx, req: LoginRequest| -> RspcResult<LoginResponse> {
                debug!("Login Request : {:?}", req);
                Ok(LoginResponse::Success)
            })
        })
        .query("register", |t| {

            #[derive(Type, serde::Deserialize, Debug)]
            pub struct RegisterRequest {
                pub email: String,
                pub password: String,
                pub code: (),
            }


            t(|ctx: Ctx, req: RegisterRequest| async move {
                debug!("Register Request : {:?}", req);
                let mut buf = [0].repeat(SALT_SIZE);
                {
                    let mut rng = rand::thread_rng();
                    rng.fill_bytes(&mut buf);
                }
                let user = ctx.db
                    .user()
                    .create(
                        argon2::hash_raw(req.password.as_bytes(), &buf, &ARGON_CONFIG)
                            .map_err(|err| rspc::Error::with_cause(ErrorCode::InternalServerError, "Argon2 error".into(), err))?,
                        false,
                        GrammaticalForm::Indeterminate,
                        vec![]
                    )
                    .exec()
                    .await.unwrap();// TODO change

                ctx.db
                    .pii_data()
                    .create(
                        user::id::equals(user.id),
                        vec![
                            pii_data::email::Set(Some(req.email)).into()
                        ]
                    )
                    .exec()
                    .await.unwrap();// TODO change

                Ok(())
            })
        })
}