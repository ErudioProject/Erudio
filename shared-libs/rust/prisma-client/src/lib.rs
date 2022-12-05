#![forbid(unsafe_code)]
#![recursion_limit = "256"]
pub mod prisma;

use crate::prisma::PrismaClient;
pub use prisma_client_rust;
use rand::{thread_rng, Rng};

pub type User = prisma::user::Data;

fn id() -> usize {
	use std::sync::atomic::{AtomicUsize, Ordering};
	static ID: AtomicUsize = AtomicUsize::new(0);
	ID.fetch_add(1, Ordering::SeqCst)
}

pub async fn prisma_mocked_client(db_test_url: String) -> Result<PrismaClient, Box<dyn std::error::Error>> {
	let rand: usize = thread_rng().gen(); // TODO figure out why schema dont self reset
	let client = PrismaClient::_builder()
		.with_url(db_test_url + &id().to_string() + &rand.to_string())
		.build()
		.await?;
	client._db_push().accept_data_loss().await?;

	Ok(client)
}

#[cfg(test)]
mod tests {
	use super::*;
	use prisma_client_rust::tokio;
	use serial_test::serial;
	use std::env;

	#[tokio::test]
	async fn test_prisma_mock() {
		dotenvy::dotenv().expect(".env file loading error");
		let db = prisma_mocked_client(env::var("DATABASE_URL_TESTS").expect("DATABASE_URL_TESTS not found")).await;
		assert!(db.is_ok())
	}

	#[tokio::test]
	#[serial]
	async fn test_id() {
		let id_v = id();
		assert_eq!(id(), id_v + 1);
		assert_eq!(id(), id_v + 2);
		assert_eq!(id(), id_v + 3);
	}
}
