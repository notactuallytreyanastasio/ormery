#![allow(warnings)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
fn appendHex__3(n__7: i32, sb__8: std::sync::Arc<std::sync::RwLock<String>>) {
    let mut t___26: i32;
    let i__10: i32 = n__7 & 15;
    'ok___5787: {
        'orelse___11: {
            if Some(i__10) < Some(10) {
                t___26 = 48;
            } else {
                t___26 = 87;
            }
            match temper_core::string::builder::append_code_point( & sb__8, i__10.wrapping_add(t___26)) {
                Ok(x) => x,
                _ => break 'orelse___11
            };
            break 'ok___5787;
        }
        return panic!();
    }
}
pub fn percent_escape_octet_to(octet__4: i32, sb__5: std::sync::Arc<std::sync::RwLock<String>>) {
    temper_core::string::builder::append( & sb__5, "%");
    let mut t___21: i32 = (octet__4 & 255).wrapping_div(16);
    appendHex__3(t___21, sb__5.clone());
    let mut t___23: i32 = octet__4 & 15;
    appendHex__3(t___23, sb__5.clone());
}
