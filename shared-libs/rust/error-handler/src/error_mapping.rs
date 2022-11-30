use hex::FromHexError;
use log::warn;
use redis::RedisError;
use rspc::ErrorCode;

#[derive(Debug)]
pub enum InternalError {
	Rspc(rspc::Error),
	Unreachable,       // This error should be unreachable
	TestError(String), // This is for tests TODO check if there is a way to enforce it
}
pub type InternalResult<T> = Result<T, InternalError>;

impl InternalError {
	pub fn new_rspc(code: ErrorCode, message: String) -> Self {
		InternalError::Rspc(rspc::Error::new(code, message))
	}
}

impl From<serde_json::Error> for InternalError {
	fn from(value: serde_json::Error) -> Self {
		InternalError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Internal json serialization failed".into(),
			value,
		))
	}
}

impl From<argon2::Error> for InternalError {
	fn from(value: argon2::Error) -> Self {
		warn!("Argon2 failed with error: {:?}", value);
		InternalError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Argon2 Error".into(),
			value,
		))
	}
}

impl From<RedisError> for InternalError {
	fn from(value: RedisError) -> Self {
		warn!("Redis failed with error: {:?}", value);
		InternalError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Redis error".into(),
			value,
		))
	}
}
impl From<FromHexError> for InternalError {
	fn from(value: FromHexError) -> Self {
		warn!("Hex failed with error: {:?}", value);
		InternalError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Error decoding hex value".into(),
			value,
		))
	}
}

impl From<prisma_client::prisma_client_rust::QueryError> for InternalError {
	fn from(value: prisma_client::prisma_client_rust::QueryError) -> Self {
		InternalError::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Prisma query error".into(),
			value,
		))
	}
}

impl From<InternalError> for rspc::Error {
	fn from(value: InternalError) -> Self {
		match value {
			InternalError::Rspc(x) => x,
			InternalError::Unreachable => rspc::Error::new(
				ErrorCode::InternalServerError,
				"This should have been unreachable".to_string(),
			),
			InternalError::TestError(_) => rspc::Error::new(
				ErrorCode::InternalServerError,
				"This is an error that is allowed only in tests".to_string(),
			),
		}
	}
}
