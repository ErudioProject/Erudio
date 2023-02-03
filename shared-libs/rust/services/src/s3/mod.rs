pub use s3::creds::Credentials;
use s3::Bucket;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Buckets {
	MessageAttachments,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BucketConfig {
	pub name: String,
	pub region: String,
	//pub credentials: Credentials, // TODO fix
}

/// # Panics
///
/// Will panic with invalid config, in future will be checked ar startup
#[must_use]
pub fn get_bucket(conf: &BucketConfig) -> Bucket {
	Bucket::new(
		&conf.name,
		conf.region.parse().unwrap(),
		Credentials::default().unwrap(),
	)
	.unwrap()
	// TODO check bucket validity at startup
}
