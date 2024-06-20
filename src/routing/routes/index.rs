use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{load_html, router};
use crate::expressions::expression::Expression;
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::IsLegalResponse;

router!(
    get "/" => index,
    get "/openapi" => open_api,
    get "/is-valid/:exp" => is_valid
);

async fn index() -> &'static str {
    "Welcome to the Simplify Truths API!\n"
}

async fn open_api() -> Response {
    load_html!("openapi.html").into_response()
}

async fn is_valid(Path(path): Path<String>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(_) => IsLegalResponse { is_legal: true }.into_response(),
        Err(error) => Error::new(error.to_string(), ErrorKind::InvalidExpression).into_response()
    }
}

pub(crate) async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, load_html!("not-found.html")).into_response()
}
