use std::path::PathBuf;
use std::sync::Arc;
use rspc::{Config, ErrorCode};
use crate::{GrammaticalForm, PrismaClient};
use super::prisma::user::Data as User;

#[derive(Clone)]
pub struct Ctx {
    //pub(crate) db: Arc<PrismaClient>,
    //pub(crate) session_id: Option<String>,
    //cookies: Cookies,
}

#[derive(Clone)]
pub struct AuthCtx {
    db: Arc<PrismaClient>,
    user: User,
}

pub(crate) fn router() -> rspc::Router<Ctx> {
    rspc::Router::<Ctx>::new()
        .config(
            Config::new()
                // Doing this will automatically export the bindings when the `build` function is called.
                .export_ts_bindings(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/frontend/data-access-api/src/lib/frontend-data-access-api.ts"))
        )
        .query("version", |t| {
            t(|_, _: ()| {
                env!("CARGO_PKG_VERSION").to_string()
            })
        })/*.middleware(|mw| mw.middleware(|mw| async move {
            let mut old_ctx: Ctx = mw.ctx;
            //old_ctx.session_id = Some("A".to_string());
            /*match old_ctx.session_id {
                Some(ref session_id) => {
                    Ok(mw.with_ctx(AuthCtx {
                        db: old_ctx.db,
                        user: User {
                            id: "".to_string(),
                            password_hash: vec![],
                            two_factor_auth: false,
                            grammatical_form: GrammaticalForm::Masculinine,
                            pii_data: None
                        }
                    }))
                }
                None => Err(rspc::Error::new(
                    ErrorCode::Unauthorized,
                    "Unauthorized".into(),
                )),
            }*/
        }))*/
        .build()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_rspc_router() {
        router();
    }
}