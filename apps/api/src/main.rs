#![forbid(unsafe_code)]
extern crate argon2;

mod routes;
mod shutdown_signal;

use crate::{
	eyre::{eyre, Context},
	routes::{router, Ctx},
};
use axum::routing::get;
use backend_prisma_client::{prisma, prisma::PrismaClient};
use color_eyre::eyre;
use log::{error, info};
use std::{
	env,
	net::{Ipv4Addr, SocketAddr},
	sync::Arc,
};
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::{cors, cors::CorsLayer};

// TODO clean up a bit
pub fn main() {
	dotenvy::dotenv().ok();
	env_logger::init();
	let result = start();
	if let Some(err) = result.err() {
		error!("{:?}", err);
	}
}

#[tokio::main]
async fn start() -> eyre::Result<()> {
	#[cfg(target_family = "unix")]
	let url = env::var("DATABASE_URL").context("No DATABASE_URL environmental variable")?;
	#[cfg(target_family = "windows")]
	let url = env::var("DATABASE_URL_WIN").context("No DATABASE_URL_WIN environmental variable")?;

	let db: Arc<PrismaClient> = Arc::new(
		PrismaClient::_builder()
			.with_url(url)
			.build()
			.await
			.map_err(|err| eyre!("Database client error: {:?}", err))?,
	);

	let redis = redis::Client::open("redis://127.0.0.1/")?;
	let conn = redis.get_multiplexed_async_connection().await?;

	let router = router().arced();

	let app = axum::Router::new()
		.route("/", get(|| async { "Erudio" }))
		.route(
			"/rspc/:id",
			router
				.endpoint(move |cookies: Cookies| Ctx {
					db: db.clone(),
					redis: conn,
					cookies,
				})
				.axum(),
		)
		.layer(
			CorsLayer::new()
				.allow_origin(cors::Any)
				.allow_headers(cors::Any)
				.allow_methods(cors::Any),
		)
		.layer(CookieManagerLayer::new());

	let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));

	info!("listening on {}", addr);
	axum::Server::try_bind(&addr)?
		.serve(app.into_make_service())
		.with_graceful_shutdown(shutdown_signal::shutdown_signal())
		.await?;

	Ok(())
}
