use config::Config;
use prisma_client::prisma::PrismaClient;
use redis::aio::MultiplexedConnection;
use services::session;
use std::net::IpAddr;
use std::sync::Arc;
use tower_cookies::Cookies;

#[derive(Clone)]
pub struct Public {
	pub config: Config,
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub ip: IpAddr,
}

#[derive(Clone)]
pub struct Auth {
	pub config: Config,
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub session_data: session::Info,
	pub session_id: String,
	pub ip: IpAddr,
	pub is_super_admin: bool,
}

#[derive(Clone)]
pub struct SuperAdmin {
	pub config: Config,
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
}
