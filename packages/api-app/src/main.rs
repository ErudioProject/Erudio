extern crate argon2;

mod prisma;
mod routes;

use prisma_client_rust::NewClientError;
use crate::prisma::{GrammaticalForm, new_client, PrismaClient};
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use axum::extract::Path;
use axum::routing::get;
use color_eyre::eyre;
use log::{debug, error, info};
use rspc::Config;
use tokio::signal;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::cors;
use tower_http::cors::CorsLayer;
use crate::routes::{Ctx, router};


// TODO clean up a bit
pub fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let result = start();
    if let Some(err) = result.err() {
        error!("{}", err);
    }
}


#[tokio::main]
async fn start() -> eyre::Result<()> {
    let client: Arc<PrismaClient> = Arc::new(new_client().await?); // Update on new release
    let router = router().arced();

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .route("/rspc/:id", router.endpoint(move |cookies: Cookies| {
            Ctx {
                db: client.clone(),
                cookies
            }
        }).axum())
        .layer(
            CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_headers(cors::Any)
                .allow_methods(cors::Any),
        )
        .layer(CookieManagerLayer::new());;

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));

    info!("listening on {}", addr);
    axum::Server::try_bind(&addr)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
