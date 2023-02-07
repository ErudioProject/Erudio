use config::Config;
use prisma_client_rust::bigdecimal::Zero;
use prisma_client_rust::rspc;

#[derive(rspc::Type, serde::Deserialize, Debug, Default)]
pub struct Pagination {
	pub skip: i64,
	pub take: i64,
}

impl Pagination {
	pub fn unpack(&self, config: &Config) -> (i64, i64) {
		let take = if self.take.is_zero() {
			config.db_default_take
		} else {
			self.take
		};
		(self.skip, take.min(config.db_max_take))
	}
}
