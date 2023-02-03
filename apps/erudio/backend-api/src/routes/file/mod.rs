mod upload;

use crate::helpers::ctx::Auth;
use crate::helpers::idempotent;
use rspc::{Router, RouterBuilder};
use upload::upload;

pub fn mount() -> RouterBuilder<Auth> {
	Router::<Auth>::new().mutation("upload", |t| {
		t(idempotent!(upload, Auth, upload::UploadRequest, upload::UploadResponse))
	})
}
