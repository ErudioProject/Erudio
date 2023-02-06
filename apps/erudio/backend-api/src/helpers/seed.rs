use crate::helpers::argon::get_argon_config;
use color_eyre::eyre::eyre;
use config::Config;
use error_handler::{InternalError, InternalResult};
use prisma_client::prisma;
use prisma_client::prisma::PrismaClient;
use rand::RngCore;

pub async fn seed_super_admin(db: &PrismaClient, config: Config) -> InternalResult<()> {
	let mut rng = rand::thread_rng();
	let argon_config = get_argon_config(&config.argon2);
	for (login, admin_data) in config.admins {
		let mut salt = vec![0; config.salt_size];
		{
			rng.fill_bytes(&mut salt);
		}

		let hash = admin_data
			.password_hash
			.as_ref()
			.ok_or_else(|| InternalError::ServerError("Config error".to_string(), eyre!("Error with admins config")))
			.map(std::clone::Clone::clone)
			.or_else(|_| {
				argon2::hash_encoded(admin_data.password.as_bytes(), &salt, &argon_config)
					.map_err(Into::<InternalError>::into)
			})?;

		db.super_admin()
			.upsert(
				prisma::super_admin::UniqueWhereParam::LoginEquals(login.to_string()),
				prisma::super_admin::create(hash, login.to_string(), vec![]),
				vec![],
			)
			.exec()
			.await?;
	}

	Ok(())
}
