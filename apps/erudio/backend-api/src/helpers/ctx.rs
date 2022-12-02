use prisma_client::{prisma::PrismaClient, User};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tower_cookies::Cookies;

#[derive(Clone)]
pub(crate) struct Ctx {
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub region_id: String,
	pub argon_secret: Arc<Vec<u8>>,
}

#[derive(Clone)]
#[allow(dead_code)] // TODO
pub(crate) struct AuthCtx {
	pub db: Arc<PrismaClient>,
	pub redis: MultiplexedConnection,
	pub cookies: Cookies,
	pub user: User,
	pub region_id: String,
	pub session_id: String,
}
