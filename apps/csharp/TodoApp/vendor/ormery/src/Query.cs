using O = Ormery;
using S0 = Ormery.Sql;
using S1 = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
namespace Ormery
{
    public class Query
    {
        readonly Schema schema__376;
        readonly InMemoryStore store__377;
        G::IList<WhereClause> whereClauses__378;
        G::IReadOnlyList<string> selectFields__379;
        G::IList<OrderClause> orderByClauses__380;
        int limitValue__381;
        int offsetValue__382;
        public Query(Schema schema__384, InMemoryStore store__385)
        {
            this.schema__376 = schema__384;
            this.store__377 = store__385;
            G::IList<WhereClause> t___3898 = new G::List<WhereClause>();
            this.whereClauses__378 = t___3898;
            G::IReadOnlyList<string> t___2220 = C::Listed.CreateReadOnlyList<string>();
            this.selectFields__379 = t___2220;
            G::IList<OrderClause> t___3899 = new G::List<OrderClause>();
            this.orderByClauses__380 = t___3899;
            this.limitValue__381 = -1;
            this.offsetValue__382 = 0;
        }
        public Query Where(string field__388, string operator__389, string value__390)
        {
            WhereClause t___3896 = new WhereClause(field__388, operator__389, value__390);
            C::Listed.Add(this.whereClauses__378, t___3896);
            return this;
        }
        public Query Select(G::IReadOnlyList<string> fields__393)
        {
            this.selectFields__379 = fields__393;
            return this;
        }
        public Query OrderBy(string field__396, string direction__397)
        {
            OrderClause t___3894 = new OrderClause(field__396, direction__397);
            C::Listed.Add(this.orderByClauses__380, t___3894);
            return this;
        }
        public Query Limit(int n__400)
        {
            int t___2209;
            if (n__400 < 0)
            {
                t___2209 = 0;
            }
            else
            {
                t___2209 = n__400;
            }
            this.limitValue__381 = t___2209;
            return this;
        }
        public Query Offset(int n__403)
        {
            this.offsetValue__382 = n__403;
            return this;
        }
        bool MatchesWhere(Record record__406)
        {
            bool return__152;
            string t___3883;
            string t___3885;
            string t___3887;
            string t___3889;
            string t___3890;
            bool t___3891;
            string t___3892;
            string t___3893;
            {
                {
                    G::IReadOnlyList<WhereClause> this__2521 = C::Listed.ToReadOnlyList(this.whereClauses__378);
                    int n__2522 = this__2521.Count;
                    int i__2523 = 0;
                    while (i__2523 < n__2522)
                    {
                        WhereClause el__2524 = this__2521[i__2523];
                        i__2523 = i__2523 + 1;
                        WhereClause clause__408 = el__2524;
                        t___3883 = clause__408.Field;
                        string recordValue__409 = record__406.GetOr(t___3883, "");
                        t___3885 = clause__408.Field;
                        if (!this.schema__376.HasField(t___3885))
                        {
                            return__152 = false;
                            goto fn__407;
                        }
                        t___3887 = clause__408.Field;
                        Field t___2198;
                        t___2198 = this.schema__376.GetField(t___3887);
                        Field fieldInfo__410 = t___2198;
                        string fieldType__411 = fieldInfo__410.FieldType;
                        bool matches__412;
                        if (fieldType__411 == "Int")
                        {
                            t___3889 = clause__408.Operator;
                            t___3890 = clause__408.Value;
                            t___3891 = O::OrmeryGlobal.compareInt__195(recordValue__409, t___3889, t___3890);
                            matches__412 = t___3891;
                        }
                        else if (fieldType__411 == "String")
                        {
                            t___3892 = clause__408.Operator;
                            t___3893 = clause__408.Value;
                            matches__412 = O::OrmeryGlobal.compareString__196(recordValue__409, t___3892, t___3893);
                        }
                        else
                        {
                            matches__412 = false;
                        }
                        if (!matches__412)
                        {
                            return__152 = false;
                            goto fn__407;
                        }
                    }
                    return__152 = true;
                }
                fn__407:
                {
                }
            }
            return return__152;
        }
        Record ProjectRecord(Record record__414)
        {
            Record return__153;
            G::IReadOnlyDictionary<string, string> t___3878;
            {
                {
                    if (this.selectFields__379.Count == 0)
                    {
                        return__153 = record__414;
                        goto fn__415;
                    }
                    G::IDictionary<string, string> builder__416 = new C::OrderedDictionary<string, string>();
                    void fn__3873(string fieldName__417)
                    {
                        string value__418 = record__414.GetOr(fieldName__417, "");
                        builder__416[fieldName__417] = value__418;
                    }
                    C::Listed.ForEach(this.selectFields__379, (S1::Action<string>) fn__3873);
                    t___3878 = C::Mapped.ToMap(builder__416);
                    return__153 = new Record(t___3878);
                }
                fn__415:
                {
                }
            }
            return return__153;
        }
        int CompareRecords(Record a__420, Record b__421, G::IReadOnlyList<OrderClause> orderClauses__422)
        {
            int return__154;
            string t___3862;
            string t___3864;
            string t___3866;
            string t___3868;
            {
                {
                    G::IReadOnlyList<OrderClause> this__2526 = orderClauses__422;
                    int n__2527 = this__2526.Count;
                    int i__2528 = 0;
                    while (i__2528 < n__2527)
                    {
                        OrderClause el__2529 = this__2526[i__2528];
                        i__2528 = i__2528 + 1;
                        OrderClause clause__424 = el__2529;
                        int t___2177;
                        int t___2179;
                        t___3862 = clause__424.Field;
                        string aVal__425 = a__420.GetOr(t___3862, "");
                        t___3864 = clause__424.Field;
                        string bVal__426 = b__421.GetOr(t___3864, "");
                        t___3866 = clause__424.Field;
                        if (!this.schema__376.HasField(t___3866)) continue;
                        t___3868 = clause__424.Field;
                        Field t___2174;
                        t___2174 = this.schema__376.GetField(t___3868);
                        Field fieldInfo__427 = t___2174;
                        string fieldType__428 = fieldInfo__427.FieldType;
                        int cmp__429;
                        if (fieldType__428 == "Int")
                        {
                            int aInt__430;
                            try
                            {
                                t___2177 = C::Core.ToInt(aVal__425);
                                aInt__430 = t___2177;
                            }
                            catch
                            {
                                aInt__430 = 0;
                            }
                            int bInt__431;
                            try
                            {
                                t___2179 = C::Core.ToInt(bVal__426);
                                bInt__431 = t___2179;
                            }
                            catch
                            {
                                bInt__431 = 0;
                            }
                            cmp__429 = C::Core.Compare(aInt__430, bInt__431);
                        }
                        else if (fieldType__428 == "String")
                        {
                            cmp__429 = C::Core.Compare(aVal__425, bVal__426);
                        }
                        else
                        {
                            cmp__429 = 0;
                        }
                        if (cmp__429 != 0)
                        {
                            if (clause__424.Direction == "desc")
                            {
                                return__154 = -cmp__429;
                            }
                            else
                            {
                                return__154 = cmp__429;
                            }
                            goto fn__423;
                        }
                    }
                    return__154 = 0;
                }
                fn__423:
                {
                }
            }
            return return__154;
        }
        public G::IReadOnlyList<Record> All()
        {
            G::IReadOnlyList<Record> t___3853;
            G::IReadOnlyList<Record> t___3854;
            int t___3855;
            int t___3856;
            G::IReadOnlyList<Record> t___3857;
            string t___3846 = this.schema__376.TableName;
            G::IReadOnlyList<Record> allRecords__434 = this.store__377.All(t___3846);
            bool fn__3845(Record r__436)
            {
                return this.MatchesWhere(r__436);
            }
            G::IReadOnlyList<Record> filtered__435 = C::Listed.ToReadOnlyList(L::Enumerable.Where(allRecords__434, (S1::Func<Record, bool>) fn__3845));
            G::IReadOnlyList<Record> sorted__438;
            if (this.orderByClauses__380.Count > 0)
            {
                G::IReadOnlyList<OrderClause> clauses__439 = C::Listed.ToReadOnlyList(this.orderByClauses__380);
                int fn__3844(Record a__440, Record b__441)
                {
                    return this.CompareRecords(a__440, b__441, clauses__439);
                }
                t___3853 = C::Listed.Sorted(filtered__435, (S1::Func<Record, Record, int>) fn__3844);
                sorted__438 = t___3853;
            }
            else
            {
                sorted__438 = filtered__435;
            }
            G::IReadOnlyList<Record> sliced__443;
            if (this.limitValue__381 >= 0)
            {
                int start__444 = this.offsetValue__382;
                int end__445 = this.offsetValue__382 + this.limitValue__381;
                t___3854 = C::Listed.Slice(sorted__438, start__444, end__445);
                sliced__443 = t___3854;
            }
            else if (this.offsetValue__382 > 0)
            {
                t___3856 = this.offsetValue__382;
                t___3855 = sorted__438.Count;
                t___3857 = C::Listed.Slice(sorted__438, t___3856, t___3855);
                sliced__443 = t___3857;
            }
            else
            {
                sliced__443 = sorted__438;
            }
            Record fn__3843(Record r__446)
            {
                return this.ProjectRecord(r__446);
            }
            return C::Listed.ToReadOnlyList(L::Enumerable.Select(sliced__443, (S1::Func<Record, Record>) fn__3843));
        }
        public S0::SqlFragment ToSql()
        {
            return O::OrmeryGlobal.ToSqlQuery(this.schema__376, this.selectFields__379, C::Listed.ToReadOnlyList(this.whereClauses__378), C::Listed.ToReadOnlyList(this.orderByClauses__380), this.limitValue__381, this.offsetValue__382);
        }
        public Schema Schema
        {
            get
            {
                return this.schema__376;
            }
        }
        public InMemoryStore Store
        {
            get
            {
                return this.store__377;
            }
        }
    }
}
