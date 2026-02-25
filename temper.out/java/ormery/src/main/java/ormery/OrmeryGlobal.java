package ormery;
import ormery.sql.SqlFragment;
import temper.core.Core;
import ormery.sql.SqlBuilder;
import java.util.List;
import java.util.AbstractMap.SimpleImmutableEntry;
import ormery.html.SafeHtml;
import ormery.html.SafeHtmlBuilder;
import java.util.function.Predicate;
import java.util.function.Consumer;
import java.util.Map;
import java.util.logging.Logger;
import temper.core.Core.Console;
import java.util.function.Function;
import java.util.ArrayList;
public final class OrmeryGlobal {
    private OrmeryGlobal() {
    }
    static final Console console_737;
    static final List<String> temperKeywords__204;
    static final List<String> temperTypes__205;
    static boolean compareInt__195(String recordValue__450, String operator__451, String clauseValue__452) {
        boolean return__157;
        String t_4028;
        int t_2376;
        int t_2378;
        fn__453: {
            int rv__454;
            int rv_5874;
            try {
                t_2376 = Core.stringToInt(recordValue__450);
                rv_5874 = t_2376;
            } catch (RuntimeException ignored$3) {
                rv_5874 = 0;
            }
            rv__454 = rv_5874;
            int cv__455;
            int cv_5875;
            try {
                t_2378 = Core.stringToInt(clauseValue__452);
                cv_5875 = t_2378;
            } catch (RuntimeException ignored$4) {
                cv_5875 = 0;
            }
            cv__455 = cv_5875;
            t_4028 = Integer.toString(cv__455);
            if (!clauseValue__452.equals(t_4028)) {
                return__157 = false;
                break fn__453;
            }
            if (operator__451.equals("==")) {
                return__157 = rv__454 == cv__455;
            } else if (operator__451.equals("!=")) {
                return__157 = rv__454 != cv__455;
            } else if (operator__451.equals(">")) {
                return__157 = rv__454 > cv__455;
            } else if (operator__451.equals("<")) {
                return__157 = rv__454 < cv__455;
            } else if (operator__451.equals(">=")) {
                return__157 = rv__454 >= cv__455;
            } else if (operator__451.equals("<=")) {
                return__157 = rv__454 <= cv__455;
            } else {
                return__157 = false;
            }
        }
        return return__157;
    }
    static boolean compareString__196(String recordValue__456, String operator__457, String clauseValue__458) {
        boolean return__158;
        if (operator__457.equals("==")) {
            return__158 = recordValue__456.equals(clauseValue__458);
        } else if (operator__457.equals("!=")) {
            return__158 = !recordValue__456.equals(clauseValue__458);
        } else if (operator__457.equals(">")) {
            return__158 = recordValue__456.compareTo(clauseValue__458) > 0;
        } else if (operator__457.equals("<")) {
            return__158 = recordValue__456.compareTo(clauseValue__458) < 0;
        } else if (operator__457.equals(">=")) {
            return__158 = recordValue__456.compareTo(clauseValue__458) >= 0;
        } else if (operator__457.equals("<=")) {
            return__158 = recordValue__456.compareTo(clauseValue__458) <= 0;
        } else {
            return__158 = false;
        }
        return return__158;
    }
    static SqlFragment safeSql__198(String trusted__462) {
        SqlBuilder b__464 = new SqlBuilder();
        b__464.appendSafe(trusted__462);
        return b__464.getAccumulated();
    }
    static SqlFragment columnListSql__199(List<String> selectFields__465) {
        SqlFragment return__161;
        SqlBuilder t_4009;
        String t_4012;
        SqlBuilder t_4014;
        SqlFragment t_4016;
        int t_4017;
        String t_4018;
        SqlBuilder t_4020;
        SqlFragment t_4024;
        if (selectFields__465.size() == 0) {
            t_4009 = new SqlBuilder();
            t_4009.appendSafe("*");
            return__161 = t_4009.getAccumulated();
        } else {
            t_4012 = Core.listGet(selectFields__465, 0);
            SqlFragment first__467 = OrmeryGlobal.safeSql__198(t_4012);
            t_4014 = new SqlBuilder();
            t_4014.appendFragment(first__467);
            t_4016 = t_4014.getAccumulated();
            SqlFragment result__468 = t_4016;
            int i__469 = 1;
            while (true) {
                t_4017 = selectFields__465.size();
                if (i__469 >= t_4017) {
                    break;
                }
                t_4018 = Core.listGet(selectFields__465, i__469);
                SqlFragment col__470 = OrmeryGlobal.safeSql__198(t_4018);
                t_4020 = new SqlBuilder();
                t_4020.appendFragment(result__468);
                t_4020.appendSafe(", ");
                t_4020.appendFragment(col__470);
                t_4024 = t_4020.getAccumulated();
                result__468 = t_4024;
                i__469 = i__469 + 1;
            }
            return__161 = result__468;
        }
        return return__161;
    }
    static String validOperator__197(String op__460) {
        String return__159;
        if (op__460.equals("=")) {
            return__159 = "=";
        } else if (op__460.equals("==")) {
            return__159 = "=";
        } else if (op__460.equals("!=")) {
            return__159 = "!=";
        } else if (op__460.equals("<>")) {
            return__159 = "<>";
        } else if (op__460.equals(">")) {
            return__159 = ">";
        } else if (op__460.equals("<")) {
            return__159 = "<";
        } else if (op__460.equals(">=")) {
            return__159 = ">=";
        } else if (op__460.equals("<=")) {
            return__159 = "<=";
        } else {
            return__159 = "=";
        }
        return return__159;
    }
    static SqlFragment whereConditionSql__200(WhereClause clause__471, Schema schema__472) {
        SqlFragment return__162;
        SqlBuilder t_3990;
        SqlBuilder t_3993;
        SqlBuilder t_4001;
        int t_2320;
        String t_3981 = clause__471.getField();
        SqlFragment col__474 = OrmeryGlobal.safeSql__198(t_3981);
        String t_3983 = clause__471.getOperator();
        SqlFragment op__475 = OrmeryGlobal.safeSql__198(OrmeryGlobal.validOperator__197(t_3983));
        String t_3985 = clause__471.getField();
        Field t_2316;
        t_2316 = schema__472.getField(t_3985);
        Field fieldInfo__476 = t_2316;
        if (fieldInfo__476.getFieldType().equals("Int")) {
            int intVal__477;
            int intVal_5876;
            try {
                t_2320 = Core.stringToInt(clause__471.getValue());
                intVal_5876 = t_2320;
            } catch (RuntimeException ignored$5) {
                intVal_5876 = 0;
            }
            intVal__477 = intVal_5876;
            if (!clause__471.getValue().equals(Integer.toString(intVal__477))) {
                t_3990 = new SqlBuilder();
                t_3990.appendSafe("1 = 0");
                return__162 = t_3990.getAccumulated();
            } else {
                t_3993 = new SqlBuilder();
                t_3993.appendFragment(col__474);
                t_3993.appendSafe(" ");
                t_3993.appendFragment(op__475);
                t_3993.appendSafe(" ");
                t_3993.appendInt32(intVal__477);
                return__162 = t_3993.getAccumulated();
            }
        } else {
            String strVal__478 = clause__471.getValue();
            t_4001 = new SqlBuilder();
            t_4001.appendFragment(col__474);
            t_4001.appendSafe(" ");
            t_4001.appendFragment(op__475);
            t_4001.appendSafe(" ");
            t_4001.appendString(strVal__478);
            return__162 = t_4001.getAccumulated();
        }
        return return__162;
    }
    static SqlFragment orderBySql__201(List<OrderClause> clauses__479) {
        SqlFragment t_3961;
        SqlFragment t_3962;
        int t_3967;
        String t_3969;
        SqlFragment t_3973;
        SqlFragment t_3974;
        SqlBuilder t_3975;
        SqlFragment t_3980;
        SqlFragment t_2303;
        String t_3957 = Core.listGet(clauses__479, 0).getField();
        SqlFragment first__481 = OrmeryGlobal.safeSql__198(t_3957);
        SqlFragment firstDir__482;
        if (Core.listGet(clauses__479, 0).getDirection().equals("desc")) {
            t_3961 = OrmeryGlobal.safeSql__198(" DESC");
            firstDir__482 = t_3961;
        } else {
            t_3962 = OrmeryGlobal.safeSql__198(" ASC");
            firstDir__482 = t_3962;
        }
        SqlBuilder t_3963 = new SqlBuilder();
        t_3963.appendFragment(first__481);
        t_3963.appendFragment(firstDir__482);
        SqlFragment t_3966 = t_3963.getAccumulated();
        SqlFragment result__483 = t_3966;
        int i__484 = 1;
        while (true) {
            t_3967 = clauses__479.size();
            if (i__484 >= t_3967) {
                break;
            }
            t_3969 = Core.listGet(clauses__479, i__484).getField();
            SqlFragment col__485 = OrmeryGlobal.safeSql__198(t_3969);
            if (Core.listGet(clauses__479, i__484).getDirection().equals("desc")) {
                t_3973 = OrmeryGlobal.safeSql__198(" DESC");
                t_2303 = t_3973;
            } else {
                t_3974 = OrmeryGlobal.safeSql__198(" ASC");
                t_2303 = t_3974;
            }
            SqlFragment dir__486 = t_2303;
            t_3975 = new SqlBuilder();
            t_3975.appendFragment(result__483);
            t_3975.appendSafe(", ");
            t_3975.appendFragment(col__485);
            t_3975.appendFragment(dir__486);
            t_3980 = t_3975.getAccumulated();
            result__483 = t_3980;
            i__484 = i__484 + 1;
        }
        return result__483;
    }
    public static SqlFragment toSqlQuery(Schema schema__487, List<String> selectFields__488, List<WhereClause> whereClauses__489, List<OrderClause> orderClauses__490, int limitValue__491, int offsetValue__492) {
        WhereClause t_3924;
        SqlFragment t_3925;
        int t_3926;
        WhereClause t_3927;
        SqlBuilder t_3929;
        SqlFragment t_3933;
        SqlBuilder t_3934;
        SqlFragment t_3938;
        SqlBuilder t_3941;
        SqlFragment t_3945;
        SqlBuilder t_3946;
        SqlFragment t_3950;
        SqlBuilder t_3951;
        SqlFragment t_3955;
        Predicate<String> fn__3907 = f__495 -> schema__487.hasField(f__495);
        List<String> validSelect__494 = Core.listFilterObj(selectFields__488, fn__3907);
        Predicate<WhereClause> fn__3906 = c__498 -> {
            String t_3902 = c__498.getField();
            return schema__487.hasField(t_3902);
        };
        List<WhereClause> validWhere__497 = Core.listFilterObj(whereClauses__489, fn__3906);
        Predicate<OrderClause> fn__3905 = c__501 -> {
            String t_3900 = c__501.getField();
            return schema__487.hasField(t_3900);
        };
        List<OrderClause> validOrder__500 = Core.listFilterObj(orderClauses__490, fn__3905);
        String t_3914 = schema__487.getTableName();
        SqlFragment table__503 = OrmeryGlobal.safeSql__198(t_3914);
        SqlFragment cols__504 = OrmeryGlobal.columnListSql__199(validSelect__494);
        SqlBuilder t_3917 = new SqlBuilder();
        t_3917.appendSafe("SELECT ");
        t_3917.appendFragment(cols__504);
        t_3917.appendSafe(" FROM ");
        t_3917.appendFragment(table__503);
        SqlFragment t_3922 = t_3917.getAccumulated();
        SqlFragment result__505 = t_3922;
        if (validWhere__497.size() > 0) {
            t_3924 = Core.listGet(validWhere__497, 0);
            t_3925 = OrmeryGlobal.whereConditionSql__200(t_3924, schema__487);
            SqlFragment conditions__506 = t_3925;
            int i__507 = 1;
            while (true) {
                t_3926 = validWhere__497.size();
                if (i__507 >= t_3926) {
                    break;
                }
                t_3927 = Core.listGet(validWhere__497, i__507);
                SqlFragment next__508 = OrmeryGlobal.whereConditionSql__200(t_3927, schema__487);
                t_3929 = new SqlBuilder();
                t_3929.appendFragment(conditions__506);
                t_3929.appendSafe(" AND ");
                t_3929.appendFragment(next__508);
                t_3933 = t_3929.getAccumulated();
                conditions__506 = t_3933;
                i__507 = i__507 + 1;
            }
            t_3934 = new SqlBuilder();
            t_3934.appendFragment(result__505);
            t_3934.appendSafe(" WHERE ");
            t_3934.appendFragment(conditions__506);
            t_3938 = t_3934.getAccumulated();
            result__505 = t_3938;
        }
        if (validOrder__500.size() > 0) {
            SqlFragment ordering__509 = OrmeryGlobal.orderBySql__201(validOrder__500);
            t_3941 = new SqlBuilder();
            t_3941.appendFragment(result__505);
            t_3941.appendSafe(" ORDER BY ");
            t_3941.appendFragment(ordering__509);
            t_3945 = t_3941.getAccumulated();
            result__505 = t_3945;
        }
        if (limitValue__491 >= 0) {
            t_3946 = new SqlBuilder();
            t_3946.appendFragment(result__505);
            t_3946.appendSafe(" LIMIT ");
            t_3946.appendInt32(limitValue__491);
            t_3950 = t_3946.getAccumulated();
            result__505 = t_3950;
        }
        if (offsetValue__492 > 0) {
            t_3951 = new SqlBuilder();
            t_3951.appendFragment(result__505);
            t_3951.appendSafe(" OFFSET ");
            t_3951.appendInt32(offsetValue__492);
            t_3955 = t_3951.getAccumulated();
            result__505 = t_3955;
        }
        return result__505;
    }
    public static Field field(String name__293, String fieldType__294, boolean primaryKey__295, boolean nullable__296) {
        return new Field(name__293, fieldType__294, primaryKey__295, nullable__296);
    }
    public static boolean isValidIdentifier(String name__298) {
        boolean return__123;
        int t_3781;
        fn__299: {
            if (name__298.isEmpty()) {
                return__123 = false;
                break fn__299;
            }
            String this__2531 = name__298;
            int index__2533 = 0;
            while (true) {
                if (!Core.stringHasIndex(this__2531, index__2533)) {
                    break;
                }
                int codePoint__2534 = this__2531.codePointAt(index__2533);
                int c__300 = codePoint__2534;
                if (c__300 != 95) {
                    if (c__300 >= 97) {
                        if (c__300 > 122) {
                            return__123 = false;
                            break fn__299;
                        }
                    } else if (c__300 >= 65) {
                        if (c__300 > 90) {
                            return__123 = false;
                            break fn__299;
                        }
                    } else if (c__300 >= 48) {
                        if (c__300 > 57) {
                            return__123 = false;
                            break fn__299;
                        }
                    } else {
                        return__123 = false;
                        break fn__299;
                    }
                }
                t_3781 = Core.stringNext(this__2531, index__2533);
                index__2533 = t_3781;
            }
            return__123 = true;
        }
        return return__123;
    }
    public static Schema schema(String tableName__301, List<Field> fields__302) {
        if (!OrmeryGlobal.isValidIdentifier(tableName__301)) {
            throw Core.bubble();
        }
        Field idField__304 = new Field("id", "Int", true, false);
        List<Field> allFields__305 = new ArrayList<>();
        Core.listAdd(allFields__305, idField__304);
        Core.listAddAll(allFields__305, fields__302);
        List<Field> t_3776 = List.copyOf(allFields__305);
        return new Schema(tableName__301, t_3776);
    }
    public static SqlFragment toInsertSql(Schema schema__510, Map<String, String> values__511) {
        SqlFragment return__165;
        String t_3724;
        List<Field> t_3726;
        SqlBuilder t_3730;
        List<String> t_3733;
        String t_3736;
        SqlBuilder t_3740;
        SqlFragment t_3742;
        SqlBuilder t_3743;
        SqlFragment t_3745;
        int t_3746;
        String t_3748;
        SqlBuilder t_3752;
        SqlFragment t_3756;
        SqlBuilder t_3757;
        SqlFragment t_3761;
        SqlBuilder t_3762;
        int t_2047;
        SqlFragment t_2058;
        int t_2065;
        int t_2066;
        fn__512: {
            t_3724 = schema__510.getTableName();
            SqlFragment table__513 = OrmeryGlobal.safeSql__198(t_3724);
            t_3726 = schema__510.getFields();
            Predicate<Field> fn__3723 = f__515 -> {
                String t_3720 = f__515.getName();
                return values__511.containsKey(t_3720);
            };
            List<Field> fieldList__514 = Core.listFilterObj(t_3726, fn__3723);
            if (fieldList__514.size() == 0) {
                t_3730 = new SqlBuilder();
                return__165 = t_3730.getAccumulated();
                break fn__512;
            }
            Function<Field, String> fn__3722 = f__518 -> f__518.getName();
            t_3733 = Core.listMapObjToObj(fieldList__514, fn__3722);
            SqlFragment colNames__517 = OrmeryGlobal.columnListSql__199(t_3733);
            t_3736 = Core.listGet(fieldList__514, 0).getName();
            String firstVal__520 = values__511.getOrDefault(t_3736, "");
            if (Core.listGet(fieldList__514, 0).getFieldType().equals("Int")) {
                int iv__522;
                int iv_5882;
                try {
                    t_2047 = Core.stringToInt(firstVal__520);
                    iv_5882 = t_2047;
                } catch (RuntimeException ignored$6) {
                    iv_5882 = 0;
                }
                iv__522 = iv_5882;
                t_3740 = new SqlBuilder();
                t_3740.appendInt32(iv__522);
                t_3742 = t_3740.getAccumulated();
                t_2058 = t_3742;
            } else {
                t_3743 = new SqlBuilder();
                t_3743.appendString(firstVal__520);
                t_3745 = t_3743.getAccumulated();
                t_2058 = t_3745;
            }
            SqlFragment vals__521 = t_2058;
            int i__523 = 1;
            while (true) {
                t_3746 = fieldList__514.size();
                if (i__523 >= t_3746) {
                    break;
                }
                t_3748 = Core.listGet(fieldList__514, i__523).getName();
                String val__524 = values__511.getOrDefault(t_3748, "");
                if (Core.listGet(fieldList__514, i__523).getFieldType().equals("Int")) {
                    try {
                        t_2065 = Core.stringToInt(val__524);
                        t_2066 = t_2065;
                    } catch (RuntimeException ignored$7) {
                        t_2066 = 0;
                    }
                    int iv__525 = t_2066;
                    t_3752 = new SqlBuilder();
                    t_3752.appendFragment(vals__521);
                    t_3752.appendSafe(", ");
                    t_3752.appendInt32(iv__525);
                    t_3756 = t_3752.getAccumulated();
                    vals__521 = t_3756;
                } else {
                    t_3757 = new SqlBuilder();
                    t_3757.appendFragment(vals__521);
                    t_3757.appendSafe(", ");
                    t_3757.appendString(val__524);
                    t_3761 = t_3757.getAccumulated();
                    vals__521 = t_3761;
                }
                i__523 = i__523 + 1;
            }
            t_3762 = new SqlBuilder();
            t_3762.appendSafe("INSERT INTO ");
            t_3762.appendFragment(table__513);
            t_3762.appendSafe(" (");
            t_3762.appendFragment(colNames__517);
            t_3762.appendSafe(") VALUES (");
            t_3762.appendFragment(vals__521);
            t_3762.appendSafe(")");
            return__165 = t_3762.getAccumulated();
        }
        return return__165;
    }
    public static void main() {
        console_737.log("=== ORMery Demo ===\n");
        List<Field> userFields__642 = List.of(OrmeryGlobal.field("name", "String", false, false), OrmeryGlobal.field("age", "Int", false, false), OrmeryGlobal.field("email", "String", false, true));
        Schema userSchema__643 = OrmeryGlobal.schema("users", userFields__642);
        String t_3304 = userSchema__643.describe();
        console_737.log(t_3304);
        console_737.log("");
        InMemoryStore store__644 = new InMemoryStore();
        Record rec1__645 = store__644.insert("users", Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"), new SimpleImmutableEntry<>("age", "25"), new SimpleImmutableEntry<>("email", "alice@example.com"))));
        Record rec2__646 = store__644.insert("users", Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Bob"), new SimpleImmutableEntry<>("age", "30"), new SimpleImmutableEntry<>("email", "bob@example.com"))));
        Record rec3__647 = store__644.insert("users", Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Charlie"), new SimpleImmutableEntry<>("age", "17"), new SimpleImmutableEntry<>("email", "charlie@example.com"))));
        console_737.log("Inserted 3 users:");
        String t_3324 = rec1__645.describe();
        console_737.log("  " + t_3324);
        String t_3326 = rec2__646.describe();
        console_737.log("  " + t_3326);
        String t_3328 = rec3__647.describe();
        console_737.log("  " + t_3328);
        console_737.log("");
        console_737.log("=== In-Memory Queries ===\n");
        console_737.log("All users:");
        List<Record> allUsers__648 = new Query(userSchema__643, store__644).all();
        Consumer<Record> fn__3301 = u__649 -> {
            String t_3298 = u__649.describe();
            console_737.log("  " + t_3298);
        };
        allUsers__648.forEach(fn__3301);
        console_737.log("");
        console_737.log("Adults (age >= 18):");
        List<Record> adults__650 = new Query(userSchema__643, store__644).where("age", ">=", "18").all();
        Consumer<Record> fn__3300 = u__651 -> {
            String t_3296 = u__651.describe();
            console_737.log("  " + t_3296);
        };
        adults__650.forEach(fn__3300);
        console_737.log("");
        console_737.log("=== SQL Generation (secure-composition) ===\n");
        Query q1__652 = new Query(userSchema__643, store__644);
        String t_3348 = q1__652.toSql().toString();
        console_737.log("SELECT all: " + t_3348);
        Query q2__653 = new Query(userSchema__643, store__644).select(List.of("name", "age")).where("age", ">=", "18").orderBy("age", "desc").limit(10);
        String t_3356 = q2__653.toSql().toString();
        console_737.log("Complex:    " + t_3356);
        String bobby__654 = "Robert'); DROP TABLE users;--";
        Query q3__655 = new Query(userSchema__643, store__644).where("name", "=", "Robert'); DROP TABLE users;--");
        String t_3361 = q3__655.toSql().toString();
        console_737.log("Injection:  " + t_3361);
        Map<String, String> insertVals__656 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "O'Malley"), new SimpleImmutableEntry<>("age", "42")));
        String t_3367 = OrmeryGlobal.toInsertSql(userSchema__643, insertVals__656).toString();
        console_737.log("INSERT:     " + t_3367);
        console_737.log("\n=== Demo Complete ===");
    }
    public static TokenType classifyWord(String word__685) {
        TokenType return__187;
        fn__686: {
            List<String> this__2536 = temperKeywords__204;
            int n__2537 = this__2536.size();
            int i__2538 = 0;
            while (i__2538 < n__2537) {
                String el__2539 = Core.listGet(this__2536, i__2538);
                i__2538 = i__2538 + 1;
                String kw__687 = el__2539;
                if (kw__687.equals(word__685)) {
                    return__187 = new TokenType("keyword");
                    break fn__686;
                }
            }
            List<String> this__2541 = temperTypes__205;
            int n__2542 = this__2541.size();
            int i__2543 = 0;
            while (i__2543 < n__2542) {
                String el__2544 = Core.listGet(this__2541, i__2543);
                i__2543 = i__2543 + 1;
                String tp__688 = el__2544;
                if (tp__688.equals(word__685)) {
                    return__187 = new TokenType("type");
                    break fn__686;
                }
            }
            return__187 = new TokenType("identifier");
        }
        return return__187;
    }
    public static SafeHtml highlightWord(String word__689) {
        SafeHtml return__188;
        SafeHtmlBuilder t_3275;
        fn__690: {
            if (word__689.equals("")) {
                t_3275 = new SafeHtmlBuilder();
                return__188 = t_3275.getAccumulated();
                break fn__690;
            }
            TokenType tokenType__691 = OrmeryGlobal.classifyWord(word__689);
            Token token__692 = new Token(tokenType__691, word__689);
            return__188 = token__692.toHtml();
        }
        return return__188;
    }
    public static SafeHtml highlightLine(String line__693) {
        SafeHtml return__189;
        SafeHtmlBuilder t_3263;
        SafeHtml t_3266;
        int t_3267;
        SafeHtmlBuilder t_3270;
        SafeHtml t_3274;
        fn__694: {
            List<String> words__695 = Core.stringSplit(line__693, " ");
            if (words__695.size() == 0) {
                t_3263 = new SafeHtmlBuilder();
                return__189 = t_3263.getAccumulated();
                break fn__694;
            }
            t_3266 = OrmeryGlobal.highlightWord(Core.listGet(words__695, 0));
            SafeHtml result__696 = t_3266;
            int i__697 = 1;
            while (true) {
                t_3267 = words__695.size();
                if (i__697 >= t_3267) {
                    break;
                }
                SafeHtml word__698 = OrmeryGlobal.highlightWord(Core.listGet(words__695, i__697));
                t_3270 = new SafeHtmlBuilder();
                t_3270.appendSafeHtml(result__696);
                t_3270.appendSafe(" ");
                t_3270.appendSafeHtml(word__698);
                t_3274 = t_3270.getAccumulated();
                result__696 = t_3274;
                i__697 = i__697 + 1;
            }
            return__189 = result__696;
        }
        return return__189;
    }
    public static SafeHtml highlightSource(String source__699) {
        SafeHtml return__190;
        SafeHtmlBuilder t_3249;
        SafeHtml t_3252;
        int t_3253;
        SafeHtmlBuilder t_3256;
        SafeHtml t_3260;
        fn__700: {
            List<String> lines__701 = Core.stringSplit(source__699, "\n");
            if (lines__701.size() == 0) {
                t_3249 = new SafeHtmlBuilder();
                return__190 = t_3249.getAccumulated();
                break fn__700;
            }
            t_3252 = OrmeryGlobal.highlightLine(Core.listGet(lines__701, 0));
            SafeHtml result__702 = t_3252;
            int i__703 = 1;
            while (true) {
                t_3253 = lines__701.size();
                if (i__703 >= t_3253) {
                    break;
                }
                SafeHtml line__704 = OrmeryGlobal.highlightLine(Core.listGet(lines__701, i__703));
                t_3256 = new SafeHtmlBuilder();
                t_3256.appendSafeHtml(result__702);
                t_3256.appendSafe("\\n");
                t_3256.appendSafeHtml(line__704);
                t_3260 = t_3256.getAccumulated();
                result__702 = t_3260;
                i__703 = i__703 + 1;
            }
            return__190 = result__702;
        }
        return return__190;
    }
    public static SafeHtml highlightBlock(String source__705) {
        SafeHtml highlighted__707 = OrmeryGlobal.highlightSource(source__705);
        SafeHtmlBuilder t_3242 = new SafeHtmlBuilder();
        t_3242.appendSafe("<pre class='temper-code'><code>");
        t_3242.appendSafeHtml(highlighted__707);
        t_3242.appendSafe("</code></pre>");
        return t_3242.getAccumulated();
    }
    static {
        console_737 = Core.getConsole(Logger.getLogger("ormery"));
        temperKeywords__204 = List.of("if", "else", "for", "while", "do", "when", "break", "continue", "return", "let", "var", "class", "export", "import", "public", "private", "protected", "throws", "new", "this", "get", "set", "static", "extends", "implements", "true", "false", "null", "bubble", "orelse", "of");
        temperTypes__205 = List.of("String", "Int", "Boolean", "List", "Map", "Bubble", "Pair", "Float", "Double", "Byte", "Short", "Long", "Char", "Void", "Record", "Schema", "Field", "Query", "InMemoryStore", "ListBuilder", "MapBuilder", "WhereClause", "OrderClause");
    }
}
