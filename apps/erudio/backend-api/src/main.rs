#![forbid(unsafe_code)]
#![recursion_limit = "256"]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::as_conversions)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::module_name_repetitions)]
extern crate argon2;

mod cookies;
mod helpers;
mod routes;
mod shutdown_signal;
mod zod_bindings;

use crate::helpers::seed;
use crate::{eyre::Context, helpers::ctx::Public, routes::router};
use axum::extract::ConnectInfo;
use axum::routing::get;
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use config::Config;
use error_handler::InternalResult;
use log::{debug, error, info, warn};
use prisma_client::{prisma, prisma::PrismaClient};
use prisma_client_rust::{chrono::Utc, raw};
use redis::AsyncCommands;
use std::path::PathBuf;
use std::{
	net::{Ipv4Addr, SocketAddr},
	sync::Arc,
};
use tokio::fs;
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
	#[cfg(debug_assertions)]
	zod_bindings::generate_zod().await?;

	// TODO pull over http from server
	let contents = fs::read_to_string("./Config.ron")
		.await
		.context("no Config.ron file")?;
	let config: Config = ron::from_str(&contents).context("Config.ron is invalid")?;
	if config.argon2.secret.len() != 32 {
		warn!(
			"Recommended ARGON_SECRET length is 32 actual: {}",
			config.argon2.secret.len()
		);
	}
	debug!("Config: {:?}", config);

	let db: Arc<PrismaClient> = Arc::new(
		PrismaClient::_builder()
			.with_url(config.db_url.clone())
			.build()
			.await
			.context("Database ERROR")?,
	);

	#[cfg(debug_assertions)]
	db._db_push().await?;
	#[cfg(not(debug_assertions))]
	db._migrate_deploy().await?;

	seed::seed_super_admin(&db, config.clone())
		.await
		.map_err(|e| eyre!("Error {e:?}"))?;

	let redis = redis::Client::open(config.redis_url.clone())?;
	let conn = redis
		.get_multiplexed_async_connection()
		.await
		.context("REDIS ERROR")?;

	let router = router().arced();
	#[cfg(debug_assertions)]
	router
		.export_ts(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bindings.ts"))
		.context("Binding export failed")?;

	let app = axum::Router::new()
		.route("/", get(|| async { "Erudio" }))
		.route(
			"/health",
			get({
				let db_health = db.clone();
				let redis_health = conn.clone();

				move || async move {
					check_health(&db_health, redis_health).await.map_or_else(
						|err| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
						|_| (axum::http::StatusCode::OK, "OK".into()),
					)
				}
			}),
		)
		.route(
			"/rspc/:id",
			router
				.endpoint({
					let config = config.clone();
					move |cookies: Cookies, ConnectInfo(addr): ConnectInfo<SocketAddr>| Public {
						config,
						db: db.clone(),
						redis: conn,
						cookies,
						ip: addr.ip(),
					}
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

	let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.api_port));
	info!("listening on {}", addr);
	axum::Server::try_bind(&addr)?
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.with_graceful_shutdown(shutdown_signal::shutdown_signal())
		.await?;

	Ok(())
}

async fn check_health<R: AsyncCommands>(db: &PrismaClient, mut redis: R) -> InternalResult<()> {
	let _: i64 = db._execute_raw(raw!("SELECT 1;")).exec().await?;
	redis.set("HEALTH", Utc::now().timestamp()).await?;
	Ok(())
}
