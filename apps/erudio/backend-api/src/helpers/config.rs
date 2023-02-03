use serde::Deserialize;
use services::s3::{BucketConfig, Buckets};
use std::collections::HashMap;

pub const SESSION_COOKIE_NAME: &str = "SessionId";
// Config file?
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	pub argon2: Argon2ConfigVec,
	pub salt_size: usize,
	pub secret_size: usize,
	pub db_url: String,
	pub redis_url: String,
	pub region_id: String,
	pub api_port: u16,
	pub buckets: HashMap<Buckets, BucketConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Argon2ConfigVec {
	pub ad: Vec<u8>,
	pub hash_length: u32,
	pub lanes: u32,
	pub mem_cost: u32,
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
