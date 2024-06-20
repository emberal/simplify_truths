use std::net::SocketAddr;

use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::routing::routes::*;
use crate::routing::routes::index::not_found;

mod expressions;
mod parsing;
mod routing;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(config::SOCKET);
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let routes = simplify::router()
        .merge(table::router())
        .merge(index::router())
        .fallback(not_found);

    let app = routes
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new()
                .level(Level::INFO))
        );

    tracing::info!("Starting server on: {addr}");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
