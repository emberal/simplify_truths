#![allow(dead_code)]
pub const IS_DEV: bool = cfg!(debug_assertions);
pub const RESOURCE_DIR: &str = if IS_DEV {
    "./src/resources/static"
} else {
    "./static"
};
pub const OPENAPI_PATH: &str = if IS_DEV {
    "/openapi"
} else {
    "/simplify-truths/v2/openapi"
};
