pub mod prisma;
pub use prisma_client_rust::*;

pub type User = prisma::user::Data;

pub fn backend_prisma_client() -> String {
	return "backend_prisma_client".into();
}
pub fn backend_prisma_client_2(a: bool) -> Vec<u8> {
	"ELO".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(backend_prisma_client(), "backend_prisma_client".to_string());
	}
}
