use crate::helpers::ctx::SuperAdmin;
use rspc::{Router, RouterBuilder};

pub mod add_school;
pub mod add_user_to_school;
pub mod get_school;
pub mod get_user;
pub mod search_schools;
pub mod search_users;
pub mod update_school;
pub mod version;
use crate::helpers::{ctx, idempotent};
use crate::routes::super_admin::search_schools::search_schools_amount;
use crate::routes::super_admin::search_users::search_users_amount;
use add_school::add_school;
use add_user_to_school::add_user_to_school;
use get_school::get_school;
use get_user::get_user;
use prisma_client::prisma;
use search_schools::search_schools;
use search_users::search_users;
use update_school::update_school;
use version::version;

pub fn mount() -> RouterBuilder<SuperAdmin> {
	Router::<SuperAdmin>::new()
		.query("version", |t| t(version))
		// School
		.query("getSchool", |t| t(get_school))
		.query("searchSchools", |t| t(search_schools))
		.query("searchSchoolsAmount", |t| t(search_schools_amount))
		.mutation("addSchool", |t| {
			t(idempotent!(
				add_school,
				ctx::SuperAdmin,
				add_school::AddSchoolRequest,
				prisma::school::Data
			))
		})
		.mutation("updateSchool", |t| {
			t(idempotent!(
				update_school,
				ctx::SuperAdmin,
				update_school::UpdateSchoolRequest,
				prisma::school::Data
			))
		})
		// User
		.query("searchUsers", |t| t(search_users)) // TODO move to user space while adding validation
		.query("searchUsersAmount", |t| t(search_users_amount))
		.query("getUser", |t| t(get_user))
		.mutation("addUserToSchool", |t| {
			t(idempotent!(
				add_user_to_school,
				ctx::SuperAdmin,
				add_user_to_school::AddUserToSchoolRequest,
				prisma::user_school_relation::Data
			))
		})
}

prisma::user::select!((filters: Vec<prisma::user_school_relation::WhereParam>) => user_full {
	id
	two_factor_auth_settings
	user_school_relation(filters): include {
		school
	}
	pii_data
});

/*
admin.editUser

-
admin.addSchool
admin.getSchool
admin.updateSchool
admin.searchSchools

admin.searchUsers
admin.getUser
admin.addUserToSchool
 */
