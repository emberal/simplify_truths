use lib::axum::app::AppBuilder;
use tower_http::cors::CorsLayer;

use crate::routing::routes::*;
use crate::routing::routes::index::not_found;

mod config;
mod expressions;
mod parsing;
mod routing;
mod utils;

#[tokio::main]
async fn main() {
    AppBuilder::new()
        .routes([index::router(), simplify::router(), table::router()])
        .fallback(not_found)
        .cors(CorsLayer::permissive())
        .serve()
        .await
        .unwrap();
}
