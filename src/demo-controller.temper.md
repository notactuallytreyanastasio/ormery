# Demo Controller

Business logic for the ORMery tutorial demo, written in Temper.

## DemoController Class

    export class DemoController(
      public schema: Schema,
      public store: InMemoryStore,
    ) {
      private var queryCount: Int;

      public constructor(schema: Schema, store: InMemoryStore) {
        this.schema = schema;
        this.store = store;
        queryCount = 0;
      }

      // Get total record count
      public getRecordCount(): Int {
        store.count(schema.tableName)
      }

      // Get count of adults (age >= 18)
      public getAdultCount(): Int {
        new Query(schema, store)
          .where("age", ">=", "18")
          .all()
          .length
      }

      // Get number of queries run
      public getQueryCount(): Int {
        queryCount
      }

      // Increment query counter
      private incrementQueryCount(): Void {
        queryCount = queryCount + 1;
      }

      // Format a list of records as text
      private formatRecords(records: List<Record>): String {
        let lines = records.map { (record: Record): String =>
          "  ${record.describe()}"
        };
        lines.join("\n") { (s: String): String => s }
      }

      // Demo 1: Get all users
      public runDemo1(): String {
        incrementQueryCount();
        let results = new Query(schema, store).all();
        let formatted = formatRecords(results);
        "=== Demo 1: All Users ===\n\nQuery: new Query(userSchema, store).all()\n\nResults:\n${formatted}\n\nTotal: ${results.length} records"
      }

      // Demo 2: Filter adults (age >= 18)
      public runDemo2(): String {
        incrementQueryCount();
        let results = new Query(schema, store)
          .where("age", ">=", "18")
          .all();
        let formatted = formatRecords(results);
        "=== Demo 2: Filter Adults ===\n\nQuery: new Query(userSchema, store)\n  .where(\"age\", \">=\", \"18\")\n  .all()\n\nResults:\n${formatted}\n\nTotal: ${results.length} adults found"
      }

      // Demo 3: Sort by age descending
      public runDemo3(): String {
        incrementQueryCount();
        let results = new Query(schema, store)
          .orderBy("age", "desc")
          .all();
        let formatted = formatRecords(results);
        "=== Demo 3: Sort by Age (Descending) ===\n\nQuery: new Query(userSchema, store)\n  .orderBy(\"age\", \"desc\")\n  .all()\n\nResults (ordered by age, oldest first):\n${formatted}"
      }

      // Demo 4: Pagination
      public runDemo4(): String {
        incrementQueryCount();
        let page1 = new Query(schema, store)
          .orderBy("id", "asc")
          .limit(2)
          .all();
        let page2 = new Query(schema, store)
          .orderBy("id", "asc")
          .offset(2)
          .limit(2)
          .all();
        let formatted1 = formatRecords(page1);
        let formatted2 = formatRecords(page2);
        "=== Demo 4: Pagination ===\n\nPage 1: .orderBy(\"id\", \"asc\").limit(2)\n\n${formatted1}\n\nPage 2: .orderBy(\"id\", \"asc\").offset(2).limit(2)\n\n${formatted2}"
      }

      // Demo 5: Complex query with all features
      public runDemo5(): String {
        incrementQueryCount();
        let results = new Query(schema, store)
          .where("age", ">=", "18")
          .where("email", "!=", "")
          .orderBy("age", "desc")
          .select(["name", "age"])
          .limit(2)
          .all();
        let formatted = formatRecords(results);
        "=== Demo 5: Complex Query ===\n\nQuery: new Query(userSchema, store)\n  .where(\"age\", \">=\", \"18\")\n  .where(\"email\", \"!=\", \"\")\n  .orderBy(\"age\", \"desc\")\n  .select([\"name\", \"age\"])\n  .limit(2)\n  .all()\n\nResults (adults with email, showing name/age only, oldest first, max 2):\n${formatted}"
      }
    }
