use axum::{Router, routing::get};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::expressions::expression::Expression;
use crate::expressions::simplify::Simplify;
use crate::expressions::truth_table::{Hide, Sort, TruthTable, TruthTableOptions};
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
struct SimplifyOptions {
    #[serde(default = "default_true")]
    simplify: bool,
    #[serde(default = "default_true")]
    case_sensitive: bool,
}

// TODO
async fn simplify(Path(path): Path<String>, Query(query): Query<SimplifyOptions>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(mut expression) => {
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
        }
        Err(error) => {
            (StatusCode::BAD_REQUEST, Error::new(error.to_string(), ErrorKind::InvalidExpression)).into_response()
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SimplifyAndTableQuery {
    #[serde(flatten)]
    simplify_options: SimplifyOptions,
    #[serde(default)]
    sort: Sort,
    #[serde(default)]
    hide: Hide,
}

async fn simplify_and_table(Path(path): Path<String>, Query(query): Query<SimplifyAndTableQuery>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(mut expression) => {
            let before = expression.to_string();
            if query.simplify_options.simplify {
                expression = expression.simplify();
            }
            let truth_table = TruthTable::new(&expression, TruthTableOptions {
                sort: query.sort,
                hide: query.hide,
            });
            SimplifyResponse {
                before,
                after: expression.to_string(),
                order_of_operations: vec![], // TODO
                expression,
                truth_table: Some(truth_table),
            }.into_response()
        }
        Err(error) => {
            (StatusCode::BAD_REQUEST, Error::new(error.to_string(), ErrorKind::InvalidExpression)).into_response()
        }
    }
}
