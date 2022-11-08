use hex::FromHexError;
use log::warn;
use redis::RedisError;
use rspc::ErrorCode;

#[derive(Debug)]
pub enum ApiError {
	Rspc(rspc::Error),
	Unreachable,       // This error should be unreachable
	TestError(String), // This error should be unreachable
}
pub type ApiResult<T> = Result<T, ApiError>;

impl From<serde_json::Error> for ApiError {
	fn from(value: serde_json::Error) -> Self {
		ApiError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Internal json serialization failed".into(),
			value,
		))
	}
}

impl From<argon2::Error> for ApiError {
	fn from(value: argon2::Error) -> Self {
		warn!("Argon2 failed with error: {:?}", value);
		ApiError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Argon2 Error".into(),
			value,
		))
	}
}

impl From<RedisError> for ApiError {
	fn from(value: RedisError) -> Self {
		warn!("Redis failed with error: {:?}", value);
		ApiError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Redis error".into(),
			value,
		))
	}
}
impl From<FromHexError> for ApiError {
	fn from(value: FromHexError) -> Self {
		warn!("Hex failed with error: {:?}", value);
		ApiError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Error decoding hex value".into(),
			value,
		))
	}
}

impl From<backend_prisma_client::prisma_client_rust::QueryError> for ApiError {
	fn from(value: backend_prisma_client::prisma_client_rust::QueryError) -> Self {
		ApiError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Prisma query error".into(),
			value,
		))
	}
}

impl From<ApiError> for rspc::Error {
	fn from(value: ApiError) -> Self {
		match value {
			ApiError::Rspc(x) => x,
			ApiError::Unreachable => rspc::Error::new(
				ErrorCode::InternalServerError,
				"This should have been unreachable".to_string(),
			),
			ApiError::TestError(_) => {
				rspc::Error::new(ErrorCode::InternalServerError, "This is a test error".to_string())
			}
		}
	}
}
