# ORMery - All-in-One Demo

A simplified version of Ecto in Temper.

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

      // Add a where clause
      public where(field: String, operator: String, value: String): Query {
        whereClauses.add(new WhereClause(field, operator, value));
        this
      }

      // Set select fields
      public select(fields: List<String>): Query {
        selectFields = fields;
        this
      }

      // Add order by clause
      public orderBy(field: String, direction: String): Query {
        orderByClauses.add(new OrderClause(field, direction));
        this
      }

      // Set limit
      public limit(n: Int): Query {
        limitValue = n;
        this
      }

      // Set offset
      public offset(n: Int): Query {
        offsetValue = n;
        this
      }

      // Check if a record matches all where clauses
      private matchesWhere(record: Record): Boolean {
        for (let clause of whereClauses.toList()) {
          let recordValue = record.getOr(clause.field, "");

          // Get field type from schema
          if (!schema.hasField(clause.field)) {
            return false;
          }

          let fieldInfo = schema.getField(clause.field) orelse panic();
          let fieldType = fieldInfo.fieldType;

          // Compare based on type
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

      // Project record to selected fields
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

      // Compare two records for sorting
      private compareRecords(a: Record, b: Record, orderClauses: List<OrderClause>): Int {
        for (let clause of orderClauses) {
          let aVal = a.getOr(clause.field, "");
          let bVal = b.getOr(clause.field, "");

          // Get field type from schema
          if (!schema.hasField(clause.field)) {
            continue;
          }

          let fieldInfo = schema.getField(clause.field) orelse panic();
          let fieldType = fieldInfo.fieldType;

          // Compare based on type
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
            // Apply direction
            return if (clause.direction == "desc") { -cmp } else { cmp };
          }
        }
        0
      }

      // Execute query
      public all(): List<Record> {
        let allRecords = store.all(schema.tableName);

        // 1. Filter
        let filtered = allRecords.filter { (r: Record): Boolean =>
          matchesWhere(r)
        };

        // 2. Sort
        let sorted = if (orderByClauses.length > 0) {
          let clauses = orderByClauses.toList();
          filtered.sorted { (a: Record, b: Record): Int =>
            compareRecords(a, b, clauses)
          }
        } else {
          filtered
        };

        // 3. Slice (offset and limit)
        let sliced = if (limitValue > 0) {
          let start = offsetValue;
          let end = offsetValue + limitValue;
          sorted.slice(start, end)
        } else if (offsetValue > 0) {
          sorted.slice(offsetValue, sorted.length)
        } else {
          sorted
        };

        // 4. Project
        sliced.map { (r: Record): Record => projectRecord(r) }
      }
    }

    // Comparison helpers
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

## Demo

    export let main(): Void {
      console.log("=== ORMery Demo ===\n");

      // Define schema
      let userFields = [
        field("name", "String", false, false),
        field("age", "Int", false, false),
        field("email", "String", false, true),
      ];
      let userSchema = schema("users", userFields);

      console.log(userSchema.describe());
      console.log("");

      // Create store and insert data
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

      // Basic query - all users
      console.log("=== Query 1: All users ===");
      let allUsers = new Query(userSchema, store).all();
      for (let u of allUsers) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Query with where clause - adults only
      console.log("=== Query 2: Users age >= 18 ===");
      let adults = new Query(userSchema, store)
        .where("age", ">=", "18")
        .all();
      for (let u of adults) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Query with select - project specific fields
      console.log("=== Query 3: Just names and ages ===");
      let namesAndAges = new Query(userSchema, store)
        .select(["name", "age"])
        .all();
      for (let u of namesAndAges) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Combined where + select
      console.log("=== Query 4: Adult names only ===");
      let adultNames = new Query(userSchema, store)
        .where("age", ">=", "18")
        .select(["name"])
        .all();
      for (let u of adultNames) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Multiple where clauses (AND logic)
      console.log("=== Query 5: Age >= 18 AND age < 30 ===");
      let youngAdults = new Query(userSchema, store)
        .where("age", ">=", "18")
        .where("age", "<", "30")
        .all();
      for (let u of youngAdults) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Order by ascending
      console.log("=== Query 6: All users ordered by age (asc) ===");
      let byAgeAsc = new Query(userSchema, store)
        .orderBy("age", "asc")
        .all();
      for (let u of byAgeAsc) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Order by descending
      console.log("=== Query 7: All users ordered by age (desc) ===");
      let byAgeDesc = new Query(userSchema, store)
        .orderBy("age", "desc")
        .all();
      for (let u of byAgeDesc) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Limit
      console.log("=== Query 8: First 2 users (limit) ===");
      let first2 = new Query(userSchema, store)
        .orderBy("id", "asc")
        .limit(2)
        .all();
      for (let u of first2) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Offset
      console.log("=== Query 9: Skip first user (offset) ===");
      let skip1 = new Query(userSchema, store)
        .orderBy("id", "asc")
        .offset(1)
        .all();
      for (let u of skip1) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Limit + Offset (pagination)
      console.log("=== Query 10: Page 2, size 1 (offset + limit) ===");
      let page2 = new Query(userSchema, store)
        .orderBy("id", "asc")
        .offset(1)
        .limit(1)
        .all();
      for (let u of page2) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      // Complex: where + orderBy + limit
      console.log("=== Query 11: Oldest adult (where + orderBy + limit) ===");
      let oldestAdult = new Query(userSchema, store)
        .where("age", ">=", "18")
        .orderBy("age", "desc")
        .limit(1)
        .all();
      for (let u of oldestAdult) {
        console.log("  ${u.describe()}");
      }
      console.log("");

      console.log("=== Demo Complete ===");
    }
