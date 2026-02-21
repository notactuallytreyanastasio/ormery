# ORMery - All-in-One Demo

A simplified version of Ecto in Temper, using secure-composition for
injection-proof SQL generation.

    let { sql, SqlFragment, SqlBuilder } = import("./sql");

## Field and Schema

    export class Field(
      public name: String,
      public fieldType: String,
      public primaryKey: Boolean,
      public nullable: Boolean,
    ) {
      public get description(): String {
        let pk = if (primaryKey) { " (PK)" } else { "" };
        let null = if (nullable) { " (nullable)" } else { "" };
        "${name}: ${fieldType}${pk}${null}"
      }
    }

    export class Schema(
      public tableName: String,
      public fields: List<Field>,
    ) {
      public getField(name: String): Field throws Bubble {
        for (let field of fields) {
          if (field.name == name) {
            return field;
          }
        }
        bubble()
      }

      public hasField(name: String): Boolean {
        for (let field of fields) {
          if (field.name == name) {
            return true;
          }
        }
        false
      }

      public get primaryKeyField(): Field throws Bubble {
        for (let field of fields) {
          if (field.primaryKey) {
            return field;
          }
        }
        bubble()
      }

      public get fieldNames(): List<String> {
        fields.map { (f: Field): String => f.name }
      }

      public describe(): String {
        let header = "Schema: ${tableName}\n";
        let fieldList = fields
          .map { (f: Field): String => "  - ${f.description}" }
          .join("\n") { (s: String): String => s };
        "${header}${fieldList}"
      }
    }

    export let field(
      name: String,
      fieldType: String,
      primaryKey: Boolean,
      nullable: Boolean,
    ): Field {
      new Field(name, fieldType, primaryKey, nullable)
    }

    export let schema(tableName: String, fields: List<Field>): Schema {
      let idField = new Field("id", "Int", true, false);
      let allFields = new ListBuilder<Field>();
      allFields.add(idField);
      allFields.addAll(fields);
      new Schema(tableName, allFields.toList())
    }

## Record and Store

    export class Record(
      public data: Map<String, String>,
    ) {
      public get(field: String): String throws Bubble {
        data.get(field)
      }

      public getOr(field: String, fallback: String): String {
        data.getOr(field, fallback)
      }

      public has(field: String): Boolean {
        data.has(field)
      }

      public get id(): Int throws Bubble {
        let idStr = data.get("id");
        idStr.toInt32() orelse bubble()
      }

      public describe(): String {
        let pairs = data.toListWith { (k: String, v: String): String =>
          "${k}: ${v}"
        };
        pairs.join(", ") { (s: String): String => s }
      }
    }

    export class InMemoryStore() {
      private var tables: MapBuilder<String, ListBuilder<Record>>;
      private var nextIds: MapBuilder<String, Int>;

      public constructor() {
        tables = new MapBuilder<String, ListBuilder<Record>>();
        nextIds = new MapBuilder<String, Int>();
      }

      private ensureTable(tableName: String): Void {
        if (!tables.has(tableName)) {
          tables.set(tableName, new ListBuilder<Record>());
          nextIds.set(tableName, 1);
        }
      }

      public insert(tableName: String, data: Map<String, String>): Record {
        ensureTable(tableName);
        let id = nextIds.getOr(tableName, 1);
        nextIds.set(tableName, id + 1);

        let dataBuilder = data.toMapBuilder();
        dataBuilder.set("id", id.toString());
        let record = new Record(dataBuilder.toMap());

        let table = tables.getOr(tableName, new ListBuilder<Record>());
        table.add(record);
        record
      }

      public all(tableName: String): List<Record> {
        ensureTable(tableName);
        let table = tables.getOr(tableName, new ListBuilder<Record>());
        table.toList()
      }

      public get(tableName: String, id: Int): Record throws Bubble {
        ensureTable(tableName);
        let table = tables.getOr(tableName, new ListBuilder<Record>());

        for (let record of table.toList()) {
          let recordId = record.id orelse bubble();
          if (recordId == id) {
            return record;
          }
        }
        bubble()
      }

      public count(tableName: String): Int {
        ensureTable(tableName);
        let table = tables.getOr(tableName, new ListBuilder<Record>());
        table.length
      }
    }

