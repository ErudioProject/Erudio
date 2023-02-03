use argon2::{ThreadMode, Variant, Version};

pub fn get_argon_config(argon_secret: &[u8]) -> argon2::Config {
	argon2::Config {
		variant: Variant::Argon2i,
		version: Version::Version13,
		mem_cost: 16384,
		time_cost: 3,
		lanes: 4,
		thread_mode: ThreadMode::Parallel,
		secret: argon_secret,
		ad: &[],
		hash_length: 32,
	}
}
