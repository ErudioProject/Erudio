use crate::{routes::RspcResult, Ctx};
use rspc::RouterBuilder;

pub(crate) trait VersionBuilder {
	fn version_query(self, key: &'static str) -> Self;
}

impl VersionBuilder for RouterBuilder<Ctx> {
	fn version_query(self, key: &'static str) -> Self {
		self.query(key, |t| {
			t(|_: Ctx, _: ()| async move { Ok(env!("CARGO_PKG_VERSION").to_string()) as RspcResult<String> })
		})
	}
}
