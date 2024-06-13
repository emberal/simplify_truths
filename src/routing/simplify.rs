use axum::{Json, Router, routing::get};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::expressions::expression::Expression;
use crate::expressions::simplify::Simplify;
use crate::expressions::truth_table::{TruthTable, TruthTableOptions};
use crate::language::{AcceptLanguage, Language};

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

#[derive(Deserialize, Debug)]
struct QueryOptions {
    #[serde(default)]
    lang: Language,
    #[serde(default = "default_true")]
    simplify: bool,
    #[serde(default = "default_true")]
    case_sensitive: bool,
}

#[derive(Serialize)]
enum Law {
    // TODO
}

#[derive(Serialize)]
struct OrderOfOperation {
    before: String,
    after: String,
    law: Law, // TODO
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SimplifyResponse {
    version: String,  // TODO better versioning
    before: String,
    after: String,
    order_of_operations: Vec<OrderOfOperation>,
    expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    truth_table: Option<TruthTable>,
}

#[derive(Serialize)]
struct Error {
    message: String,
}

// TODO
async fn simplify(Path(path): Path<String>, query: Query<QueryOptions>, accept_language: Option<AcceptLanguage>) -> Response {
    if let Ok(mut expression) = Expression::try_from(path.as_str()) {
        let before = expression.to_string();
        if query.simplify {
            expression = expression.simplify();
        }
        Json(SimplifyResponse {
            version: "2.0.0".to_string(),
            before,
            after: expression.to_string(),
            order_of_operations: vec![], // TODO
            expression,
            truth_table: None,
        }).into_response()
    } else {
        (StatusCode::BAD_REQUEST, Json(Error { message: "Invalid expression".into() })).into_response()
    }
}

async fn simplify_and_table(Path(path): Path<String>, query: Query<QueryOptions>, accept_language: Option<AcceptLanguage>) -> Response {
    if let Ok(mut expression) = Expression::try_from(path.as_str()) {
        let before = expression.to_string();
        if query.simplify {
            expression = expression.simplify();
        }
        // TODO options
        let truth_table = TruthTable::new(&expression, TruthTableOptions::default());
        Json(SimplifyResponse {
            version: "2.0.0".to_string(),
            before,
            after: expression.to_string(),
            order_of_operations: vec![], // TODO
            expression,
            truth_table: Some(truth_table),
        }).into_response()
    } else {
        (StatusCode::BAD_REQUEST, Json(Error { message: "Invalid expression".into() })).into_response()
    }
}
