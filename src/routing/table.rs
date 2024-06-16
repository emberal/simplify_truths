use axum::response::Response;
use axum::Router;
use axum::routing::post;

pub fn router() -> Router<()> {
    Router::new()
        .nest("/table", Router::new()
            .route("/", post(table)),
        )
}

async fn table() -> Response {
    unimplemented!()
}
