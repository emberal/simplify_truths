pub const PORT: u16 = 8000;
pub const IS_DEV: bool = cfg!(debug_assertions);
pub const RESOURCE_DIR: &str = if IS_DEV {
    "./src/resources/static"
} else {
    "./static"
};
