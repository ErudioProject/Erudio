#![forbid(unsafe_code)]
#![recursion_limit = "256"]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::as_conversions)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]
extern crate argon2;

mod helpers;
mod routes;
mod shutdown_signal;

use crate::{eyre::Context, helpers::ctx::Public, routes::router};
use axum::routing::get;
use color_eyre::eyre;
use error_handler::InternalResult;
use log::{error, info, warn};
use prisma_client::{prisma, prisma::PrismaClient};
use prisma_client_rust::{chrono::Utc, raw};
use redis::AsyncCommands;
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

	let redis_url = env::var("REDIS_URL").context("No REDIS_URL environmental variable")?;
	let region_id = env::var("REGION_ID").context("No REGION_ID environmental variable")?;
	let port = env::var("API_PORT")
		.context("No API_PORT environmental variable")?
		.parse::<u16>()
		.context("API_PORT is invalid example value '3000'")?;
	let argon_secret =
		hex::decode(env::var("ARGON_SECRET").context("No hex value ARGON_SECRET environmental variable")?)
			.context("ARGON_SECRET is not hex value")?;
	if argon_secret.len() != 32 {
		warn!("Recommended ARGON_SECRET length is 32 actual: {}", argon_secret.len());
	}

	let db: Arc<PrismaClient> = Arc::new(
		PrismaClient::_builder()
			.with_url(url)
			.build()
			.await
			.context("Database ERROR")?,
	);

	#[cfg(debug_assertions)]
	db._db_push().await?;
	#[cfg(not(debug_assertions))]
	db._migrate_deploy().await?;

	let redis = redis::Client::open(redis_url)?;
	let conn = redis
		.get_multiplexed_async_connection()
		.await
		.context("REDIS ERROR")?;

	let app = axum::Router::new()
		.route("/", get(|| async { "Erudio" }))
		.route(
			"/health",
			get({
				let db_health = db.clone();
				let redis_health = conn.clone();

				move || async move {
					let result = check_health(&db_health, redis_health).await;
					match result {
						Ok(_) => (axum::http::StatusCode::OK, "OK".into()),
						Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
					}
				}
			}),
		)
		.route(
			"/rspc/:id",
			router()
				.arced()
				.endpoint(move |cookies: Cookies| Public {
					db: db.clone(),
					redis: conn,
					cookies,
					region_id,
					argon_secret: Arc::new(argon_secret),
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

	let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
	info!("listening on {}", addr);
	axum::Server::try_bind(&addr)?
		.serve(app.into_make_service())
		.with_graceful_shutdown(shutdown_signal::shutdown_signal())
		.await?;

	Ok(())
}

async fn check_health<R: AsyncCommands>(db: &PrismaClient, mut redis: R) -> InternalResult<()> {
	let _: i64 = db._execute_raw(raw!("SELECT 1;")).exec().await?;
	redis.set("HEALTH", Utc::now().timestamp()).await?;
	Ok(())
}
