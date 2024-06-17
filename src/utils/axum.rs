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
