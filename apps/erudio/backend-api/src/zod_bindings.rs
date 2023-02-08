use crate::helpers::pagination::Pagination;
use crate::routes::file::upload::UploadRequest;
use crate::routes::public::login::LoginRequest;
use crate::routes::public::register::RegisterRequest;
use crate::routes::super_admin::add_school::AddSchoolRequest;
use crate::routes::super_admin::add_user_to_school::AddUserToSchoolRequest;
use crate::routes::super_admin::get_school::GetSchoolRequest;
use crate::routes::super_admin::get_user::GetUserRequest;
use crate::routes::super_admin::search_schools::SearchSchoolsRequest;
use crate::routes::super_admin::search_users::SearchUsersRequest;
use crate::routes::super_admin::update_school::UpdateSchoolRequest;
use color_eyre::eyre;
use color_eyre::eyre::{Context, ContextCompat};
use error_handler::FieldErrorType;
use tokio::fs;

pub async fn generate_zod() -> eyre::Result<()> {
	// cursed i will think what to do with this
	let field_error_type = FieldErrorType::codegen();
	let split = field_error_type.split('=');
	let def = split.last().context("Zod strange")?;

	// TODO refactor it already is hard to find what is missing
	let lines = vec![
		// I don't like the fact that this is manual
		LoginRequest::print_imports(),
		Pagination::codegen(),
		format!("export const ErrorFields = z.tuple([z.string(), {def}]).array()"),
		LoginRequest::codegen(),
		UploadRequest::codegen(),
		RegisterRequest::codegen(),
		AddSchoolRequest::codegen(),
		UpdateSchoolRequest::codegen(),
		SearchSchoolsRequest::codegen(),
		GetSchoolRequest::codegen(),
		AddUserToSchoolRequest::codegen(),
		GetUserRequest::codegen(),
		SearchUsersRequest::codegen(),
	];
	fs::write("./apps/erudio/frontend/src/lib/zod.ts", lines.join("\n"))
		.await
		.context("Zod failed")?;

	Ok(())
}
