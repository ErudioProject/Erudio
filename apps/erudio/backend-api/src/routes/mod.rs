pub mod file;
pub mod public;
pub mod user;

use crate::helpers::{
	consts::SESSION_COOKIE_NAME,
	ctx::{Auth, Public},
};
use rspc::{Config, ErrorCode};
use services::session;

pub type RspcResult<T> = Result<T, rspc::Error>;

pub fn router() -> rspc::Router<Public> {
	rspc::Router::<Public>::new()
		.config(Config::new())
		.merge("public.", public::mount())
		.middleware(|mw| {
			mw.middleware(|mw| async move {
				let mut old_ctx: Public = mw.ctx.clone();
				match old_ctx.cookies.get(SESSION_COOKIE_NAME) {
					None => Err(rspc::Error::new(ErrorCode::Unauthorized, "Unauthorized".into())),
					Some(session_id) => {
						match session::load(&old_ctx.db, &mut old_ctx.redis, session_id.value(), Some(3600)).await? {
							None => Err(rspc::Error::new(ErrorCode::Unauthorized, "Unauthorized".into())),
							Some(session_data) => Ok(mw.with_ctx(Auth {
								config: old_ctx.config,
								db: old_ctx.db,
								redis: old_ctx.redis,
								session_id: session_id.value().to_string(),
								cookies: old_ctx.cookies,
								session_data,
							})),
						}
					}
				}
			})
		})
		.merge("user.", user::mount())
		.merge("file.", file::mount())
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