## Query Builder

    export class WhereClause(
      public field: String,
      public operator: String,
      public value: String,
    ) {
      public describe(): String {
        "${field} ${operator} ${value}"
      }
    }

    export class OrderClause(
      public field: String,
      public direction: String,
    ) {
      public describe(): String {
        "${field} ${direction}"
      }
    }

    export class Query(
      public schema: Schema,
      public store: InMemoryStore,
    ) {
      private var whereClauses: ListBuilder<WhereClause>;
      private var selectFields: List<String>;
      private var orderByClauses: ListBuilder<OrderClause>;
      private var limitValue: Int;
      private var offsetValue: Int;

      public constructor(schema: Schema, store: InMemoryStore) {
        this.schema = schema;
        this.store = store;
        whereClauses = new ListBuilder<WhereClause>();
        selectFields = [];
        orderByClauses = new ListBuilder<OrderClause>();
        limitValue = -1;
        offsetValue = 0;
      }

      public where(field: String, operator: String, value: String): Query {
        whereClauses.add(new WhereClause(field, operator, value));
        this
      }

      public select(fields: List<String>): Query {
        selectFields = fields;
        this
      }

      public orderBy(field: String, direction: String): Query {
        orderByClauses.add(new OrderClause(field, direction));
        this
      }

      public limit(n: Int): Query {
        limitValue = n;
        this
      }

      public offset(n: Int): Query {
        offsetValue = n;
        this
      }

      private matchesWhere(record: Record): Boolean {
        for (let clause of whereClauses.toList()) {
          let recordValue = record.getOr(clause.field, "");
          if (!schema.hasField(clause.field)) {
            return false;
          }
          let fieldInfo = schema.getField(clause.field) orelse panic();
          let fieldType = fieldInfo.fieldType;
          let matches = when (fieldType) {
            "Int" -> compareInt(recordValue, clause.operator, clause.value);
            "String" -> compareString(recordValue, clause.operator, clause.value);
            else -> false;
          };
          if (!matches) {
            return false;
          }
        }
        true
      }

      private projectRecord(record: Record): Record {
        if (selectFields.length == 0) {
          return record;
        }
        let builder = new MapBuilder<String, String>();
        for (let fieldName of selectFields) {
          let value = record.getOr(fieldName, "");
          builder.set(fieldName, value);
        }
        new Record(builder.toMap())
      }

      private compareRecords(a: Record, b: Record, orderClauses: List<OrderClause>): Int {
        for (let clause of orderClauses) {
          let aVal = a.getOr(clause.field, "");
          let bVal = b.getOr(clause.field, "");
          if (!schema.hasField(clause.field)) {
            continue;
          }
          let fieldInfo = schema.getField(clause.field) orelse panic();
          let fieldType = fieldInfo.fieldType;
          let cmp = when (fieldType) {
            "Int" -> do {
              let aInt = aVal.toInt32() orelse 0;
              let bInt = bVal.toInt32() orelse 0;
              aInt <=> bInt
            };
            "String" -> aVal <=> bVal;
            else -> 0;
          };
          if (cmp != 0) {
            return if (clause.direction == "desc") { -cmp } else { cmp };
          }
        }
        0
      }

      public all(): List<Record> {
        let allRecords = store.all(schema.tableName);
        let filtered = allRecords.filter { (r: Record): Boolean =>
          matchesWhere(r)
        };
        let sorted = if (orderByClauses.length > 0) {
          let clauses = orderByClauses.toList();
          filtered.sorted { (a: Record, b: Record): Int =>
            compareRecords(a, b, clauses)
          }
        } else {
          filtered
        };
        let sliced = if (limitValue > 0) {
          let start = offsetValue;
          let end = offsetValue + limitValue;
          sorted.slice(start, end)
        } else if (offsetValue > 0) {
          sorted.slice(offsetValue, sorted.length)
        } else {
          sorted
        };
        sliced.map { (r: Record): Record => projectRecord(r) }
      }

      public toSql(): SqlFragment {
        toSqlQuery(schema, selectFields, whereClauses.toList(),
                   orderByClauses.toList(), limitValue, offsetValue)
      }
    }

    let compareInt(recordValue: String, operator: String, clauseValue: String): Boolean {
      let rv = recordValue.toInt32() orelse 0;
      let cv = clauseValue.toInt32() orelse 0;
      when (operator) {
        "==" -> rv == cv;
        "!=" -> rv != cv;
        ">" -> rv > cv;
        "<" -> rv < cv;
        ">=" -> rv >= cv;
        "<=" -> rv <= cv;
        else -> false;
      }
    }

    let compareString(recordValue: String, operator: String, clauseValue: String): Boolean {
      when (operator) {
        "==" -> recordValue == clauseValue;
        "!=" -> recordValue != clauseValue;
        ">" -> recordValue > clauseValue;
        "<" -> recordValue < clauseValue;
        ">=" -> recordValue >= clauseValue;
        "<=" -> recordValue <= clauseValue;
        else -> false;
      }
    }

