#![forbid(unsafe_code)]

mod check;
mod destroy;
mod destroy_all_for_user;
pub mod init;
mod load;
mod recover;

pub use check::check;
pub use destroy::destroy;
pub use destroy_all_for_user::destroy_all_for_user;
use eyre::eyre;
pub use load::load;

use error_handler::InternalError;
use prisma_client::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
	pub user: User,
}

impl TryFrom<User> for Info {
	type Error = InternalError;

	fn try_from(value: User) -> Result<Self, Self::Error> {
		value
			.pii_data
			.clone()
			.flatten()
			.ok_or_else(|| InternalError::ServerError("Invalid Value In database".into(), eyre!("Error")))?;

		Ok(Self { user: value })
	}
}