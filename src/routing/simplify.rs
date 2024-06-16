use axum::{Router, routing::get};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize};

use crate::expressions::expression::Expression;
use crate::expressions::truth_table::{Hide, Sort, TruthTable, TruthTableOptions};
use crate::routing::error::{Error, ErrorKind};
use crate::routing::response::SimplifyResponse;
use crate::utils::serialize::{ret_true, deserialize_bool};

pub fn router() -> Router<()> {
    Router::new()
        .nest("/simplify",
              Router::new()
                  .route("/:exp", get(simplify))
                  .route("/table/:exp", get(simplify_and_table)),
        )
}

#[derive(Deserialize)]
struct SimplifyOptions {
    #[serde(
        default = "ret_true",
        deserialize_with = "deserialize_bool"
    )]
    simplify: bool,
    #[serde(default = "ret_true")]
    case_sensitive: bool, // TODO: Implement case sensitivity
}

async fn simplify(Path(path): Path<String>, Query(query): Query<SimplifyOptions>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(mut expression) => {
            let before = expression.to_string();
            let mut operations = vec![];
            if query.simplify {
                (expression, operations) = expression.simplify();
            }
            SimplifyResponse {
                before,
                after: expression.to_string(),
                operations,
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
    #[serde(default)]
    hide_intermediate_steps: bool, // TODO
}

async fn simplify_and_table(Path(path): Path<String>, Query(query): Query<SimplifyAndTableQuery>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(mut expression) => {
            let before = expression.to_string();
            let mut operations = vec![];
            if query.simplify_options.simplify {
                (expression, operations) = expression.simplify();
            }
            let truth_table = TruthTable::new(&expression, TruthTableOptions {
                sort: query.sort,
                hide: query.hide,
            });
            SimplifyResponse {
                before,
                after: expression.to_string(),
                operations,
                expression,
                truth_table: Some(truth_table),
            }.into_response()
        }
        Err(error) => {
            (StatusCode::BAD_REQUEST, Error::new(error.to_string(), ErrorKind::InvalidExpression)).into_response()
        }
    }
}
