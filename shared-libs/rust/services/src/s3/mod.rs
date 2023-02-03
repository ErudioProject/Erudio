use config::BucketConfig;
pub use s3::creds::Credentials;
use s3::Bucket;

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
