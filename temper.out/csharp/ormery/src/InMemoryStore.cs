using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace Ormery
{
    public class InMemoryStore
    {
        G::IDictionary<string, G::IList<Record>> tables__330;
        G::IDictionary<string, int> nextIds__331;
        public InMemoryStore()
        {
            G::IDictionary<string, G::IList<Record>> t___4058 = new C::OrderedDictionary<string, G::IList<Record>>();
            this.tables__330 = t___4058;
            G::IDictionary<string, int> t___4059 = new C::OrderedDictionary<string, int>();
            this.nextIds__331 = t___4059;
        }
        void EnsureTable(string tableName__335)
        {
            G::IList<Record> t___4055;
            if (!C::Mapped.ContainsKey(this.tables__330, tableName__335))
            {
                t___4055 = new G::List<Record>();
                this.tables__330[tableName__335] = t___4055;
                this.nextIds__331[tableName__335] = 1;
            }
        }
        public Record Insert(string tableName__338, G::IReadOnlyDictionary<string, string> data__339)
        {
            this.EnsureTable(tableName__338);
            int id__341 = C::Mapped.GetOrDefault(this.nextIds__331, tableName__338, 1);
            this.nextIds__331[tableName__338] = id__341 + 1;
            G::IDictionary<string, string> dataBuilder__342 = C::Mapped.ToMapBuilder(data__339);
            dataBuilder__342["id"] = S::Convert.ToString(id__341);
            Record record__343 = new Record(C::Mapped.ToMap(dataBuilder__342));
            G::IList<Record> t___4051 = new G::List<Record>();
            G::IList<Record> table__344 = C::Mapped.GetOrDefault(this.tables__330, tableName__338, t___4051);
            C::Listed.Add(table__344, record__343);
            return record__343;
        }
        public G::IReadOnlyList<Record> All(string tableName__346)
        {
            this.EnsureTable(tableName__346);
            G::IList<Record> t___4040 = new G::List<Record>();
            G::IList<Record> table__348 = C::Mapped.GetOrDefault(this.tables__330, tableName__346, t___4040);
            return C::Listed.ToReadOnlyList(table__348);
        }
        public Record Get(string tableName__350, int id__351)
        {
            Record return__138;
            G::IList<Record> t___4034;
            {
                {
                    this.EnsureTable(tableName__350);
                    t___4034 = new G::List<Record>();
                    G::IList<Record> table__353 = C::Mapped.GetOrDefault(this.tables__330, tableName__350, t___4034);
                    G::IReadOnlyList<Record> this__2516 = C::Listed.ToReadOnlyList(table__353);
                    int n__2517 = this__2516.Count;
                    int i__2518 = 0;
                    while (i__2518 < n__2517)
                    {
                        Record el__2519 = this__2516[i__2518];
                        i__2518 = i__2518 + 1;
                        Record record__354 = el__2519;
                        int recordId__355;
                        recordId__355 = record__354.Id;
                        if (recordId__355 == id__351)
                        {
                            return__138 = record__354;
                            goto fn__352;
                        }
                    }
                    throw new S::Exception();
                }
                fn__352:
                {
                }
            }
            return return__138;
        }
        public int Count(string tableName__357)
        {
            this.EnsureTable(tableName__357);
            G::IList<Record> t___4030 = new G::List<Record>();
            G::IList<Record> table__359 = C::Mapped.GetOrDefault(this.tables__330, tableName__357, t___4030);
            return table__359.Count;
        }
    }
}
