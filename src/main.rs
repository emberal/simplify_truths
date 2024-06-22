use std::net::SocketAddr;
use lib::{create_app, join_routes};

use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::routing::routes::*;

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

    let routes = join_routes![
        simplify::router(),
        index::router(),
        table::router()
    ].fallback(index::not_found);

    // TODO layer to remove trailing slashes
    let app = create_app!(routes,
        CorsLayer::new().allow_origin(Any),
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
    );

    tracing::info!("Starting server on: {addr}");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
