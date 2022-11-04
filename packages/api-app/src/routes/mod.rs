mod public;

use std::path::PathBuf;
use std::sync::Arc;
use rspc::{Config, ErrorCode};
use tower_cookies::Cookies;
use crate::{Cookie, GrammaticalForm, PrismaClient};
use super::prisma::user::Data as User;

pub type RspcResult<T> = Result<T, rspc::Error>;

#[derive(Clone)]
pub struct Ctx {
    pub(crate) db: Arc<PrismaClient>,
    pub(crate) cookies: Cookies,
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
        .merge("public.", public::mount())
        .middleware(|mw| mw.middleware(|mw| async move {
            let mut old_ctx: Ctx = mw.ctx.clone();
            match old_ctx.cookies.get("SessionId") {
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
            }
        }))
        .build()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_rspc_router() {
        router();
    }
}