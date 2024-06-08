use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::routing::{index, simplify, table};

mod expressions;
mod parsing;
mod routing;
mod language;
mod config;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], config::PORT));
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    let routes = simplify::router()
        .merge(table::router())
        .merge(index::router());

    axum::serve(listener, routes).await.unwrap();
}
