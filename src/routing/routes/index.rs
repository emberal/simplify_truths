use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::{load_html, router};
use crate::expressions::expression::Expression;
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::IsValidResponse;

router!(
    get "/" => index,
    get "/openapi" => open_api,
    get "/is-valid/:exp" => is_valid
);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Info {
    message: &'static str,
    docs: &'static str,
    created_by: String,
}

async fn index() -> Json<Info> {
    let author = env!("CARGO_PKG_AUTHORS");
    Json(Info {
        message: "Welcome to the Simplify Truths API!",
        docs: "The API documentation can be found at /openapi",
        created_by: format!("Created by: {}", author),
    })
}

async fn open_api() -> impl IntoResponse {
    load_html!("openapi.html")
}

async fn is_valid(Path(path): Path<String>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(_) => IsValidResponse::valid().into_response(),
        Err(error) => Error::new(error.to_string(), ErrorKind::InvalidExpression).into_response()
    }
}

pub(crate) async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, load_html!("not-found.html"))
}
