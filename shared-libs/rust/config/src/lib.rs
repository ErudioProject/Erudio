use derivative::Derivative;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	pub argon2: Argon2ConfigVec,
	pub salt_size: usize,
	pub secret_size: usize,
	pub db_url: String,
	pub db_url_test: String,
	pub redis_url: String,
	pub region_id: String,
	pub api_port: u16,
	pub buckets: HashMap<Buckets, BucketConfig>,
}

#[derive(Clone, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Argon2ConfigVec {
	#[derivative(Debug = "ignore")]
	pub ad: Vec<u8>,
	pub hash_length: u32,
	pub lanes: u32,
	pub mem_cost: u32,
	#[derivative(Debug = "ignore")]
	pub secret: Vec<u8>,
	pub thread_mode: ThreadModeDef,
	pub time_cost: u32,
	pub variant: VariantDef,
	pub version: VersionDef,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ThreadModeDef {
	Sequential,
	Parallel,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VariantDef {
	Argon2d = 0,
	Argon2i = 1,
	Argon2id = 2,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VersionDef {
	Version10 = 0x10,
	Version13 = 0x13,
}

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
