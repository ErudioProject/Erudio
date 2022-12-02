use crate::{helpers::ctx::AuthCtx, routes::RspcResult};
use session_manager::destroy_session;

pub(crate) async fn logout(ctx: AuthCtx, _: ()) -> RspcResult<()> {
	destroy_session(&ctx.db, &mut ctx.redis.clone(), &ctx.session_id)
		.await
		.map_err(Into::into)
}
