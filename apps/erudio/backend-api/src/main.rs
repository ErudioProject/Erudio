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
#[cfg(debug_assertions)]
mod zod_bindings;

use crate::helpers::seed;
use crate::{eyre::Context, helpers::ctx::Public, routes::router};
use axum::extract::ConnectInfo;
use axum::routing::get;
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use config::Config;
use error_handler::InternalResult;
use prisma_client::{prisma, prisma::PrismaClient};
use prisma_client_rust::{chrono::Utc, raw};
use redis::AsyncCommands;
#[cfg(debug_assertions)]
use std::path::PathBuf;
use std::{
	net::{Ipv4Addr, SocketAddr},
	sync::Arc,
};
use tokio::fs;
use tokio::net::TcpStream;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::trace::TraceLayer;
use tracing::{debug, info, warn};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

// TODO clean up a bit
pub fn main() {
	dotenvy::dotenv().ok();
	LogTracer::init().expect("Unable to setup log tracer!");
	let result = start();
	if let Some(err) = result.err() {
		println!("ERROR: {err:?}");
	}
}

#[allow(clippy::too_many_lines)] // TODO maybe remove
#[tokio::main]
async fn start() -> eyre::Result<()> {
	std::env::set_var("RUST_BACKTRACE", "1");
	let app_name = concat!(
		env!("CARGO_PKG_NAME"),
		"-",
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("GIT_HASH")
	)
	.to_string();

	let stream = TcpStream::connect("127.0.0.1:5170")
		.await
		.context("fluent bit connect")?
		.into_std()?;
	let (non_blocking_writer, _guard) = tracing_appender::non_blocking(stream);
	let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name.clone(), non_blocking_writer);

	let subscriber = Registry::default()
		.with(EnvFilter::from_default_env())
		.with(JsonStorageLayer)
		.with(bunyan_formatting_layer)
		.with(fmt::layer().pretty());

	tracing::subscriber::set_global_default(subscriber).context("Tracing error")?;

	info!(
		"Build Version: {}	Build Date: {} 	BuildHash: {}",
		env!("CARGO_PKG_VERSION"),
		env!("BUILD_DATE"),
		env!("GIT_HASH")
	);
	//#[cfg(debug_assertions)]
	//zod_bindings::generate_zod().await?;

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

	let router = router().arced();
	#[cfg(debug_assertions)]
	router
		.export_ts(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bindings.ts"))
		.context("Binding export failed")?;

	let db: Arc<PrismaClient> = Arc::new(
		PrismaClient::_builder()
			.with_url(config.db_url.clone())
			.build()
			.await
			.context("Database ERROR")?,
	);

	#[cfg(debug_assertions)]
	db._db_push().await?;

	seed::seed_super_admin(&db, config.clone())
		.await
		.map_err(|e| eyre!("Error {e:?}"))?;

	let redis = redis::Client::open(config.redis_url.clone())?;
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
		.layer(CookieManagerLayer::new())
		.layer(TraceLayer::new_for_http());

	let port = config.api_port;
	#[cfg(debug_assertions)]
	let port = port_selector::select_from_given_port(port).expect("What");

	let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
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
