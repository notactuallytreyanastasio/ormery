using S = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
namespace Ormery
{
    public class DemoController
    {
        readonly Schema schema__207;
        readonly InMemoryStore store__208;
        int queryCount__209;
        public DemoController(Schema schema__211, InMemoryStore store__212)
        {
            this.schema__207 = schema__211;
            this.store__208 = store__212;
            this.queryCount__209 = 0;
        }
        public int GetRecordCount()
        {
            string t___3833 = this.schema__207.TableName;
            return this.store__208.Count(t___3833);
        }
        public int GetAdultCount()
        {
            return new Query(this.schema__207, this.store__208).Where("age", ">=", "18").All().Count;
        }
        public int GetQueryCount()
        {
            return this.queryCount__209;
        }
        void IncrementQueryCount()
        {
            int t___734 = this.queryCount__209 + 1;
            this.queryCount__209 = t___734;
        }
        string FormatRecords(G::IReadOnlyList<Record> records__223)
        {
            string fn__3824(Record record__226)
            {
                return "  " + record__226.Describe();
            }
            G::IReadOnlyList<string> lines__225 = C::Listed.ToReadOnlyList(L::Enumerable.Select(records__223, (S::Func<Record, string>) fn__3824));
            string fn__3823(string s__228)
            {
                return s__228;
            }
            return C::Listed.Join(lines__225, "\n", (S::Func<string, string>) fn__3823);
        }
        public string RunDemo1()
        {
            this.IncrementQueryCount();
            G::IReadOnlyList<Record> results__232 = new Query(this.schema__207, this.store__208).All();
            string formatted__233 = this.FormatRecords(results__232);
            return "=== Demo 1: All Users ===" + "\n" + "\n" + "Query: new Query(userSchema, store).all()" + "\n" + "\n" + "Results:" + "\n" + formatted__233 + "\n" + "\n" + "Total: " + S::Convert.ToString(results__232.Count) + " records";
        }
        public string RunDemo2()
        {
            this.IncrementQueryCount();
            G::IReadOnlyList<Record> results__236 = new Query(this.schema__207, this.store__208).Where("age", ">=", "18").All();
            string formatted__237 = this.FormatRecords(results__236);
            return "=== Demo 2: Filter Adults ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .where(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + ">=" + "\u0022" + ", " + "\u0022" + "18" + "\u0022" + ")" + "\n" + "  .all()" + "\n" + "\n" + "Results:" + "\n" + formatted__237 + "\n" + "\n" + "Total: " + S::Convert.ToString(results__236.Count) + " adults found";
        }
        public string RunDemo3()
        {
            this.IncrementQueryCount();
            G::IReadOnlyList<Record> results__240 = new Query(this.schema__207, this.store__208).OrderBy("age", "desc").All();
            string formatted__241 = this.FormatRecords(results__240);
            return "=== Demo 3: Sort by Age (Descending) ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .orderBy(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + "desc" + "\u0022" + ")" + "\n" + "  .all()" + "\n" + "\n" + "Results (ordered by age, oldest first):" + "\n" + formatted__241;
        }
        public string RunDemo4()
        {
            this.IncrementQueryCount();
            G::IReadOnlyList<Record> page1__244 = new Query(this.schema__207, this.store__208).OrderBy("id", "asc").Limit(2).All();
            G::IReadOnlyList<Record> page2__245 = new Query(this.schema__207, this.store__208).OrderBy("id", "asc").Offset(2).Limit(2).All();
            string formatted1__246 = this.FormatRecords(page1__244);
            string formatted2__247 = this.FormatRecords(page2__245);
            return "=== Demo 4: Pagination ===" + "\n" + "\n" + "Page 1: .orderBy(" + "\u0022" + "id" + "\u0022" + ", " + "\u0022" + "asc" + "\u0022" + ").limit(2)" + "\n" + "\n" + formatted1__246 + "\n" + "\n" + "Page 2: .orderBy(" + "\u0022" + "id" + "\u0022" + ", " + "\u0022" + "asc" + "\u0022" + ").offset(2).limit(2)" + "\n" + "\n" + formatted2__247;
        }
        public string RunDemo5()
        {
            this.IncrementQueryCount();
            G::IReadOnlyList<Record> results__250 = new Query(this.schema__207, this.store__208).Where("age", ">=", "18").Where("email", "!=", "").OrderBy("age", "desc").Select(C::Listed.CreateReadOnlyList<string>("name", "age")).Limit(2).All();
            string formatted__251 = this.FormatRecords(results__250);
            return "=== Demo 5: Complex Query ===" + "\n" + "\n" + "Query: new Query(userSchema, store)" + "\n" + "  .where(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + ">=" + "\u0022" + ", " + "\u0022" + "18" + "\u0022" + ")" + "\n" + "  .where(" + "\u0022" + "email" + "\u0022" + ", " + "\u0022" + "!=" + "\u0022" + ", " + "\u0022" + "\u0022" + ")" + "\n" + "  .orderBy(" + "\u0022" + "age" + "\u0022" + ", " + "\u0022" + "desc" + "\u0022" + ")" + "\n" + "  .select([" + "\u0022" + "name" + "\u0022" + ", " + "\u0022" + "age" + "\u0022" + "])" + "\n" + "  .limit(2)" + "\n" + "  .all()" + "\n" + "\n" + "Results (adults with email, showing name/age only, oldest first, max 2):" + "\n" + formatted__251;
        }
        public Schema Schema
        {
            get
            {
                return this.schema__207;
            }
        }
        public InMemoryStore Store
        {
            get
            {
                return this.store__208;
            }
        }
    }
}
