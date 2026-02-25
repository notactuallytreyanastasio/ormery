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
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__96: impl temper_core::ToArcString) {
        let sqlSource__96 = sqlSource__96.to_arc_string();
        let mut t___1163: SqlSource = SqlSource::new(sqlSource__96.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1163.clone()), None);
    }
    pub fn append_fragment(& self, fragment__99: SqlFragment) {
        let mut t___1161: temper_core::List<SqlPart> = fragment__99.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___1161.clone()), None);
    }
    pub fn append_part(& self, part__102: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__102.clone(), None);
    }
    pub fn append_part_list(& self, values__105: impl temper_core::ToList<SqlPart>) {
        let values__105 = values__105.to_list();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__12: SqlBuilder
        }
        impl ClosureGroup___1 {
            fn fn__1157(& self, x__107: SqlPart) {
                self.this__12.append_part(x__107.clone());
            }
        }
        let closure_group = ClosureGroup___1 {
            this__12: self.clone()
        };
        let fn__1157 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__107: SqlPart | closure_group.fn__1157(x__107))
        };
        self.append_list(temper_core::ToListed::to_listed(values__105.clone()), fn__1157.clone());
    }
    pub fn append_boolean(& self, value__109: bool) {
        let mut t___1154: SqlBoolean = SqlBoolean::new(value__109);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1154.clone()), None);
    }
    pub fn append_boolean_list(& self, values__112: impl temper_core::ToListed<bool>) {
        let values__112 = values__112.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___2 {
            this__14: SqlBuilder
        }
        impl ClosureGroup___2 {
            fn fn__1151(& self, x__114: bool) {
                self.this__14.append_boolean(x__114);
            }
        }
        let closure_group = ClosureGroup___2 {
            this__14: self.clone()
        };
        let fn__1151 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__114: bool | closure_group.fn__1151(x__114))
        };
        self.append_list(values__112.clone(), fn__1151.clone());
    }
    pub fn append_date(& self, value__116: temper_std::temporal::Date) {
        let mut t___1148: SqlDate = SqlDate::new(value__116.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1148.clone()), None);
    }
    pub fn append_date_list(& self, values__119: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__119 = values__119.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___3 {
            this__16: SqlBuilder
        }
        impl ClosureGroup___3 {
            fn fn__1145(& self, x__121: temper_std::temporal::Date) {
                self.this__16.append_date(x__121.clone());
            }
        }
        let closure_group = ClosureGroup___3 {
            this__16: self.clone()
        };
        let fn__1145 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__121: temper_std::temporal::Date | closure_group.fn__1145(x__121))
        };
        self.append_list(values__119.clone(), fn__1145.clone());
    }
    pub fn append_float64(& self, value__123: f64) {
        let mut t___1142: SqlFloat64 = SqlFloat64::new(value__123);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1142.clone()), None);
    }
    pub fn append_float64_list(& self, values__126: impl temper_core::ToListed<f64>) {
        let values__126 = values__126.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___4 {
            this__18: SqlBuilder
        }
        impl ClosureGroup___4 {
            fn fn__1139(& self, x__128: f64) {
                self.this__18.append_float64(x__128);
            }
        }
        let closure_group = ClosureGroup___4 {
            this__18: self.clone()
        };
        let fn__1139 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__128: f64 | closure_group.fn__1139(x__128))
        };
        self.append_list(values__126.clone(), fn__1139.clone());
    }
    pub fn append_int32(& self, value__130: i32) {
        let mut t___1136: SqlInt32 = SqlInt32::new(value__130);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1136.clone()), None);
    }
    pub fn append_int32_list(& self, values__133: impl temper_core::ToListed<i32>) {
        let values__133 = values__133.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___5 {
            this__20: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__1133(& self, x__135: i32) {
                self.this__20.append_int32(x__135);
            }
        }
        let closure_group = ClosureGroup___5 {
            this__20: self.clone()
        };
        let fn__1133 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__135: i32 | closure_group.fn__1133(x__135))
        };
        self.append_list(values__133.clone(), fn__1133.clone());
    }
    pub fn append_int64(& self, value__137: i64) {
        let mut t___1130: SqlInt64 = SqlInt64::new(value__137);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1130.clone()), None);
    }
    pub fn append_int64_list(& self, values__140: impl temper_core::ToListed<i64>) {
        let values__140 = values__140.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___6 {
            this__22: SqlBuilder
        }
        impl ClosureGroup___6 {
            fn fn__1127(& self, x__142: i64) {
                self.this__22.append_int64(x__142);
            }
        }
        let closure_group = ClosureGroup___6 {
            this__22: self.clone()
        };
        let fn__1127 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__142: i64 | closure_group.fn__1127(x__142))
        };
        self.append_list(values__140.clone(), fn__1127.clone());
    }
    pub fn append_string(& self, value__144: impl temper_core::ToArcString) {
        let value__144 = value__144.to_arc_string();
        let mut t___1124: SqlString = SqlString::new(value__144.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___1124.clone()), None);
    }
    pub fn append_string_list(& self, values__147: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__147 = values__147.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___7 {
            this__24: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__1121(& self, x__149: impl temper_core::ToArcString) {
                let x__149 = x__149.to_arc_string();
                self.this__24.append_string(x__149.clone());
            }
        }
        let closure_group = ClosureGroup___7 {
            this__24: self.clone()
        };
        let fn__1121 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__149: std::sync::Arc<String> | closure_group.fn__1121(x__149))
        };
        self.append_list(values__147.clone(), fn__1121.clone());
    }
    fn append_list<T>(& self, values__151: impl temper_core::ToListed<T>, appendValue__152: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__151 = values__151.to_listed();
        let mut t___1116: i32;
        let mut t___1118: T;
        let mut i__154: i32 = 0;
        'loop___5906: loop {
            t___1116 = temper_core::ListedTrait::len( & ( * values__151));
            if ! (Some(i__154) < Some(t___1116)) {
                break;
            }
            if Some(i__154) > Some(0) {
                self.append_safe(", ");
            }
            t___1118 = temper_core::ListedTrait::get( & ( * values__151), i__154);
            appendValue__152(t___1118.clone());
            i__154 = i__154.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___1113: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___1113.clone();
        let selfish = SqlBuilder(std::sync::Arc::new(SqlBuilderStruct {
                    buffer
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(SqlBuilder, []);
struct SqlFragmentStruct {
    parts: temper_core::List<SqlPart>
}
#[derive(Clone)]
pub struct SqlFragment(std::sync::Arc<SqlFragmentStruct>);
impl SqlFragment {
    pub fn to_source(& self) -> SqlSource {
        return SqlSource::new(self.to_string());
    }
    pub fn to_string(& self) -> std::sync::Arc<String> {
        let mut t___1186: i32;
        let builder__166: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__167: i32 = 0;
        'loop___5907: loop {
            t___1186 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__167) < Some(t___1186)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__167).format_to(builder__166.clone());
            i__167 = i__167.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__166);
    }
    pub fn new(parts__169: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__169 = parts__169.to_list();
        let parts;
        parts = parts__169.clone();
        let selfish = SqlFragment(std::sync::Arc::new(SqlFragmentStruct {
                    parts
        }));
        return selfish;
    }
    pub fn parts(& self) -> temper_core::List<SqlPart> {
        return self.0.parts.clone();
    }
}
temper_core::impl_any_value_trait!(SqlFragment, []);
pub enum SqlPartEnum {
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlDate(SqlDate), SqlFloat64(SqlFloat64), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlString(SqlString)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__171: std::sync::Arc<std::sync::RwLock<String>>);
}
#[derive(Clone)]
pub struct SqlPart(std::sync::Arc<dyn SqlPartTrait>);
impl SqlPart {
    pub fn new(selfish: impl SqlPartTrait + 'static) -> SqlPart {
        SqlPart(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(SqlPart);
impl std::ops::Deref for SqlPart {
    type Target = dyn SqlPartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct SqlSourceStruct {
    source: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlSource(std::sync::Arc<SqlSourceStruct>);
impl SqlSource {
    pub fn format_to(& self, builder__175: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__175, self.0.source.clone());
    }
    pub fn new(source__178: impl temper_core::ToArcString) -> SqlSource {
        let source__178 = source__178.to_arc_string();
        let source;
        source = source__178.clone();
        let selfish = SqlSource(std::sync::Arc::new(SqlSourceStruct {
                    source
        }));
        return selfish;
    }
    pub fn source(& self) -> std::sync::Arc<String> {
        return self.0.source.clone();
    }
}
impl SqlPartTrait for SqlSource {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlSource(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__175: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__175)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__181: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___718: std::sync::Arc<String>;
        if self.0.value {
            t___718 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___718 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__181, t___718.clone());
    }
    pub fn new(value__184: bool) -> SqlBoolean {
        let value;
        value = value__184;
        let selfish = SqlBoolean(std::sync::Arc::new(SqlBooleanStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> bool {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlBoolean {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlBoolean(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__181: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__181)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__187: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__187, "'");
        let s__189: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___8 {
            builder__187: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___8 {
            fn fn__1178(& self, c__190: i32) {
                if Some(c__190) == Some(39) {
                    temper_core::string::builder::append( & self.builder__187, "''");
                } else {
                    'ok___5768: {
                        'orelse___326: {
                            match temper_core::string::builder::append_code_point( & self.builder__187, c__190) {
                                Ok(x) => x,
                                _ => break 'orelse___326
                            };
                            break 'ok___5768;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___8 {
            builder__187: builder__187.clone()
        };
        let fn__1178 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__190: i32 | closure_group.fn__1178(c__190))
        };
        temper_core::string::for_each( & s__189, & ( * fn__1178.clone()));
        temper_core::string::builder::append( & builder__187, "'");
    }
    pub fn new(value__192: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__192.clone();
        let selfish = SqlDate(std::sync::Arc::new(SqlDateStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> temper_std::temporal::Date {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlDate {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDate(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__187: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__187)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__195: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___1175: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        temper_core::string::builder::append( & builder__195, t___1175.clone());
    }
    pub fn new(value__198: f64) -> SqlFloat64 {
        let value;
        value = value__198;
        let selfish = SqlFloat64(std::sync::Arc::new(SqlFloat64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> f64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlFloat64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlFloat64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__195: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__195)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__201: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___1173: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__201, t___1173.clone());
    }
    pub fn new(value__204: i32) -> SqlInt32 {
        let value;
        value = value__204;
        let selfish = SqlInt32(std::sync::Arc::new(SqlInt32Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i32 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt32 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt32(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__201: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__201)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__207: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___1171: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__207, t___1171.clone());
    }
    pub fn new(value__210: i64) -> SqlInt64 {
        let value;
        value = value__210;
        let selfish = SqlInt64(std::sync::Arc::new(SqlInt64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__207: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__207)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__213: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__213, "'");
        #[derive(Clone)]
        struct ClosureGroup___9 {
            builder__213: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___9 {
            fn fn__1166(& self, c__215: i32) {
                if Some(c__215) == Some(39) {
                    temper_core::string::builder::append( & self.builder__213, "''");
                } else {
                    'ok___5774: {
                        'orelse___327: {
                            match temper_core::string::builder::append_code_point( & self.builder__213, c__215) {
                                Ok(x) => x,
                                _ => break 'orelse___327
                            };
                            break 'ok___5774;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___9 {
            builder__213: builder__213.clone()
        };
        let fn__1166 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__215: i32 | closure_group.fn__1166(c__215))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__1166.clone()));
        temper_core::string::builder::append( & builder__213, "'");
    }
    pub fn new(value__217: impl temper_core::ToArcString) -> SqlString {
        let value__217 = value__217.to_arc_string();
        let value;
        value = value__217.clone();
        let selfish = SqlString(std::sync::Arc::new(SqlStringStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlString {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlString(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__213: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__213)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
#[cfg(test)]
mod tests {
    #[test]
    fn stringEscaping__268() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___4 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___10 {}
        impl ClosureGroup___10 {
            fn build__219(& self, name__221: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__221 = name__221.to_arc_string();
                let mut t___1095: SqlBuilder = SqlBuilder::new();
                t___1095.append_safe("select * from hi where name = ");
                t___1095.append_string(name__221.clone());
                return t___1095.accumulated().to_string();
            }
            fn buildWrong__220(& self, name__223: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__223 = name__223.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__223.clone()));
            }
        }
        let closure_group = ClosureGroup___10 {};
        let build__219 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__221: std::sync::Arc<String> | closure_group.build__219(name__221))
        };
        let buildWrong__220 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__223: std::sync::Arc<String> | closure_group.buildWrong__220(name__223))
        };
        let actual___270: std::sync::Arc<String> = build__219(std::sync::Arc::new("world".to_string()));
        let mut t___1105: bool = Some(actual___270.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___11 {
            actual___270: std::sync::Arc<String>
        }
        impl ClosureGroup___11 {
            fn fn__1102(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___270.clone()));
            }
        }
        let closure_group = ClosureGroup___11 {
            actual___270: actual___270.clone()
        };
        let fn__1102 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1102())
        };
        test___4.assert(t___1105, fn__1102.clone());
        let bobbyTables__225: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___272: std::sync::Arc<String> = build__219(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___1109: bool = Some(actual___272.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___12 {
            actual___272: std::sync::Arc<String>
        }
        impl ClosureGroup___12 {
            fn fn__1101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___272.clone()));
            }
        }
        let closure_group = ClosureGroup___12 {
            actual___272: actual___272.clone()
        };
        let fn__1101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1101())
        };
        test___4.assert(t___1109, fn__1101.clone());
        #[derive(Clone)]
        struct ClosureGroup___13 {}
        impl ClosureGroup___13 {
            fn fn__1100(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___13 {};
        let fn__1100 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1100())
        };
        test___4.assert(true, fn__1100.clone());
        test___4.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__276() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___5 = temper_std::testing::Test::new();
        let mut t___1063: SqlBuilder = SqlBuilder::new();
        t___1063.append_safe("v = ");
        t___1063.append_string("");
        let actual___277: std::sync::Arc<String> = t___1063.accumulated().to_string();
        let mut t___1069: bool = Some(actual___277.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___14 {
            actual___277: std::sync::Arc<String>
        }
        impl ClosureGroup___14 {
            fn fn__1062(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___277.clone()));
            }
        }
        let closure_group = ClosureGroup___14 {
            actual___277: actual___277.clone()
        };
        let fn__1062 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1062())
        };
        test___5.assert(t___1069, fn__1062.clone());
        let mut t___1071: SqlBuilder = SqlBuilder::new();
        t___1071.append_safe("v = ");
        t___1071.append_string("a''b");
        let actual___280: std::sync::Arc<String> = t___1071.accumulated().to_string();
        let mut t___1077: bool = Some(actual___280.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___15 {
            actual___280: std::sync::Arc<String>
        }
        impl ClosureGroup___15 {
            fn fn__1061(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___280.clone()));
            }
        }
        let closure_group = ClosureGroup___15 {
            actual___280: actual___280.clone()
        };
        let fn__1061 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1061())
        };
        test___5.assert(t___1077, fn__1061.clone());
        let mut t___1079: SqlBuilder = SqlBuilder::new();
        t___1079.append_safe("v = ");
        t___1079.append_string("Hello 世界");
        let actual___283: std::sync::Arc<String> = t___1079.accumulated().to_string();
        let mut t___1085: bool = Some(actual___283.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___16 {
            actual___283: std::sync::Arc<String>
        }
        impl ClosureGroup___16 {
            fn fn__1060(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___283.clone()));
            }
        }
        let closure_group = ClosureGroup___16 {
            actual___283: actual___283.clone()
        };
        let fn__1060 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1060())
        };
        test___5.assert(t___1085, fn__1060.clone());
        let mut t___1087: SqlBuilder = SqlBuilder::new();
        t___1087.append_safe("v = ");
        t___1087.append_string("Line1\x0aLine2");
        let actual___286: std::sync::Arc<String> = t___1087.accumulated().to_string();
        let mut t___1093: bool = Some(actual___286.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___17 {
            actual___286: std::sync::Arc<String>
        }
        impl ClosureGroup___17 {
            fn fn__1059(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___286.clone()));
            }
        }
        let closure_group = ClosureGroup___17 {
            actual___286: actual___286.clone()
        };
        let fn__1059 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1059())
        };
        test___5.assert(t___1093, fn__1059.clone());
        test___5.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__289() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___6 = temper_std::testing::Test::new();
        let mut t___624: temper_std::temporal::Date;
        let mut t___1034: SqlBuilder = SqlBuilder::new();
        t___1034.append_safe("select ");
        t___1034.append_int32(42);
        t___1034.append_safe(", ");
        t___1034.append_int64(43);
        t___1034.append_safe(", ");
        t___1034.append_float64(19.99f64);
        t___1034.append_safe(", ");
        t___1034.append_boolean(true);
        t___1034.append_safe(", ");
        t___1034.append_boolean(false);
        let actual___290: std::sync::Arc<String> = t___1034.accumulated().to_string();
        let mut t___1048: bool = Some(actual___290.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            actual___290: std::sync::Arc<String>
        }
        impl ClosureGroup___18 {
            fn fn__1033(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___290.clone()));
            }
        }
        let closure_group = ClosureGroup___18 {
            actual___290: actual___290.clone()
        };
        let fn__1033 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1033())
        };
        test___6.assert(t___1048, fn__1033.clone());
        let date__228: temper_std::temporal::Date;
        'ok___5775: {
            'orelse___328: {
                t___624 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___328
                };
                date__228 = t___624.clone();
                break 'ok___5775;
            }
            date__228 = panic!();
        }
        let mut t___1050: SqlBuilder = SqlBuilder::new();
        t___1050.append_safe("insert into t values (");
        t___1050.append_date(date__228.clone());
        t___1050.append_safe(")");
        let actual___293: std::sync::Arc<String> = t___1050.accumulated().to_string();
        let mut t___1057: bool = Some(actual___293.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___19 {
            actual___293: std::sync::Arc<String>
        }
        impl ClosureGroup___19 {
            fn fn__1032(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___293.clone()));
            }
        }
        let closure_group = ClosureGroup___19 {
            actual___293: actual___293.clone()
        };
        let fn__1032 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__1032())
        };
        test___6.assert(t___1057, fn__1032.clone());
        test___6.soft_fail_to_hard()
    }
    #[test]
    fn lists__296() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___7 = temper_std::testing::Test::new();
        let mut t___596: temper_std::temporal::Date;
        let mut t___597: temper_std::temporal::Date;
        let mut t___598: temper_std::temporal::Date;
        let mut t___599: temper_std::temporal::Date;
        let mut t___978: SqlBuilder = SqlBuilder::new();
        t___978.append_safe("v IN (");
        t___978.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___978.append_safe(")");
        let actual___297: std::sync::Arc<String> = t___978.accumulated().to_string();
        let mut t___985: bool = Some(actual___297.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___20 {
            actual___297: std::sync::Arc<String>
        }
        impl ClosureGroup___20 {
            fn fn__977(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___297.clone()));
            }
        }
        let closure_group = ClosureGroup___20 {
            actual___297: actual___297.clone()
        };
        let fn__977 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__977())
        };
        test___7.assert(t___985, fn__977.clone());
        let mut t___987: SqlBuilder = SqlBuilder::new();
        t___987.append_safe("v IN (");
        t___987.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___987.append_safe(")");
        let actual___300: std::sync::Arc<String> = t___987.accumulated().to_string();
        let mut t___994: bool = Some(actual___300.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___21 {
            actual___300: std::sync::Arc<String>
        }
        impl ClosureGroup___21 {
            fn fn__976(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___300.clone()));
            }
        }
        let closure_group = ClosureGroup___21 {
            actual___300: actual___300.clone()
        };
        let fn__976 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__976())
        };
        test___7.assert(t___994, fn__976.clone());
        let mut t___996: SqlBuilder = SqlBuilder::new();
        t___996.append_safe("v IN (");
        t___996.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___996.append_safe(")");
        let actual___303: std::sync::Arc<String> = t___996.accumulated().to_string();
        let mut t___1003: bool = Some(actual___303.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___22 {
            actual___303: std::sync::Arc<String>
        }
        impl ClosureGroup___22 {
            fn fn__975(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___303.clone()));
            }
        }
        let closure_group = ClosureGroup___22 {
            actual___303: actual___303.clone()
        };
        let fn__975 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__975())
        };
        test___7.assert(t___1003, fn__975.clone());
        let mut t___1005: SqlBuilder = SqlBuilder::new();
        t___1005.append_safe("v IN (");
        t___1005.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___1005.append_safe(")");
        let actual___306: std::sync::Arc<String> = t___1005.accumulated().to_string();
        let mut t___1012: bool = Some(actual___306.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___23 {
            actual___306: std::sync::Arc<String>
        }
        impl ClosureGroup___23 {
            fn fn__974(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___306.clone()));
            }
        }
        let closure_group = ClosureGroup___23 {
            actual___306: actual___306.clone()
        };
        let fn__974 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__974())
        };
        test___7.assert(t___1012, fn__974.clone());
        let mut t___1014: SqlBuilder = SqlBuilder::new();
        t___1014.append_safe("v IN (");
        t___1014.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___1014.append_safe(")");
        let actual___309: std::sync::Arc<String> = t___1014.accumulated().to_string();
        let mut t___1021: bool = Some(actual___309.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___24 {
            actual___309: std::sync::Arc<String>
        }
        impl ClosureGroup___24 {
            fn fn__973(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___309.clone()));
            }
        }
        let closure_group = ClosureGroup___24 {
            actual___309: actual___309.clone()
        };
        let fn__973 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__973())
        };
        test___7.assert(t___1021, fn__973.clone());
        'ok___5776: {
            'orelse___329: {
                t___596 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___329
                };
                t___597 = t___596.clone();
                break 'ok___5776;
            }
            t___597 = panic!();
        }
        'ok___5777: {
            'orelse___330: {
                t___598 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___330
                };
                t___599 = t___598.clone();
                break 'ok___5777;
            }
            t___599 = panic!();
        }
        let dates__230: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___597.clone(), t___599.clone()]);
        let mut t___1023: SqlBuilder = SqlBuilder::new();
        t___1023.append_safe("v IN (");
        t___1023.append_date_list(temper_core::ToListed::to_listed(dates__230.clone()));
        t___1023.append_safe(")");
        let actual___312: std::sync::Arc<String> = t___1023.accumulated().to_string();
        let mut t___1030: bool = Some(actual___312.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___25 {
            actual___312: std::sync::Arc<String>
        }
        impl ClosureGroup___25 {
            fn fn__972(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___312.clone()));
            }
        }
        let closure_group = ClosureGroup___25 {
            actual___312: actual___312.clone()
        };
        let fn__972 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__972())
        };
        test___7.assert(t___1030, fn__972.clone());
        test___7.soft_fail_to_hard()
    }
    #[test]
    fn nesting__315() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___8 = temper_std::testing::Test::new();
        let name__232: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___941: SqlBuilder = SqlBuilder::new();
        t___941.append_safe("where p.last_name = ");
        t___941.append_string("Someone");
        let condition__233: SqlFragment = t___941.accumulated();
        let mut t___945: SqlBuilder = SqlBuilder::new();
        t___945.append_safe("select p.id from person p ");
        t___945.append_fragment(condition__233.clone());
        let actual___317: std::sync::Arc<String> = t___945.accumulated().to_string();
        let mut t___951: bool = Some(actual___317.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___26 {
            actual___317: std::sync::Arc<String>
        }
        impl ClosureGroup___26 {
            fn fn__940(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___317.clone()));
            }
        }
        let closure_group = ClosureGroup___26 {
            actual___317: actual___317.clone()
        };
        let fn__940 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__940())
        };
        test___8.assert(t___951, fn__940.clone());
        let mut t___953: SqlBuilder = SqlBuilder::new();
        t___953.append_safe("select p.id from person p ");
        t___953.append_part(SqlPart::new(condition__233.to_source()));
        let actual___320: std::sync::Arc<String> = t___953.accumulated().to_string();
        let mut t___960: bool = Some(actual___320.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___27 {
            actual___320: std::sync::Arc<String>
        }
        impl ClosureGroup___27 {
            fn fn__939(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___320.clone()));
            }
        }
        let closure_group = ClosureGroup___27 {
            actual___320: actual___320.clone()
        };
        let fn__939 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__939())
        };
        test___8.assert(t___960, fn__939.clone());
        let parts__234: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___964: SqlBuilder = SqlBuilder::new();
        t___964.append_safe("select ");
        t___964.append_part_list(parts__234.clone());
        let actual___323: std::sync::Arc<String> = t___964.accumulated().to_string();
        let mut t___970: bool = Some(actual___323.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___28 {
            actual___323: std::sync::Arc<String>
        }
        impl ClosureGroup___28 {
            fn fn__938(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//sql/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___323.clone()));
            }
        }
        let closure_group = ClosureGroup___28 {
            actual___323: actual___323.clone()
        };
        let fn__938 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__938())
        };
        test___8.assert(t___970, fn__938.clone());
        test___8.soft_fail_to_hard()
    }
    use super::*;
}
