package ormery;
import temper.std.testing.Test;
import java.util.function.Supplier;
import java.util.List;
import java.util.AbstractMap.SimpleImmutableEntry;
import temper.core.Core;
import ormery.sql.SqlFragment;
import java.util.Map;
public final class OrmeryTest {
    private OrmeryTest() {
    }
    @org.junit.jupiter.api.Test public void toSqlSelectAll__830() {
        Test test_17 = new Test();
        try {
            Schema s__527 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__528 = new InMemoryStore();
            Query q__529 = new Query(s__527, store__528);
            String actual_831 = q__529.toSql().toString();
            boolean t_3717 = actual_831.equals("SELECT * FROM users");
            Supplier<String> fn__3710 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual_831 + ")";
            test_17.assert_(t_3717, fn__3710);
        } finally {
            test_17.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlSelectColumns__833() {
        Test test_18 = new Test();
        try {
            Schema s__531 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false), OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__532 = new InMemoryStore();
            Query q__533 = new Query(s__531, store__532).select(List.of("name", "age"));
            String actual_834 = q__533.toSql().toString();
            boolean t_3708 = actual_834.equals("SELECT name, age FROM users");
            Supplier<String> fn__3700 = () -> "expected q.toSql().toString() == (" + "SELECT name, age FROM users" + ") not (" + actual_834 + ")";
            test_18.assert_(t_3708, fn__3700);
        } finally {
            test_18.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlWhereString__836() {
        Test test_19 = new Test();
        try {
            Schema s__535 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__536 = new InMemoryStore();
            Query q__537 = new Query(s__535, store__536).where("name", "=", "Alice");
            String actual_837 = q__537.toSql().toString();
            boolean t_3698 = actual_837.equals("SELECT * FROM users WHERE name = 'Alice'");
            Supplier<String> fn__3690 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual_837 + ")";
            test_19.assert_(t_3698, fn__3690);
        } finally {
            test_19.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlWhereInt__839() {
        Test test_20 = new Test();
        try {
            Schema s__539 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__540 = new InMemoryStore();
            Query q__541 = new Query(s__539, store__540).where("age", ">=", "18");
            String actual_840 = q__541.toSql().toString();
            boolean t_3688 = actual_840.equals("SELECT * FROM users WHERE age >= 18");
            Supplier<String> fn__3680 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE age >= 18" + ") not (" + actual_840 + ")";
            test_20.assert_(t_3688, fn__3680);
        } finally {
            test_20.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlSqlInjectionBlocked__842() {
        Test test_21 = new Test();
        try {
            Schema s__543 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__544 = new InMemoryStore();
            String bobby__545 = "Robert'); DROP TABLE users;--";
            Query q__546 = new Query(s__543, store__544).where("name", "=", "Robert'); DROP TABLE users;--");
            String result__547 = q__546.toSql().toString();
            String actual_843 = result__547;
            boolean t_3678 = actual_843.equals("SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'");
            Supplier<String> fn__3670 = () -> "expected result == (" + "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'" + ") not (" + actual_843 + ")";
            test_21.assert_(t_3678, fn__3670);
        } finally {
            test_21.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlOperatorNormalization__845() {
        Test test_22 = new Test();
        try {
            Schema s__549 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__550 = new InMemoryStore();
            Query q__551 = new Query(s__549, store__550).where("name", "==", "Alice");
            String actual_846 = q__551.toSql().toString();
            boolean t_3668 = actual_846.equals("SELECT * FROM users WHERE name = 'Alice'");
            Supplier<String> fn__3660 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual_846 + ")";
            test_22.assert_(t_3668, fn__3660);
        } finally {
            test_22.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlInvalidOperatorFallback__848() {
        Test test_23 = new Test();
        try {
            Schema s__553 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__554 = new InMemoryStore();
            Query q__555 = new Query(s__553, store__554).where("name", "LIKE", "Alice");
            String actual_849 = q__555.toSql().toString();
            boolean t_3658 = actual_849.equals("SELECT * FROM users WHERE name = 'Alice'");
            Supplier<String> fn__3650 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual_849 + ")";
            test_23.assert_(t_3658, fn__3650);
        } finally {
            test_23.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlMultipleWhere__851() {
        Test test_24 = new Test();
        try {
            Schema s__557 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__558 = new InMemoryStore();
            Query q__559 = new Query(s__557, store__558).where("age", ">=", "18").where("age", "<", "30");
            String actual_852 = q__559.toSql().toString();
            boolean t_3648 = actual_852.equals("SELECT * FROM users WHERE age >= 18 AND age < 30");
            Supplier<String> fn__3639 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE age >= 18 AND age < 30" + ") not (" + actual_852 + ")";
            test_24.assert_(t_3648, fn__3639);
        } finally {
            test_24.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlOrderBy__854() {
        Test test_25 = new Test();
        try {
            Schema s__561 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__562 = new InMemoryStore();
            Query q__563 = new Query(s__561, store__562).orderBy("name", "asc");
            String actual_855 = q__563.toSql().toString();
            boolean t_3637 = actual_855.equals("SELECT * FROM users ORDER BY name ASC");
            Supplier<String> fn__3629 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users ORDER BY name ASC" + ") not (" + actual_855 + ")";
            test_25.assert_(t_3637, fn__3629);
        } finally {
            test_25.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlOrderByDesc__857() {
        Test test_26 = new Test();
        try {
            Schema s__565 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__566 = new InMemoryStore();
            Query q__567 = new Query(s__565, store__566).orderBy("age", "desc");
            String actual_858 = q__567.toSql().toString();
            boolean t_3627 = actual_858.equals("SELECT * FROM users ORDER BY age DESC");
            Supplier<String> fn__3619 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users ORDER BY age DESC" + ") not (" + actual_858 + ")";
            test_26.assert_(t_3627, fn__3619);
        } finally {
            test_26.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlLimit__860() {
        Test test_27 = new Test();
        try {
            Schema s__569 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__570 = new InMemoryStore();
            Query q__571 = new Query(s__569, store__570).limit(10);
            String actual_861 = q__571.toSql().toString();
            boolean t_3617 = actual_861.equals("SELECT * FROM users LIMIT 10");
            Supplier<String> fn__3609 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users LIMIT 10" + ") not (" + actual_861 + ")";
            test_27.assert_(t_3617, fn__3609);
        } finally {
            test_27.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlOffset__863() {
        Test test_28 = new Test();
        try {
            Schema s__573 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__574 = new InMemoryStore();
            Query q__575 = new Query(s__573, store__574).offset(5);
            String actual_864 = q__575.toSql().toString();
            boolean t_3607 = actual_864.equals("SELECT * FROM users OFFSET 5");
            Supplier<String> fn__3599 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users OFFSET 5" + ") not (" + actual_864 + ")";
            test_28.assert_(t_3607, fn__3599);
        } finally {
            test_28.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlComplexQuery__866() {
        Test test_29 = new Test();
        try {
            Schema s__577 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false), OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__578 = new InMemoryStore();
            Query q__579 = new Query(s__577, store__578).select(List.of("name", "age")).where("age", ">=", "18").orderBy("age", "desc").limit(10).offset(20);
            String actual_867 = q__579.toSql().toString();
            boolean t_3597 = actual_867.equals("SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20");
            Supplier<String> fn__3585 = () -> "expected q.toSql().toString() == (" + "SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20" + ") not (" + actual_867 + ")";
            test_29.assert_(t_3597, fn__3585);
        } finally {
            test_29.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlUnicodeEscaping__869() {
        Test test_30 = new Test();
        try {
            Schema s__581 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__582 = new InMemoryStore();
            Query q__583 = new Query(s__581, store__582).where("name", "=", "Hello \u4e16\u754c");
            String actual_870 = q__583.toSql().toString();
            boolean t_3583 = actual_870.equals("SELECT * FROM users WHERE name = 'Hello \u4e16\u754c'");
            Supplier<String> fn__3575 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Hello \u4e16\u754c'" + ") not (" + actual_870 + ")";
            test_30.assert_(t_3583, fn__3575);
        } finally {
            test_30.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlEmbeddedQuotes__872() {
        Test test_31 = new Test();
        try {
            Schema s__585 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__586 = new InMemoryStore();
            Query q__587 = new Query(s__585, store__586).where("name", "=", "O'Brien");
            String actual_873 = q__587.toSql().toString();
            boolean t_3573 = actual_873.equals("SELECT * FROM users WHERE name = 'O''Brien'");
            Supplier<String> fn__3565 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'O''Brien'" + ") not (" + actual_873 + ")";
            test_31.assert_(t_3573, fn__3565);
        } finally {
            test_31.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlEmptyString__875() {
        Test test_32 = new Test();
        try {
            Schema s__589 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__590 = new InMemoryStore();
            Query q__591 = new Query(s__589, store__590).where("name", "=", "");
            String actual_876 = q__591.toSql().toString();
            boolean t_3563 = actual_876.equals("SELECT * FROM users WHERE name = ''");
            Supplier<String> fn__3555 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = ''" + ") not (" + actual_876 + ")";
            test_32.assert_(t_3563, fn__3555);
        } finally {
            test_32.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlBasicInsert__878() {
        Test test_33 = new Test();
        try {
            Schema s__593 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false), OrmeryGlobal.field("age", "Int", false, false)));
            Map<String, String> vals__594 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"), new SimpleImmutableEntry<>("age", "25")));
            SqlFragment result__595 = OrmeryGlobal.toInsertSql(s__593, vals__594);
            String actual_879 = result__595.toString();
            boolean t_3553 = actual_879.equals("INSERT INTO users (name, age) VALUES ('Alice', 25)");
            Supplier<String> fn__3545 = () -> "expected result.toString() == (" + "INSERT INTO users (name, age) VALUES ('Alice', 25)" + ") not (" + actual_879 + ")";
            test_33.assert_(t_3553, fn__3545);
        } finally {
            test_33.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlInjectionBlocked__881() {
        Test test_34 = new Test();
        try {
            Schema s__597 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            Map<String, String> vals__598 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Robert'); DROP TABLE users;--")));
            SqlFragment result__599 = OrmeryGlobal.toInsertSql(s__597, vals__598);
            String actual_882 = result__599.toString();
            boolean t_3543 = actual_882.equals("INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')");
            Supplier<String> fn__3536 = () -> "expected result.toString() == (" + "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')" + ") not (" + actual_882 + ")";
            test_34.assert_(t_3543, fn__3536);
        } finally {
            test_34.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlQueryStandalone__884() {
        Test test_35 = new Test();
        try {
            Schema s__601 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false), OrmeryGlobal.field("age", "Int", false, false)));
            SqlFragment result__602 = OrmeryGlobal.toSqlQuery(s__601, List.of("name"), List.of(new WhereClause("age", ">", "21")), List.of(new OrderClause("name", "asc")), 5, 0);
            String actual_885 = result__602.toString();
            boolean t_3534 = actual_885.equals("SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5");
            Supplier<String> fn__3525 = () -> "expected result.toString() == (" + "SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5" + ") not (" + actual_885 + ")";
            test_35.assert_(t_3534, fn__3525);
        } finally {
            test_35.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlAdversarialFieldNameBlocked__887() {
        Test test_36 = new Test();
        try {
            Schema s__604 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__605 = new InMemoryStore();
            Query q__606 = new Query(s__604, store__605).where("1=1; DROP TABLE users; --", "=", "Alice");
            String actual_888 = q__606.toSql().toString();
            boolean t_3523 = actual_888.equals("SELECT * FROM users");
            Supplier<String> fn__3515 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual_888 + ")";
            test_36.assert_(t_3523, fn__3515);
        } finally {
            test_36.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlAdversarialSelectColumnBlocked__890() {
        Test test_37 = new Test();
        try {
            Schema s__608 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__609 = new InMemoryStore();
            Query q__610 = new Query(s__608, store__609).select(List.of("name", "1; DROP TABLE users"));
            String actual_891 = q__610.toSql().toString();
            boolean t_3513 = actual_891.equals("SELECT name FROM users");
            Supplier<String> fn__3505 = () -> "expected q.toSql().toString() == (" + "SELECT name FROM users" + ") not (" + actual_891 + ")";
            test_37.assert_(t_3513, fn__3505);
        } finally {
            test_37.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlAdversarialOrderByBlocked__893() {
        Test test_38 = new Test();
        try {
            Schema s__612 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__613 = new InMemoryStore();
            Query q__614 = new Query(s__612, store__613).orderBy("1; DROP TABLE users", "asc");
            String actual_894 = q__614.toSql().toString();
            boolean t_3503 = actual_894.equals("SELECT * FROM users");
            Supplier<String> fn__3495 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual_894 + ")";
            test_38.assert_(t_3503, fn__3495);
        } finally {
            test_38.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void isValidIdentifierValidNames__896() {
        Test test_39 = new Test();
        try {
            boolean t_3480 = OrmeryGlobal.isValidIdentifier("users");
            Supplier<String> fn__3479 = () -> "expected `-work/src//`.isValidIdentifier(\"users\")";
            test_39.assert_(t_3480, fn__3479);
            boolean t_3483 = OrmeryGlobal.isValidIdentifier("user_table");
            Supplier<String> fn__3478 = () -> "expected `-work/src//`.isValidIdentifier(\"user_table\")";
            test_39.assert_(t_3483, fn__3478);
            boolean t_3486 = OrmeryGlobal.isValidIdentifier("Table1");
            Supplier<String> fn__3477 = () -> "expected `-work/src//`.isValidIdentifier(\"Table1\")";
            test_39.assert_(t_3486, fn__3477);
            boolean t_3489 = OrmeryGlobal.isValidIdentifier("_private");
            Supplier<String> fn__3476 = () -> "expected `-work/src//`.isValidIdentifier(\"_private\")";
            test_39.assert_(t_3489, fn__3476);
            boolean t_3492 = OrmeryGlobal.isValidIdentifier("a");
            Supplier<String> fn__3475 = () -> "expected `-work/src//`.isValidIdentifier(\"a\")";
            test_39.assert_(t_3492, fn__3475);
        } finally {
            test_39.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void isValidIdentifierInvalidNames__897() {
        Test test_40 = new Test();
        try {
            boolean t_3453 = !OrmeryGlobal.isValidIdentifier("");
            Supplier<String> fn__3450 = () -> "expected !`-work/src//`.isValidIdentifier(\"\")";
            test_40.assert_(t_3453, fn__3450);
            boolean t_3457 = !OrmeryGlobal.isValidIdentifier("users; DROP TABLE");
            Supplier<String> fn__3449 = () -> "expected !`-work/src//`.isValidIdentifier(\"users; DROP TABLE\")";
            test_40.assert_(t_3457, fn__3449);
            boolean t_3461 = !OrmeryGlobal.isValidIdentifier("users--");
            Supplier<String> fn__3448 = () -> "expected !`-work/src//`.isValidIdentifier(\"users--\")";
            test_40.assert_(t_3461, fn__3448);
            boolean t_3465 = !OrmeryGlobal.isValidIdentifier("ta ble");
            Supplier<String> fn__3447 = () -> "expected !`-work/src//`.isValidIdentifier(\"ta ble\")";
            test_40.assert_(t_3465, fn__3447);
            boolean t_3469 = !OrmeryGlobal.isValidIdentifier("table.name");
            Supplier<String> fn__3446 = () -> "expected !`-work/src//`.isValidIdentifier(\"table.name\")";
            test_40.assert_(t_3469, fn__3446);
            boolean t_3473 = !OrmeryGlobal.isValidIdentifier("Robert'); DROP TABLE users;--");
            Supplier<String> fn__3445 = () -> "expected !`-work/src//`.isValidIdentifier(\"Robert'); DROP TABLE users;--\")";
            test_40.assert_(t_3473, fn__3445);
        } finally {
            test_40.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlNonNumericIntValueProducesAlwaysFalse__898() {
        Test test_41 = new Test();
        try {
            Schema s__618 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__619 = new InMemoryStore();
            Query q__620 = new Query(s__618, store__619).where("age", "=", "admin");
            String actual_899 = q__620.toSql().toString();
            boolean t_3443 = actual_899.equals("SELECT * FROM users WHERE 1 = 0");
            Supplier<String> fn__3435 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE 1 = 0" + ") not (" + actual_899 + ")";
            test_41.assert_(t_3443, fn__3435);
        } finally {
            test_41.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void inMemoryNonNumericIntValueMatchesNothing__901() {
        Test test_42 = new Test();
        try {
            Schema s__622 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false), OrmeryGlobal.field("age", "Int", false, false)));
            InMemoryStore store__623 = new InMemoryStore();
            store__623.insert("users", Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"), new SimpleImmutableEntry<>("age", "0"))));
            List<Record> results__624 = new Query(s__622, store__623).where("age", "=", "admin").all();
            int actual_902 = results__624.size();
            boolean t_3433 = actual_902 == 0;
            Supplier<String> fn__3421 = () -> "expected results.length == (" + Integer.toString(0) + ") not (" + Integer.toString(actual_902) + ")";
            test_42.assert_(t_3433, fn__3421);
        } finally {
            test_42.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toSqlLimitZeroEmitsLimit0__904() {
        Test test_43 = new Test();
        try {
            Schema s__626 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__627 = new InMemoryStore();
            Query q__628 = new Query(s__626, store__627).limit(0);
            String actual_905 = q__628.toSql().toString();
            boolean t_3417 = actual_905.equals("SELECT * FROM users LIMIT 0");
            Supplier<String> fn__3409 = () -> "expected q.toSql().toString() == (" + "SELECT * FROM users LIMIT 0" + ") not (" + actual_905 + ")";
            test_43.assert_(t_3417, fn__3409);
        } finally {
            test_43.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void inMemoryLimitZeroReturnsEmpty__907() {
        Test test_44 = new Test();
        try {
            Schema s__630 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__631 = new InMemoryStore();
            store__631.insert("users", Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"))));
            List<Record> results__632 = new Query(s__630, store__631).limit(0).all();
            int actual_908 = results__632.size();
            boolean t_3407 = actual_908 == 0;
            Supplier<String> fn__3396 = () -> "expected results.length == (" + Integer.toString(0) + ") not (" + Integer.toString(actual_908) + ")";
            test_44.assert_(t_3407, fn__3396);
        } finally {
            test_44.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void inMemoryNegativeLimitClampedToZero__910() {
        Test test_45 = new Test();
        try {
            Schema s__634 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            InMemoryStore store__635 = new InMemoryStore();
            store__635.insert("users", Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"))));
            List<Record> results__636 = new Query(s__634, store__635).limit(-5).all();
            int actual_911 = results__636.size();
            boolean t_3392 = actual_911 == 0;
            Supplier<String> fn__3381 = () -> "expected results.length == (" + Integer.toString(0) + ") not (" + Integer.toString(actual_911) + ")";
            test_45.assert_(t_3392, fn__3381);
        } finally {
            test_45.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlNoMatchingFieldsReturnsEmpty__913() {
        Test test_46 = new Test();
        try {
            Schema s__638 = OrmeryGlobal.schema("users", List.of(OrmeryGlobal.field("name", "String", false, false)));
            Map<String, String> vals__639 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("nonexistent", "value")));
            SqlFragment result__640 = OrmeryGlobal.toInsertSql(s__638, vals__639);
            String actual_914 = result__640.toString();
            boolean t_3377 = actual_914.equals("");
            Supplier<String> fn__3370 = () -> "expected result.toString() == (" + "" + ") not (" + actual_914 + ")";
            test_46.assert_(t_3377, fn__3370);
        } finally {
            test_46.softFailToHard();
        }
    }
}
