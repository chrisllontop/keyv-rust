#![allow(dead_code)]

pub const DEFAUTL_TABLE_NAME: &str = "keyv";

mod keyv;
pub use keyv::*;

mod store;
pub use store::*;
