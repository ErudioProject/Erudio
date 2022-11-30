#![forbid(unsafe_code)]
#![recursion_limit = "256"]
pub mod prisma;
pub use prisma_client_rust;

pub type User = prisma::user::Data;

pub fn prisma_client() -> String {
	"prisma_client".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(prisma_client(), "prisma_client".to_string());
	}
}
