using H = Ormery.Html;
using S0 = Ormery.Sql;
using S1 = Ormery.Support;
using S2 = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
namespace Ormery
{
    public static class OrmeryGlobal
    {
        internal static C::ILoggingConsole console___737;
        internal static bool compareInt__195(string recordValue__450, string operator__451, string clauseValue__452)
        {
            bool return__157;
            string t___4028;
            int t___2376;
            int t___2378;
            {
                {
                    int rv__454;
                    try
                    {
                        t___2376 = C::Core.ToInt(recordValue__450);
                        rv__454 = t___2376;
                    }
                    catch
                    {
                        rv__454 = 0;
                    }
                    int cv__455;
                    try
                    {
                        t___2378 = C::Core.ToInt(clauseValue__452);
                        cv__455 = t___2378;
                    }
                    catch
                    {
                        cv__455 = 0;
                    }
                    t___4028 = S2::Convert.ToString(cv__455);
                    if (clauseValue__452 != t___4028)
                    {
                        return__157 = false;
                        goto fn__453;
                    }
                    if (operator__451 == "==")
                    {
                        return__157 = rv__454 == cv__455;
                    }
                    else if (operator__451 == "!=")
                    {
                        return__157 = rv__454 != cv__455;
                    }
                    else if (operator__451 == ">")
                    {
                        return__157 = rv__454 > cv__455;
                    }
                    else if (operator__451 == "<")
                    {
                        return__157 = rv__454 < cv__455;
                    }
                    else if (operator__451 == ">=")
                    {
                        return__157 = rv__454 >= cv__455;
                    }
                    else if (operator__451 == "<=")
                    {
                        return__157 = rv__454 <= cv__455;
                    }
                    else
                    {
                        return__157 = false;
                    }
                }
                fn__453:
                {
                }
            }
            return return__157;
        }
        internal static bool compareString__196(string recordValue__456, string operator__457, string clauseValue__458)
        {
            bool return__158;
            if (operator__457 == "==")
            {
                return__158 = recordValue__456 == clauseValue__458;
            }
            else if (operator__457 == "!=")
            {
                return__158 = recordValue__456 != clauseValue__458;
            }
            else if (operator__457 == ">")
            {
                return__158 = C::StringUtil.CompareStringsByCodePoint(recordValue__456, clauseValue__458) > 0;
            }
            else if (operator__457 == "<")
            {
                return__158 = C::StringUtil.CompareStringsByCodePoint(recordValue__456, clauseValue__458) < 0;
            }
            else if (operator__457 == ">=")
            {
                return__158 = C::StringUtil.CompareStringsByCodePoint(recordValue__456, clauseValue__458) >= 0;
            }
            else if (operator__457 == "<=")
            {
                return__158 = C::StringUtil.CompareStringsByCodePoint(recordValue__456, clauseValue__458) <= 0;
            }
            else
            {
                return__158 = false;
            }
            return return__158;
        }
        internal static S0::SqlFragment safeSql__198(string trusted__462)
        {
            S0::SqlBuilder b__464 = new S0::SqlBuilder();
            b__464.AppendSafe(trusted__462);
            return b__464.Accumulated;
        }
        internal static S0::SqlFragment columnListSql__199(G::IReadOnlyList<string> selectFields__465)
        {
            S0::SqlFragment return__161;
            S0::SqlBuilder t___4009;
            string t___4012;
            S0::SqlBuilder t___4014;
            S0::SqlFragment t___4016;
            int t___4017;
            string t___4018;
            S0::SqlBuilder t___4020;
            S0::SqlFragment t___4024;
            if (selectFields__465.Count == 0)
            {
                t___4009 = new S0::SqlBuilder();
                t___4009.AppendSafe("*");
                return__161 = t___4009.Accumulated;
            }
            else
            {
                t___4012 = selectFields__465[0];
                S0::SqlFragment first__467 = safeSql__198(t___4012);
                t___4014 = new S0::SqlBuilder();
                t___4014.AppendFragment(first__467);
                t___4016 = t___4014.Accumulated;
                S0::SqlFragment result__468 = t___4016;
                int i__469 = 1;
                while (true)
                {
                    t___4017 = selectFields__465.Count;
                    if (!(i__469 < t___4017)) break;
                    t___4018 = selectFields__465[i__469];
                    S0::SqlFragment col__470 = safeSql__198(t___4018);
                    t___4020 = new S0::SqlBuilder();
                    t___4020.AppendFragment(result__468);
                    t___4020.AppendSafe(", ");
                    t___4020.AppendFragment(col__470);
                    t___4024 = t___4020.Accumulated;
                    result__468 = t___4024;
                    i__469 = i__469 + 1;
                }
                return__161 = result__468;
            }
            return return__161;
        }
        internal static string validOperator__197(string op__460)
        {
            string return__159;
            if (op__460 == "=")
            {
                return__159 = "=";
            }
            else if (op__460 == "==")
            {
                return__159 = "=";
            }
            else if (op__460 == "!=")
            {
                return__159 = "!=";
            }
            else if (op__460 == "<>")
            {
                return__159 = "<>";
            }
            else if (op__460 == ">")
            {
                return__159 = ">";
            }
            else if (op__460 == "<")
            {
                return__159 = "<";
            }
            else if (op__460 == ">=")
            {
                return__159 = ">=";
            }
            else if (op__460 == "<=")
            {
                return__159 = "<=";
            }
            else
            {
                return__159 = "=";
            }
            return return__159;
        }
        internal static S0::SqlFragment whereConditionSql__200(WhereClause clause__471, Schema schema__472)
        {
            S0::SqlFragment return__162;
            S0::SqlBuilder t___3990;
            S0::SqlBuilder t___3993;
            S0::SqlBuilder t___4001;
            int t___2320;
            string t___3981 = clause__471.Field;
            S0::SqlFragment col__474 = safeSql__198(t___3981);
            string t___3983 = clause__471.Operator;
            S0::SqlFragment op__475 = safeSql__198(validOperator__197(t___3983));
            string t___3985 = clause__471.Field;
            Field t___2316;
            t___2316 = schema__472.GetField(t___3985);
            Field fieldInfo__476 = t___2316;
            if (fieldInfo__476.FieldType == "Int")
            {
                int intVal__477;
                try
                {
                    t___2320 = C::Core.ToInt(clause__471.Value);
                    intVal__477 = t___2320;
                }
                catch
                {
                    intVal__477 = 0;
                }
                if (clause__471.Value != S2::Convert.ToString(intVal__477))
                {
                    t___3990 = new S0::SqlBuilder();
                    t___3990.AppendSafe("1 = 0");
                    return__162 = t___3990.Accumulated;
                }
                else
                {
                    t___3993 = new S0::SqlBuilder();
                    t___3993.AppendFragment(col__474);
                    t___3993.AppendSafe(" ");
                    t___3993.AppendFragment(op__475);
                    t___3993.AppendSafe(" ");
                    t___3993.AppendInt32(intVal__477);
                    return__162 = t___3993.Accumulated;
                }
            }
            else
            {
                string strVal__478 = clause__471.Value;
                t___4001 = new S0::SqlBuilder();
                t___4001.AppendFragment(col__474);
                t___4001.AppendSafe(" ");
                t___4001.AppendFragment(op__475);
                t___4001.AppendSafe(" ");
                t___4001.AppendString(strVal__478);
                return__162 = t___4001.Accumulated;
            }
            return return__162;
        }
        internal static S0::SqlFragment orderBySql__201(G::IReadOnlyList<OrderClause> clauses__479)
        {
            S0::SqlFragment t___3961;
            S0::SqlFragment t___3962;
            int t___3967;
            string t___3969;
            S0::SqlFragment t___3973;
            S0::SqlFragment t___3974;
            S0::SqlBuilder t___3975;
            S0::SqlFragment t___3980;
            S0::SqlFragment t___2303;
            string t___3957 = clauses__479[0].Field;
            S0::SqlFragment first__481 = safeSql__198(t___3957);
            S0::SqlFragment firstDir__482;
            if (clauses__479[0].Direction == "desc")
            {
                t___3961 = safeSql__198(" DESC");
                firstDir__482 = t___3961;
            }
            else
            {
                t___3962 = safeSql__198(" ASC");
                firstDir__482 = t___3962;
            }
            S0::SqlBuilder t___3963 = new S0::SqlBuilder();
            t___3963.AppendFragment(first__481);
            t___3963.AppendFragment(firstDir__482);
            S0::SqlFragment t___3966 = t___3963.Accumulated;
            S0::SqlFragment result__483 = t___3966;
            int i__484 = 1;
            while (true)
            {
                t___3967 = clauses__479.Count;
                if (!(i__484 < t___3967)) break;
                t___3969 = clauses__479[i__484].Field;
                S0::SqlFragment col__485 = safeSql__198(t___3969);
                if (clauses__479[i__484].Direction == "desc")
                {
                    t___3973 = safeSql__198(" DESC");
                    t___2303 = t___3973;
                }
                else
                {
                    t___3974 = safeSql__198(" ASC");
                    t___2303 = t___3974;
                }
                S0::SqlFragment dir__486 = t___2303;
                t___3975 = new S0::SqlBuilder();
                t___3975.AppendFragment(result__483);
                t___3975.AppendSafe(", ");
                t___3975.AppendFragment(col__485);
                t___3975.AppendFragment(dir__486);
                t___3980 = t___3975.Accumulated;
                result__483 = t___3980;
                i__484 = i__484 + 1;
            }
            return result__483;
        }
        public static S0::SqlFragment ToSqlQuery(Schema schema__487, G::IReadOnlyList<string> selectFields__488, G::IReadOnlyList<WhereClause> whereClauses__489, G::IReadOnlyList<OrderClause> orderClauses__490, int limitValue__491, int offsetValue__492)
        {
            WhereClause t___3924;
            S0::SqlFragment t___3925;
            int t___3926;
            WhereClause t___3927;
            S0::SqlBuilder t___3929;
            S0::SqlFragment t___3933;
            S0::SqlBuilder t___3934;
            S0::SqlFragment t___3938;
            S0::SqlBuilder t___3941;
            S0::SqlFragment t___3945;
            S0::SqlBuilder t___3946;
            S0::SqlFragment t___3950;
            S0::SqlBuilder t___3951;
            S0::SqlFragment t___3955;
            bool fn__3907(string f__495)
            {
                return schema__487.HasField(f__495);
            }
            G::IReadOnlyList<string> validSelect__494 = C::Listed.ToReadOnlyList(L::Enumerable.Where(selectFields__488, (S2::Func<string, bool>) fn__3907));
            bool fn__3906(WhereClause c__498)
            {
                string t___3902 = c__498.Field;
                return schema__487.HasField(t___3902);
            }
            G::IReadOnlyList<WhereClause> validWhere__497 = C::Listed.ToReadOnlyList(L::Enumerable.Where(whereClauses__489, (S2::Func<WhereClause, bool>) fn__3906));
            bool fn__3905(OrderClause c__501)
            {
                string t___3900 = c__501.Field;
                return schema__487.HasField(t___3900);
            }
            G::IReadOnlyList<OrderClause> validOrder__500 = C::Listed.ToReadOnlyList(L::Enumerable.Where(orderClauses__490, (S2::Func<OrderClause, bool>) fn__3905));
            string t___3914 = schema__487.TableName;
            S0::SqlFragment table__503 = safeSql__198(t___3914);
            S0::SqlFragment cols__504 = columnListSql__199(validSelect__494);
            S0::SqlBuilder t___3917 = new S0::SqlBuilder();
            t___3917.AppendSafe("SELECT ");
            t___3917.AppendFragment(cols__504);
            t___3917.AppendSafe(" FROM ");
            t___3917.AppendFragment(table__503);
            S0::SqlFragment t___3922 = t___3917.Accumulated;
            S0::SqlFragment result__505 = t___3922;
            if (validWhere__497.Count > 0)
            {
                t___3924 = validWhere__497[0];
                t___3925 = whereConditionSql__200(t___3924, schema__487);
                S0::SqlFragment conditions__506 = t___3925;
                int i__507 = 1;
                while (true)
                {
                    t___3926 = validWhere__497.Count;
                    if (!(i__507 < t___3926)) break;
                    t___3927 = validWhere__497[i__507];
                    S0::SqlFragment next__508 = whereConditionSql__200(t___3927, schema__487);
                    t___3929 = new S0::SqlBuilder();
                    t___3929.AppendFragment(conditions__506);
                    t___3929.AppendSafe(" AND ");
                    t___3929.AppendFragment(next__508);
                    t___3933 = t___3929.Accumulated;
                    conditions__506 = t___3933;
                    i__507 = i__507 + 1;
                }
                t___3934 = new S0::SqlBuilder();
                t___3934.AppendFragment(result__505);
                t___3934.AppendSafe(" WHERE ");
                t___3934.AppendFragment(conditions__506);
                t___3938 = t___3934.Accumulated;
                result__505 = t___3938;
            }
            if (validOrder__500.Count > 0)
            {
                S0::SqlFragment ordering__509 = orderBySql__201(validOrder__500);
                t___3941 = new S0::SqlBuilder();
                t___3941.AppendFragment(result__505);
                t___3941.AppendSafe(" ORDER BY ");
                t___3941.AppendFragment(ordering__509);
                t___3945 = t___3941.Accumulated;
                result__505 = t___3945;
            }
            if (limitValue__491 >= 0)
            {
                t___3946 = new S0::SqlBuilder();
                t___3946.AppendFragment(result__505);
                t___3946.AppendSafe(" LIMIT ");
                t___3946.AppendInt32(limitValue__491);
                t___3950 = t___3946.Accumulated;
                result__505 = t___3950;
            }
            if (offsetValue__492 > 0)
            {
                t___3951 = new S0::SqlBuilder();
                t___3951.AppendFragment(result__505);
                t___3951.AppendSafe(" OFFSET ");
                t___3951.AppendInt32(offsetValue__492);
                t___3955 = t___3951.Accumulated;
                result__505 = t___3955;
            }
            return result__505;
        }
        public static Field Field(string name__293, string fieldType__294, bool primaryKey__295, bool nullable__296)
        {
            return new Field(name__293, fieldType__294, primaryKey__295, nullable__296);
        }
        public static bool IsValidIdentifier(string name__298)
        {
            bool return__123;
            int t___3781;
            {
                {
                    if (string.IsNullOrEmpty(name__298))
                    {
                        return__123 = false;
                        goto fn__299;
                    }
                    string this__2531 = name__298;
                    int index__2533 = 0;
                    while (true)
                    {
                        if (!C::StringUtil.HasIndex(this__2531, index__2533)) break;
                        int codePoint__2534 = C::StringUtil.Get(this__2531, index__2533);
                        int c__300 = codePoint__2534;
                        if (c__300 != 95) if (c__300 >= 97) if (c__300 > 122)
                        {
                            return__123 = false;
                            goto fn__299;
                        }
                        else if (c__300 >= 65) if (c__300 > 90)
                        {
                            return__123 = false;
                            goto fn__299;
                        }
                        else if (c__300 >= 48) if (c__300 > 57)
                        {
                            return__123 = false;
                            goto fn__299;
                        }
                        else
                        {
                            return__123 = false;
                            goto fn__299;
                        }
                        t___3781 = C::StringUtil.Next(this__2531, index__2533);
                        index__2533 = t___3781;
                    }
                    return__123 = true;
                }
                fn__299:
                {
                }
            }
            return return__123;
        }
        public static Schema Schema(string tableName__301, G::IReadOnlyList<Field> fields__302)
        {
            if (!IsValidIdentifier(tableName__301)) throw new S2::Exception();
            Field idField__304 = new Field("id", "Int", true, false);
            G::IList<Field> allFields__305 = new G::List<Field>();
            C::Listed.Add(allFields__305, idField__304);
            C::Listed.AddAll(allFields__305, fields__302);
            G::IReadOnlyList<Field> t___3776 = C::Listed.ToReadOnlyList(allFields__305);
            return new Schema(tableName__301, t___3776);
        }
        public static S0::SqlFragment ToInsertSql(Schema schema__510, G::IReadOnlyDictionary<string, string> values__511)
        {
            S0::SqlFragment return__165;
            string t___3724;
            G::IReadOnlyList<Field> t___3726;
            S0::SqlBuilder t___3730;
            G::IReadOnlyList<string> t___3733;
            string t___3736;
            S0::SqlBuilder t___3740;
            S0::SqlFragment t___3742;
            S0::SqlBuilder t___3743;
            S0::SqlFragment t___3745;
            int t___3746;
            string t___3748;
            S0::SqlBuilder t___3752;
            S0::SqlFragment t___3756;
            S0::SqlBuilder t___3757;
            S0::SqlFragment t___3761;
            S0::SqlBuilder t___3762;
            int t___2047;
            S0::SqlFragment t___2058;
            int t___2065;
            int t___2066;
            {
                {
                    t___3724 = schema__510.TableName;
                    S0::SqlFragment table__513 = safeSql__198(t___3724);
                    t___3726 = schema__510.Fields;
                    bool fn__3723(Field f__515)
                    {
                        string t___3720 = f__515.Name;
                        return C::Mapped.ContainsKey(values__511, t___3720);
                    }
                    G::IReadOnlyList<Field> fieldList__514 = C::Listed.ToReadOnlyList(L::Enumerable.Where(t___3726, (S2::Func<Field, bool>) fn__3723));
                    if (fieldList__514.Count == 0)
                    {
                        t___3730 = new S0::SqlBuilder();
                        return__165 = t___3730.Accumulated;
                        goto fn__512;
                    }
                    string fn__3722(Field f__518)
                    {
                        return f__518.Name;
                    }
                    t___3733 = C::Listed.ToReadOnlyList(L::Enumerable.Select(fieldList__514, (S2::Func<Field, string>) fn__3722));
                    S0::SqlFragment colNames__517 = columnListSql__199(t___3733);
                    t___3736 = fieldList__514[0].Name;
                    string firstVal__520 = C::Mapped.GetOrDefault(values__511, t___3736, "");
                    if (fieldList__514[0].FieldType == "Int")
                    {
                        int iv__522;
                        try
                        {
                            t___2047 = C::Core.ToInt(firstVal__520);
                            iv__522 = t___2047;
                        }
                        catch
                        {
                            iv__522 = 0;
                        }
                        t___3740 = new S0::SqlBuilder();
                        t___3740.AppendInt32(iv__522);
                        t___3742 = t___3740.Accumulated;
                        t___2058 = t___3742;
                    }
                    else
                    {
                        t___3743 = new S0::SqlBuilder();
                        t___3743.AppendString(firstVal__520);
                        t___3745 = t___3743.Accumulated;
                        t___2058 = t___3745;
                    }
                    S0::SqlFragment vals__521 = t___2058;
                    int i__523 = 1;
                    while (true)
                    {
                        t___3746 = fieldList__514.Count;
                        if (!(i__523 < t___3746)) break;
                        t___3748 = fieldList__514[i__523].Name;
                        string val__524 = C::Mapped.GetOrDefault(values__511, t___3748, "");
                        if (fieldList__514[i__523].FieldType == "Int")
                        {
                            try
                            {
                                t___2065 = C::Core.ToInt(val__524);
                                t___2066 = t___2065;
                            }
                            catch
                            {
                                t___2066 = 0;
                            }
                            int iv__525 = t___2066;
                            t___3752 = new S0::SqlBuilder();
                            t___3752.AppendFragment(vals__521);
                            t___3752.AppendSafe(", ");
                            t___3752.AppendInt32(iv__525);
                            t___3756 = t___3752.Accumulated;
                            vals__521 = t___3756;
                        }
                        else
                        {
                            t___3757 = new S0::SqlBuilder();
                            t___3757.AppendFragment(vals__521);
                            t___3757.AppendSafe(", ");
                            t___3757.AppendString(val__524);
                            t___3761 = t___3757.Accumulated;
                            vals__521 = t___3761;
                        }
                        i__523 = i__523 + 1;
                    }
                    t___3762 = new S0::SqlBuilder();
                    t___3762.AppendSafe("INSERT INTO ");
                    t___3762.AppendFragment(table__513);
                    t___3762.AppendSafe(" (");
                    t___3762.AppendFragment(colNames__517);
                    t___3762.AppendSafe(") VALUES (");
                    t___3762.AppendFragment(vals__521);
                    t___3762.AppendSafe(")");
                    return__165 = t___3762.Accumulated;
                }
                fn__512:
                {
                }
            }
            return return__165;
        }
        public static void Main()
        {
            console___737.Log("=== ORMery Demo ===\n");
            G::IReadOnlyList<Field> userFields__642 = C::Listed.CreateReadOnlyList<Field>(Field("name", "String", false, false), Field("age", "Int", false, false), Field("email", "String", false, true));
            Schema userSchema__643 = Schema("users", userFields__642);
            string t___3304 = userSchema__643.Describe();
            console___737.Log(t___3304);
            console___737.Log("");
            InMemoryStore store__644 = new InMemoryStore();
            Record rec1__645 = store__644.Insert("users", C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"), new G::KeyValuePair<string, string>("age", "25"), new G::KeyValuePair<string, string>("email", "alice@example.com"))));
            Record rec2__646 = store__644.Insert("users", C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Bob"), new G::KeyValuePair<string, string>("age", "30"), new G::KeyValuePair<string, string>("email", "bob@example.com"))));
            Record rec3__647 = store__644.Insert("users", C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Charlie"), new G::KeyValuePair<string, string>("age", "17"), new G::KeyValuePair<string, string>("email", "charlie@example.com"))));
            console___737.Log("Inserted 3 users:");
            string t___3324 = rec1__645.Describe();
            console___737.Log("  " + t___3324);
            string t___3326 = rec2__646.Describe();
            console___737.Log("  " + t___3326);
            string t___3328 = rec3__647.Describe();
            console___737.Log("  " + t___3328);
            console___737.Log("");
            console___737.Log("=== In-Memory Queries ===\n");
            console___737.Log("All users:");
            G::IReadOnlyList<Record> allUsers__648 = new Query(userSchema__643, store__644).All();
            void fn__3301(Record u__649)
            {
                string t___3298 = u__649.Describe();
                console___737.Log("  " + t___3298);
            }
            C::Listed.ForEach(allUsers__648, (S2::Action<Record>) fn__3301);
            console___737.Log("");
            console___737.Log("Adults (age >= 18):");
            G::IReadOnlyList<Record> adults__650 = new Query(userSchema__643, store__644).Where("age", ">=", "18").All();
            void fn__3300(Record u__651)
            {
                string t___3296 = u__651.Describe();
                console___737.Log("  " + t___3296);
            }
            C::Listed.ForEach(adults__650, (S2::Action<Record>) fn__3300);
            console___737.Log("");
            console___737.Log("=== SQL Generation (secure-composition) ===\n");
            Query q1__652 = new Query(userSchema__643, store__644);
            string t___3348 = q1__652.ToSql().ToString();
            console___737.Log("SELECT all: " + t___3348);
            Query q2__653 = new Query(userSchema__643, store__644).Select(C::Listed.CreateReadOnlyList<string>("name", "age")).Where("age", ">=", "18").OrderBy("age", "desc").Limit(10);
            string t___3356 = q2__653.ToSql().ToString();
            console___737.Log("Complex:    " + t___3356);
            string bobby__654 = "Robert'); DROP TABLE users;--";
            Query q3__655 = new Query(userSchema__643, store__644).Where("name", "=", "Robert'); DROP TABLE users;--");
            string t___3361 = q3__655.ToSql().ToString();
            console___737.Log("Injection:  " + t___3361);
            G::IReadOnlyDictionary<string, string> insertVals__656 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "O'Malley"), new G::KeyValuePair<string, string>("age", "42")));
            string t___3367 = ToInsertSql(userSchema__643, insertVals__656).ToString();
            console___737.Log("INSERT:     " + t___3367);
            console___737.Log("\n=== Demo Complete ===");
        }
        internal static G::IReadOnlyList<string> temperKeywords__204;
        internal static G::IReadOnlyList<string> temperTypes__205;
        public static TokenType ClassifyWord(string word__685)
        {
            TokenType return__187;
            {
                {
                    G::IReadOnlyList<string> this__2536 = temperKeywords__204;
                    int n__2537 = this__2536.Count;
                    int i__2538 = 0;
                    while (i__2538 < n__2537)
                    {
                        string el__2539 = this__2536[i__2538];
                        i__2538 = i__2538 + 1;
                        string kw__687 = el__2539;
                        if (kw__687 == word__685)
                        {
                            return__187 = new TokenType("keyword");
                            goto fn__686;
                        }
                    }
                    G::IReadOnlyList<string> this__2541 = temperTypes__205;
                    int n__2542 = this__2541.Count;
                    int i__2543 = 0;
                    while (i__2543 < n__2542)
                    {
                        string el__2544 = this__2541[i__2543];
                        i__2543 = i__2543 + 1;
                        string tp__688 = el__2544;
                        if (tp__688 == word__685)
                        {
                            return__187 = new TokenType("type");
                            goto fn__686;
                        }
                    }
                    return__187 = new TokenType("identifier");
                }
                fn__686:
                {
                }
            }
            return return__187;
        }
        public static H::SafeHtml HighlightWord(string word__689)
        {
            H::SafeHtml return__188;
            H::SafeHtmlBuilder t___3275;
            {
                {
                    if (word__689 == "")
                    {
                        t___3275 = new H::SafeHtmlBuilder();
                        return__188 = t___3275.Accumulated;
                        goto fn__690;
                    }
                    TokenType tokenType__691 = ClassifyWord(word__689);
                    Token token__692 = new Token(tokenType__691, word__689);
                    return__188 = token__692.ToHtml();
                }
                fn__690:
                {
                }
            }
            return return__188;
        }
        public static H::SafeHtml HighlightLine(string line__693)
        {
            H::SafeHtml return__189;
            H::SafeHtmlBuilder t___3263;
            H::SafeHtml t___3266;
            int t___3267;
            H::SafeHtmlBuilder t___3270;
            H::SafeHtml t___3274;
            {
                {
                    G::IReadOnlyList<string> words__695 = C::Core.Split(line__693, " ");
                    if (words__695.Count == 0)
                    {
                        t___3263 = new H::SafeHtmlBuilder();
                        return__189 = t___3263.Accumulated;
                        goto fn__694;
                    }
                    t___3266 = HighlightWord(words__695[0]);
                    H::SafeHtml result__696 = t___3266;
                    int i__697 = 1;
                    while (true)
                    {
                        t___3267 = words__695.Count;
                        if (!(i__697 < t___3267)) break;
                        H::SafeHtml word__698 = HighlightWord(words__695[i__697]);
                        t___3270 = new H::SafeHtmlBuilder();
                        t___3270.AppendSafeHtml(result__696);
                        t___3270.AppendSafe(" ");
                        t___3270.AppendSafeHtml(word__698);
                        t___3274 = t___3270.Accumulated;
                        result__696 = t___3274;
                        i__697 = i__697 + 1;
                    }
                    return__189 = result__696;
                }
                fn__694:
                {
                }
            }
            return return__189;
        }
        public static H::SafeHtml HighlightSource(string source__699)
        {
            H::SafeHtml return__190;
            H::SafeHtmlBuilder t___3249;
            H::SafeHtml t___3252;
            int t___3253;
            H::SafeHtmlBuilder t___3256;
            H::SafeHtml t___3260;
            {
                {
                    G::IReadOnlyList<string> lines__701 = C::Core.Split(source__699, "\n");
                    if (lines__701.Count == 0)
                    {
                        t___3249 = new H::SafeHtmlBuilder();
                        return__190 = t___3249.Accumulated;
                        goto fn__700;
                    }
                    t___3252 = HighlightLine(lines__701[0]);
                    H::SafeHtml result__702 = t___3252;
                    int i__703 = 1;
                    while (true)
                    {
                        t___3253 = lines__701.Count;
                        if (!(i__703 < t___3253)) break;
                        H::SafeHtml line__704 = HighlightLine(lines__701[i__703]);
                        t___3256 = new H::SafeHtmlBuilder();
                        t___3256.AppendSafeHtml(result__702);
                        t___3256.AppendSafe("\\n");
                        t___3256.AppendSafeHtml(line__704);
                        t___3260 = t___3256.Accumulated;
                        result__702 = t___3260;
                        i__703 = i__703 + 1;
                    }
                    return__190 = result__702;
                }
                fn__700:
                {
                }
            }
            return return__190;
        }
        public static H::SafeHtml HighlightBlock(string source__705)
        {
            H::SafeHtml highlighted__707 = HighlightSource(source__705);
            H::SafeHtmlBuilder t___3242 = new H::SafeHtmlBuilder();
            t___3242.AppendSafe("<pre class='temper-code'><code>");
            t___3242.AppendSafeHtml(highlighted__707);
            t___3242.AppendSafe("</code></pre>");
            return t___3242.Accumulated;
        }
        static OrmeryGlobal()
        {
            console___737 = S1::Logging.LoggingConsoleFactory.CreateConsole("Ormery");
            temperKeywords__204 = C::Listed.CreateReadOnlyList<string>("if", "else", "for", "while", "do", "when", "break", "continue", "return", "let", "var", "class", "export", "import", "public", "private", "protected", "throws", "new", "this", "get", "set", "static", "extends", "implements", "true", "false", "null", "bubble", "orelse", "of");
            temperTypes__205 = C::Listed.CreateReadOnlyList<string>("String", "Int", "Boolean", "List", "Map", "Bubble", "Pair", "Float", "Double", "Byte", "Short", "Long", "Char", "Void", "Record", "Schema", "Field", "Query", "InMemoryStore", "ListBuilder", "MapBuilder", "WhereClause", "OrderClause");
        }
    }
}
