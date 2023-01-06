use hex::FromHexError;
use log::error;
use redis::RedisError;
use rspc::ErrorCode;
use std::sync::Arc;

#[derive(Debug)]
pub enum InternalError {
	IntoRspc(ErrorCode, String),
	IntoRspcWithCause(ErrorCode, String, Arc<dyn std::error::Error + Send + Sync>),
	Rspc(rspc::Error),
	TestError(String), // This is for tests TODO check if there is a way to enforce it
}
pub type InternalResult<T> = Result<T, InternalError>;

impl InternalError {
	#[must_use]
	pub const fn new_rspc(code: ErrorCode, message: String) -> Self {
		Self::Rspc(rspc::Error::new(code, message))
	}
}

impl From<serde_json::Error> for InternalError {
	fn from(value: serde_json::Error) -> Self {
		error!("Serde_json failed with error: {:?}", value);
		Self::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Internal json serialization failed".into(),
			value,
		))
	}
}

impl From<argon2::Error> for InternalError {
	fn from(value: argon2::Error) -> Self {
		error!("Argon2 failed with error: {:?}", value);
		Self::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Argon2 Error".into(),
			value,
		))
	}
}

impl From<RedisError> for InternalError {
	fn from(value: RedisError) -> Self {
		error!("Redis failed with error: {:?}", value);
		Self::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Redis error".into(),
			value,
		))
	}
}
impl From<FromHexError> for InternalError {
	fn from(value: FromHexError) -> Self {
		error!("Hex failed with error: {:?}", value);
		Self::Rspc(rspc::Error::with_cause(
			ErrorCode::InternalServerError,
			"Error decoding hex value".into(),
			value,
		))
	}
}

impl From<prisma_client::prisma_client_rust::QueryError> for InternalError {
	fn from(value: prisma_client::prisma_client_rust::QueryError) -> Self {
		error!("Prisma failed with error: {:?}", value);
		Self::Rspc(rspc::Error::with_cause(
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
			InternalError::TestError(_) => Self::new(
				ErrorCode::InternalServerError,
				"This is an error that is allowed only in tests".to_string(),
			),
			InternalError::IntoRspc(code, message) => Self::new(code, message),
			InternalError::IntoRspcWithCause(code, message, cause) => Self::with_cause(code, message, cause),
		}
	}
}
