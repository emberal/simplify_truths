use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::router;

router!(
    get "/" => index,
    get "/openapi" => open_api
);

async fn index() -> &'static str {
    "Welcome to the Simplify Truths API!\n"
}

async fn open_api() -> Response {
    let file_path = if cfg!(debug_assertions) {
        "./spec/dist/index.html"
    } else {
        "./openapi/index.html"
    };
    let file = match File::open(file_path).await {
        Ok(file) => file,
        Err(err) => return (StatusCode::NOT_FOUND, format!("File not found: {err}")).into_response(),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Html(body).into_response()
}
