#![allow(warnings)]
pub mod testing;
pub mod json;
pub mod temporal;
pub mod regex;
pub mod net;
mod support;
pub (crate) use support::*;
pub fn init(config: Option<temper_core::Config>) -> temper_core::Result<temper_core::AsyncRunner> {
    crate::CONFIG.get_or_init(| | config.unwrap_or_else(| | temper_core::Config::default()));
    testing::init() ? ;
    json::init() ? ;
    temporal::init() ? ;
    regex::init() ? ;
    net::init() ? ;
    Ok(crate::config().runner().clone())
}
