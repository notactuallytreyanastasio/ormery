package ormery.sql;
import java.util.function.Supplier;
import temper.std.testing.Test;
import java.time.LocalDate;
import java.util.List;
import java.util.function.Function;
public final class SqlTest {
    private SqlTest() {
    }
    @org.junit.jupiter.api.Test public void stringEscaping__268() {
        Test test_4 = new Test();
        try {
            Function<String, String> build__219 = name__221 -> {
                SqlBuilder t_1095 = new SqlBuilder();
                t_1095.appendSafe("select * from hi where name = ");
                t_1095.appendString(name__221);
                return t_1095.getAccumulated().toString();
            };
            Function<String, String> buildWrong__220 = name__223 -> "select * from hi where name = '" + name__223 + "'";
            String actual_270 = build__219.apply("world");
            boolean t_1105 = actual_270.equals("select * from hi where name = 'world'");
            Supplier<String> fn__1102 = () -> "expected build(\"world\") == (" + "select * from hi where name = 'world'" + ") not (" + actual_270 + ")";
            test_4.assert_(t_1105, fn__1102);
            String bobbyTables__225 = "Robert'); drop table hi;--";
            String actual_272 = build__219.apply("Robert'); drop table hi;--");
            boolean t_1109 = actual_272.equals("select * from hi where name = 'Robert''); drop table hi;--'");
            Supplier<String> fn__1101 = () -> "expected build(bobbyTables) == (" + "select * from hi where name = 'Robert''); drop table hi;--'" + ") not (" + actual_272 + ")";
            test_4.assert_(t_1109, fn__1101);
            Supplier<String> fn__1100 = () -> "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
            test_4.assert_(true, fn__1100);
        } finally {
            test_4.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void stringEdgeCases__276() {
        Test test_5 = new Test();
        try {
            SqlBuilder t_1063 = new SqlBuilder();
            t_1063.appendSafe("v = ");
            t_1063.appendString("");
            String actual_277 = t_1063.getAccumulated().toString();
            boolean t_1069 = actual_277.equals("v = ''");
            Supplier<String> fn__1062 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (" + "v = ''" + ") not (" + actual_277 + ")";
            test_5.assert_(t_1069, fn__1062);
            SqlBuilder t_1071 = new SqlBuilder();
            t_1071.appendSafe("v = ");
            t_1071.appendString("a''b");
            String actual_280 = t_1071.getAccumulated().toString();
            boolean t_1077 = actual_280.equals("v = 'a''''b'");
            Supplier<String> fn__1061 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (" + "v = 'a''''b'" + ") not (" + actual_280 + ")";
            test_5.assert_(t_1077, fn__1061);
            SqlBuilder t_1079 = new SqlBuilder();
            t_1079.appendSafe("v = ");
            t_1079.appendString("Hello \u4e16\u754c");
            String actual_283 = t_1079.getAccumulated().toString();
            boolean t_1085 = actual_283.equals("v = 'Hello \u4e16\u754c'");
            Supplier<String> fn__1060 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"Hello \u4e16\u754c\").toString() == (" + "v = 'Hello \u4e16\u754c'" + ") not (" + actual_283 + ")";
            test_5.assert_(t_1085, fn__1060);
            SqlBuilder t_1087 = new SqlBuilder();
            t_1087.appendSafe("v = ");
            t_1087.appendString("Line1\nLine2");
            String actual_286 = t_1087.getAccumulated().toString();
            boolean t_1093 = actual_286.equals("v = 'Line1\nLine2'");
            Supplier<String> fn__1059 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (" + "v = 'Line1\nLine2'" + ") not (" + actual_286 + ")";
            test_5.assert_(t_1093, fn__1059);
        } finally {
            test_5.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void numbersAndBooleans__289() {
        Test test_6 = new Test();
        try {
            SqlBuilder t_1034 = new SqlBuilder();
            t_1034.appendSafe("select ");
            t_1034.appendInt32(42);
            t_1034.appendSafe(", ");
            t_1034.appendInt64(43);
            t_1034.appendSafe(", ");
            t_1034.appendFloat64(19.99D);
            t_1034.appendSafe(", ");
            t_1034.appendBoolean(true);
            t_1034.appendSafe(", ");
            t_1034.appendBoolean(false);
            String actual_290 = t_1034.getAccumulated().toString();
            boolean t_1048 = actual_290.equals("select 42, 43, 19.99, TRUE, FALSE");
            Supplier<String> fn__1033 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (" + "select 42, 43, 19.99, TRUE, FALSE" + ") not (" + actual_290 + ")";
            test_6.assert_(t_1048, fn__1033);
            LocalDate t_624;
            t_624 = LocalDate.of(2024, 12, 25);
            LocalDate date__228 = t_624;
            SqlBuilder t_1050 = new SqlBuilder();
            t_1050.appendSafe("insert into t values (");
            t_1050.appendDate(date__228);
            t_1050.appendSafe(")");
            String actual_293 = t_1050.getAccumulated().toString();
            boolean t_1057 = actual_293.equals("insert into t values ('2024-12-25')");
            Supplier<String> fn__1032 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (" + "insert into t values ('2024-12-25')" + ") not (" + actual_293 + ")";
            test_6.assert_(t_1057, fn__1032);
        } finally {
            test_6.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void lists__296() {
        Test test_7 = new Test();
        try {
            SqlBuilder t_978 = new SqlBuilder();
            t_978.appendSafe("v IN (");
            t_978.appendStringList(List.of("a", "b", "c'd"));
            t_978.appendSafe(")");
            String actual_297 = t_978.getAccumulated().toString();
            boolean t_985 = actual_297.equals("v IN ('a', 'b', 'c''d')");
            Supplier<String> fn__977 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (" + "v IN ('a', 'b', 'c''d')" + ") not (" + actual_297 + ")";
            test_7.assert_(t_985, fn__977);
            SqlBuilder t_987 = new SqlBuilder();
            t_987.appendSafe("v IN (");
            t_987.appendInt32List(List.of(1, 2, 3));
            t_987.appendSafe(")");
            String actual_300 = t_987.getAccumulated().toString();
            boolean t_994 = actual_300.equals("v IN (1, 2, 3)");
            Supplier<String> fn__976 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (" + "v IN (1, 2, 3)" + ") not (" + actual_300 + ")";
            test_7.assert_(t_994, fn__976);
            SqlBuilder t_996 = new SqlBuilder();
            t_996.appendSafe("v IN (");
            t_996.appendInt64List(List.of(1, 2));
            t_996.appendSafe(")");
            String actual_303 = t_996.getAccumulated().toString();
            boolean t_1003 = actual_303.equals("v IN (1, 2)");
            Supplier<String> fn__975 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (" + "v IN (1, 2)" + ") not (" + actual_303 + ")";
            test_7.assert_(t_1003, fn__975);
            SqlBuilder t_1005 = new SqlBuilder();
            t_1005.appendSafe("v IN (");
            t_1005.appendFloat64List(List.of(1.0D, 2.0D));
            t_1005.appendSafe(")");
            String actual_306 = t_1005.getAccumulated().toString();
            boolean t_1012 = actual_306.equals("v IN (1.0, 2.0)");
            Supplier<String> fn__974 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (" + "v IN (1.0, 2.0)" + ") not (" + actual_306 + ")";
            test_7.assert_(t_1012, fn__974);
            SqlBuilder t_1014 = new SqlBuilder();
            t_1014.appendSafe("v IN (");
            t_1014.appendBooleanList(List.of(true, false));
            t_1014.appendSafe(")");
            String actual_309 = t_1014.getAccumulated().toString();
            boolean t_1021 = actual_309.equals("v IN (TRUE, FALSE)");
            Supplier<String> fn__973 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (" + "v IN (TRUE, FALSE)" + ") not (" + actual_309 + ")";
            test_7.assert_(t_1021, fn__973);
            LocalDate t_596;
            t_596 = LocalDate.of(2024, 1, 1);
            LocalDate t_597 = t_596;
            LocalDate t_598;
            t_598 = LocalDate.of(2024, 12, 25);
            LocalDate t_599 = t_598;
            List<LocalDate> dates__230 = List.of(t_597, t_599);
            SqlBuilder t_1023 = new SqlBuilder();
            t_1023.appendSafe("v IN (");
            t_1023.appendDateList(dates__230);
            t_1023.appendSafe(")");
            String actual_312 = t_1023.getAccumulated().toString();
            boolean t_1030 = actual_312.equals("v IN ('2024-01-01', '2024-12-25')");
            Supplier<String> fn__972 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (" + "v IN ('2024-01-01', '2024-12-25')" + ") not (" + actual_312 + ")";
            test_7.assert_(t_1030, fn__972);
        } finally {
            test_7.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void nesting__315() {
        Test test_8 = new Test();
        try {
            String name__232 = "Someone";
            SqlBuilder t_941 = new SqlBuilder();
            t_941.appendSafe("where p.last_name = ");
            t_941.appendString("Someone");
            SqlFragment condition__233 = t_941.getAccumulated();
            SqlBuilder t_945 = new SqlBuilder();
            t_945.appendSafe("select p.id from person p ");
            t_945.appendFragment(condition__233);
            String actual_317 = t_945.getAccumulated().toString();
            boolean t_951 = actual_317.equals("select p.id from person p where p.last_name = 'Someone'");
            Supplier<String> fn__940 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_317 + ")";
            test_8.assert_(t_951, fn__940);
            SqlBuilder t_953 = new SqlBuilder();
            t_953.appendSafe("select p.id from person p ");
            t_953.appendPart(condition__233.toSource());
            String actual_320 = t_953.getAccumulated().toString();
            boolean t_960 = actual_320.equals("select p.id from person p where p.last_name = 'Someone'");
            Supplier<String> fn__939 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_320 + ")";
            test_8.assert_(t_960, fn__939);
            List<SqlPart> parts__234 = List.of(new SqlString("a'b"), new SqlInt32(3));
            SqlBuilder t_964 = new SqlBuilder();
            t_964.appendSafe("select ");
            t_964.appendPartList(parts__234);
            String actual_323 = t_964.getAccumulated().toString();
            boolean t_970 = actual_323.equals("select 'a''b', 3");
            Supplier<String> fn__938 = () -> "expected stringExpr(`-work/src//sql/`.sql, true, \"select \", \\interpolate, parts).toString() == (" + "select 'a''b', 3" + ") not (" + actual_323 + ")";
            test_8.assert_(t_970, fn__938);
        } finally {
            test_8.softFailToHard();
        }
    }
}
