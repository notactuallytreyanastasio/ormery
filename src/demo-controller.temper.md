# Demo Controller

Business logic for the Skinny Ecto tutorial demo, written in Temper.

```temper
import { Schema, InMemoryStore, Query, Record } from "skinny-ecto"

// Demo controller handles all business logic for the tutorial
export class DemoController(
  public schema: Schema,
  public store: InMemoryStore,
) {
  private var queryCount: Int = 0;

  // Get total record count
  public getRecordCount(): Int {
    store.count("users")
  }

  // Get count of adults (age >= 18)
  public getAdultCount(): Int {
    new Query(schema, store)
      .where("age", ">=", "18")
      .all()
      .length()
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
    var result = "";
    for (record in records) {
      result = result + "  " + record.describe() + "\n";
    }
    result
  }

  // Demo 1: Get all users
  public runDemo1(): String {
    incrementQueryCount();
    let results = new Query(schema, store).all();

    var output = "=== Demo 1: All Users ===\n\n";
    output = output + "Query: new Query(userSchema, store).all()\n\n";
    output = output + "Results:\n";
    output = output + formatRecords(results);
    output = output + "\nTotal: ${results.length()} records";
    output
  }

  // Demo 2: Filter adults (age >= 18)
  public runDemo2(): String {
    incrementQueryCount();
    let results = new Query(schema, store)
      .where("age", ">=", "18")
      .all();

    var output = "=== Demo 2: Filter Adults ===\n\n";
    output = output + "Query: new Query(userSchema, store)\n";
    output = output + "  .where(\"age\", \">=\", \"18\")\n";
    output = output + "  .all()\n\n";
    output = output + "Results:\n";
    output = output + formatRecords(results);
    output = output + "\nTotal: ${results.length()} adults found";
    output
  }

  // Demo 3: Sort by age descending
  public runDemo3(): String {
    incrementQueryCount();
    let results = new Query(schema, store)
      .orderBy("age", "desc")
      .all();

    var output = "=== Demo 3: Sort by Age (Descending) ===\n\n";
    output = output + "Query: new Query(userSchema, store)\n";
    output = output + "  .orderBy(\"age\", \"desc\")\n";
    output = output + "  .all()\n\n";
    output = output + "Results (ordered by age, oldest first):\n";
    output = output + formatRecords(results);
    output
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

    var output = "=== Demo 4: Pagination ===\n\n";
    output = output + "Page 1: .orderBy(\"id\", \"asc\").limit(2)\n\n";
    output = output + formatRecords(page1);
    output = output + "\nPage 2: .orderBy(\"id\", \"asc\").offset(2).limit(2)\n\n";
    output = output + formatRecords(page2);
    output
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

    var output = "=== Demo 5: Complex Query ===\n\n";
    output = output + "Query: new Query(userSchema, store)\n";
    output = output + "  .where(\"age\", \">=\", \"18\")\n";
    output = output + "  .where(\"email\", \"!=\", \"\")\n";
    output = output + "  .orderBy(\"age\", \"desc\")\n";
    output = output + "  .select([\"name\", \"age\"])\n";
    output = output + "  .limit(2)\n";
    output = output + "  .all()\n\n";
    output = output + "Results (adults with email, showing name/age only, oldest first, max 2):\n";
    output = output + formatRecords(results);
    output
  }
}
```
