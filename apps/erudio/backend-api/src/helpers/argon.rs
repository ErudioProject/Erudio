use config::{Argon2ConfigVec, ThreadModeDef, VariantDef, VersionDef};

pub fn get_argon_config(argon2conf: &Argon2ConfigVec) -> argon2::Config {
	// yes that boilerplate to make config work properly
	argon2::Config {
		ad: &argon2conf.ad,
		hash_length: argon2conf.hash_length,
		lanes: argon2conf.lanes,
		mem_cost: argon2conf.mem_cost,
		secret: &argon2conf.secret,
		thread_mode: match &argon2conf.thread_mode {
			ThreadModeDef::Sequential => argon2::ThreadMode::Sequential,
			ThreadModeDef::Parallel => argon2::ThreadMode::Parallel,
		},
		time_cost: argon2conf.time_cost,
		variant: match &argon2conf.variant {
			VariantDef::Argon2d => argon2::Variant::Argon2d,
			VariantDef::Argon2i => argon2::Variant::Argon2i,
			VariantDef::Argon2id => argon2::Variant::Argon2id,
		},
		version: match &argon2conf.version {
			VersionDef::Version10 => argon2::Version::Version10,
			VersionDef::Version13 => argon2::Version::Version13,
		},
	}
}
