#![forbid(unsafe_code)]
pub mod error_mapping;
pub use error_mapping::*;

pub fn error_handler() -> String {
	"error_handler".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(error_handler(), "error_handler".to_string());
	}
}
