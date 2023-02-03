use config::Config;
use prisma_client::prisma::PrismaClient;
use redis::aio::MultiplexedConnection;
use services::session;
use std::sync::Arc;
use tower_cookies::Cookies;

#[derive(Clone)]
pub struct Public {
	pub config: Config,
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
}

#[derive(Clone)]
pub struct Auth {
	pub config: Config,
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub session_data: session::Info,
	pub session_id: String,
}
