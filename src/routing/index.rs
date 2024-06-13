use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
}

async fn index() -> &'static str {
    "Welcome to the Simplify Truths API!\n"
}
