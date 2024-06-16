use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;

use crate::expressions::expression::Expression;
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::IsLegalResponse;

pub fn router() -> Router<()> {
    Router::new()
        .route("/is-legal/:exp", get(is_legal))
}

async fn is_legal(Path(path): Path<String>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(_) => IsLegalResponse { is_legal: true }.into_response(),
        Err(error) => Error::new(error.to_string(), ErrorKind::InvalidExpression).into_response()
    }
}
