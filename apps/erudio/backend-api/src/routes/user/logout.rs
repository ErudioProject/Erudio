use crate::{
	helpers::{config::SESSION_COOKIE_NAME, ctx::Auth},
	routes::RspcResult,
};
use cookie::Cookie;
use error_handler::InternalError;
use services::session;

pub async fn logout(ctx: Auth, _: ()) -> RspcResult<()> {
	session::destroy(&ctx.db, &mut ctx.redis.clone(), &ctx.session_id)
		.await
		.map_err(Into::<InternalError>::into)?;
	ctx.cookies.remove(Cookie::new(SESSION_COOKIE_NAME, ""));
	Ok(())
}
