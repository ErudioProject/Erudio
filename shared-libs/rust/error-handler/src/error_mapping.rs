use eyre::Report;
use hex::FromHexError;
use log::error;
use rand::Rng;
use redis::RedisError;
use rspc::ErrorCode;
use std::sync::Arc;

#[derive(Debug)]
pub enum InternalError {
	IntoRspc(ErrorCode, String),
	IntoRspcWithCause(ErrorCode, String, Arc<dyn std::error::Error + Send + Sync>),
	Rspc(rspc::Error),
	ServerError(String, Report),
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
		Self::ServerError("Serde_json failed".into(), value.into())
	}
}

impl From<argon2::Error> for InternalError {
	fn from(value: argon2::Error) -> Self {
		Self::ServerError("Argon2 failed".into(), value.into())
	}
}

impl From<RedisError> for InternalError {
	fn from(value: RedisError) -> Self {
		Self::ServerError("Redis failed".into(), value.into())
	}
}
impl From<FromHexError> for InternalError {
	fn from(value: FromHexError) -> Self {
		Self::ServerError("Hex failed".into(), value.into())
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
			InternalError::ServerError(message, report) => {
				let buf = &mut [0u8; 128];
				rand::thread_rng().fill(buf);
				let trace_id = hex::encode(buf);
				error!("trace: #{trace_id}# message: {message} err: {report:?}");
				Self::new(
					ErrorCode::InternalServerError,
					format!("Server error trace #{trace_id}#"),
				)
			}
			InternalError::IntoRspc(code, message) => Self::new(code, message),
			InternalError::IntoRspcWithCause(code, message, cause) => Self::with_cause(code, message, cause),
		}
	}
}
