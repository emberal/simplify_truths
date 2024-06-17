use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::expressions::expression::Expression;
use crate::expressions::truth_table::TruthTable;
use crate::{router, routes};
use crate::routing::error::{Error, ErrorKind};
use crate::routing::options::{SimplifyAndTableOptions, SimplifyOptions};
use crate::routing::response::SimplifyResponse;

router!("/simplify", routes!(
    get "/:exp" => simplify,
    get "/table/:exp" => simplify_and_table
));

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

async fn simplify_and_table(Path(path): Path<String>, Query(query): Query<SimplifyAndTableOptions>) -> Response {
    match Expression::try_from(path.as_str()) {
        Ok(mut expression) => {
            let before = expression.to_string();
            let mut operations = vec![];
            if query.simplify_options.simplify {
                (expression, operations) = expression.simplify();
            }
            let truth_table = TruthTable::new(&expression, query.table_options);
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
