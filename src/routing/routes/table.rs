use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;

use crate::expressions::expression::Expression;
use crate::expressions::truth_table::TruthTable;
use crate::routing::error::{Error, ErrorKind};
use crate::routing::options::TruthTableOptions;
use crate::routing::response::TruthTableResponse;

pub fn router() -> Router<()> {
    Router::new()
        .nest("/table", Router::new()
            .route("/:exp", get(table)),
        )
}

// TODO Expression as input in body
async fn table(Path(value): Path<String>, Query(query): Query<TruthTableOptions>) -> Response {
    match Expression::try_from(value) {
        Ok(expression) => {
            TruthTableResponse { truth_table: TruthTable::new(&expression, query) }.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Error::new(e.to_string(), ErrorKind::InvalidExpression)).into_response(),
    }
}
