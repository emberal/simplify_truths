/// Load an HTML file from the given file path, relative to the resource directory.
/// The file is loading on compile time as a string literal.
/// # Arguments
/// * `filename` - The path to the HTML file.
/// # Returns
/// The HTML file as a `Html` object containing the content-type 'text/html'.
/// # Examples
/// ```
/// let html = load_html!("openapi.html");
/// ```
#[macro_export]
macro_rules! load_html {
    ($filename:expr) => {
        lib::load_html!($crate::absolute_path!($filename), "{{docs}}" => $crate::config::OPENAPI_PATH)
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! absolute_path {
    ($filename:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/resources/static/", $filename)
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! absolute_path {
    ($filename:literal) => {
        concat!("/static/", $filename)
    };
}