## SQL Generation

Pure functions that produce `SqlFragment` from query state using
secure-composition's `sql"..."` tagged strings. Interpolated values are
automatically escaped by type. Trusted SQL identifiers (table names, column
names, operators) are composed via fragment nesting.

### Operator validation

Only allow known SQL comparison operators. Returns the operator if valid,
or `=` as a safe fallback.

    let validOperator(op: String): String {
      when (op) {
        "=" -> "=";
        "==" -> "=";
        "!=" -> "!=";
        "<>" -> "<>";
        ">" -> ">";
        "<" -> "<";
        ">=" -> ">=";
        "<=" -> "<=";
        else -> "=";
      }
    }

### Trusted identifier fragment

Wraps a trusted identifier (table name, column name, operator) as a
`SqlFragment`. These come from schema definitions, not user input.

    let safeSql(trusted: String): SqlFragment {
      let b = new SqlBuilder();
      b.appendSafe(trusted);
      b.accumulated
    }

### Column list helper

Builds the SELECT column list. If no fields specified, returns `*`.

    let columnListSql(selectFields: List<String>): SqlFragment {
      if (selectFields.length == 0) {
        sql"*"
      } else {
        let first = safeSql(selectFields[0]);
        var result = sql"${first}";
        for (var i = 1; i < selectFields.length; i = i + 1) {
          let col = safeSql(selectFields[i]);
          result = sql"${result}, ${col}";
        }
        result
      }
    }

### WHERE clause helper

Builds a single WHERE condition. The value is untrusted user input —
`sql"..."` escapes it by type automatically.

    let whereConditionSql(clause: WhereClause, schema: Schema): SqlFragment {
      let col = safeSql(clause.field);
      let op = safeSql(validOperator(clause.operator));
      let fieldInfo = schema.getField(clause.field) orelse panic();
      if (fieldInfo.fieldType == "Int") {
        let intVal = clause.value.toInt32() orelse 0;
        sql"${col} ${op} ${intVal}"
      } else {
        let strVal = clause.value;
        sql"${col} ${op} ${strVal}"
      }
    }

### ORDER BY clause helper

    let orderBySql(clauses: List<OrderClause>): SqlFragment {
      let first = safeSql(clauses[0].field);
      let firstDir = if (clauses[0].direction == "desc") { safeSql(" DESC") } else { safeSql(" ASC") };
      var result = sql"${first}${firstDir}";
      for (var i = 1; i < clauses.length; i = i + 1) {
        let col = safeSql(clauses[i].field);
        let dir = if (clauses[i].direction == "desc") { safeSql(" DESC") } else { safeSql(" ASC") };
        result = sql"${result}, ${col}${dir}";
      }
      result
    }

### Full SELECT query builder

Assembles a complete SELECT statement from parts. This is the main pure
function: it takes query state in, returns `SqlFragment` out. Field names
are validated against the schema — only declared fields pass through
`safeSql`. Unknown fields are silently dropped, closing the confused deputy
vector where user-controlled strings could reach `appendSafe`.

    export let toSqlQuery(
      schema: Schema,
      selectFields: List<String>,
      whereClauses: List<WhereClause>,
      orderClauses: List<OrderClause>,
      limitValue: Int,
      offsetValue: Int,
    ): SqlFragment {
      let validSelect = selectFields.filter { (f: String): Boolean =>
        schema.hasField(f)
      };
      let validWhere = whereClauses.filter { (c: WhereClause): Boolean =>
        schema.hasField(c.field)
      };
      let validOrder = orderClauses.filter { (c: OrderClause): Boolean =>
        schema.hasField(c.field)
      };
      let table = safeSql(schema.tableName);
      let cols = columnListSql(validSelect);
      var result = sql"SELECT ${cols} FROM ${table}";
      if (validWhere.length > 0) {
        var conditions = whereConditionSql(validWhere[0], schema);
        for (var i = 1; i < validWhere.length; i = i + 1) {
          let next = whereConditionSql(validWhere[i], schema);
          conditions = sql"${conditions} AND ${next}";
        }
        result = sql"${result} WHERE ${conditions}";
      }
      if (validOrder.length > 0) {
        let ordering = orderBySql(validOrder);
        result = sql"${result} ORDER BY ${ordering}";
      }
      if (limitValue > 0) {
        result = sql"${result} LIMIT ${limitValue}";
      }
      if (offsetValue > 0) {
        result = sql"${result} OFFSET ${offsetValue}";
      }
      result
    }

