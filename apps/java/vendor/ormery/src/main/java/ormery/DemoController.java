package ormery;
import java.util.List;
import java.util.function.Function;
import temper.core.Core;
public final class DemoController {
    public final Schema schema;
    public final InMemoryStore store;
    int queryCount;
    public static final class Builder {
        Schema schema;
        public Builder schema(Schema schema) {
            this.schema = schema;
            return this;
        }
        InMemoryStore store;
        public Builder store(InMemoryStore store) {
            this.store = store;
            return this;
        }
        public DemoController build() {
            return new DemoController(schema, store);
        }
    }
    public DemoController(Schema schema__211, InMemoryStore store__212) {
        this.schema = schema__211;
        this.store = store__212;
        this.queryCount = 0;
    }
    public int getRecordCount() {
        String t_3833 = this.schema.getTableName();
        return this.store.count(t_3833);
    }
    public int getAdultCount() {
        return new Query(this.schema, this.store).where("age", ">=", "18").all().size();
    }
    public int getQueryCount() {
        return this.queryCount;
    }
    void incrementQueryCount() {
        int t_734 = this.queryCount + 1;
        this.queryCount = t_734;
    }
    String formatRecords(List<Record> records__223) {
        Function<Record, String> fn__3824 = record__226 -> "  " + record__226.describe();
        List<String> lines__225 = Core.listMapObjToObj(records__223, fn__3824);
        Function<String, String> fn__3823 = s__228 -> s__228;
        return Core.listJoinObj(lines__225, "\n", fn__3823);
    }
    public String runDemo1() {
        this.incrementQueryCount();
        List<Record> results__232 = new Query(this.schema, this.store).all();
        String formatted__233 = this.formatRecords(results__232);
        return "=== Demo 1: All Users ===" + "\n" + "\n" + "Query: new Query(userSchema, store).all()" + "\n" + "\n" + "Results:" + "\n" + formatted__233 + "\n" + "\n" + "Total: " + Integer.toString(results__232.size()) + " records";
    }
    public String runDemo2() {
        this.incrementQueryCount();
        List<Record> results__236 = new Query(this.schema, this.store).where("age", ">=", "18").all();
        String formatted__237 = this.formatRecords(results__236);
        return "=== Demo 2: Filter Adults ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .where(" + "\"" + "age" + "\"" + ", " + "\"" + ">=" + "\"" + ", " + "\"" + "18" + "\"" + ")" + "\n" + "  .all()" + "\n" + "\n" + "Results:" + "\n" + formatted__237 + "\n" + "\n" + "Total: " + Integer.toString(results__236.size()) + " adults found";
    }
    public String runDemo3() {
        this.incrementQueryCount();
        List<Record> results__240 = new Query(this.schema, this.store).orderBy("age", "desc").all();
        String formatted__241 = this.formatRecords(results__240);
        return "=== Demo 3: Sort by Age (Descending) ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .orderBy(" + "\"" + "age" + "\"" + ", " + "\"" + "desc" + "\"" + ")" + "\n" + "  .all()" + "\n" + "\n" + "Results (ordered by age, oldest first):" + "\n" + formatted__241;
    }
    public String runDemo4() {
        this.incrementQueryCount();
        List<Record> page1__244 = new Query(this.schema, this.store).orderBy("id", "asc").limit(2).all();
        List<Record> page2__245 = new Query(this.schema, this.store).orderBy("id", "asc").offset(2).limit(2).all();
        String formatted1__246 = this.formatRecords(page1__244);
        String formatted2__247 = this.formatRecords(page2__245);
        return "=== Demo 4: Pagination ===" + "\n" + "\n" + "Page 1: .orderBy(" + "\"" + "id" + "\"" + ", " + "\"" + "asc" + "\"" + ").limit(2)" + "\n" + "\n" + formatted1__246 + "\n" + "\n" + "Page 2: .orderBy(" + "\"" + "id" + "\"" + ", " + "\"" + "asc" + "\"" + ").offset(2).limit(2)" + "\n" + "\n" + formatted2__247;
    }
    public String runDemo5() {
        this.incrementQueryCount();
        List<Record> results__250 = new Query(this.schema, this.store).where("age", ">=", "18").where("email", "!=", "").orderBy("age", "desc").select(List.of("name", "age")).limit(2).all();
        String formatted__251 = this.formatRecords(results__250);
        return "=== Demo 5: Complex Query ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .where(" + "\"" + "age" + "\"" + ", " + "\"" + ">=" + "\"" + ", " + "\"" + "18" + "\"" + ")" + "\n" + "  .where(" + "\"" + "email" + "\"" + ", " + "\"" + "!=" + "\"" + ", " + "\"" + "\"" + ")" + "\n" + "  .orderBy(" + "\"" + "age" + "\"" + ", " + "\"" + "desc" + "\"" + ")" + "\n" + "  .select([" + "\"" + "name" + "\"" + ", " + "\"" + "age" + "\"" + "])" + "\n" + "  .limit(2)" + "\n" + "  .all()" + "\n" + "\n" + "Results (adults with email, showing name/age only, oldest first, max 2):" + "\n" + formatted__251;
    }
    public Schema getSchema() {
        return this.schema;
    }
    public InMemoryStore getStore() {
        return this.store;
    }
}
