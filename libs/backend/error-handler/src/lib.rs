pub mod error_mapping;
pub use error_mapping::*;

pub fn backend_error_handler() -> String {
	"backend_error_handler".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(backend_error_handler(), "backend_error_handler".to_string());
	}
}
