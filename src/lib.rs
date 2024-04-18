#![allow(dead_code)]

pub const DEFAUTL_NAMESPACE_NAME: &str = "keyv";

mod keyv;
pub use keyv::*;

mod store;
pub use store::*;
