using U = Microsoft.VisualStudio.TestTools.UnitTesting;
using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
using T = TemperLang.Std.Testing;
namespace Ormery.Sql
{
    [U::TestClass]
    public class SqlTests
    {
        [U::TestMethod]
        public void stringEscaping__268()
        {
            T::Test test___4 = new T::Test();
            try
            {
                string build__219(string name__221)
                {
                    SqlBuilder t___1095 = new SqlBuilder();
                    t___1095.AppendSafe("select * from hi where name = ");
                    t___1095.AppendString(name__221);
                    return t___1095.Accumulated.ToString();
                }
                string buildWrong__220(string name__223)
                {
                    return "select * from hi where name = '" + name__223 + "'";
                }
                string actual___270 = build__219("world");
                bool t___1105 = actual___270 == "select * from hi where name = 'world'";
                string fn__1102()
                {
                    return "expected build(\u0022world\u0022) == (" + "select * from hi where name = 'world'" + ") not (" + actual___270 + ")";
                }
                test___4.Assert(t___1105, (S::Func<string>) fn__1102);
                string bobbyTables__225 = "Robert'); drop table hi;--";
                string actual___272 = build__219("Robert'); drop table hi;--");
                bool t___1109 = actual___272 == "select * from hi where name = 'Robert''); drop table hi;--'";
                string fn__1101()
                {
                    return "expected build(bobbyTables) == (" + "select * from hi where name = 'Robert''); drop table hi;--'" + ") not (" + actual___272 + ")";
                }
                test___4.Assert(t___1109, (S::Func<string>) fn__1101);
                string fn__1100()
                {
                    return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
                }
                test___4.Assert(true, (S::Func<string>) fn__1100);
            }
            finally
            {
                test___4.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void stringEdgeCases__276()
        {
            T::Test test___5 = new T::Test();
            try
            {
                SqlBuilder t___1063 = new SqlBuilder();
                t___1063.AppendSafe("v = ");
                t___1063.AppendString("");
                string actual___277 = t___1063.Accumulated.ToString();
                bool t___1069 = actual___277 == "v = ''";
                string fn__1062()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022\u0022).toString() == (" + "v = ''" + ") not (" + actual___277 + ")";
                }
                test___5.Assert(t___1069, (S::Func<string>) fn__1062);
                SqlBuilder t___1071 = new SqlBuilder();
                t___1071.AppendSafe("v = ");
                t___1071.AppendString("a''b");
                string actual___280 = t___1071.Accumulated.ToString();
                bool t___1077 = actual___280 == "v = 'a''''b'";
                string fn__1061()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022a''b\u0022).toString() == (" + "v = 'a''''b'" + ") not (" + actual___280 + ")";
                }
                test___5.Assert(t___1077, (S::Func<string>) fn__1061);
                SqlBuilder t___1079 = new SqlBuilder();
                t___1079.AppendSafe("v = ");
                t___1079.AppendString("Hello 世界");
                string actual___283 = t___1079.Accumulated.ToString();
                bool t___1085 = actual___283 == "v = 'Hello 世界'";
                string fn__1060()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022Hello 世界\u0022).toString() == (" + "v = 'Hello 世界'" + ") not (" + actual___283 + ")";
                }
                test___5.Assert(t___1085, (S::Func<string>) fn__1060);
                SqlBuilder t___1087 = new SqlBuilder();
                t___1087.AppendSafe("v = ");
                t___1087.AppendString("Line1\nLine2");
                string actual___286 = t___1087.Accumulated.ToString();
                bool t___1093 = actual___286 == "v = 'Line1\nLine2'";
                string fn__1059()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022Line1\\nLine2\u0022).toString() == (" + "v = 'Line1\nLine2'" + ") not (" + actual___286 + ")";
                }
                test___5.Assert(t___1093, (S::Func<string>) fn__1059);
            }
            finally
            {
                test___5.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void numbersAndBooleans__289()
        {
            T::Test test___6 = new T::Test();
            try
            {
                SqlBuilder t___1034 = new SqlBuilder();
                t___1034.AppendSafe("select ");
                t___1034.AppendInt32(42);
                t___1034.AppendSafe(", ");
                t___1034.AppendInt64(43);
                t___1034.AppendSafe(", ");
                t___1034.AppendFloat64(19.99);
                t___1034.AppendSafe(", ");
                t___1034.AppendBoolean(true);
                t___1034.AppendSafe(", ");
                t___1034.AppendBoolean(false);
                string actual___290 = t___1034.Accumulated.ToString();
                bool t___1048 = actual___290 == "select 42, 43, 19.99, TRUE, FALSE";
                string fn__1033()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select \u0022, \\interpolate, 42, \u0022, \u0022, \\interpolate, 43, \u0022, \u0022, \\interpolate, 19.99, \u0022, \u0022, \\interpolate, true, \u0022, \u0022, \\interpolate, false).toString() == (" + "select 42, 43, 19.99, TRUE, FALSE" + ") not (" + actual___290 + ")";
                }
                test___6.Assert(t___1048, (S::Func<string>) fn__1033);
                S::DateTime t___624;
                t___624 = new S::DateTime(2024, 12, 25);
                S::DateTime date__228 = t___624;
                SqlBuilder t___1050 = new SqlBuilder();
                t___1050.AppendSafe("insert into t values (");
                t___1050.AppendDate(date__228);
                t___1050.AppendSafe(")");
                string actual___293 = t___1050.Accumulated.ToString();
                bool t___1057 = actual___293 == "insert into t values ('2024-12-25')";
                string fn__1032()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022insert into t values (\u0022, \\interpolate, date, \u0022)\u0022).toString() == (" + "insert into t values ('2024-12-25')" + ") not (" + actual___293 + ")";
                }
                test___6.Assert(t___1057, (S::Func<string>) fn__1032);
            }
            finally
            {
                test___6.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void lists__296()
        {
            T::Test test___7 = new T::Test();
            try
            {
                SqlBuilder t___978 = new SqlBuilder();
                t___978.AppendSafe("v IN (");
                t___978.AppendStringList(C::Listed.CreateReadOnlyList<string>("a", "b", "c'd"));
                t___978.AppendSafe(")");
                string actual___297 = t___978.Accumulated.ToString();
                bool t___985 = actual___297 == "v IN ('a', 'b', 'c''d')";
                string fn__977()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(\u0022a\u0022, \u0022b\u0022, \u0022c'd\u0022), \u0022)\u0022).toString() == (" + "v IN ('a', 'b', 'c''d')" + ") not (" + actual___297 + ")";
                }
                test___7.Assert(t___985, (S::Func<string>) fn__977);
                SqlBuilder t___987 = new SqlBuilder();
                t___987.AppendSafe("v IN (");
                t___987.AppendInt32_List(C::Listed.CreateReadOnlyList<int>(1, 2, 3));
                t___987.AppendSafe(")");
                string actual___300 = t___987.Accumulated.ToString();
                bool t___994 = actual___300 == "v IN (1, 2, 3)";
                string fn__976()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1, 2, 3), \u0022)\u0022).toString() == (" + "v IN (1, 2, 3)" + ") not (" + actual___300 + ")";
                }
                test___7.Assert(t___994, (S::Func<string>) fn__976);
                SqlBuilder t___996 = new SqlBuilder();
                t___996.AppendSafe("v IN (");
                t___996.AppendInt64_List(C::Listed.CreateReadOnlyList<long>(1, 2));
                t___996.AppendSafe(")");
                string actual___303 = t___996.Accumulated.ToString();
                bool t___1003 = actual___303 == "v IN (1, 2)";
                string fn__975()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1, 2), \u0022)\u0022).toString() == (" + "v IN (1, 2)" + ") not (" + actual___303 + ")";
                }
                test___7.Assert(t___1003, (S::Func<string>) fn__975);
                SqlBuilder t___1005 = new SqlBuilder();
                t___1005.AppendSafe("v IN (");
                t___1005.AppendFloat64_List(C::Listed.CreateReadOnlyList<double>(1.0, 2.0));
                t___1005.AppendSafe(")");
                string actual___306 = t___1005.Accumulated.ToString();
                bool t___1012 = actual___306 == "v IN (1.0, 2.0)";
                string fn__974()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1.0, 2.0), \u0022)\u0022).toString() == (" + "v IN (1.0, 2.0)" + ") not (" + actual___306 + ")";
                }
                test___7.Assert(t___1012, (S::Func<string>) fn__974);
                SqlBuilder t___1014 = new SqlBuilder();
                t___1014.AppendSafe("v IN (");
                t___1014.AppendBooleanList(C::Listed.CreateReadOnlyList<bool>(true, false));
                t___1014.AppendSafe(")");
                string actual___309 = t___1014.Accumulated.ToString();
                bool t___1021 = actual___309 == "v IN (TRUE, FALSE)";
                string fn__973()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(true, false), \u0022)\u0022).toString() == (" + "v IN (TRUE, FALSE)" + ") not (" + actual___309 + ")";
                }
                test___7.Assert(t___1021, (S::Func<string>) fn__973);
                S::DateTime t___596;
                t___596 = new S::DateTime(2024, 1, 1);
                S::DateTime t___597 = t___596;
                S::DateTime t___598;
                t___598 = new S::DateTime(2024, 12, 25);
                S::DateTime t___599 = t___598;
                G::IReadOnlyList<S::DateTime> dates__230 = C::Listed.CreateReadOnlyList<S::DateTime>(t___597, t___599);
                SqlBuilder t___1023 = new SqlBuilder();
                t___1023.AppendSafe("v IN (");
                t___1023.AppendDateList(dates__230);
                t___1023.AppendSafe(")");
                string actual___312 = t___1023.Accumulated.ToString();
                bool t___1030 = actual___312 == "v IN ('2024-01-01', '2024-12-25')";
                string fn__972()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, dates, \u0022)\u0022).toString() == (" + "v IN ('2024-01-01', '2024-12-25')" + ") not (" + actual___312 + ")";
                }
                test___7.Assert(t___1030, (S::Func<string>) fn__972);
            }
            finally
            {
                test___7.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void nesting__315()
        {
            T::Test test___8 = new T::Test();
            try
            {
                string name__232 = "Someone";
                SqlBuilder t___941 = new SqlBuilder();
                t___941.AppendSafe("where p.last_name = ");
                t___941.AppendString("Someone");
                SqlFragment condition__233 = t___941.Accumulated;
                SqlBuilder t___945 = new SqlBuilder();
                t___945.AppendSafe("select p.id from person p ");
                t___945.AppendFragment(condition__233);
                string actual___317 = t___945.Accumulated.ToString();
                bool t___951 = actual___317 == "select p.id from person p where p.last_name = 'Someone'";
                string fn__940()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select p.id from person p \u0022, \\interpolate, condition).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual___317 + ")";
                }
                test___8.Assert(t___951, (S::Func<string>) fn__940);
                SqlBuilder t___953 = new SqlBuilder();
                t___953.AppendSafe("select p.id from person p ");
                t___953.AppendPart(condition__233.ToSource());
                string actual___320 = t___953.Accumulated.ToString();
                bool t___960 = actual___320 == "select p.id from person p where p.last_name = 'Someone'";
                string fn__939()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select p.id from person p \u0022, \\interpolate, condition.toSource()).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual___320 + ")";
                }
                test___8.Assert(t___960, (S::Func<string>) fn__939);
                G::IReadOnlyList<ISqlPart> parts__234 = C::Listed.CreateReadOnlyList<ISqlPart>(new SqlString("a'b"), new SqlInt32(3));
                SqlBuilder t___964 = new SqlBuilder();
                t___964.AppendSafe("select ");
                t___964.AppendPartList(parts__234);
                string actual___323 = t___964.Accumulated.ToString();
                bool t___970 = actual___323 == "select 'a''b', 3";
                string fn__938()
                {
                    return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select \u0022, \\interpolate, parts).toString() == (" + "select 'a''b', 3" + ") not (" + actual___323 + ")";
                }
                test___8.Assert(t___970, (S::Func<string>) fn__938);
            }
            finally
            {
                test___8.SoftFailToHard();
            }
        }
    }
}
