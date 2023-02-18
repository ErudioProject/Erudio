use color_eyre::Report;
use hex::FromHexError;
use rand::Rng;
use redis::RedisError;
use rspc::ErrorCode;
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Debug)]
pub enum InternalError {
	IntoRspc(ErrorCode, Option<Errors>),
	IntoRspcWithCause(ErrorCode, Option<Errors>, Arc<dyn std::error::Error + Send + Sync>),
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

pub type Errors = Vec<(String, FieldErrorType)>;
fn errors_to_string(errors: &Errors) -> String {
	format!("^{}", serde_json::to_string(errors).expect("how?"))
}
fn get_trace_id() -> String {
	let buf = &mut [0u8; 128];
	rand::thread_rng().fill(buf);
	hex::encode(buf)
}

#[derive(serde::Serialize, Debug)]
pub enum FieldErrorType {
	InvalidValue,
	NotFound,
	Conflict,
	TooLong(usize),
	TooShort(usize),
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
impl From<chrono::RoundingError> for InternalError {
	fn from(value: chrono::RoundingError) -> Self {
		Self::ServerError("Chrono rounding failed?? failed".into(), value.into())
	}
}

impl From<prisma_client::prisma_client_rust::QueryError> for InternalError {
	fn from(value: prisma_client::prisma_client_rust::QueryError) -> Self {
		Self::ServerError("Prisma failed".into(), value.into())
	}
}

// tracing macros drive complexity ut tp 37/25 without them it should be around 7 or 2 depending on the way you calculate it (each macro adds 7).
// There is complex issue about this https://github.com/rust-lang/rust-clippy/issues/3793
#[allow(clippy::cognitive_complexity)]
impl From<InternalError> for rspc::Error {
	fn from(value: InternalError) -> Self {
		match value {
			InternalError::Rspc(err) => {
				debug!("Rspc Error: {err}");
				err
			}
			InternalError::TestError(err) => {
				error!("TEST ERROR: {err}");
				Self::new(
					ErrorCode::InternalServerError,
					"This is an error that is allowed only in tests".to_string(),
				)
			}
			InternalError::ServerError(message, report) => {
				let trace_id = get_trace_id();
				error!("trace: #{trace_id}# message: {message} err: {report:?}");
				Self::new(ErrorCode::InternalServerError, format!("#trace#{trace_id}#"))
			}
			InternalError::IntoRspc(code, message) => {
				debug!("Rspc Error: {code:?} Message: {message:?}");
				let message = message.as_ref().map_or_else(String::new, errors_to_string);
				Self::new(code, message)
			}
			InternalError::IntoRspcWithCause(code, message, cause) => {
				debug!("Rspc Error: {code:?} Message: {message:?}  Cause {cause:?}");
				let message = message.as_ref().map_or_else(String::new, errors_to_string);
				Self::with_cause(code, message, cause)
			}
		}
	}
}
