use config::Config;
use prisma_client_rust::bigdecimal::Zero;
use prisma_client_rust::rspc;

#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug, Default)]
pub struct Pagination {
	pub skip: i32,
	pub take: i32,
}

impl Pagination {
	pub fn unpack(&self, config: &Config) -> (i64, i64) {
		let take = if self.take.is_zero() {
			config.db_default_take
		} else {
			self.take.into()
		};
		(self.skip.into(), take.min(config.db_max_take))
	}
}
