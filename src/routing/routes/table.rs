use axum::body::Body;
use axum::response::Response;
use axum::Router;
use axum::routing::post;

pub fn router() -> Router<()> {
    Router::new()
        .nest("/table", Router::new()
            .route("/", post(table)),
        )
}

// TODO Json Deserialize not working on Axum? Manually parse the body?
async fn table(body: Body) -> Response {
    unimplemented!()
}
