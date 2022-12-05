use crate::{
	helpers::{consts::SESSION_COOKIE_NAME, ctx::AuthCtx},
	routes::RspcResult,
};
use cookie::Cookie;
use error_handler::InternalError;
use services::session;

pub(crate) async fn logout(ctx: AuthCtx, _: ()) -> RspcResult<()> {
	session::destroy(&ctx.db, &mut ctx.redis.clone(), &ctx.session_id)
		.await
		.map_err(Into::<InternalError>::into)?;
	ctx.cookies.remove(Cookie::new(SESSION_COOKIE_NAME, ""));
	Ok(())
}
