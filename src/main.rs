use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::routing::{simplify, table};

mod expressions;
mod parsing;
mod routing;
mod language;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], config::PORT));
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    let routes = simplify::router()
        .merge(table::router());

    axum::serve(listener, routes).await.unwrap();
}
