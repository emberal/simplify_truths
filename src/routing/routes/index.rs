use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::expressions::expression::Expression;
use crate::router;
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::IsLegalResponse;
use crate::utils::axum::load_html;

router!(
    get "/" => index,
    get "/openapi" => open_api,
    get "/is-valid/:exp" => is_valid
);

async fn index() -> &'static str {
    "Welcome to the Simplify Truths API!\n"
}

async fn open_api() -> Response {
    match load_html("openapi.html").await {
        Ok(html) => html.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    }
}

async fn is_valid(Path(path): Path<String>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(_) => IsLegalResponse { is_legal: true }.into_response(),
        Err(error) => Error::new(error.to_string(), ErrorKind::InvalidExpression).into_response()
    }
}

pub(crate) async fn not_found() -> Response {
    match load_html("not-found.html").await {
        Ok(html) => (StatusCode::NOT_FOUND, html).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    }
}
