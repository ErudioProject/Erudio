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
use prisma_client::prisma::{pii_data, SchoolRelationType};
use prisma_client::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
	pub user: User,
	pub pii_data: pii_data::Data,
	pub schools: Vec<SchoolRelation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchoolRelation {
	school_id: String,
	relation: SchoolRelationType,
}

impl Info {
	#[must_use]
	pub fn new(user: User, pii_data: pii_data::Data, schools: Vec<SchoolRelation>) -> Self {
		Self {
			user,
			pii_data,
			schools,
		}
	}
}

impl TryFrom<User> for Info {
	type Error = InternalError;

	/// user pii data and relations must be fetched
	fn try_from(value: User) -> Result<Self, Self::Error> {
		debug_assert!(value.pii_data.is_some());
		debug_assert!(value.user_school_relation.is_some());
		debug_assert!(value.pii_data.as_ref().unwrap().is_some());
		let pii_data = value
			.pii_data
			.as_ref()
			.ok_or_else(|| {
				InternalError::ServerError(
					format!("Pii data not fetched from db for user {:?}", &value),
					eyre!("ERROR"),
				)
			})?
			.clone()
			.ok_or_else(|| {
				InternalError::ServerError(
					format!(
						"Invalid Value in database user {:?} have no associated pii_data",
						&value
					),
					eyre!("ERROR"),
				)
			})?;

		let relations = value
			.user_school_relation
			.as_ref()
			.ok_or_else(|| {
				InternalError::ServerError(
					format!("School relations not fetched from db for user {:?}", &value),
					eyre!("ERROR"),
				)
			})?
			.clone()
			.into_iter()
			.map(|rel| SchoolRelation {
				school_id: rel.school_id,
				relation: rel.school_relation_type,
			})
			.collect();

		Ok(Self {
			user: value,
			pii_data: *pii_data,
			schools: relations,
		})
	}
}
