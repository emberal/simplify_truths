use axum::{Router, routing::get};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::expressions::expression::Expression;
use crate::expressions::simplify::Simplify;
use crate::expressions::truth_table::{TruthTable, TruthTableOptions};
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::SimplifyResponse;

pub fn router() -> Router<()> {
    Router::new()
        .nest("/simplify",
              Router::new()
                  .route("/:exp", get(simplify))
                  .route("/table/:exp", get(simplify_and_table)),
        )
}

const fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct QueryOptions {
    #[serde(default = "default_true")]
    simplify: bool,
    #[serde(default = "default_true")]
    case_sensitive: bool,
}

// TODO
async fn simplify(Path(path): Path<String>, query: Query<QueryOptions>) -> Response {
    if let Ok(mut expression) = Expression::try_from(path.as_str()) {
        let before = expression.to_string();
        if query.simplify {
            expression = expression.simplify();
        }
        SimplifyResponse {
            before,
            after: expression.to_string(),
            order_of_operations: vec![], // TODO
            expression,
            truth_table: None,
        }.into_response()
    } else {
        (StatusCode::BAD_REQUEST, Error::new("Invalid expression", ErrorKind::InvalidExpression)).into_response()
    }
}

async fn simplify_and_table(Path(path): Path<String>, query: Query<QueryOptions>) -> Response {
    if let Ok(mut expression) = Expression::try_from(path.as_str()) {
        let before = expression.to_string();
        if query.simplify {
            expression = expression.simplify();
        }
        // TODO options
        let truth_table = TruthTable::new(&expression, TruthTableOptions::default());
        SimplifyResponse {
            before,
            after: expression.to_string(),
            order_of_operations: vec![], // TODO
            expression,
            truth_table: Some(truth_table),
        }.into_response()
    } else {
        (StatusCode::BAD_REQUEST, Error::new("Invalid expression", ErrorKind::InvalidExpression)).into_response()
    }
}
