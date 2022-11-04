extern crate argon2;

mod prisma;
mod routes;
mod shutdown_signal;

use crate::eyre::{eyre, Context};
use crate::prisma::{new_client_with_url, GrammaticalForm, PrismaClient};
use crate::routes::{router, Ctx};
use axum::routing::get;
use color_eyre::eyre;
use log::{error, info};
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::cors;
use tower_http::cors::CorsLayer;

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

    let client: Arc<PrismaClient> = Arc::new(
        new_client_with_url(&url)
            .await
            .map_err(|err| eyre!("Database client error: {:?}", err))?,
    ); // Update on new release
    let router = router().arced();

    let app = axum::Router::new()
        .route("/", get(|| async { "Erudio" }))
        .route(
            "/rspc/:id",
            router
                .endpoint(move |cookies: Cookies| Ctx {
                    db: client.clone(),
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
