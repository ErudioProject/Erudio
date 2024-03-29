use crate::helpers::{ctx::Auth, IdempotenceToken};
use crate::routes::RspcResult;
use config::Buckets;
use rspc::Type;
use services::s3::get_bucket;

#[derive(Type, serde::Deserialize, Debug)]
pub struct UploadRequest {
	pub idempotence_token: IdempotenceToken,
	pub idk: String,
}

#[derive(Type, serde::Serialize, serde::Deserialize, Debug)]
pub struct UploadResponse {
	pub presigned_url: String,
}

#[allow(clippy::unused_async)]
pub async fn upload(ctx: Auth, _req: UploadRequest) -> RspcResult<UploadResponse> {
	let bucket = get_bucket(
		ctx.config
			.buckets
			.get(&Buckets::MessageAttachments)
			.unwrap(),
	);
	let res = bucket.presign_put("/test.file", 3600, None).unwrap();
	Ok(UploadResponse { presigned_url: res })
}
