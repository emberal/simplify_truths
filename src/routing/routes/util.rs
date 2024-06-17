use axum::extract::Path;
use axum::response::{IntoResponse, Response};

use crate::expressions::expression::Expression;
use crate::router;
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::IsLegalResponse;

router!(
    get "/is-legal/:exp" => is_legal
);

async fn is_legal(Path(path): Path<String>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(_) => IsLegalResponse { is_legal: true }.into_response(),
        Err(error) => Error::new(error.to_string(), ErrorKind::InvalidExpression).into_response()
    }
}
