use hex::FromHexError;
use redis::RedisError;
use rspc::ErrorCode;

pub struct ApiError(rspc::Error);

impl From<serde_json::Error> for ApiError {
	fn from(value: serde_json::Error) -> Self {
		ApiError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Internal json serialization failed".into(),
			value,
		))
	}
}
impl From<argon2::Error> for ApiError {
	fn from(value: argon2::Error) -> Self {
		ApiError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Argon2 Error".into(),
			value,
		))
	}
}

impl From<RedisError> for ApiError {
	fn from(value: RedisError) -> Self {
		ApiError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Redis error".into(),
			value,
		))
	}
}
impl From<FromHexError> for ApiError {
	fn from(value: FromHexError) -> Self {
		ApiError(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Error decoding hex value".into(),
			value,
		))
	}
}
impl From<ApiError> for rspc::Error {
	fn from(value: ApiError) -> Self {
		value.0
	}
}
