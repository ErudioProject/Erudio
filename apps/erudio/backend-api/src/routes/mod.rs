mod public;
mod user;

use crate::helpers::{
	consts::SESSION_COOKIE_NAME,
	ctx::{AuthCtx, Ctx},
};
use rspc::{Config, ErrorCode};
use services::session;
use std::path::PathBuf;

pub type RspcResult<T> = Result<T, rspc::Error>;

pub(crate) fn router() -> rspc::Router<Ctx> {
	rspc::Router::<Ctx>::new()
		.config(
			Config::new()
				// Doing this will automatically export the bindings when the `build` function is called.
				.export_ts_bindings(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bindings.ts")),
		)
		.merge("public.", public::mount())
		.middleware(|mw| {
			mw.middleware(|mw| async move {
				let old_ctx: Ctx = mw.ctx.clone();
				match old_ctx.cookies.get(SESSION_COOKIE_NAME) {
					Some(session_id) => {
						match session::load(
							&old_ctx.db.clone(),
							&mut old_ctx.redis.clone(),
							session_id.value(),
							None,
						)
						.await?
						{
							Some(user) => Ok(mw.with_ctx(AuthCtx {
								db: old_ctx.db,
								redis: old_ctx.redis,
								session_id: session_id.value().to_string(),
								cookies: old_ctx.cookies,
								user,
							})),
							None => Err(rspc::Error::new(ErrorCode::Unauthorized, "Unauthorized".into())),
						}
					}
					None => Err(rspc::Error::new(ErrorCode::Unauthorized, "Unauthorized".into())),
				}
			})
		})
		.merge("user.", user::mount())
		.build()
}

#[cfg(test)]
mod tests {
	use crate::router;

	#[test]
	fn test_rspc_router() {
		router();
	}
}
