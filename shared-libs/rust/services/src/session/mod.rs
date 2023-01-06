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
pub use load::load;

use prisma_client::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
	pub user: User,
}

impl From<User> for Info {
	fn from(value: User) -> Self {
		{
			Self { user: value }
		}
	}
}