### INSERT statement builder

Generates an INSERT statement from a schema and a map of field values.
Field names come from the schema (trusted). Values are escaped via
`sql"..."` by type.

    export let toInsertSql(
      schema: Schema,
      values: Map<String, String>,
    ): SqlFragment {
      let table = safeSql(schema.tableName);
      let fieldList = schema.fields.filter { (f: Field): Boolean =>
        values.has(f.name)
      };
      let colNames = columnListSql(
        fieldList.map { (f: Field): String => f.name }
      );
      let firstVal = values.getOr(fieldList[0].name, "");
      var vals = if (fieldList[0].fieldType == "Int") {
        let iv = firstVal.toInt32() orelse 0;
        sql"${iv}"
      } else {
        sql"${firstVal}"
      };
      for (var i = 1; i < fieldList.length; i = i + 1) {
        let val = values.getOr(fieldList[i].name, "");
        if (fieldList[i].fieldType == "Int") {
          let iv = val.toInt32() orelse 0;
          vals = sql"${vals}, ${iv}";
        } else {
          vals = sql"${vals}, ${val}";
        }
      }
      sql"INSERT INTO ${table} (${colNames}) VALUES (${vals})"
    }

## SQL Generation Tests

### Basic SELECT

    test("toSql: select all") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store);
      assert(q.toSql().toString() == "SELECT * FROM users");
    }

### SELECT with specific columns

    test("toSql: select columns") {
      let s = schema("users", [
        field("name", "String", false, false),
        field("age", "Int", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).select(["name", "age"]);
      assert(q.toSql().toString() == "SELECT name, age FROM users");
    }

### WHERE with string value

    test("toSql: where string") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("name", "=", "Alice");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name = 'Alice'");
    }

### WHERE with integer value

    test("toSql: where int") {
      let s = schema("users", [
        field("age", "Int", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("age", ">=", "18");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE age >= 18");
    }

### SQL injection protection

The Bobby Tables attack string is safely escaped — single quotes are doubled.

    test("toSql: SQL injection blocked") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let bobby = "Robert'); DROP TABLE users;--";
      let q = new Query(s, store).where("name", "=", bobby);
      let result = q.toSql().toString();
      assert(result == "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'");
    }

### Operator normalization

The `==` operator from the in-memory query API is normalized to SQL `=`.

    test("toSql: operator normalization") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("name", "==", "Alice");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name = 'Alice'");
    }

### Invalid operator fallback

Unknown operators fall back to `=` for safety.

    test("toSql: invalid operator fallback") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("name", "LIKE", "Alice");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name = 'Alice'");
    }

### Multiple WHERE clauses

    test("toSql: multiple where") {
      let s = schema("users", [
        field("age", "Int", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store)
        .where("age", ">=", "18")
        .where("age", "<", "30");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE age >= 18 AND age < 30");
    }

### ORDER BY

    test("toSql: order by") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).orderBy("name", "asc");
      assert(q.toSql().toString() == "SELECT * FROM users ORDER BY name ASC");
    }

### ORDER BY descending

    test("toSql: order by desc") {
      let s = schema("users", [
        field("age", "Int", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).orderBy("age", "desc");
      assert(q.toSql().toString() == "SELECT * FROM users ORDER BY age DESC");
    }

### LIMIT and OFFSET

    test("toSql: limit") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).limit(10);
      assert(q.toSql().toString() == "SELECT * FROM users LIMIT 10");
    }

    test("toSql: offset") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).offset(5);
      assert(q.toSql().toString() == "SELECT * FROM users OFFSET 5");
    }

### Complex query

    test("toSql: complex query") {
      let s = schema("users", [
        field("name", "String", false, false),
        field("age", "Int", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store)
        .select(["name", "age"])
        .where("age", ">=", "18")
        .orderBy("age", "desc")
        .limit(10)
        .offset(20);
      assert(q.toSql().toString() ==
        "SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20");
    }

### Unicode in values

    test("toSql: unicode escaping") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("name", "=", "Hello 世界");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name = 'Hello 世界'");
    }

### Embedded quotes in values

    test("toSql: embedded quotes") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("name", "=", "O'Brien");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name = 'O''Brien'");
    }

