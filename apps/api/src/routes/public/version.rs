use crate::{routes::RspcResult, Ctx};

pub(crate) async fn version(_: Ctx, _: ()) -> RspcResult<String> {
	Ok(env!("CARGO_PKG_VERSION").to_string())
}
