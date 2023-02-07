use crate::helpers::ctx::SuperAdmin;
use rspc::{Router, RouterBuilder};

pub mod add_school;
pub mod get_school;
pub mod search_schools;
pub mod update_school;
pub mod version;
use crate::helpers::{ctx, idempotent};
use add_school::add_school;
use get_school::get_school;
use prisma_client::prisma;
use search_schools::search_schools;
use update_school::update_school;
use version::version;

pub fn mount() -> RouterBuilder<SuperAdmin> {
	Router::<SuperAdmin>::new()
		.query("version", |t| t(version))
		// School
		.query("getSchool", |t| t(get_school))
		.query("searchSchools", |t| t(search_schools))
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
}

/*
admin.searchSchoolUsers
admin.getUser
admin.addSchoolUser
admin.editUser

-
admin.addSchool
admin.getSchool
admin.updateSchool
admin.searchSchools
 */