### Empty string value

    test("toSql: empty string") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store).where("name", "=", "");
      assert(q.toSql().toString() == "SELECT * FROM users WHERE name = ''");
    }

### INSERT statement

    test("toInsertSql: basic insert") {
      let s = schema("users", [
        field("name", "String", false, false),
        field("age", "Int", false, false),
      ]);
      let vals = new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("age", "25"),
      ]);
      let result = toInsertSql(s, vals);
      assert(result.toString() == "INSERT INTO users (name, age) VALUES ('Alice', 25)");
    }

### INSERT with injection protection

    test("toInsertSql: injection blocked") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let vals = new Map<String, String>([
        new Pair("name", "Robert'); DROP TABLE users;--"),
      ]);
      let result = toInsertSql(s, vals);
      assert(result.toString() ==
        "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')");
    }

### toSqlQuery as standalone pure function

    test("toSqlQuery: standalone") {
      let s = schema("users", [
        field("name", "String", false, false),
        field("age", "Int", false, false),
      ]);
      let result = toSqlQuery(
        s, ["name"], [new WhereClause("age", ">", "21")],
        [new OrderClause("name", "asc")], 5, 0,
      );
      assert(result.toString() ==
        "SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5");
    }

### Adversarial field name protection

Field names not in the schema are silently dropped from SQL generation.
This prevents confused deputy attacks where user-controlled strings
could reach `appendSafe` through field name positions.

    test("toSql: adversarial field name blocked") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store)
        .where("1=1; DROP TABLE users; --", "=", "Alice");
      assert(q.toSql().toString() == "SELECT * FROM users");
    }

    test("toSql: adversarial select column blocked") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store)
        .select(["name", "1; DROP TABLE users"]);
      assert(q.toSql().toString() == "SELECT name FROM users");
    }

    test("toSql: adversarial order by blocked") {
      let s = schema("users", [
        field("name", "String", false, false),
      ]);
      let store = new InMemoryStore();
      let q = new Query(s, store)
        .orderBy("1; DROP TABLE users", "asc");
      assert(q.toSql().toString() == "SELECT * FROM users");
    }

## Demo

    export let main(): Void {
      console.log("=== ORMery Demo ===\n");

      let userFields = [
        field("name", "String", false, false),
        field("age", "Int", false, false),
        field("email", "String", false, true),
      ];
      let userSchema = schema("users", userFields);

      console.log(userSchema.describe());
      console.log("");

      let store = new InMemoryStore();

      let rec1 = store.insert("users", new Map<String, String>([
        new Pair("name", "Alice"),
        new Pair("age", "25"),
        new Pair("email", "alice@example.com"),
      ]));

      let rec2 = store.insert("users", new Map<String, String>([
        new Pair("name", "Bob"),
        new Pair("age", "30"),
        new Pair("email", "bob@example.com"),
      ]));

      let rec3 = store.insert("users", new Map<String, String>([
        new Pair("name", "Charlie"),
        new Pair("age", "17"),
        new Pair("email", "charlie@example.com"),
      ]));

      console.log("Inserted 3 users:");
      console.log("  ${rec1.describe()}");
      console.log("  ${rec2.describe()}");
      console.log("  ${rec3.describe()}");
      console.log("");

      console.log("=== In-Memory Queries ===\n");

      console.log("All users:");
      let allUsers = new Query(userSchema, store).all();
      for (let u of allUsers) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      console.log("Adults (age >= 18):");
      let adults = new Query(userSchema, store)
        .where("age", ">=", "18")
        .all();
      for (let u of adults) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      console.log("=== SQL Generation (secure-composition) ===\n");

      let q1 = new Query(userSchema, store);
      console.log("SELECT all: ${q1.toSql().toString()}");

      let q2 = new Query(userSchema, store)
        .select(["name", "age"])
        .where("age", ">=", "18")
        .orderBy("age", "desc")
        .limit(10);
      console.log("Complex:    ${q2.toSql().toString()}");

      let bobby = "Robert'); DROP TABLE users;--";
      let q3 = new Query(userSchema, store)
        .where("name", "=", bobby);
      console.log("Injection:  ${q3.toSql().toString()}");

      let insertVals = new Map<String, String>([
        new Pair("name", "O'Malley"),
        new Pair("age", "42"),
      ]);
      console.log("INSERT:     ${toInsertSql(userSchema, insertVals).toString()}");

      console.log("\n=== Demo Complete ===");
    }
