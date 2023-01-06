use prisma_client::prisma::PrismaClient;
use redis::aio::MultiplexedConnection;
use services::session::SessionData;
use std::sync::Arc;
use tower_cookies::Cookies;

#[derive(Clone)]
pub struct Public {
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub region_id: String,
	pub argon_secret: Arc<Vec<u8>>,
}

#[derive(Clone)]
pub struct Auth {
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub session_data: SessionData,
	pub session_id: String,
}
