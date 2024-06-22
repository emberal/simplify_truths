#![allow(dead_code)]

use std::net::Ipv4Addr;

pub const IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const PORT: u16 = 8000;
pub const SOCKET: (Ipv4Addr, u16) = (IP, PORT);
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
