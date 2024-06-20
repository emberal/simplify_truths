use axum::body::Body;
use axum::response::Html;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::config::RESOURCE_DIR;

/// Create an axum router function with the given body or routes.
/// # Examples
/// ```
/// router!(
///     get "/" => index,
///     get "/openapi" => open_api
/// );
/// router!("/simplify", routes!(
///     get "/:exp" => simplify,
///     get "/table/:exp" => simplify_and_table
/// ));
/// ```
#[macro_export]
macro_rules! router {
    ($body:expr) => {
        pub(crate) fn router() -> axum::Router<()> {
            $body
        }
    };
    ($route:expr, $router:expr) => {
        router!(axum::Router::new().nest($route, $router));
    };
    ($($method:ident $route:expr => $func:expr),* $(,)?) => {
        router!($crate::routes!($($method $route => $func),*));
    };
}

/// Create a router with the given routes.
/// # Examples
/// ```
/// routes!(
///     get "/" => index,
///     post "/" => create
/// );
/// ```
#[macro_export]
macro_rules! routes {
    ($($method:ident $route:expr => $func:expr),* $(,)?) => {
        axum::Router::new()
            $(.route($route, axum::routing::$method($func)))*
    };
}

/// Load an HTML file from the given file path, relative to the resource directory.
/// # Arguments
/// * `file_path` - The path to the HTML file.
/// # Returns
/// The HTML file as a `Html` object containing the content-type 'text/html' or an error message if the file is not found or cannot be read.
/// # Examples
/// ```
/// let html = load_html("openapi.html").await.unwrap();
/// ```
pub async fn load_html(file_path: &str) -> Result<Html<Body>, String> {
    let file = match File::open(format!("{}/{}", RESOURCE_DIR, file_path)).await {
        Ok(file) => file,
        Err(err) => return Err(format!("File not found: {err}")),
    };
    let stream = ReaderStream::new(file);
    Ok(Html(Body::from_stream(stream)))
}

#[macro_export]
macro_rules! load_html {
    ($filename:literal) => {
        axum::response::Html(
            axum::body::Body::new(
                $crate::absolute_path!($filename)
            )
        )
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! absolute_path {
    ($filename:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/resources/static/", $filename))
            .replace("{{docs}}", "/openapi")
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! absolute_path {
    ($filename:literal) => {
        include_str!(concat!("/static/", $filename))
            .replace("{{docs}}", "simplify-truths/v2/openapi")
    };
}
