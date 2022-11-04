use redis::RedisError;
use rspc::ErrorCode;

pub(crate) struct RspcError(rspc::Error);

impl From<serde_json::Error> for RspcError {
	fn from(value: serde_json::Error) -> Self {
		RspcError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Internal json serialization failed".into(),
			value,
		))
	}
}
impl From<argon2::Error> for RspcError {
	fn from(value: argon2::Error) -> Self {
		RspcError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Argon2 Error".into(),
			value,
		))
	}
}

impl From<redis::RedisError> for RspcError {
	fn from(value: RedisError) -> Self {
		RspcError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Redis error".into(),
			value,
		))
	}
}

impl From<RspcError> for rspc::Error {
	fn from(value: RspcError) -> Self {
		value.0
	}
}
