#![allow(warnings)]
pub mod sql;
mod r#mod;
pub use r#mod::*;
mod support;
pub (crate) use support::*;
pub fn init(config: Option<temper_core::Config>) -> temper_core::Result<temper_core::AsyncRunner> {
    crate::CONFIG.get_or_init(| | config.unwrap_or_else(| | temper_core::Config::default()));
    temper_std::init(Some(crate::config().clone())) ? ;
    sql::init() ? ;
    r#mod::init() ? ;
    Ok(crate::config().runner().clone())
}
