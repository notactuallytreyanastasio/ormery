#![allow(warnings)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
use crate::core::ContextualAutoescapingAccumulatorTrait;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            TEMPER_KEYWORDS.set(std::sync::Arc::new(vec![std::sync::Arc::new("if".to_string()), std::sync::Arc::new("else".to_string()), std::sync::Arc::new("for".to_string()), std::sync::Arc::new("while".to_string()), std::sync::Arc::new("do".to_string()), std::sync::Arc::new("when".to_string()), std::sync::Arc::new("break".to_string()), std::sync::Arc::new("continue".to_string()), std::sync::Arc::new("return".to_string()), std::sync::Arc::new("let".to_string()), std::sync::Arc::new("var".to_string()), std::sync::Arc::new("class".to_string()), std::sync::Arc::new("export".to_string()), std::sync::Arc::new("import".to_string()), std::sync::Arc::new("public".to_string()), std::sync::Arc::new("private".to_string()), std::sync::Arc::new("protected".to_string()), std::sync::Arc::new("throws".to_string()), std::sync::Arc::new("new".to_string()), std::sync::Arc::new("this".to_string()), std::sync::Arc::new("get".to_string()), std::sync::Arc::new("set".to_string()), std::sync::Arc::new("static".to_string()), std::sync::Arc::new("extends".to_string()), std::sync::Arc::new("implements".to_string()), std::sync::Arc::new("true".to_string()), std::sync::Arc::new("false".to_string()), std::sync::Arc::new("null".to_string()), std::sync::Arc::new("bubble".to_string()), std::sync::Arc::new("orelse".to_string()), std::sync::Arc::new("of".to_string())])).unwrap_or_else(| _ | panic!());
            TEMPER_TYPES.set(std::sync::Arc::new(vec![std::sync::Arc::new("String".to_string()), std::sync::Arc::new("Int".to_string()), std::sync::Arc::new("Boolean".to_string()), std::sync::Arc::new("List".to_string()), std::sync::Arc::new("Map".to_string()), std::sync::Arc::new("Bubble".to_string()), std::sync::Arc::new("Pair".to_string()), std::sync::Arc::new("Float".to_string()), std::sync::Arc::new("Double".to_string()), std::sync::Arc::new("Byte".to_string()), std::sync::Arc::new("Short".to_string()), std::sync::Arc::new("Long".to_string()), std::sync::Arc::new("Char".to_string()), std::sync::Arc::new("Void".to_string()), std::sync::Arc::new("Record".to_string()), std::sync::Arc::new("Schema".to_string()), std::sync::Arc::new("Field".to_string()), std::sync::Arc::new("Query".to_string()), std::sync::Arc::new("InMemoryStore".to_string()), std::sync::Arc::new("ListBuilder".to_string()), std::sync::Arc::new("MapBuilder".to_string()), std::sync::Arc::new("WhereClause".to_string()), std::sync::Arc::new("OrderClause".to_string())])).unwrap_or_else(| _ | panic!());
            Ok(())
    }).clone()
}
static TEMPER_KEYWORDS: std::sync::OnceLock<temper_core::List<std::sync::Arc<String>>> = std::sync::OnceLock::new();
fn temper_keywords() -> temper_core::List<std::sync::Arc<String>> {
    ( * TEMPER_KEYWORDS.get().unwrap()).clone()
}
static TEMPER_TYPES: std::sync::OnceLock<temper_core::List<std::sync::Arc<String>>> = std::sync::OnceLock::new();
fn temper_types() -> temper_core::List<std::sync::Arc<String>> {
    ( * TEMPER_TYPES.get().unwrap()).clone()
}
struct DemoControllerStruct {
    schema: Schema, store: InMemoryStore, query_count: i32
}
#[derive(Clone)]
pub struct DemoController(std::sync::Arc<std::sync::RwLock<DemoControllerStruct>>);
#[derive(Clone)]
pub struct DemoControllerBuilder {
    pub schema: Schema, pub store: InMemoryStore
}
impl DemoControllerBuilder {
    pub fn build(self) -> DemoController {
        DemoController::new(self.schema, self.store)
    }
}
impl DemoController {
    pub fn new(schema__211: Schema, store__212: InMemoryStore) -> DemoController {
        let schema;
        let store;
        let query_count;
        schema = schema__211.clone();
        store = store__212.clone();
        query_count = 0;
        let selfish = DemoController(std::sync::Arc::new(std::sync::RwLock::new(DemoControllerStruct {
                        schema, store, query_count
        })));
        return selfish;
    }
    pub fn get_record_count(& self) -> i32 {
        let mut t___3833: std::sync::Arc<String> = self.0.read().unwrap().schema.table_name();
        return self.0.read().unwrap().store.count(t___3833.clone());
    }
    pub fn get_adult_count(& self) -> i32 {
        return temper_core::ListedTrait::len( & Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).r#where("age", ">=", "18").all());
    }
    pub fn get_query_count(& self) -> i32 {
        return self.0.read().unwrap().query_count;
    }
    fn increment_query_count(& self) {
        let t___734: i32 = self.0.read().unwrap().query_count.wrapping_add(1);
        self.0.write().unwrap().query_count = t___734;
    }
    fn format_records(& self, records__223: impl temper_core::ToList<Record>) -> std::sync::Arc<String> {
        let records__223 = records__223.to_list();
        #[derive(Clone)]
        struct ClosureGroup___1 {}
        impl ClosureGroup___1 {
            fn fn__3824(& self, record__226: Record) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("  {}", record__226.describe()));
            }
        }
        let closure_group = ClosureGroup___1 {};
        let fn__3824 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | record__226: Record | closure_group.fn__3824(record__226))
        };
        let lines__225: temper_core::List<std::sync::Arc<String>> = temper_core::listed::map( & records__223, & ( * fn__3824.clone()));
        #[derive(Clone)]
        struct ClosureGroup___2 {}
        impl ClosureGroup___2 {
            fn fn__3823(& self, s__228: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let s__228 = s__228.to_arc_string();
                return s__228.clone();
            }
        }
        let closure_group = ClosureGroup___2 {};
        let fn__3823 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | s__228: std::sync::Arc<String> | closure_group.fn__3823(s__228))
        };
        return temper_core::listed::join( & lines__225, std::sync::Arc::new("\x0a".to_string()), & ( * fn__3823.clone()));
    }
    pub fn run_demo1(& self) -> std::sync::Arc<String> {
        self.increment_query_count();
        let results__232: temper_core::List<Record> = Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).all();
        let formatted__233: std::sync::Arc<String> = self.format_records(results__232.clone());
        return std::sync::Arc::new(format!("=== Demo 1: All Users ===\x0a\x0aQuery: new Query(userSchema, store).all()\x0a\x0aResults:\x0a{}\x0a\x0aTotal: {} records", formatted__233.clone(), temper_core::ListedTrait::len( & results__232)));
    }
    pub fn run_demo2(& self) -> std::sync::Arc<String> {
        self.increment_query_count();
        let results__236: temper_core::List<Record> = Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).r#where("age", ">=", "18").all();
        let formatted__237: std::sync::Arc<String> = self.format_records(results__236.clone());
        return std::sync::Arc::new(format!("=== Demo 2: Filter Adults ===\x0a\x0aQuery: new Query(userSchema, store)\x0a  .where(\"age\", \">=\", \"18\")\x0a  .all()\x0a\x0aResults:\x0a{}\x0a\x0aTotal: {} adults found", formatted__237.clone(), temper_core::ListedTrait::len( & results__236)));
    }
    pub fn run_demo3(& self) -> std::sync::Arc<String> {
        self.increment_query_count();
        let results__240: temper_core::List<Record> = Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).order_by("age", "desc").all();
        let formatted__241: std::sync::Arc<String> = self.format_records(results__240.clone());
        return std::sync::Arc::new(format!("=== Demo 3: Sort by Age (Descending) ===\x0a\x0aQuery: new Query(userSchema, store)\x0a  .orderBy(\"age\", \"desc\")\x0a  .all()\x0a\x0aResults (ordered by age, oldest first):\x0a{}", formatted__241.clone()));
    }
    pub fn run_demo4(& self) -> std::sync::Arc<String> {
        self.increment_query_count();
        let page1__244: temper_core::List<Record> = Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).order_by("id", "asc").limit(2).all();
        let page2__245: temper_core::List<Record> = Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).order_by("id", "asc").offset(2).limit(2).all();
        let formatted1__246: std::sync::Arc<String> = self.format_records(page1__244.clone());
        let formatted2__247: std::sync::Arc<String> = self.format_records(page2__245.clone());
        return std::sync::Arc::new(format!("=== Demo 4: Pagination ===\x0a\x0aPage 1: .orderBy(\"id\", \"asc\").limit(2)\x0a\x0a{}\x0a\x0aPage 2: .orderBy(\"id\", \"asc\").offset(2).limit(2)\x0a\x0a{}", formatted1__246.clone(), formatted2__247.clone()));
    }
    pub fn run_demo5(& self) -> std::sync::Arc<String> {
        self.increment_query_count();
        let results__250: temper_core::List<Record> = Query::new(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().store.clone()).r#where("age", ">=", "18").r#where("email", "!=", "").order_by("age", "desc").select([std::sync::Arc::new("name".to_string()), std::sync::Arc::new("age".to_string())]).limit(2).all();
        let formatted__251: std::sync::Arc<String> = self.format_records(results__250.clone());
        return std::sync::Arc::new(format!("=== Demo 5: Complex Query ===\x0a\x0aQuery: new Query(userSchema, store)\x0a  .where(\"age\", \">=\", \"18\")\x0a  .where(\"email\", \"!=\", \"\")\x0a  .orderBy(\"age\", \"desc\")\x0a  .select([\"name\", \"age\"])\x0a  .limit(2)\x0a  .all()\x0a\x0aResults (adults with email, showing name/age only, oldest first, max 2):\x0a{}", formatted__251.clone()));
    }
    pub fn schema(& self) -> Schema {
        return self.0.read().unwrap().schema.clone();
    }
    pub fn store(& self) -> InMemoryStore {
        return self.0.read().unwrap().store.clone();
    }
}
temper_core::impl_any_value_trait!(DemoController, []);
struct FieldStruct {
    name: std::sync::Arc<String>, field_type: std::sync::Arc<String>, primary_key: bool, nullable: bool
}
#[derive(Clone)]
pub struct Field(std::sync::Arc<FieldStruct>);
#[derive(Clone)]
pub struct FieldBuilder {
    pub name: std::sync::Arc<String>, pub field_type: std::sync::Arc<String>, pub primary_key: bool, pub nullable: bool
}
impl FieldBuilder {
    pub fn build(self) -> Field {
        Field::new(self.name, self.field_type, self.primary_key, self.nullable)
    }
}
impl Field {
    pub fn description(& self) -> std::sync::Arc<String> {
        let pk__258: std::sync::Arc<String>;
        if self.0.primary_key {
            pk__258 = std::sync::Arc::new(" (PK)".to_string());
        } else {
            pk__258 = std::sync::Arc::new("".to_string());
        }
        let null__259: std::sync::Arc<String>;
        if self.0.nullable {
            null__259 = std::sync::Arc::new(" (nullable)".to_string());
        } else {
            null__259 = std::sync::Arc::new("".to_string());
        }
        return std::sync::Arc::new(format!("{}: {}{}{}", self.0.name, self.0.field_type.clone(), pk__258.clone(), null__259.clone()));
    }
    pub fn new(name__261: impl temper_core::ToArcString, fieldType__262: impl temper_core::ToArcString, primaryKey__263: bool, nullable__264: bool) -> Field {
        let name__261 = name__261.to_arc_string();
        let fieldType__262 = fieldType__262.to_arc_string();
        let name;
        let field_type;
        let primary_key;
        let nullable;
        name = name__261.clone();
        field_type = fieldType__262.clone();
        primary_key = primaryKey__263;
        nullable = nullable__264;
        let selfish = Field(std::sync::Arc::new(FieldStruct {
                    name, field_type, primary_key, nullable
        }));
        return selfish;
    }
    pub fn name(& self) -> std::sync::Arc<String> {
        return self.0.name.clone();
    }
    pub fn field_type(& self) -> std::sync::Arc<String> {
        return self.0.field_type.clone();
    }
    pub fn primary_key(& self) -> bool {
        return self.0.primary_key;
    }
    pub fn nullable(& self) -> bool {
        return self.0.nullable;
    }
}
temper_core::impl_any_value_trait!(Field, []);
struct SchemaStruct {
    table_name: std::sync::Arc<String>, fields: temper_core::List<Field>
}
#[derive(Clone)]
pub struct Schema(std::sync::Arc<SchemaStruct>);
#[derive(Clone)]
pub struct SchemaBuilder {
    pub table_name: std::sync::Arc<String>, pub fields: temper_core::List<Field>
}
impl SchemaBuilder {
    pub fn build(self) -> Schema {
        Schema::new(self.table_name, self.fields)
    }
}
impl Schema {
    pub fn get_field(& self, name__268: impl temper_core::ToArcString) -> temper_core::Result<Field> {
        let name__268 = name__268.to_arc_string();
        let return__117: Field;
        'fn__269: {
            let this__2501: temper_core::List<Field> = self.0.fields.clone();
            let n__2502: i32 = temper_core::ListedTrait::len( & this__2501);
            let mut i__2503: i32 = 0;
            'loop___5995: while Some(i__2503) < Some(n__2502) {
                let el__2504: Field = temper_core::ListedTrait::get( & this__2501, i__2503);
                i__2503 = i__2503.wrapping_add(1);
                let field__270: Field = el__2504.clone();
                if Some(field__270.name().as_str()) == Some(name__268.as_str()) {
                    return__117 = field__270.clone();
                    break 'fn__269;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__117.clone());
    }
    pub fn has_field(& self, name__272: impl temper_core::ToArcString) -> bool {
        let name__272 = name__272.to_arc_string();
        let return__118: bool;
        'fn__273: {
            let this__2506: temper_core::List<Field> = self.0.fields.clone();
            let n__2507: i32 = temper_core::ListedTrait::len( & this__2506);
            let mut i__2508: i32 = 0;
            'loop___5996: while Some(i__2508) < Some(n__2507) {
                let el__2509: Field = temper_core::ListedTrait::get( & this__2506, i__2508);
                i__2508 = i__2508.wrapping_add(1);
                let field__274: Field = el__2509.clone();
                if Some(field__274.name().as_str()) == Some(name__272.as_str()) {
                    return__118 = true;
                    break 'fn__273;
                }
            }
            return__118 = false;
        }
        return return__118;
    }
    pub fn primary_key_field(& self) -> temper_core::Result<Field> {
        let return__119: Field;
        'fn__276: {
            let this__2511: temper_core::List<Field> = self.0.fields.clone();
            let n__2512: i32 = temper_core::ListedTrait::len( & this__2511);
            let mut i__2513: i32 = 0;
            'loop___5997: while Some(i__2513) < Some(n__2512) {
                let el__2514: Field = temper_core::ListedTrait::get( & this__2511, i__2513);
                i__2513 = i__2513.wrapping_add(1);
                let field__277: Field = el__2514.clone();
                if field__277.primary_key() {
                    return__119 = field__277.clone();
                    break 'fn__276;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__119.clone());
    }
    pub fn field_names(& self) -> temper_core::List<std::sync::Arc<String>> {
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__4076(& self, f__280: Field) -> std::sync::Arc<String> {
                return f__280.name();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__4076 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__280: Field | closure_group.fn__4076(f__280))
        };
        return temper_core::listed::map( & self.0.fields, & ( * fn__4076.clone()));
    }
    pub fn describe(& self) -> std::sync::Arc<String> {
        let header__284: std::sync::Arc<String> = std::sync::Arc::new(format!("Schema: {}\x0a", self.0.table_name.clone()));
        #[derive(Clone)]
        struct ClosureGroup___4 {}
        impl ClosureGroup___4 {
            fn fn__4070(& self, f__286: Field) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("  - {}", f__286.description()));
            }
        }
        let closure_group = ClosureGroup___4 {};
        let fn__4070 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__286: Field | closure_group.fn__4070(f__286))
        };
        let mut t___4072: temper_core::List<std::sync::Arc<String>> = temper_core::listed::map( & self.0.fields, & ( * fn__4070.clone()));
        #[derive(Clone)]
        struct ClosureGroup___5 {}
        impl ClosureGroup___5 {
            fn fn__4069(& self, s__288: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let s__288 = s__288.to_arc_string();
                return s__288.clone();
            }
        }
        let closure_group = ClosureGroup___5 {};
        let fn__4069 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | s__288: std::sync::Arc<String> | closure_group.fn__4069(s__288))
        };
        let fieldList__285: std::sync::Arc<String> = temper_core::listed::join( & t___4072, std::sync::Arc::new("\x0a".to_string()), & ( * fn__4069.clone()));
        return std::sync::Arc::new(format!("{}{}", header__284, fieldList__285.clone()));
    }
    pub fn new(tableName__291: impl temper_core::ToArcString, fields__292: impl temper_core::ToList<Field>) -> Schema {
        let tableName__291 = tableName__291.to_arc_string();
        let fields__292 = fields__292.to_list();
        let table_name;
        let fields;
        table_name = tableName__291.clone();
        fields = fields__292.clone();
        let selfish = Schema(std::sync::Arc::new(SchemaStruct {
                    table_name, fields
        }));
        return selfish;
    }
    pub fn table_name(& self) -> std::sync::Arc<String> {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<Field> {
        return self.0.fields.clone();
    }
}
temper_core::impl_any_value_trait!(Schema, []);
struct InMemoryStoreStruct {
    tables: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<Record>>, next_ids: temper_core::MapBuilder<std::sync::Arc<String>, i32>
}
#[derive(Clone)]
pub struct InMemoryStore(std::sync::Arc<std::sync::RwLock<InMemoryStoreStruct>>);
impl InMemoryStore {
    pub fn new() -> InMemoryStore {
        let tables;
        let next_ids;
        let mut t___4058: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<Record>> = temper_core::MapBuilder::new();
        tables = t___4058.clone();
        let mut t___4059: temper_core::MapBuilder<std::sync::Arc<String>, i32> = temper_core::MapBuilder::new();
        next_ids = t___4059.clone();
        let selfish = InMemoryStore(std::sync::Arc::new(std::sync::RwLock::new(InMemoryStoreStruct {
                        tables, next_ids
        })));
        return selfish;
    }
    fn ensure_table(& self, tableName__335: impl temper_core::ToArcString) {
        let tableName__335 = tableName__335.to_arc_string();
        let mut t___4055: temper_core::ListBuilder<Record>;
        if ! temper_core::MappedTrait::has( & self.0.read().unwrap().tables, tableName__335.clone()) {
            t___4055 = temper_core::listed::new_builder();
            temper_core::MapBuilder::set( & self.0.read().unwrap().tables, tableName__335.clone(), t___4055.clone());
            temper_core::MapBuilder::set( & self.0.read().unwrap().next_ids, tableName__335.clone(), 1);
        }
    }
    pub fn insert(& self, tableName__338: impl temper_core::ToArcString, data__339: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Record {
        let tableName__338 = tableName__338.to_arc_string();
        self.ensure_table(tableName__338.clone());
        let id__341: i32 = temper_core::MappedTrait::get_or( & self.0.read().unwrap().next_ids, tableName__338.clone(), 1);
        temper_core::MapBuilder::set( & self.0.read().unwrap().next_ids, tableName__338.clone(), id__341.wrapping_add(1));
        let dataBuilder__342: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MappedTrait::to_map_builder( & data__339);
        temper_core::MapBuilder::set( & dataBuilder__342, std::sync::Arc::new("id".to_string()), temper_core::int_to_string(id__341, None));
        let record__343: Record = Record::new(temper_core::MappedTrait::to_map( & dataBuilder__342));
        let mut t___4051: temper_core::ListBuilder<Record> = temper_core::listed::new_builder();
        let table__344: temper_core::ListBuilder<Record> = temper_core::MappedTrait::get_or( & self.0.read().unwrap().tables, tableName__338.clone(), t___4051.clone());
        temper_core::listed::add( & table__344, record__343.clone(), None);
        return record__343.clone();
    }
    pub fn all(& self, tableName__346: impl temper_core::ToArcString) -> temper_core::List<Record> {
        let tableName__346 = tableName__346.to_arc_string();
        self.ensure_table(tableName__346.clone());
        let mut t___4040: temper_core::ListBuilder<Record> = temper_core::listed::new_builder();
        let table__348: temper_core::ListBuilder<Record> = temper_core::MappedTrait::get_or( & self.0.read().unwrap().tables, tableName__346.clone(), t___4040.clone());
        return temper_core::ListedTrait::to_list( & table__348);
    }
    pub fn get(& self, tableName__350: impl temper_core::ToArcString, id__351: i32) -> temper_core::Result<Record> {
        let tableName__350 = tableName__350.to_arc_string();
        let return__138: Record;
        let mut t___4034: temper_core::ListBuilder<Record>;
        'fn__352: {
            self.ensure_table(tableName__350.clone());
            t___4034 = temper_core::listed::new_builder();
            let table__353: temper_core::ListBuilder<Record> = temper_core::MappedTrait::get_or( & self.0.read().unwrap().tables, tableName__350.clone(), t___4034.clone());
            let this__2516: temper_core::List<Record> = temper_core::ListedTrait::to_list( & table__353);
            let n__2517: i32 = temper_core::ListedTrait::len( & this__2516);
            let mut i__2518: i32 = 0;
            'loop___5998: while Some(i__2518) < Some(n__2517) {
                let el__2519: Record = temper_core::ListedTrait::get( & this__2516, i__2518);
                i__2518 = i__2518.wrapping_add(1);
                let record__354: Record = el__2519.clone();
                let recordId__355: i32;
                'ok___5879: {
                    'orelse___939: {
                        recordId__355 = match record__354.id() {
                            Ok(x) => x,
                            _ => break 'orelse___939
                        };
                        break 'ok___5879;
                    }
                    return Err(temper_core::Error::new());
                }
                if Some(recordId__355) == Some(id__351) {
                    return__138 = record__354.clone();
                    break 'fn__352;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__138.clone());
    }
    pub fn count(& self, tableName__357: impl temper_core::ToArcString) -> i32 {
        let tableName__357 = tableName__357.to_arc_string();
        self.ensure_table(tableName__357.clone());
        let mut t___4030: temper_core::ListBuilder<Record> = temper_core::listed::new_builder();
        let table__359: temper_core::ListBuilder<Record> = temper_core::MappedTrait::get_or( & self.0.read().unwrap().tables, tableName__357.clone(), t___4030.clone());
        return temper_core::ListedTrait::len( & table__359);
    }
}
temper_core::impl_any_value_trait!(InMemoryStore, []);
struct RecordStruct {
    data: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>
}
#[derive(Clone)]
pub struct Record(std::sync::Arc<RecordStruct>);
impl Record {
    pub fn get(& self, field__308: impl temper_core::ToArcString) -> temper_core::Result<std::sync::Arc<String>> {
        let field__308 = field__308.to_arc_string();
        let return__129: std::sync::Arc<String>;
        return__129 = temper_core::MappedTrait::get( & self.0.data, field__308.clone()) ? ;
        return Ok(return__129.clone());
    }
    pub fn get_or(& self, field__311: impl temper_core::ToArcString, fallback__312: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let field__311 = field__311.to_arc_string();
        let fallback__312 = fallback__312.to_arc_string();
        return temper_core::MappedTrait::get_or( & self.0.data, field__311.clone(), fallback__312.clone());
    }
    pub fn has(& self, field__315: impl temper_core::ToArcString) -> bool {
        let field__315 = field__315.to_arc_string();
        return temper_core::MappedTrait::has( & self.0.data, field__315.clone());
    }
    pub fn id(& self) -> temper_core::Result<i32> {
        let return__132: i32;
        let idStr__319: std::sync::Arc<String>;
        idStr__319 = temper_core::MappedTrait::get( & self.0.data, std::sync::Arc::new("id".to_string())) ? ;
        'ok___5881: {
            'orelse___938: {
                return__132 = match temper_core::string::to_int( & idStr__319, None) {
                    Ok(x) => x,
                    _ => break 'orelse___938
                };
                break 'ok___5881;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__132);
    }
    pub fn describe(& self) -> std::sync::Arc<String> {
        #[derive(Clone)]
        struct ClosureGroup___6 {}
        impl ClosureGroup___6 {
            fn fn__4061(& self, k__323: impl temper_core::ToArcString, v__324: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let k__323 = k__323.to_arc_string();
                let v__324 = v__324.to_arc_string();
                return std::sync::Arc::new(format!("{}: {}", k__323, v__324.clone()));
            }
        }
        let closure_group = ClosureGroup___6 {};
        let fn__4061 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | k__323: std::sync::Arc<String>, v__324: std::sync::Arc<String> | closure_group.fn__4061(k__323, v__324))
        };
        let pairs__322: temper_core::List<std::sync::Arc<String>> = temper_core::mapped_to_list_with( & self.0.data, & ( * fn__4061.clone()));
        #[derive(Clone)]
        struct ClosureGroup___7 {}
        impl ClosureGroup___7 {
            fn fn__4060(& self, s__326: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let s__326 = s__326.to_arc_string();
                return s__326.clone();
            }
        }
        let closure_group = ClosureGroup___7 {};
        let fn__4060 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | s__326: std::sync::Arc<String> | closure_group.fn__4060(s__326))
        };
        return temper_core::listed::join( & pairs__322, std::sync::Arc::new(", ".to_string()), & ( * fn__4060.clone()));
    }
    pub fn new(data__329: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Record {
        let data;
        data = data__329.clone();
        let selfish = Record(std::sync::Arc::new(RecordStruct {
                    data
        }));
        return selfish;
    }
    pub fn data(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        return self.0.data.clone();
    }
}
temper_core::impl_any_value_trait!(Record, []);
struct QueryStruct {
    schema: Schema, store: InMemoryStore, where_clauses: temper_core::ListBuilder<WhereClause>, select_fields: temper_core::List<std::sync::Arc<String>>, order_by_clauses: temper_core::ListBuilder<OrderClause>, limit_value: i32, offset_value: i32
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<std::sync::RwLock<QueryStruct>>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub schema: Schema, pub store: InMemoryStore
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.schema, self.store)
    }
}
impl Query {
    pub fn new(schema__384: Schema, store__385: InMemoryStore) -> Query {
        let schema;
        let store;
        let where_clauses;
        let select_fields;
        let order_by_clauses;
        let limit_value;
        let offset_value;
        schema = schema__384.clone();
        store = store__385.clone();
        let mut t___3898: temper_core::ListBuilder<WhereClause> = temper_core::listed::new_builder();
        where_clauses = t___3898.clone();
        let mut t___2220: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![]);
        select_fields = t___2220.clone();
        let mut t___3899: temper_core::ListBuilder<OrderClause> = temper_core::listed::new_builder();
        order_by_clauses = t___3899.clone();
        limit_value = -1;
        offset_value = 0;
        let selfish = Query(std::sync::Arc::new(std::sync::RwLock::new(QueryStruct {
                        schema, store, where_clauses, select_fields, order_by_clauses, limit_value, offset_value
        })));
        return selfish;
    }
    pub fn r#where(& self, field__388: impl temper_core::ToArcString, operator__389: impl temper_core::ToArcString, value__390: impl temper_core::ToArcString) -> Query {
        let field__388 = field__388.to_arc_string();
        let operator__389 = operator__389.to_arc_string();
        let value__390 = value__390.to_arc_string();
        let mut t___3896: WhereClause = WhereClause::new(field__388.clone(), operator__389.clone(), value__390.clone());
        temper_core::listed::add( & self.0.read().unwrap().where_clauses, t___3896.clone(), None);
        return self.clone();
    }
    pub fn select(& self, fields__393: impl temper_core::ToList<std::sync::Arc<String>>) -> Query {
        let fields__393 = fields__393.to_list();
        self.0.write().unwrap().select_fields = fields__393.clone();
        return self.clone();
    }
    pub fn order_by(& self, field__396: impl temper_core::ToArcString, direction__397: impl temper_core::ToArcString) -> Query {
        let field__396 = field__396.to_arc_string();
        let direction__397 = direction__397.to_arc_string();
        let mut t___3894: OrderClause = OrderClause::new(field__396.clone(), direction__397.clone());
        temper_core::listed::add( & self.0.read().unwrap().order_by_clauses, t___3894.clone(), None);
        return self.clone();
    }
    pub fn limit(& self, n__400: i32) -> Query {
        let mut t___2209: i32;
        if Some(n__400) < Some(0) {
            t___2209 = 0;
        } else {
            t___2209 = n__400;
        }
        self.0.write().unwrap().limit_value = t___2209;
        return self.clone();
    }
    pub fn offset(& self, n__403: i32) -> Query {
        self.0.write().unwrap().offset_value = n__403;
        return self.clone();
    }
    fn matches_where(& self, record__406: Record) -> bool {
        let return__152: bool;
        let mut t___3883: std::sync::Arc<String>;
        let mut t___3885: std::sync::Arc<String>;
        let mut t___3887: std::sync::Arc<String>;
        let mut t___3889: std::sync::Arc<String>;
        let mut t___3890: std::sync::Arc<String>;
        let mut t___3891: bool;
        let mut t___3892: std::sync::Arc<String>;
        let mut t___3893: std::sync::Arc<String>;
        'fn__407: {
            let this__2521: temper_core::List<WhereClause> = temper_core::ListedTrait::to_list( & self.0.read().unwrap().where_clauses);
            let n__2522: i32 = temper_core::ListedTrait::len( & this__2521);
            let mut i__2523: i32 = 0;
            'loop___5999: while Some(i__2523) < Some(n__2522) {
                let el__2524: WhereClause = temper_core::ListedTrait::get( & this__2521, i__2523);
                i__2523 = i__2523.wrapping_add(1);
                let clause__408: WhereClause = el__2524.clone();
                let mut t___2198: Field;
                t___3883 = clause__408.field();
                let recordValue__409: std::sync::Arc<String> = record__406.get_or(t___3883.clone(), "");
                t___3885 = clause__408.field();
                if ! self.0.read().unwrap().schema.has_field(t___3885.clone()) {
                    return__152 = false;
                    break 'fn__407;
                }
                let fieldInfo__410: Field;
                'ok___5884: {
                    'orelse___944: {
                        t___3887 = clause__408.field();
                        t___2198 = match self.0.read().unwrap().schema.get_field(t___3887.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___944
                        };
                        fieldInfo__410 = t___2198.clone();
                        break 'ok___5884;
                    }
                    fieldInfo__410 = panic!();
                }
                let fieldType__411: std::sync::Arc<String> = fieldInfo__410.field_type();
                let matches__412: bool;
                if Some(fieldType__411.as_str()) == Some("Int") {
                    t___3889 = clause__408.operator();
                    t___3890 = clause__408.value();
                    t___3891 = compareInt__195(recordValue__409.clone(), t___3889.clone(), t___3890.clone());
                    matches__412 = t___3891;
                } else {
                    if Some(fieldType__411.as_str()) == Some("String") {
                        t___3892 = clause__408.operator();
                        t___3893 = clause__408.value();
                        matches__412 = compareString__196(recordValue__409.clone(), t___3892.clone(), t___3893.clone());
                    } else {
                        matches__412 = false;
                    }
                }
                if ! matches__412 {
                    return__152 = false;
                    break 'fn__407;
                }
            }
            return__152 = true;
        }
        return return__152;
    }
    fn project_record(& self, record__414: Record) -> Record {
        let return__153: Record;
        let mut t___3878: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__415: {
            if Some(temper_core::ListedTrait::len( & self.0.read().unwrap().select_fields)) == Some(0) {
                return__153 = record__414.clone();
                break 'fn__415;
            }
            let builder__416: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
            #[derive(Clone)]
            struct ClosureGroup___8 {
                record__414: Record, builder__416: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
            }
            impl ClosureGroup___8 {
                fn fn__3873(& self, fieldName__417: impl temper_core::ToArcString) {
                    let fieldName__417 = fieldName__417.to_arc_string();
                    let value__418: std::sync::Arc<String> = self.record__414.get_or(fieldName__417.clone(), "");
                    temper_core::MapBuilder::set( & self.builder__416, fieldName__417.clone(), value__418.clone());
                }
            }
            let closure_group = ClosureGroup___8 {
                record__414: record__414.clone(), builder__416: builder__416.clone()
            };
            let fn__3873 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | fieldName__417: std::sync::Arc<String> | closure_group.fn__3873(fieldName__417))
            };
            temper_core::listed::list_for_each( & self.0.read().unwrap().select_fields, & ( * fn__3873.clone()));
            t___3878 = temper_core::MappedTrait::to_map( & builder__416);
            return__153 = Record::new(t___3878.clone());
        }
        return return__153.clone();
    }
    fn compare_records(& self, a__420: Record, b__421: Record, orderClauses__422: impl temper_core::ToList<OrderClause>) -> i32 {
        let orderClauses__422 = orderClauses__422.to_list();
        let return__154: i32;
        let mut t___3862: std::sync::Arc<String>;
        let mut t___3864: std::sync::Arc<String>;
        let mut t___3866: std::sync::Arc<String>;
        let mut t___3868: std::sync::Arc<String>;
        'fn__423: {
            let this__2526: temper_core::List<OrderClause> = orderClauses__422.clone();
            let n__2527: i32 = temper_core::ListedTrait::len( & this__2526);
            let mut i__2528: i32 = 0;
            'loop___6000: while Some(i__2528) < Some(n__2527) {
                let el__2529: OrderClause = temper_core::ListedTrait::get( & this__2526, i__2528);
                i__2528 = i__2528.wrapping_add(1);
                let clause__424: OrderClause = el__2529.clone();
                let mut t___2174: Field;
                let mut t___2177: i32;
                let mut t___2179: i32;
                t___3862 = clause__424.field();
                let aVal__425: std::sync::Arc<String> = a__420.get_or(t___3862.clone(), "");
                t___3864 = clause__424.field();
                let bVal__426: std::sync::Arc<String> = b__421.get_or(t___3864.clone(), "");
                t___3866 = clause__424.field();
                if ! self.0.read().unwrap().schema.has_field(t___3866.clone()) {
                    continue;
                }
                let fieldInfo__427: Field;
                'ok___5886: {
                    'orelse___945: {
                        t___3868 = clause__424.field();
                        t___2174 = match self.0.read().unwrap().schema.get_field(t___3868.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___945
                        };
                        fieldInfo__427 = t___2174.clone();
                        break 'ok___5886;
                    }
                    fieldInfo__427 = panic!();
                }
                let fieldType__428: std::sync::Arc<String> = fieldInfo__427.field_type();
                let cmp__429: i32;
                if Some(fieldType__428.as_str()) == Some("Int") {
                    let aInt__430: i32;
                    'ok___5887: {
                        'orelse___946: {
                            t___2177 = match temper_core::string::to_int( & aVal__425, None) {
                                Ok(x) => x,
                                _ => break 'orelse___946
                            };
                            aInt__430 = t___2177;
                            break 'ok___5887;
                        }
                        aInt__430 = 0;
                    }
                    let bInt__431: i32;
                    'ok___5888: {
                        'orelse___947: {
                            t___2179 = match temper_core::string::to_int( & bVal__426, None) {
                                Ok(x) => x,
                                _ => break 'orelse___947
                            };
                            bInt__431 = t___2179;
                            break 'ok___5888;
                        }
                        bInt__431 = 0;
                    }
                    cmp__429 = Some(aInt__430).cmp( & Some(bInt__431)) as i32;
                } else {
                    if Some(fieldType__428.as_str()) == Some("String") {
                        cmp__429 = Some(aVal__425.clone()).cmp( & Some(bVal__426.clone())) as i32;
                    } else {
                        cmp__429 = 0;
                    }
                }
                if Some(cmp__429) != Some(0) {
                    if Some(clause__424.direction().as_str()) == Some("desc") {
                        return__154 = cmp__429.wrapping_neg();
                    } else {
                        return__154 = cmp__429;
                    }
                    break 'fn__423;
                }
            }
            return__154 = 0;
        }
        return return__154;
    }
    pub fn all(& self) -> temper_core::List<Record> {
        let mut t___3853: temper_core::List<Record>;
        let mut t___3854: temper_core::List<Record>;
        let mut t___3855: i32;
        let mut t___3856: i32;
        let mut t___3857: temper_core::List<Record>;
        let mut t___3846: std::sync::Arc<String> = self.0.read().unwrap().schema.table_name();
        let allRecords__434: temper_core::List<Record> = self.0.read().unwrap().store.all(t___3846.clone());
        #[derive(Clone)]
        struct ClosureGroup___9 {
            this__86: Query
        }
        impl ClosureGroup___9 {
            fn fn__3845(& self, r__436: Record) -> bool {
                return self.this__86.matches_where(r__436.clone());
            }
        }
        let closure_group = ClosureGroup___9 {
            this__86: self.clone()
        };
        let fn__3845 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | r__436: Record | closure_group.fn__3845(r__436))
        };
        let filtered__435: temper_core::List<Record> = temper_core::listed::filter( & allRecords__434, & ( * fn__3845.clone()));
        let sorted__438: temper_core::List<Record>;
        if Some(temper_core::ListedTrait::len( & self.0.read().unwrap().order_by_clauses)) > Some(0) {
            let clauses__439: temper_core::List<OrderClause> = temper_core::ListedTrait::to_list( & self.0.read().unwrap().order_by_clauses);
            #[derive(Clone)]
            struct ClosureGroup___10 {
                this__86: Query, clauses__439: temper_core::List<OrderClause>
            }
            impl ClosureGroup___10 {
                fn fn__3844(& self, a__440: Record, b__441: Record) -> i32 {
                    return self.this__86.compare_records(a__440.clone(), b__441.clone(), self.clauses__439.clone());
                }
            }
            let closure_group = ClosureGroup___10 {
                this__86: self.clone(), clauses__439: clauses__439.clone()
            };
            let fn__3844 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | a__440: Record, b__441: Record | closure_group.fn__3844(a__440, b__441))
            };
            t___3853 = temper_core::listed::sorted( & filtered__435, & ( * fn__3844.clone()));
            sorted__438 = t___3853.clone();
        } else {
            sorted__438 = filtered__435.clone();
        }
        let sliced__443: temper_core::List<Record>;
        if Some(self.0.read().unwrap().limit_value) >= Some(0) {
            let start__444: i32 = self.0.read().unwrap().offset_value;
            let end__445: i32 = self.0.read().unwrap().offset_value.wrapping_add(self.0.read().unwrap().limit_value);
            t___3854 = temper_core::listed::slice( & sorted__438, start__444, end__445);
            sliced__443 = t___3854.clone();
        } else {
            if Some(self.0.read().unwrap().offset_value) > Some(0) {
                t___3856 = self.0.read().unwrap().offset_value;
                t___3855 = temper_core::ListedTrait::len( & sorted__438);
                t___3857 = temper_core::listed::slice( & sorted__438, t___3856, t___3855);
                sliced__443 = t___3857.clone();
            } else {
                sliced__443 = sorted__438.clone();
            }
        }
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__86: Query
        }
        impl ClosureGroup___11 {
            fn fn__3843(& self, r__446: Record) -> Record {
                return self.this__86.project_record(r__446.clone());
            }
        }
        let closure_group = ClosureGroup___11 {
            this__86: self.clone()
        };
        let fn__3843 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | r__446: Record | closure_group.fn__3843(r__446))
        };
        return temper_core::listed::map( & sliced__443, & ( * fn__3843.clone()));
    }
    pub fn to_sql(& self) -> crate::sql::SqlFragment {
        return toSqlQuery(self.0.read().unwrap().schema.clone(), self.0.read().unwrap().select_fields.clone(), temper_core::ListedTrait::to_list( & self.0.read().unwrap().where_clauses), temper_core::ListedTrait::to_list( & self.0.read().unwrap().order_by_clauses), self.0.read().unwrap().limit_value, self.0.read().unwrap().offset_value);
    }
    pub fn schema(& self) -> Schema {
        return self.0.read().unwrap().schema.clone();
    }
    pub fn store(& self) -> InMemoryStore {
        return self.0.read().unwrap().store.clone();
    }
}
temper_core::impl_any_value_trait!(Query, []);
struct WhereClauseStruct {
    field: std::sync::Arc<String>, operator: std::sync::Arc<String>, value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct WhereClause(std::sync::Arc<WhereClauseStruct>);
#[derive(Clone)]
pub struct WhereClauseBuilder {
    pub field: std::sync::Arc<String>, pub operator: std::sync::Arc<String>, pub value: std::sync::Arc<String>
}
impl WhereClauseBuilder {
    pub fn build(self) -> WhereClause {
        WhereClause::new(self.field, self.operator, self.value)
    }
}
impl WhereClause {
    pub fn describe(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!("{} {} {}", self.0.field, self.0.operator.clone(), self.0.value.clone()));
    }
    pub fn new(field__366: impl temper_core::ToArcString, operator__367: impl temper_core::ToArcString, value__368: impl temper_core::ToArcString) -> WhereClause {
        let field__366 = field__366.to_arc_string();
        let operator__367 = operator__367.to_arc_string();
        let value__368 = value__368.to_arc_string();
        let field;
        let operator;
        let value;
        field = field__366.clone();
        operator = operator__367.clone();
        value = value__368.clone();
        let selfish = WhereClause(std::sync::Arc::new(WhereClauseStruct {
                    field, operator, value
        }));
        return selfish;
    }
    pub fn field(& self) -> std::sync::Arc<String> {
        return self.0.field.clone();
    }
    pub fn operator(& self) -> std::sync::Arc<String> {
        return self.0.operator.clone();
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
temper_core::impl_any_value_trait!(WhereClause, []);
struct OrderClauseStruct {
    field: std::sync::Arc<String>, direction: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct OrderClause(std::sync::Arc<OrderClauseStruct>);
#[derive(Clone)]
pub struct OrderClauseBuilder {
    pub field: std::sync::Arc<String>, pub direction: std::sync::Arc<String>
}
impl OrderClauseBuilder {
    pub fn build(self) -> OrderClause {
        OrderClause::new(self.field, self.direction)
    }
}
impl OrderClause {
    pub fn describe(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!("{} {}", self.0.field, self.0.direction.clone()));
    }
    pub fn new(field__374: impl temper_core::ToArcString, direction__375: impl temper_core::ToArcString) -> OrderClause {
        let field__374 = field__374.to_arc_string();
        let direction__375 = direction__375.to_arc_string();
        let field;
        let direction;
        field = field__374.clone();
        direction = direction__375.clone();
        let selfish = OrderClause(std::sync::Arc::new(OrderClauseStruct {
                    field, direction
        }));
        return selfish;
    }
    pub fn field(& self) -> std::sync::Arc<String> {
        return self.0.field.clone();
    }
    pub fn direction(& self) -> std::sync::Arc<String> {
        return self.0.direction.clone();
    }
}
temper_core::impl_any_value_trait!(OrderClause, []);
fn compareInt__195(recordValue__450: impl temper_core::ToArcString, operator__451: impl temper_core::ToArcString, clauseValue__452: impl temper_core::ToArcString) -> bool {
    let recordValue__450 = recordValue__450.to_arc_string();
    let operator__451 = operator__451.to_arc_string();
    let clauseValue__452 = clauseValue__452.to_arc_string();
    let return__157: bool;
    let mut t___4028: std::sync::Arc<String>;
    let mut t___2376: i32;
    let mut t___2378: i32;
    'fn__453: {
        let rv__454: i32;
        'ok___5892: {
            'orelse___940: {
                t___2376 = match temper_core::string::to_int( & recordValue__450, None) {
                    Ok(x) => x,
                    _ => break 'orelse___940
                };
                rv__454 = t___2376;
                break 'ok___5892;
            }
            rv__454 = 0;
        }
        let cv__455: i32;
        'ok___5893: {
            'orelse___941: {
                t___2378 = match temper_core::string::to_int( & clauseValue__452, None) {
                    Ok(x) => x,
                    _ => break 'orelse___941
                };
                cv__455 = t___2378;
                break 'ok___5893;
            }
            cv__455 = 0;
        }
        t___4028 = temper_core::int_to_string(cv__455, None);
        if Some(clauseValue__452.as_str()) != Some(t___4028.as_str()) {
            return__157 = false;
            break 'fn__453;
        }
        if Some(operator__451.as_str()) == Some("==") {
            return__157 = Some(rv__454) == Some(cv__455);
        } else {
            if Some(operator__451.as_str()) == Some("!=") {
                return__157 = Some(rv__454) != Some(cv__455);
            } else {
                if Some(operator__451.as_str()) == Some(">") {
                    return__157 = Some(rv__454) > Some(cv__455);
                } else {
                    if Some(operator__451.as_str()) == Some("<") {
                        return__157 = Some(rv__454) < Some(cv__455);
                    } else {
                        if Some(operator__451.as_str()) == Some(">=") {
                            return__157 = Some(rv__454) >= Some(cv__455);
                        } else {
                            if Some(operator__451.as_str()) == Some("<=") {
                                return__157 = Some(rv__454) <= Some(cv__455);
                            } else {
                                return__157 = false;
                            }
                        }
                    }
                }
            }
        }
    }
    return return__157;
}
fn compareString__196(recordValue__456: impl temper_core::ToArcString, operator__457: impl temper_core::ToArcString, clauseValue__458: impl temper_core::ToArcString) -> bool {
    let recordValue__456 = recordValue__456.to_arc_string();
    let operator__457 = operator__457.to_arc_string();
    let clauseValue__458 = clauseValue__458.to_arc_string();
    let return__158: bool;
    if Some(operator__457.as_str()) == Some("==") {
        return__158 = Some(recordValue__456.as_str()) == Some(clauseValue__458.as_str());
    } else {
        if Some(operator__457.as_str()) == Some("!=") {
            return__158 = Some(recordValue__456.as_str()) != Some(clauseValue__458.as_str());
        } else {
            if Some(operator__457.as_str()) == Some(">") {
                return__158 = Some(recordValue__456.clone()) > Some(clauseValue__458.clone());
            } else {
                if Some(operator__457.as_str()) == Some("<") {
                    return__158 = Some(recordValue__456.clone()) < Some(clauseValue__458.clone());
                } else {
                    if Some(operator__457.as_str()) == Some(">=") {
                        return__158 = Some(recordValue__456.clone()) >= Some(clauseValue__458.clone());
                    } else {
                        if Some(operator__457.as_str()) == Some("<=") {
                            return__158 = Some(recordValue__456.clone()) <= Some(clauseValue__458.clone());
                        } else {
                            return__158 = false;
                        }
                    }
                }
            }
        }
    }
    return return__158;
}
fn safeSql__198(trusted__462: impl temper_core::ToArcString) -> crate::sql::SqlFragment {
    let trusted__462 = trusted__462.to_arc_string();
    let b__464: crate::sql::SqlBuilder = crate::sql::SqlBuilder::new();
    b__464.append_safe(trusted__462.clone());
    return b__464.accumulated();
}
fn columnListSql__199(selectFields__465: impl temper_core::ToList<std::sync::Arc<String>>) -> crate::sql::SqlFragment {
    let selectFields__465 = selectFields__465.to_list();
    let return__161: crate::sql::SqlFragment;
    let mut t___4009: crate::sql::SqlBuilder;
    let mut t___4012: std::sync::Arc<String>;
    let mut t___4014: crate::sql::SqlBuilder;
    let mut t___4016: crate::sql::SqlFragment;
    let mut t___4017: i32;
    let mut t___4018: std::sync::Arc<String>;
    let mut t___4020: crate::sql::SqlBuilder;
    let mut t___4024: crate::sql::SqlFragment;
    if Some(temper_core::ListedTrait::len( & selectFields__465)) == Some(0) {
        t___4009 = crate::sql::SqlBuilder::new();
        t___4009.append_safe("*");
        return__161 = t___4009.accumulated();
    } else {
        t___4012 = temper_core::ListedTrait::get( & selectFields__465, 0);
        let first__467: crate::sql::SqlFragment = safeSql__198(t___4012.clone());
        t___4014 = crate::sql::SqlBuilder::new();
        t___4014.append_fragment(first__467.clone());
        t___4016 = t___4014.accumulated();
        let mut result__468: crate::sql::SqlFragment = t___4016.clone();
        let mut i__469: i32 = 1;
        'loop___6001: loop {
            t___4017 = temper_core::ListedTrait::len( & selectFields__465);
            if ! (Some(i__469) < Some(t___4017)) {
                break;
            }
            t___4018 = temper_core::ListedTrait::get( & selectFields__465, i__469);
            let col__470: crate::sql::SqlFragment = safeSql__198(t___4018.clone());
            t___4020 = crate::sql::SqlBuilder::new();
            t___4020.append_fragment(result__468.clone());
            t___4020.append_safe(", ");
            t___4020.append_fragment(col__470.clone());
            t___4024 = t___4020.accumulated();
            result__468 = t___4024.clone();
            i__469 = i__469.wrapping_add(1);
        }
        return__161 = result__468.clone();
    }
    return return__161.clone();
}
fn validOperator__197(op__460: impl temper_core::ToArcString) -> std::sync::Arc<String> {
    let op__460 = op__460.to_arc_string();
    let return__159: std::sync::Arc<String>;
    if Some(op__460.as_str()) == Some("=") {
        return__159 = std::sync::Arc::new("=".to_string());
    } else {
        if Some(op__460.as_str()) == Some("==") {
            return__159 = std::sync::Arc::new("=".to_string());
        } else {
            if Some(op__460.as_str()) == Some("!=") {
                return__159 = std::sync::Arc::new("!=".to_string());
            } else {
                if Some(op__460.as_str()) == Some("<>") {
                    return__159 = std::sync::Arc::new("<>".to_string());
                } else {
                    if Some(op__460.as_str()) == Some(">") {
                        return__159 = std::sync::Arc::new(">".to_string());
                    } else {
                        if Some(op__460.as_str()) == Some("<") {
                            return__159 = std::sync::Arc::new("<".to_string());
                        } else {
                            if Some(op__460.as_str()) == Some(">=") {
                                return__159 = std::sync::Arc::new(">=".to_string());
                            } else {
                                if Some(op__460.as_str()) == Some("<=") {
                                    return__159 = std::sync::Arc::new("<=".to_string());
                                } else {
                                    return__159 = std::sync::Arc::new("=".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return return__159.clone();
}
fn whereConditionSql__200(clause__471: WhereClause, schema__472: Schema) -> crate::sql::SqlFragment {
    let return__162: crate::sql::SqlFragment;
    let mut t___3985: std::sync::Arc<String>;
    let mut t___3990: crate::sql::SqlBuilder;
    let mut t___3993: crate::sql::SqlBuilder;
    let mut t___4001: crate::sql::SqlBuilder;
    let mut t___2316: Field;
    let mut t___2320: i32;
    let mut t___3981: std::sync::Arc<String> = clause__471.field();
    let col__474: crate::sql::SqlFragment = safeSql__198(t___3981.clone());
    let mut t___3983: std::sync::Arc<String> = clause__471.operator();
    let op__475: crate::sql::SqlFragment = safeSql__198(validOperator__197(t___3983.clone()));
    let fieldInfo__476: Field;
    'ok___5894: {
        'orelse___942: {
            t___3985 = clause__471.field();
            t___2316 = match schema__472.get_field(t___3985.clone()) {
                Ok(x) => x,
                _ => break 'orelse___942
            };
            fieldInfo__476 = t___2316.clone();
            break 'ok___5894;
        }
        fieldInfo__476 = panic!();
    }
    if Some(fieldInfo__476.field_type().as_str()) == Some("Int") {
        let intVal__477: i32;
        'ok___5895: {
            'orelse___943: {
                t___2320 = match temper_core::string::to_int( & clause__471.value(), None) {
                    Ok(x) => x,
                    _ => break 'orelse___943
                };
                intVal__477 = t___2320;
                break 'ok___5895;
            }
            intVal__477 = 0;
        }
        if Some(clause__471.value().as_str()) != Some(temper_core::int_to_string(intVal__477, None).as_str()) {
            t___3990 = crate::sql::SqlBuilder::new();
            t___3990.append_safe("1 = 0");
            return__162 = t___3990.accumulated();
        } else {
            t___3993 = crate::sql::SqlBuilder::new();
            t___3993.append_fragment(col__474.clone());
            t___3993.append_safe(" ");
            t___3993.append_fragment(op__475.clone());
            t___3993.append_safe(" ");
            t___3993.append_int32(intVal__477);
            return__162 = t___3993.accumulated();
        }
    } else {
        let strVal__478: std::sync::Arc<String> = clause__471.value();
        t___4001 = crate::sql::SqlBuilder::new();
        t___4001.append_fragment(col__474.clone());
        t___4001.append_safe(" ");
        t___4001.append_fragment(op__475.clone());
        t___4001.append_safe(" ");
        t___4001.append_string(strVal__478.clone());
        return__162 = t___4001.accumulated();
    }
    return return__162.clone();
}
fn orderBySql__201(clauses__479: impl temper_core::ToList<OrderClause>) -> crate::sql::SqlFragment {
    let clauses__479 = clauses__479.to_list();
    let mut t___3961: crate::sql::SqlFragment;
    let mut t___3962: crate::sql::SqlFragment;
    let mut t___3967: i32;
    let mut t___3969: std::sync::Arc<String>;
    let mut t___3973: crate::sql::SqlFragment;
    let mut t___3974: crate::sql::SqlFragment;
    let mut t___3975: crate::sql::SqlBuilder;
    let mut t___3980: crate::sql::SqlFragment;
    let mut t___2303: crate::sql::SqlFragment;
    let mut t___3957: std::sync::Arc<String> = temper_core::ListedTrait::get( & clauses__479, 0).field();
    let first__481: crate::sql::SqlFragment = safeSql__198(t___3957.clone());
    let firstDir__482: crate::sql::SqlFragment;
    if Some(temper_core::ListedTrait::get( & clauses__479, 0).direction().as_str()) == Some("desc") {
        t___3961 = safeSql__198(" DESC");
        firstDir__482 = t___3961.clone();
    } else {
        t___3962 = safeSql__198(" ASC");
        firstDir__482 = t___3962.clone();
    }
    let mut t___3963: crate::sql::SqlBuilder = crate::sql::SqlBuilder::new();
    t___3963.append_fragment(first__481.clone());
    t___3963.append_fragment(firstDir__482.clone());
    let mut t___3966: crate::sql::SqlFragment = t___3963.accumulated();
    let mut result__483: crate::sql::SqlFragment = t___3966.clone();
    let mut i__484: i32 = 1;
    'loop___6002: loop {
        t___3967 = temper_core::ListedTrait::len( & clauses__479);
        if ! (Some(i__484) < Some(t___3967)) {
            break;
        }
        t___3969 = temper_core::ListedTrait::get( & clauses__479, i__484).field();
        let col__485: crate::sql::SqlFragment = safeSql__198(t___3969.clone());
        if Some(temper_core::ListedTrait::get( & clauses__479, i__484).direction().as_str()) == Some("desc") {
            t___3973 = safeSql__198(" DESC");
            t___2303 = t___3973.clone();
        } else {
            t___3974 = safeSql__198(" ASC");
            t___2303 = t___3974.clone();
        }
        let dir__486: crate::sql::SqlFragment = t___2303.clone();
        t___3975 = crate::sql::SqlBuilder::new();
        t___3975.append_fragment(result__483.clone());
        t___3975.append_safe(", ");
        t___3975.append_fragment(col__485.clone());
        t___3975.append_fragment(dir__486.clone());
        t___3980 = t___3975.accumulated();
        result__483 = t___3980.clone();
        i__484 = i__484.wrapping_add(1);
    }
    return result__483.clone();
}
pub fn to_sql_query(schema__487: Schema, selectFields__488: impl temper_core::ToList<std::sync::Arc<String>>, whereClauses__489: impl temper_core::ToList<WhereClause>, orderClauses__490: impl temper_core::ToList<OrderClause>, limitValue__491: i32, offsetValue__492: i32) -> crate::sql::SqlFragment {
    let selectFields__488 = selectFields__488.to_list();
    let whereClauses__489 = whereClauses__489.to_list();
    let orderClauses__490 = orderClauses__490.to_list();
    let mut t___3924: WhereClause;
    let mut t___3925: crate::sql::SqlFragment;
    let mut t___3926: i32;
    let mut t___3927: WhereClause;
    let mut t___3929: crate::sql::SqlBuilder;
    let mut t___3933: crate::sql::SqlFragment;
    let mut t___3934: crate::sql::SqlBuilder;
    let mut t___3938: crate::sql::SqlFragment;
    let mut t___3941: crate::sql::SqlBuilder;
    let mut t___3945: crate::sql::SqlFragment;
    let mut t___3946: crate::sql::SqlBuilder;
    let mut t___3950: crate::sql::SqlFragment;
    let mut t___3951: crate::sql::SqlBuilder;
    let mut t___3955: crate::sql::SqlFragment;
    #[derive(Clone)]
    struct ClosureGroup___12 {
        schema__487: Schema
    }
    impl ClosureGroup___12 {
        fn fn__3907(& self, f__495: impl temper_core::ToArcString) -> bool {
            let f__495 = f__495.to_arc_string();
            return self.schema__487.has_field(f__495.clone());
        }
    }
    let closure_group = ClosureGroup___12 {
        schema__487: schema__487.clone()
    };
    let fn__3907 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | f__495: std::sync::Arc<String> | closure_group.fn__3907(f__495))
    };
    let validSelect__494: temper_core::List<std::sync::Arc<String>> = temper_core::listed::filter( & selectFields__488, & ( * fn__3907.clone()));
    #[derive(Clone)]
    struct ClosureGroup___13 {
        schema__487: Schema
    }
    impl ClosureGroup___13 {
        fn fn__3906(& self, c__498: WhereClause) -> bool {
            let mut t___3902: std::sync::Arc<String> = c__498.field();
            return self.schema__487.has_field(t___3902.clone());
        }
    }
    let closure_group = ClosureGroup___13 {
        schema__487: schema__487.clone()
    };
    let fn__3906 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | c__498: WhereClause | closure_group.fn__3906(c__498))
    };
    let validWhere__497: temper_core::List<WhereClause> = temper_core::listed::filter( & whereClauses__489, & ( * fn__3906.clone()));
    #[derive(Clone)]
    struct ClosureGroup___14 {
        schema__487: Schema
    }
    impl ClosureGroup___14 {
        fn fn__3905(& self, c__501: OrderClause) -> bool {
            let mut t___3900: std::sync::Arc<String> = c__501.field();
            return self.schema__487.has_field(t___3900.clone());
        }
    }
    let closure_group = ClosureGroup___14 {
        schema__487: schema__487.clone()
    };
    let fn__3905 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | c__501: OrderClause | closure_group.fn__3905(c__501))
    };
    let validOrder__500: temper_core::List<OrderClause> = temper_core::listed::filter( & orderClauses__490, & ( * fn__3905.clone()));
    let mut t___3914: std::sync::Arc<String> = schema__487.table_name();
    let table__503: crate::sql::SqlFragment = safeSql__198(t___3914.clone());
    let cols__504: crate::sql::SqlFragment = columnListSql__199(validSelect__494.clone());
    let mut t___3917: crate::sql::SqlBuilder = crate::sql::SqlBuilder::new();
    t___3917.append_safe("SELECT ");
    t___3917.append_fragment(cols__504.clone());
    t___3917.append_safe(" FROM ");
    t___3917.append_fragment(table__503.clone());
    let mut t___3922: crate::sql::SqlFragment = t___3917.accumulated();
    let mut result__505: crate::sql::SqlFragment = t___3922.clone();
    if Some(temper_core::ListedTrait::len( & validWhere__497)) > Some(0) {
        t___3924 = temper_core::ListedTrait::get( & validWhere__497, 0);
        t___3925 = whereConditionSql__200(t___3924.clone(), schema__487.clone());
        let mut conditions__506: crate::sql::SqlFragment = t___3925.clone();
        let mut i__507: i32 = 1;
        'loop___6003: loop {
            t___3926 = temper_core::ListedTrait::len( & validWhere__497);
            if ! (Some(i__507) < Some(t___3926)) {
                break;
            }
            t___3927 = temper_core::ListedTrait::get( & validWhere__497, i__507);
            let next__508: crate::sql::SqlFragment = whereConditionSql__200(t___3927.clone(), schema__487.clone());
            t___3929 = crate::sql::SqlBuilder::new();
            t___3929.append_fragment(conditions__506.clone());
            t___3929.append_safe(" AND ");
            t___3929.append_fragment(next__508.clone());
            t___3933 = t___3929.accumulated();
            conditions__506 = t___3933.clone();
            i__507 = i__507.wrapping_add(1);
        }
        t___3934 = crate::sql::SqlBuilder::new();
        t___3934.append_fragment(result__505.clone());
        t___3934.append_safe(" WHERE ");
        t___3934.append_fragment(conditions__506.clone());
        t___3938 = t___3934.accumulated();
        result__505 = t___3938.clone();
    }
    if Some(temper_core::ListedTrait::len( & validOrder__500)) > Some(0) {
        let ordering__509: crate::sql::SqlFragment = orderBySql__201(validOrder__500.clone());
        t___3941 = crate::sql::SqlBuilder::new();
        t___3941.append_fragment(result__505.clone());
        t___3941.append_safe(" ORDER BY ");
        t___3941.append_fragment(ordering__509.clone());
        t___3945 = t___3941.accumulated();
        result__505 = t___3945.clone();
    }
    if Some(limitValue__491) >= Some(0) {
        t___3946 = crate::sql::SqlBuilder::new();
        t___3946.append_fragment(result__505.clone());
        t___3946.append_safe(" LIMIT ");
        t___3946.append_int32(limitValue__491);
        t___3950 = t___3946.accumulated();
        result__505 = t___3950.clone();
    }
    if Some(offsetValue__492) > Some(0) {
        t___3951 = crate::sql::SqlBuilder::new();
        t___3951.append_fragment(result__505.clone());
        t___3951.append_safe(" OFFSET ");
        t___3951.append_int32(offsetValue__492);
        t___3955 = t___3951.accumulated();
        result__505 = t___3955.clone();
    }
    return result__505.clone();
}
struct TokenTypeStruct {
    name: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct TokenType(std::sync::Arc<TokenTypeStruct>);
impl TokenType {
    pub fn is_keyword(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("keyword");
    }
    pub fn is_type(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("type");
    }
    pub fn is_string(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("string");
    }
    pub fn is_number(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("number");
    }
    pub fn is_comment(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("comment");
    }
    pub fn is_operator(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("operator");
    }
    pub fn is_identifier(& self) -> bool {
        return Some(self.0.name.as_str()) == Some("identifier");
    }
    pub fn new(name__673: impl temper_core::ToArcString) -> TokenType {
        let name__673 = name__673.to_arc_string();
        let name;
        name = name__673.clone();
        let selfish = TokenType(std::sync::Arc::new(TokenTypeStruct {
                    name
        }));
        return selfish;
    }
    pub fn name(& self) -> std::sync::Arc<String> {
        return self.0.name.clone();
    }
}
temper_core::impl_any_value_trait!(TokenType, []);
struct TokenStruct {
    token_type: TokenType, value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct Token(std::sync::Arc<TokenStruct>);
#[derive(Clone)]
pub struct TokenBuilder {
    pub token_type: TokenType, pub value: std::sync::Arc<String>
}
impl TokenBuilder {
    pub fn build(self) -> Token {
        Token::new(self.token_type, self.value)
    }
}
impl Token {
    pub fn css_class(& self) -> std::sync::Arc<String> {
        let return__185: std::sync::Arc<String>;
        let name__678: std::sync::Arc<String> = self.0.token_type.name();
        if Some(name__678.as_str()) == Some("keyword") {
            return__185 = std::sync::Arc::new("kw".to_string());
        } else {
            if Some(name__678.as_str()) == Some("type") {
                return__185 = std::sync::Arc::new("typ".to_string());
            } else {
                if Some(name__678.as_str()) == Some("string") {
                    return__185 = std::sync::Arc::new("str".to_string());
                } else {
                    if Some(name__678.as_str()) == Some("number") {
                        return__185 = std::sync::Arc::new("num".to_string());
                    } else {
                        if Some(name__678.as_str()) == Some("comment") {
                            return__185 = std::sync::Arc::new("cmt".to_string());
                        } else {
                            if Some(name__678.as_str()) == Some("operator") {
                                return__185 = std::sync::Arc::new("op".to_string());
                            } else {
                                return__185 = std::sync::Arc::new("id".to_string());
                            }
                        }
                    }
                }
            }
        }
        return return__185.clone();
    }
    pub fn to_html(& self) -> crate::html::SafeHtml {
        let cls__681: std::sync::Arc<String> = self.css_class();
        let mut t___3288: crate::html::SafeHtmlBuilder = crate::html::SafeHtmlBuilder::new();
        t___3288.append_safe(std::sync::Arc::new("<span class='".to_string()));
        t___3288.append_string(cls__681.clone());
        t___3288.append_safe(std::sync::Arc::new("'>".to_string()));
        t___3288.append_string(self.0.value.clone());
        t___3288.append_safe(std::sync::Arc::new("</span>".to_string()));
        return t___3288.accumulated();
    }
    pub fn new(tokenType__683: TokenType, value__684: impl temper_core::ToArcString) -> Token {
        let value__684 = value__684.to_arc_string();
        let token_type;
        let value;
        token_type = tokenType__683.clone();
        value = value__684.clone();
        let selfish = Token(std::sync::Arc::new(TokenStruct {
                    token_type, value
        }));
        return selfish;
    }
    pub fn token_type(& self) -> TokenType {
        return self.0.token_type.clone();
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
temper_core::impl_any_value_trait!(Token, []);
pub fn field(name__293: impl temper_core::ToArcString, fieldType__294: impl temper_core::ToArcString, primaryKey__295: bool, nullable__296: bool) -> Field {
    let name__293 = name__293.to_arc_string();
    let fieldType__294 = fieldType__294.to_arc_string();
    return Field::new(name__293.clone(), fieldType__294.clone(), primaryKey__295, nullable__296);
}
pub fn is_valid_identifier(name__298: impl temper_core::ToArcString) -> bool {
    let name__298 = name__298.to_arc_string();
    let return__123: bool;
    let mut t___3781: usize;
    'fn__299: {
        if name__298.is_empty() {
            return__123 = false;
            break 'fn__299;
        }
        let this__2531: std::sync::Arc<String> = name__298.clone();
        let mut index__2533: usize = 0usize;
        'loop___6004: loop {
            if ! temper_core::string::has_index( & this__2531, index__2533) {
                break;
            }
            let codePoint__2534: i32 = temper_core::string::get( & this__2531, index__2533);
            let c__300: i32 = codePoint__2534;
            if Some(c__300) != Some(95) {
                if Some(c__300) >= Some(97) {
                    if Some(c__300) > Some(122) {
                        return__123 = false;
                        break 'fn__299;
                    }
                } else {
                    if Some(c__300) >= Some(65) {
                        if Some(c__300) > Some(90) {
                            return__123 = false;
                            break 'fn__299;
                        }
                    } else {
                        if Some(c__300) >= Some(48) {
                            if Some(c__300) > Some(57) {
                                return__123 = false;
                                break 'fn__299;
                            }
                        } else {
                            return__123 = false;
                            break 'fn__299;
                        }
                    }
                }
            }
            t___3781 = temper_core::string::next( & this__2531, index__2533);
            index__2533 = t___3781;
        }
        return__123 = true;
    }
    return return__123;
}
pub fn schema(tableName__301: impl temper_core::ToArcString, fields__302: impl temper_core::ToList<Field>) -> Schema {
    let tableName__301 = tableName__301.to_arc_string();
    let fields__302 = fields__302.to_list();
    if ! is_valid_identifier(tableName__301.clone()) {
        return panic!();
    }
    let idField__304: Field = Field::new("id", "Int", true, false);
    let allFields__305: temper_core::ListBuilder<Field> = temper_core::listed::new_builder();
    temper_core::listed::add( & allFields__305, idField__304.clone(), None);
    temper_core::listed::add_all( & allFields__305, temper_core::ToListed::to_listed(fields__302.clone()), None);
    let mut t___3776: temper_core::List<Field> = temper_core::ListedTrait::to_list( & allFields__305);
    return Schema::new(tableName__301.clone(), t___3776.clone());
}
pub fn to_insert_sql(schema__510: Schema, values__511: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> crate::sql::SqlFragment {
    let return__165: crate::sql::SqlFragment;
    let mut t___3724: std::sync::Arc<String>;
    let mut t___3726: temper_core::List<Field>;
    let mut t___3730: crate::sql::SqlBuilder;
    let mut t___3733: temper_core::List<std::sync::Arc<String>>;
    let mut t___3736: std::sync::Arc<String>;
    let mut t___3740: crate::sql::SqlBuilder;
    let mut t___3742: crate::sql::SqlFragment;
    let mut t___3743: crate::sql::SqlBuilder;
    let mut t___3745: crate::sql::SqlFragment;
    let mut t___3746: i32;
    let mut t___3748: std::sync::Arc<String>;
    let mut t___3752: crate::sql::SqlBuilder;
    let mut t___3756: crate::sql::SqlFragment;
    let mut t___3757: crate::sql::SqlBuilder;
    let mut t___3761: crate::sql::SqlFragment;
    let mut t___3762: crate::sql::SqlBuilder;
    let mut t___2047: i32;
    let mut t___2058: crate::sql::SqlFragment;
    let mut t___2065: i32;
    let mut t___2066: i32;
    'fn__512: {
        t___3724 = schema__510.table_name();
        let table__513: crate::sql::SqlFragment = safeSql__198(t___3724.clone());
        t___3726 = schema__510.fields();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            values__511: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___15 {
            fn fn__3723(& self, f__515: Field) -> bool {
                let mut t___3720: std::sync::Arc<String> = f__515.name();
                return temper_core::MappedTrait::has( & self.values__511, t___3720.clone());
            }
        }
        let closure_group = ClosureGroup___15 {
            values__511: values__511.clone()
        };
        let fn__3723 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__515: Field | closure_group.fn__3723(f__515))
        };
        let fieldList__514: temper_core::List<Field> = temper_core::listed::filter( & t___3726, & ( * fn__3723.clone()));
        if Some(temper_core::ListedTrait::len( & fieldList__514)) == Some(0) {
            t___3730 = crate::sql::SqlBuilder::new();
            return__165 = t___3730.accumulated();
            break 'fn__512;
        }
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__3722(& self, f__518: Field) -> std::sync::Arc<String> {
                return f__518.name();
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__3722 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__518: Field | closure_group.fn__3722(f__518))
        };
        t___3733 = temper_core::listed::map( & fieldList__514, & ( * fn__3722.clone()));
        let colNames__517: crate::sql::SqlFragment = columnListSql__199(t___3733.clone());
        t___3736 = temper_core::ListedTrait::get( & fieldList__514, 0).name();
        let firstVal__520: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & values__511, t___3736.clone(), std::sync::Arc::new("".to_string()));
        if Some(temper_core::ListedTrait::get( & fieldList__514, 0).field_type().as_str()) == Some("Int") {
            let iv__522: i32;
            'ok___5900: {
                'orelse___948: {
                    t___2047 = match temper_core::string::to_int( & firstVal__520, None) {
                        Ok(x) => x,
                        _ => break 'orelse___948
                    };
                    iv__522 = t___2047;
                    break 'ok___5900;
                }
                iv__522 = 0;
            }
            t___3740 = crate::sql::SqlBuilder::new();
            t___3740.append_int32(iv__522);
            t___3742 = t___3740.accumulated();
            t___2058 = t___3742.clone();
        } else {
            t___3743 = crate::sql::SqlBuilder::new();
            t___3743.append_string(firstVal__520.clone());
            t___3745 = t___3743.accumulated();
            t___2058 = t___3745.clone();
        }
        let mut vals__521: crate::sql::SqlFragment = t___2058.clone();
        let mut i__523: i32 = 1;
        'loop___6005: loop {
            t___3746 = temper_core::ListedTrait::len( & fieldList__514);
            if ! (Some(i__523) < Some(t___3746)) {
                break;
            }
            t___3748 = temper_core::ListedTrait::get( & fieldList__514, i__523).name();
            let val__524: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & values__511, t___3748.clone(), std::sync::Arc::new("".to_string()));
            if Some(temper_core::ListedTrait::get( & fieldList__514, i__523).field_type().as_str()) == Some("Int") {
                'ok___5901: {
                    'orelse___949: {
                        t___2065 = match temper_core::string::to_int( & val__524, None) {
                            Ok(x) => x,
                            _ => break 'orelse___949
                        };
                        t___2066 = t___2065;
                        break 'ok___5901;
                    }
                    t___2066 = 0;
                }
                let iv__525: i32 = t___2066;
                t___3752 = crate::sql::SqlBuilder::new();
                t___3752.append_fragment(vals__521.clone());
                t___3752.append_safe(", ");
                t___3752.append_int32(iv__525);
                t___3756 = t___3752.accumulated();
                vals__521 = t___3756.clone();
            } else {
                t___3757 = crate::sql::SqlBuilder::new();
                t___3757.append_fragment(vals__521.clone());
                t___3757.append_safe(", ");
                t___3757.append_string(val__524.clone());
                t___3761 = t___3757.accumulated();
                vals__521 = t___3761.clone();
            }
            i__523 = i__523.wrapping_add(1);
        }
        t___3762 = crate::sql::SqlBuilder::new();
        t___3762.append_safe("INSERT INTO ");
        t___3762.append_fragment(table__513.clone());
        t___3762.append_safe(" (");
        t___3762.append_fragment(colNames__517.clone());
        t___3762.append_safe(") VALUES (");
        t___3762.append_fragment(vals__521.clone());
        t___3762.append_safe(")");
        return__165 = t___3762.accumulated();
    }
    return return__165.clone();
}
pub fn main() {
    println!("{}", "=== ORMery Demo ===\x0a");
    let userFields__642: temper_core::List<Field> = std::sync::Arc::new(vec![field("name", "String", false, false), field("age", "Int", false, false), field("email", "String", false, true)]);
    let userSchema__643: Schema = schema("users", userFields__642.clone());
    let mut t___3304: std::sync::Arc<String> = userSchema__643.describe();
    println!("{}", t___3304.clone());
    println!("{}", "");
    let store__644: InMemoryStore = InMemoryStore::new();
    let rec1__645: Record = store__644.insert("users", temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]));
    let rec2__646: Record = store__644.insert("users", temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]));
    let rec3__647: Record = store__644.insert("users", temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Charlie".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("17".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("charlie@example.com".to_string()))]));
    println!("{}", "Inserted 3 users:");
    let mut t___3324: std::sync::Arc<String> = rec1__645.describe();
    println!("  {}", t___3324.clone());
    let mut t___3326: std::sync::Arc<String> = rec2__646.describe();
    println!("  {}", t___3326.clone());
    let mut t___3328: std::sync::Arc<String> = rec3__647.describe();
    println!("  {}", t___3328.clone());
    println!("{}", "");
    println!("{}", "=== In-Memory Queries ===\x0a");
    println!("{}", "All users:");
    let allUsers__648: temper_core::List<Record> = Query::new(userSchema__643.clone(), store__644.clone()).all();
    #[derive(Clone)]
    struct ClosureGroup___56 {}
    impl ClosureGroup___56 {
        fn fn__3301(& self, u__649: Record) {
            let mut t___3298: std::sync::Arc<String> = u__649.describe();
            println!("  {}", t___3298.clone());
        }
    }
    let closure_group = ClosureGroup___56 {};
    let fn__3301 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | u__649: Record | closure_group.fn__3301(u__649))
    };
    temper_core::listed::list_for_each( & allUsers__648, & ( * fn__3301.clone()));
    println!("{}", "");
    println!("{}", "Adults (age >= 18):");
    let adults__650: temper_core::List<Record> = Query::new(userSchema__643.clone(), store__644.clone()).r#where("age", ">=", "18").all();
    #[derive(Clone)]
    struct ClosureGroup___57 {}
    impl ClosureGroup___57 {
        fn fn__3300(& self, u__651: Record) {
            let mut t___3296: std::sync::Arc<String> = u__651.describe();
            println!("  {}", t___3296.clone());
        }
    }
    let closure_group = ClosureGroup___57 {};
    let fn__3300 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | u__651: Record | closure_group.fn__3300(u__651))
    };
    temper_core::listed::list_for_each( & adults__650, & ( * fn__3300.clone()));
    println!("{}", "");
    println!("{}", "=== SQL Generation (secure-composition) ===\x0a");
    let q1__652: Query = Query::new(userSchema__643.clone(), store__644.clone());
    let mut t___3348: std::sync::Arc<String> = q1__652.to_sql().to_string();
    println!("SELECT all: {}", t___3348.clone());
    let q2__653: Query = Query::new(userSchema__643.clone(), store__644.clone()).select([std::sync::Arc::new("name".to_string()), std::sync::Arc::new("age".to_string())]).r#where("age", ">=", "18").order_by("age", "desc").limit(10);
    let mut t___3356: std::sync::Arc<String> = q2__653.to_sql().to_string();
    println!("Complex:    {}", t___3356.clone());
    let bobby__654: std::sync::Arc<String> = std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string());
    let q3__655: Query = Query::new(userSchema__643.clone(), store__644.clone()).r#where("name", "=", "Robert'); DROP TABLE users;--");
    let mut t___3361: std::sync::Arc<String> = q3__655.to_sql().to_string();
    println!("Injection:  {}", t___3361.clone());
    let insertVals__656: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("O'Malley".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("42".to_string()))]);
    let mut t___3367: std::sync::Arc<String> = to_insert_sql(userSchema__643.clone(), insertVals__656.clone()).to_string();
    println!("INSERT:     {}", t___3367.clone());
    println!("{}", "\x0a=== Demo Complete ===");
}
pub fn classify_word(word__685: impl temper_core::ToArcString) -> TokenType {
    let word__685 = word__685.to_arc_string();
    let return__187: TokenType;
    'fn__686: {
        let this__2536: temper_core::List<std::sync::Arc<String>> = temper_keywords().clone();
        let n__2537: i32 = temper_core::ListedTrait::len( & this__2536);
        let mut i__2538: i32 = 0;
        'loop___6006: while Some(i__2538) < Some(n__2537) {
            let el__2539: std::sync::Arc<String> = temper_core::ListedTrait::get( & this__2536, i__2538);
            i__2538 = i__2538.wrapping_add(1);
            let kw__687: std::sync::Arc<String> = el__2539.clone();
            if Some(kw__687.as_str()) == Some(word__685.as_str()) {
                return__187 = TokenType::new("keyword");
                break 'fn__686;
            }
        }
        let this__2541: temper_core::List<std::sync::Arc<String>> = temper_types().clone();
        let n__2542: i32 = temper_core::ListedTrait::len( & this__2541);
        let mut i__2543: i32 = 0;
        'loop___6007: while Some(i__2543) < Some(n__2542) {
            let el__2544: std::sync::Arc<String> = temper_core::ListedTrait::get( & this__2541, i__2543);
            i__2543 = i__2543.wrapping_add(1);
            let tp__688: std::sync::Arc<String> = el__2544.clone();
            if Some(tp__688.as_str()) == Some(word__685.as_str()) {
                return__187 = TokenType::new("type");
                break 'fn__686;
            }
        }
        return__187 = TokenType::new("identifier");
    }
    return return__187.clone();
}
pub fn highlight_word(word__689: impl temper_core::ToArcString) -> crate::html::SafeHtml {
    let word__689 = word__689.to_arc_string();
    let return__188: crate::html::SafeHtml;
    let mut t___3275: crate::html::SafeHtmlBuilder;
    'fn__690: {
        if Some(word__689.as_str()) == Some("") {
            t___3275 = crate::html::SafeHtmlBuilder::new();
            return__188 = t___3275.accumulated();
            break 'fn__690;
        }
        let tokenType__691: TokenType = classify_word(word__689.clone());
        let token__692: Token = Token::new(tokenType__691.clone(), word__689.clone());
        return__188 = token__692.to_html();
    }
    return return__188.clone();
}
pub fn highlight_line(line__693: impl temper_core::ToArcString) -> crate::html::SafeHtml {
    let line__693 = line__693.to_arc_string();
    let return__189: crate::html::SafeHtml;
    let mut t___3263: crate::html::SafeHtmlBuilder;
    let mut t___3266: crate::html::SafeHtml;
    let mut t___3267: i32;
    let mut t___3270: crate::html::SafeHtmlBuilder;
    let mut t___3274: crate::html::SafeHtml;
    'fn__694: {
        let words__695: temper_core::List<std::sync::Arc<String>> = temper_core::string::split( & line__693, " ");
        if Some(temper_core::ListedTrait::len( & words__695)) == Some(0) {
            t___3263 = crate::html::SafeHtmlBuilder::new();
            return__189 = t___3263.accumulated();
            break 'fn__694;
        }
        t___3266 = highlight_word(temper_core::ListedTrait::get( & words__695, 0));
        let mut result__696: crate::html::SafeHtml = t___3266.clone();
        let mut i__697: i32 = 1;
        'loop___6008: loop {
            t___3267 = temper_core::ListedTrait::len( & words__695);
            if ! (Some(i__697) < Some(t___3267)) {
                break;
            }
            let word__698: crate::html::SafeHtml = highlight_word(temper_core::ListedTrait::get( & words__695, i__697));
            t___3270 = crate::html::SafeHtmlBuilder::new();
            t___3270.append_safe_html(result__696.clone());
            t___3270.append_safe(std::sync::Arc::new(" ".to_string()));
            t___3270.append_safe_html(word__698.clone());
            t___3274 = t___3270.accumulated();
            result__696 = t___3274.clone();
            i__697 = i__697.wrapping_add(1);
        }
        return__189 = result__696.clone();
    }
    return return__189.clone();
}
pub fn highlight_source(source__699: impl temper_core::ToArcString) -> crate::html::SafeHtml {
    let source__699 = source__699.to_arc_string();
    let return__190: crate::html::SafeHtml;
    let mut t___3249: crate::html::SafeHtmlBuilder;
    let mut t___3252: crate::html::SafeHtml;
    let mut t___3253: i32;
    let mut t___3256: crate::html::SafeHtmlBuilder;
    let mut t___3260: crate::html::SafeHtml;
    'fn__700: {
        let lines__701: temper_core::List<std::sync::Arc<String>> = temper_core::string::split( & source__699, "\x0a");
        if Some(temper_core::ListedTrait::len( & lines__701)) == Some(0) {
            t___3249 = crate::html::SafeHtmlBuilder::new();
            return__190 = t___3249.accumulated();
            break 'fn__700;
        }
        t___3252 = highlight_line(temper_core::ListedTrait::get( & lines__701, 0));
        let mut result__702: crate::html::SafeHtml = t___3252.clone();
        let mut i__703: i32 = 1;
        'loop___6009: loop {
            t___3253 = temper_core::ListedTrait::len( & lines__701);
            if ! (Some(i__703) < Some(t___3253)) {
                break;
            }
            let line__704: crate::html::SafeHtml = highlight_line(temper_core::ListedTrait::get( & lines__701, i__703));
            t___3256 = crate::html::SafeHtmlBuilder::new();
            t___3256.append_safe_html(result__702.clone());
            t___3256.append_safe(std::sync::Arc::new("\\n".to_string()));
            t___3256.append_safe_html(line__704.clone());
            t___3260 = t___3256.accumulated();
            result__702 = t___3260.clone();
            i__703 = i__703.wrapping_add(1);
        }
        return__190 = result__702.clone();
    }
    return return__190.clone();
}
pub fn highlight_block(source__705: impl temper_core::ToArcString) -> crate::html::SafeHtml {
    let source__705 = source__705.to_arc_string();
    let highlighted__707: crate::html::SafeHtml = highlight_source(source__705.clone());
    let mut t___3242: crate::html::SafeHtmlBuilder = crate::html::SafeHtmlBuilder::new();
    t___3242.append_safe(std::sync::Arc::new("<pre class='temper-code'><code>".to_string()));
    t___3242.append_safe_html(highlighted__707.clone());
    t___3242.append_safe(std::sync::Arc::new("</code></pre>".to_string()));
    return t___3242.accumulated();
}
#[cfg(test)]
mod tests {
    #[test]
    fn toSqlSelectAll__830() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___17 = temper_std::testing::Test::new();
        let s__527: Schema = schema("users", [field("name", "String", false, false)]);
        let store__528: InMemoryStore = InMemoryStore::new();
        let q__529: Query = Query::new(s__527.clone(), store__528.clone());
        let actual___831: std::sync::Arc<String> = q__529.to_sql().to_string();
        let mut t___3717: bool = Some(actual___831.as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___17 {
            actual___831: std::sync::Arc<String>
        }
        impl ClosureGroup___17 {
            fn fn__3710(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users) not ({})", self.actual___831.clone()));
            }
        }
        let closure_group = ClosureGroup___17 {
            actual___831: actual___831.clone()
        };
        let fn__3710 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3710())
        };
        test___17.assert(t___3717, fn__3710.clone());
        test___17.soft_fail_to_hard()
    }
    #[test]
    fn toSqlSelectColumns__833() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___18 = temper_std::testing::Test::new();
        let s__531: Schema = schema("users", [field("name", "String", false, false), field("age", "Int", false, false)]);
        let store__532: InMemoryStore = InMemoryStore::new();
        let q__533: Query = Query::new(s__531.clone(), store__532.clone()).select([std::sync::Arc::new("name".to_string()), std::sync::Arc::new("age".to_string())]);
        let actual___834: std::sync::Arc<String> = q__533.to_sql().to_string();
        let mut t___3708: bool = Some(actual___834.as_str()) == Some("SELECT name, age FROM users");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            actual___834: std::sync::Arc<String>
        }
        impl ClosureGroup___18 {
            fn fn__3700(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT name, age FROM users) not ({})", self.actual___834.clone()));
            }
        }
        let closure_group = ClosureGroup___18 {
            actual___834: actual___834.clone()
        };
        let fn__3700 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3700())
        };
        test___18.assert(t___3708, fn__3700.clone());
        test___18.soft_fail_to_hard()
    }
    #[test]
    fn toSqlWhereString__836() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___19 = temper_std::testing::Test::new();
        let s__535: Schema = schema("users", [field("name", "String", false, false)]);
        let store__536: InMemoryStore = InMemoryStore::new();
        let q__537: Query = Query::new(s__535.clone(), store__536.clone()).r#where("name", "=", "Alice");
        let actual___837: std::sync::Arc<String> = q__537.to_sql().to_string();
        let mut t___3698: bool = Some(actual___837.as_str()) == Some("SELECT * FROM users WHERE name = 'Alice'");
        #[derive(Clone)]
        struct ClosureGroup___19 {
            actual___837: std::sync::Arc<String>
        }
        impl ClosureGroup___19 {
            fn fn__3690(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE name = 'Alice') not ({})", self.actual___837.clone()));
            }
        }
        let closure_group = ClosureGroup___19 {
            actual___837: actual___837.clone()
        };
        let fn__3690 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3690())
        };
        test___19.assert(t___3698, fn__3690.clone());
        test___19.soft_fail_to_hard()
    }
    #[test]
    fn toSqlWhereInt__839() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___20 = temper_std::testing::Test::new();
        let s__539: Schema = schema("users", [field("age", "Int", false, false)]);
        let store__540: InMemoryStore = InMemoryStore::new();
        let q__541: Query = Query::new(s__539.clone(), store__540.clone()).r#where("age", ">=", "18");
        let actual___840: std::sync::Arc<String> = q__541.to_sql().to_string();
        let mut t___3688: bool = Some(actual___840.as_str()) == Some("SELECT * FROM users WHERE age >= 18");
        #[derive(Clone)]
        struct ClosureGroup___20 {
            actual___840: std::sync::Arc<String>
        }
        impl ClosureGroup___20 {
            fn fn__3680(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE age >= 18) not ({})", self.actual___840.clone()));
            }
        }
        let closure_group = ClosureGroup___20 {
            actual___840: actual___840.clone()
        };
        let fn__3680 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3680())
        };
        test___20.assert(t___3688, fn__3680.clone());
        test___20.soft_fail_to_hard()
    }
    #[test]
    fn toSqlSqlInjectionBlocked__842() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___21 = temper_std::testing::Test::new();
        let s__543: Schema = schema("users", [field("name", "String", false, false)]);
        let store__544: InMemoryStore = InMemoryStore::new();
        let bobby__545: std::sync::Arc<String> = std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string());
        let q__546: Query = Query::new(s__543.clone(), store__544.clone()).r#where("name", "=", "Robert'); DROP TABLE users;--");
        let result__547: std::sync::Arc<String> = q__546.to_sql().to_string();
        let actual___843: std::sync::Arc<String> = result__547.clone();
        let mut t___3678: bool = Some(actual___843.as_str()) == Some("SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'");
        #[derive(Clone)]
        struct ClosureGroup___21 {
            actual___843: std::sync::Arc<String>
        }
        impl ClosureGroup___21 {
            fn fn__3670(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected result == (SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--') not ({})", self.actual___843.clone()));
            }
        }
        let closure_group = ClosureGroup___21 {
            actual___843: actual___843.clone()
        };
        let fn__3670 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3670())
        };
        test___21.assert(t___3678, fn__3670.clone());
        test___21.soft_fail_to_hard()
    }
    #[test]
    fn toSqlOperatorNormalization__845() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let s__549: Schema = schema("users", [field("name", "String", false, false)]);
        let store__550: InMemoryStore = InMemoryStore::new();
        let q__551: Query = Query::new(s__549.clone(), store__550.clone()).r#where("name", "==", "Alice");
        let actual___846: std::sync::Arc<String> = q__551.to_sql().to_string();
        let mut t___3668: bool = Some(actual___846.as_str()) == Some("SELECT * FROM users WHERE name = 'Alice'");
        #[derive(Clone)]
        struct ClosureGroup___22 {
            actual___846: std::sync::Arc<String>
        }
        impl ClosureGroup___22 {
            fn fn__3660(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE name = 'Alice') not ({})", self.actual___846.clone()));
            }
        }
        let closure_group = ClosureGroup___22 {
            actual___846: actual___846.clone()
        };
        let fn__3660 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3660())
        };
        test___22.assert(t___3668, fn__3660.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn toSqlInvalidOperatorFallback__848() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let s__553: Schema = schema("users", [field("name", "String", false, false)]);
        let store__554: InMemoryStore = InMemoryStore::new();
        let q__555: Query = Query::new(s__553.clone(), store__554.clone()).r#where("name", "LIKE", "Alice");
        let actual___849: std::sync::Arc<String> = q__555.to_sql().to_string();
        let mut t___3658: bool = Some(actual___849.as_str()) == Some("SELECT * FROM users WHERE name = 'Alice'");
        #[derive(Clone)]
        struct ClosureGroup___23 {
            actual___849: std::sync::Arc<String>
        }
        impl ClosureGroup___23 {
            fn fn__3650(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE name = 'Alice') not ({})", self.actual___849.clone()));
            }
        }
        let closure_group = ClosureGroup___23 {
            actual___849: actual___849.clone()
        };
        let fn__3650 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3650())
        };
        test___23.assert(t___3658, fn__3650.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn toSqlMultipleWhere__851() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let s__557: Schema = schema("users", [field("age", "Int", false, false)]);
        let store__558: InMemoryStore = InMemoryStore::new();
        let q__559: Query = Query::new(s__557.clone(), store__558.clone()).r#where("age", ">=", "18").r#where("age", "<", "30");
        let actual___852: std::sync::Arc<String> = q__559.to_sql().to_string();
        let mut t___3648: bool = Some(actual___852.as_str()) == Some("SELECT * FROM users WHERE age >= 18 AND age < 30");
        #[derive(Clone)]
        struct ClosureGroup___24 {
            actual___852: std::sync::Arc<String>
        }
        impl ClosureGroup___24 {
            fn fn__3639(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE age >= 18 AND age < 30) not ({})", self.actual___852.clone()));
            }
        }
        let closure_group = ClosureGroup___24 {
            actual___852: actual___852.clone()
        };
        let fn__3639 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3639())
        };
        test___24.assert(t___3648, fn__3639.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn toSqlOrderBy__854() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let s__561: Schema = schema("users", [field("name", "String", false, false)]);
        let store__562: InMemoryStore = InMemoryStore::new();
        let q__563: Query = Query::new(s__561.clone(), store__562.clone()).order_by("name", "asc");
        let actual___855: std::sync::Arc<String> = q__563.to_sql().to_string();
        let mut t___3637: bool = Some(actual___855.as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___25 {
            actual___855: std::sync::Arc<String>
        }
        impl ClosureGroup___25 {
            fn fn__3629(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users ORDER BY name ASC) not ({})", self.actual___855.clone()));
            }
        }
        let closure_group = ClosureGroup___25 {
            actual___855: actual___855.clone()
        };
        let fn__3629 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3629())
        };
        test___25.assert(t___3637, fn__3629.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn toSqlOrderByDesc__857() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let s__565: Schema = schema("users", [field("age", "Int", false, false)]);
        let store__566: InMemoryStore = InMemoryStore::new();
        let q__567: Query = Query::new(s__565.clone(), store__566.clone()).order_by("age", "desc");
        let actual___858: std::sync::Arc<String> = q__567.to_sql().to_string();
        let mut t___3627: bool = Some(actual___858.as_str()) == Some("SELECT * FROM users ORDER BY age DESC");
        #[derive(Clone)]
        struct ClosureGroup___26 {
            actual___858: std::sync::Arc<String>
        }
        impl ClosureGroup___26 {
            fn fn__3619(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users ORDER BY age DESC) not ({})", self.actual___858.clone()));
            }
        }
        let closure_group = ClosureGroup___26 {
            actual___858: actual___858.clone()
        };
        let fn__3619 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3619())
        };
        test___26.assert(t___3627, fn__3619.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn toSqlLimit__860() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let s__569: Schema = schema("users", [field("name", "String", false, false)]);
        let store__570: InMemoryStore = InMemoryStore::new();
        let q__571: Query = Query::new(s__569.clone(), store__570.clone()).limit(10);
        let actual___861: std::sync::Arc<String> = q__571.to_sql().to_string();
        let mut t___3617: bool = Some(actual___861.as_str()) == Some("SELECT * FROM users LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___27 {
            actual___861: std::sync::Arc<String>
        }
        impl ClosureGroup___27 {
            fn fn__3609(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users LIMIT 10) not ({})", self.actual___861.clone()));
            }
        }
        let closure_group = ClosureGroup___27 {
            actual___861: actual___861.clone()
        };
        let fn__3609 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3609())
        };
        test___27.assert(t___3617, fn__3609.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn toSqlOffset__863() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let s__573: Schema = schema("users", [field("name", "String", false, false)]);
        let store__574: InMemoryStore = InMemoryStore::new();
        let q__575: Query = Query::new(s__573.clone(), store__574.clone()).offset(5);
        let actual___864: std::sync::Arc<String> = q__575.to_sql().to_string();
        let mut t___3607: bool = Some(actual___864.as_str()) == Some("SELECT * FROM users OFFSET 5");
        #[derive(Clone)]
        struct ClosureGroup___28 {
            actual___864: std::sync::Arc<String>
        }
        impl ClosureGroup___28 {
            fn fn__3599(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users OFFSET 5) not ({})", self.actual___864.clone()));
            }
        }
        let closure_group = ClosureGroup___28 {
            actual___864: actual___864.clone()
        };
        let fn__3599 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3599())
        };
        test___28.assert(t___3607, fn__3599.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn toSqlComplexQuery__866() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let s__577: Schema = schema("users", [field("name", "String", false, false), field("age", "Int", false, false)]);
        let store__578: InMemoryStore = InMemoryStore::new();
        let q__579: Query = Query::new(s__577.clone(), store__578.clone()).select([std::sync::Arc::new("name".to_string()), std::sync::Arc::new("age".to_string())]).r#where("age", ">=", "18").order_by("age", "desc").limit(10).offset(20);
        let actual___867: std::sync::Arc<String> = q__579.to_sql().to_string();
        let mut t___3597: bool = Some(actual___867.as_str()) == Some("SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___29 {
            actual___867: std::sync::Arc<String>
        }
        impl ClosureGroup___29 {
            fn fn__3585(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20) not ({})", self.actual___867.clone()));
            }
        }
        let closure_group = ClosureGroup___29 {
            actual___867: actual___867.clone()
        };
        let fn__3585 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3585())
        };
        test___29.assert(t___3597, fn__3585.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn toSqlUnicodeEscaping__869() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let s__581: Schema = schema("users", [field("name", "String", false, false)]);
        let store__582: InMemoryStore = InMemoryStore::new();
        let q__583: Query = Query::new(s__581.clone(), store__582.clone()).r#where("name", "=", "Hello 世界");
        let actual___870: std::sync::Arc<String> = q__583.to_sql().to_string();
        let mut t___3583: bool = Some(actual___870.as_str()) == Some("SELECT * FROM users WHERE name = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___30 {
            actual___870: std::sync::Arc<String>
        }
        impl ClosureGroup___30 {
            fn fn__3575(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE name = 'Hello 世界') not ({})", self.actual___870.clone()));
            }
        }
        let closure_group = ClosureGroup___30 {
            actual___870: actual___870.clone()
        };
        let fn__3575 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3575())
        };
        test___30.assert(t___3583, fn__3575.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn toSqlEmbeddedQuotes__872() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let s__585: Schema = schema("users", [field("name", "String", false, false)]);
        let store__586: InMemoryStore = InMemoryStore::new();
        let q__587: Query = Query::new(s__585.clone(), store__586.clone()).r#where("name", "=", "O'Brien");
        let actual___873: std::sync::Arc<String> = q__587.to_sql().to_string();
        let mut t___3573: bool = Some(actual___873.as_str()) == Some("SELECT * FROM users WHERE name = 'O''Brien'");
        #[derive(Clone)]
        struct ClosureGroup___31 {
            actual___873: std::sync::Arc<String>
        }
        impl ClosureGroup___31 {
            fn fn__3565(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE name = 'O''Brien') not ({})", self.actual___873.clone()));
            }
        }
        let closure_group = ClosureGroup___31 {
            actual___873: actual___873.clone()
        };
        let fn__3565 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3565())
        };
        test___31.assert(t___3573, fn__3565.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn toSqlEmptyString__875() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let s__589: Schema = schema("users", [field("name", "String", false, false)]);
        let store__590: InMemoryStore = InMemoryStore::new();
        let q__591: Query = Query::new(s__589.clone(), store__590.clone()).r#where("name", "=", "");
        let actual___876: std::sync::Arc<String> = q__591.to_sql().to_string();
        let mut t___3563: bool = Some(actual___876.as_str()) == Some("SELECT * FROM users WHERE name = ''");
        #[derive(Clone)]
        struct ClosureGroup___32 {
            actual___876: std::sync::Arc<String>
        }
        impl ClosureGroup___32 {
            fn fn__3555(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE name = '') not ({})", self.actual___876.clone()));
            }
        }
        let closure_group = ClosureGroup___32 {
            actual___876: actual___876.clone()
        };
        let fn__3555 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3555())
        };
        test___32.assert(t___3563, fn__3555.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBasicInsert__878() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let s__593: Schema = schema("users", [field("name", "String", false, false), field("age", "Int", false, false)]);
        let vals__594: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let result__595: crate::sql::SqlFragment = to_insert_sql(s__593.clone(), vals__594.clone());
        let actual___879: std::sync::Arc<String> = result__595.to_string();
        let mut t___3553: bool = Some(actual___879.as_str()) == Some("INSERT INTO users (name, age) VALUES ('Alice', 25)");
        #[derive(Clone)]
        struct ClosureGroup___33 {
            actual___879: std::sync::Arc<String>
        }
        impl ClosureGroup___33 {
            fn fn__3545(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected result.toString() == (INSERT INTO users (name, age) VALUES ('Alice', 25)) not ({})", self.actual___879.clone()));
            }
        }
        let closure_group = ClosureGroup___33 {
            actual___879: actual___879.clone()
        };
        let fn__3545 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3545())
        };
        test___33.assert(t___3553, fn__3545.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlInjectionBlocked__881() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let s__597: Schema = schema("users", [field("name", "String", false, false)]);
        let vals__598: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string()))]);
        let result__599: crate::sql::SqlFragment = to_insert_sql(s__597.clone(), vals__598.clone());
        let actual___882: std::sync::Arc<String> = result__599.to_string();
        let mut t___3543: bool = Some(actual___882.as_str()) == Some("INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')");
        #[derive(Clone)]
        struct ClosureGroup___34 {
            actual___882: std::sync::Arc<String>
        }
        impl ClosureGroup___34 {
            fn fn__3536(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected result.toString() == (INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')) not ({})", self.actual___882.clone()));
            }
        }
        let closure_group = ClosureGroup___34 {
            actual___882: actual___882.clone()
        };
        let fn__3536 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3536())
        };
        test___34.assert(t___3543, fn__3536.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn toSqlQueryStandalone__884() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let s__601: Schema = schema("users", [field("name", "String", false, false), field("age", "Int", false, false)]);
        let result__602: crate::sql::SqlFragment = to_sql_query(s__601.clone(), [std::sync::Arc::new("name".to_string())], [WhereClause::new("age", ">", "21")], [OrderClause::new("name", "asc")], 5, 0);
        let actual___885: std::sync::Arc<String> = result__602.to_string();
        let mut t___3534: bool = Some(actual___885.as_str()) == Some("SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___35 {
            actual___885: std::sync::Arc<String>
        }
        impl ClosureGroup___35 {
            fn fn__3525(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected result.toString() == (SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5) not ({})", self.actual___885.clone()));
            }
        }
        let closure_group = ClosureGroup___35 {
            actual___885: actual___885.clone()
        };
        let fn__3525 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3525())
        };
        test___35.assert(t___3534, fn__3525.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn toSqlAdversarialFieldNameBlocked__887() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let s__604: Schema = schema("users", [field("name", "String", false, false)]);
        let store__605: InMemoryStore = InMemoryStore::new();
        let q__606: Query = Query::new(s__604.clone(), store__605.clone()).r#where("1=1; DROP TABLE users; --", "=", "Alice");
        let actual___888: std::sync::Arc<String> = q__606.to_sql().to_string();
        let mut t___3523: bool = Some(actual___888.as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___36 {
            actual___888: std::sync::Arc<String>
        }
        impl ClosureGroup___36 {
            fn fn__3515(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users) not ({})", self.actual___888.clone()));
            }
        }
        let closure_group = ClosureGroup___36 {
            actual___888: actual___888.clone()
        };
        let fn__3515 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3515())
        };
        test___36.assert(t___3523, fn__3515.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn toSqlAdversarialSelectColumnBlocked__890() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        let s__608: Schema = schema("users", [field("name", "String", false, false)]);
        let store__609: InMemoryStore = InMemoryStore::new();
        let q__610: Query = Query::new(s__608.clone(), store__609.clone()).select([std::sync::Arc::new("name".to_string()), std::sync::Arc::new("1; DROP TABLE users".to_string())]);
        let actual___891: std::sync::Arc<String> = q__610.to_sql().to_string();
        let mut t___3513: bool = Some(actual___891.as_str()) == Some("SELECT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___37 {
            actual___891: std::sync::Arc<String>
        }
        impl ClosureGroup___37 {
            fn fn__3505(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT name FROM users) not ({})", self.actual___891.clone()));
            }
        }
        let closure_group = ClosureGroup___37 {
            actual___891: actual___891.clone()
        };
        let fn__3505 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3505())
        };
        test___37.assert(t___3513, fn__3505.clone());
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn toSqlAdversarialOrderByBlocked__893() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let s__612: Schema = schema("users", [field("name", "String", false, false)]);
        let store__613: InMemoryStore = InMemoryStore::new();
        let q__614: Query = Query::new(s__612.clone(), store__613.clone()).order_by("1; DROP TABLE users", "asc");
        let actual___894: std::sync::Arc<String> = q__614.to_sql().to_string();
        let mut t___3503: bool = Some(actual___894.as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___38 {
            actual___894: std::sync::Arc<String>
        }
        impl ClosureGroup___38 {
            fn fn__3495(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users) not ({})", self.actual___894.clone()));
            }
        }
        let closure_group = ClosureGroup___38 {
            actual___894: actual___894.clone()
        };
        let fn__3495 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3495())
        };
        test___38.assert(t___3503, fn__3495.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn isValidIdentifierValidNames__896() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let mut t___3480: bool = is_valid_identifier("users");
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__3479(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected `-work/src//`.isValidIdentifier(\"users\")".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__3479 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3479())
        };
        test___39.assert(t___3480, fn__3479.clone());
        let mut t___3483: bool = is_valid_identifier("user_table");
        #[derive(Clone)]
        struct ClosureGroup___40 {}
        impl ClosureGroup___40 {
            fn fn__3478(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected `-work/src//`.isValidIdentifier(\"user_table\")".to_string());
            }
        }
        let closure_group = ClosureGroup___40 {};
        let fn__3478 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3478())
        };
        test___39.assert(t___3483, fn__3478.clone());
        let mut t___3486: bool = is_valid_identifier("Table1");
        #[derive(Clone)]
        struct ClosureGroup___41 {}
        impl ClosureGroup___41 {
            fn fn__3477(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected `-work/src//`.isValidIdentifier(\"Table1\")".to_string());
            }
        }
        let closure_group = ClosureGroup___41 {};
        let fn__3477 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3477())
        };
        test___39.assert(t___3486, fn__3477.clone());
        let mut t___3489: bool = is_valid_identifier("_private");
        #[derive(Clone)]
        struct ClosureGroup___42 {}
        impl ClosureGroup___42 {
            fn fn__3476(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected `-work/src//`.isValidIdentifier(\"_private\")".to_string());
            }
        }
        let closure_group = ClosureGroup___42 {};
        let fn__3476 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3476())
        };
        test___39.assert(t___3489, fn__3476.clone());
        let mut t___3492: bool = is_valid_identifier("a");
        #[derive(Clone)]
        struct ClosureGroup___43 {}
        impl ClosureGroup___43 {
            fn fn__3475(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected `-work/src//`.isValidIdentifier(\"a\")".to_string());
            }
        }
        let closure_group = ClosureGroup___43 {};
        let fn__3475 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3475())
        };
        test___39.assert(t___3492, fn__3475.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn isValidIdentifierInvalidNames__897() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___3453: bool = ! is_valid_identifier("");
        #[derive(Clone)]
        struct ClosureGroup___44 {}
        impl ClosureGroup___44 {
            fn fn__3450(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected !`-work/src//`.isValidIdentifier(\"\")".to_string());
            }
        }
        let closure_group = ClosureGroup___44 {};
        let fn__3450 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3450())
        };
        test___40.assert(t___3453, fn__3450.clone());
        let mut t___3457: bool = ! is_valid_identifier("users; DROP TABLE");
        #[derive(Clone)]
        struct ClosureGroup___45 {}
        impl ClosureGroup___45 {
            fn fn__3449(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected !`-work/src//`.isValidIdentifier(\"users; DROP TABLE\")".to_string());
            }
        }
        let closure_group = ClosureGroup___45 {};
        let fn__3449 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3449())
        };
        test___40.assert(t___3457, fn__3449.clone());
        let mut t___3461: bool = ! is_valid_identifier("users--");
        #[derive(Clone)]
        struct ClosureGroup___46 {}
        impl ClosureGroup___46 {
            fn fn__3448(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected !`-work/src//`.isValidIdentifier(\"users--\")".to_string());
            }
        }
        let closure_group = ClosureGroup___46 {};
        let fn__3448 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3448())
        };
        test___40.assert(t___3461, fn__3448.clone());
        let mut t___3465: bool = ! is_valid_identifier("ta ble");
        #[derive(Clone)]
        struct ClosureGroup___47 {}
        impl ClosureGroup___47 {
            fn fn__3447(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected !`-work/src//`.isValidIdentifier(\"ta ble\")".to_string());
            }
        }
        let closure_group = ClosureGroup___47 {};
        let fn__3447 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3447())
        };
        test___40.assert(t___3465, fn__3447.clone());
        let mut t___3469: bool = ! is_valid_identifier("table.name");
        #[derive(Clone)]
        struct ClosureGroup___48 {}
        impl ClosureGroup___48 {
            fn fn__3446(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected !`-work/src//`.isValidIdentifier(\"table.name\")".to_string());
            }
        }
        let closure_group = ClosureGroup___48 {};
        let fn__3446 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3446())
        };
        test___40.assert(t___3469, fn__3446.clone());
        let mut t___3473: bool = ! is_valid_identifier("Robert'); DROP TABLE users;--");
        #[derive(Clone)]
        struct ClosureGroup___49 {}
        impl ClosureGroup___49 {
            fn fn__3445(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected !`-work/src//`.isValidIdentifier(\"Robert'); DROP TABLE users;--\")".to_string());
            }
        }
        let closure_group = ClosureGroup___49 {};
        let fn__3445 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3445())
        };
        test___40.assert(t___3473, fn__3445.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toSqlNonNumericIntValueProducesAlwaysFalse__898() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let s__618: Schema = schema("users", [field("age", "Int", false, false)]);
        let store__619: InMemoryStore = InMemoryStore::new();
        let q__620: Query = Query::new(s__618.clone(), store__619.clone()).r#where("age", "=", "admin");
        let actual___899: std::sync::Arc<String> = q__620.to_sql().to_string();
        let mut t___3443: bool = Some(actual___899.as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___50 {
            actual___899: std::sync::Arc<String>
        }
        impl ClosureGroup___50 {
            fn fn__3435(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users WHERE 1 = 0) not ({})", self.actual___899.clone()));
            }
        }
        let closure_group = ClosureGroup___50 {
            actual___899: actual___899.clone()
        };
        let fn__3435 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3435())
        };
        test___41.assert(t___3443, fn__3435.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn inMemoryNonNumericIntValueMatchesNothing__901() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let s__622: Schema = schema("users", [field("name", "String", false, false), field("age", "Int", false, false)]);
        let store__623: InMemoryStore = InMemoryStore::new();
        store__623.insert("users", temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("0".to_string()))]));
        let results__624: temper_core::List<Record> = Query::new(s__622.clone(), store__623.clone()).r#where("age", "=", "admin").all();
        let actual___902: i32 = temper_core::ListedTrait::len( & results__624);
        let mut t___3433: bool = Some(actual___902) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___51 {
            actual___902: i32
        }
        impl ClosureGroup___51 {
            fn fn__3421(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected results.length == ({}) not ({})", 0, self.actual___902));
            }
        }
        let closure_group = ClosureGroup___51 {
            actual___902
        };
        let fn__3421 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3421())
        };
        test___42.assert(t___3433, fn__3421.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toSqlLimitZeroEmitsLimit0__904() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let s__626: Schema = schema("users", [field("name", "String", false, false)]);
        let store__627: InMemoryStore = InMemoryStore::new();
        let q__628: Query = Query::new(s__626.clone(), store__627.clone()).limit(0);
        let actual___905: std::sync::Arc<String> = q__628.to_sql().to_string();
        let mut t___3417: bool = Some(actual___905.as_str()) == Some("SELECT * FROM users LIMIT 0");
        #[derive(Clone)]
        struct ClosureGroup___52 {
            actual___905: std::sync::Arc<String>
        }
        impl ClosureGroup___52 {
            fn fn__3409(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected q.toSql().toString() == (SELECT * FROM users LIMIT 0) not ({})", self.actual___905.clone()));
            }
        }
        let closure_group = ClosureGroup___52 {
            actual___905: actual___905.clone()
        };
        let fn__3409 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3409())
        };
        test___43.assert(t___3417, fn__3409.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn inMemoryLimitZeroReturnsEmpty__907() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let s__630: Schema = schema("users", [field("name", "String", false, false)]);
        let store__631: InMemoryStore = InMemoryStore::new();
        store__631.insert("users", temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]));
        let results__632: temper_core::List<Record> = Query::new(s__630.clone(), store__631.clone()).limit(0).all();
        let actual___908: i32 = temper_core::ListedTrait::len( & results__632);
        let mut t___3407: bool = Some(actual___908) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___53 {
            actual___908: i32
        }
        impl ClosureGroup___53 {
            fn fn__3396(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected results.length == ({}) not ({})", 0, self.actual___908));
            }
        }
        let closure_group = ClosureGroup___53 {
            actual___908
        };
        let fn__3396 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3396())
        };
        test___44.assert(t___3407, fn__3396.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn inMemoryNegativeLimitClampedToZero__910() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let s__634: Schema = schema("users", [field("name", "String", false, false)]);
        let store__635: InMemoryStore = InMemoryStore::new();
        store__635.insert("users", temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]));
        let results__636: temper_core::List<Record> = Query::new(s__634.clone(), store__635.clone()).limit(-5).all();
        let actual___911: i32 = temper_core::ListedTrait::len( & results__636);
        let mut t___3392: bool = Some(actual___911) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___54 {
            actual___911: i32
        }
        impl ClosureGroup___54 {
            fn fn__3381(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected results.length == ({}) not ({})", 0, self.actual___911));
            }
        }
        let closure_group = ClosureGroup___54 {
            actual___911
        };
        let fn__3381 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3381())
        };
        test___45.assert(t___3392, fn__3381.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlNoMatchingFieldsReturnsEmpty__913() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let s__638: Schema = schema("users", [field("name", "String", false, false)]);
        let vals__639: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("nonexistent".to_string()), std::sync::Arc::new("value".to_string()))]);
        let result__640: crate::sql::SqlFragment = to_insert_sql(s__638.clone(), vals__639.clone());
        let actual___914: std::sync::Arc<String> = result__640.to_string();
        let mut t___3377: bool = Some(actual___914.as_str()) == Some("");
        #[derive(Clone)]
        struct ClosureGroup___55 {
            actual___914: std::sync::Arc<String>
        }
        impl ClosureGroup___55 {
            fn fn__3370(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected result.toString() == () not ({})", self.actual___914.clone()));
            }
        }
        let closure_group = ClosureGroup___55 {
            actual___914: actual___914.clone()
        };
        let fn__3370 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3370())
        };
        test___46.assert(t___3377, fn__3370.clone());
        test___46.soft_fail_to_hard()
    }
    use super::*;
}
