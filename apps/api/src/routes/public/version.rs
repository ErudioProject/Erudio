use crate::{routes::RspcResult, Ctx};

pub(crate) async fn version(ctx: Ctx, req: ()) -> RspcResult<String> {
	Ok(env!("CARGO_PKG_VERSION").to_string()) as RspcResult<String>
}
