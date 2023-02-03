pub use s3::creds::Credentials;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Buckets {
	MessageAttachments,
}
