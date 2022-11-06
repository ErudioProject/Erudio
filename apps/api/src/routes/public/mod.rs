mod login;
mod register;
mod version;

use login::*;
use register::*;
use version::*;

use crate::Ctx;
use argon2::{Config, ThreadMode, Variant, Version};
use rand::RngCore;
use redis::AsyncCommands;
use rspc::{Router, RouterBuilder};
use std::future::Future;

const ARGON_CONFIG: Config = Config {
	variant: Variant::Argon2i,
	version: Version::Version13,
	mem_cost: 16384,
	time_cost: 3,
	lanes: 4,
	thread_mode: ThreadMode::Parallel,
	secret: &[],
	ad: &[],
	hash_length: 32,
};
const SALT_SIZE: usize = 64;
const SECRET_SIZE: usize = 512;

pub fn mount() -> RouterBuilder<Ctx> {
	Router::<Ctx>::new()
		.query("version", |t| t(version))
		.query("login", |t| t(login))
		.query("register", |t| t(register))
}
