#![forbid(unsafe_code)]
#![recursion_limit = "256"]
pub mod prisma;
pub use prisma_client_rust;

pub type User = prisma::user::Data;

pub fn backend_prisma_client() -> String {
	"backend_prisma_client".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(backend_prisma_client(), "backend_prisma_client".to_string());
	}
}
