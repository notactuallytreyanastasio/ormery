using U = Microsoft.VisualStudio.TestTools.UnitTesting;
using O = Ormery;
using S0 = Ormery.Sql;
using S1 = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
using T = TemperLang.Std.Testing;
namespace Ormery
{
    [U::TestClass]
    public class OrmeryTests
    {
        [U::TestMethod]
        public void toSqlSelectAll__830()
        {
            T::Test test___17 = new T::Test();
            try
            {
                Schema s__527 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__528 = new InMemoryStore();
                Query q__529 = new Query(s__527, store__528);
                string actual___831 = q__529.ToSql().ToString();
                bool t___3717 = actual___831 == "SELECT * FROM users";
                string fn__3710()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual___831 + ")";
                }
                test___17.Assert(t___3717, (S1::Func<string>) fn__3710);
            }
            finally
            {
                test___17.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlSelectColumns__833()
        {
            T::Test test___18 = new T::Test();
            try
            {
                Schema s__531 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false), O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__532 = new InMemoryStore();
                Query q__533 = new Query(s__531, store__532).Select(C::Listed.CreateReadOnlyList<string>("name", "age"));
                string actual___834 = q__533.ToSql().ToString();
                bool t___3708 = actual___834 == "SELECT name, age FROM users";
                string fn__3700()
                {
                    return "expected q.toSql().toString() == (" + "SELECT name, age FROM users" + ") not (" + actual___834 + ")";
                }
                test___18.Assert(t___3708, (S1::Func<string>) fn__3700);
            }
            finally
            {
                test___18.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlWhereString__836()
        {
            T::Test test___19 = new T::Test();
            try
            {
                Schema s__535 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__536 = new InMemoryStore();
                Query q__537 = new Query(s__535, store__536).Where("name", "=", "Alice");
                string actual___837 = q__537.ToSql().ToString();
                bool t___3698 = actual___837 == "SELECT * FROM users WHERE name = 'Alice'";
                string fn__3690()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual___837 + ")";
                }
                test___19.Assert(t___3698, (S1::Func<string>) fn__3690);
            }
            finally
            {
                test___19.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlWhereInt__839()
        {
            T::Test test___20 = new T::Test();
            try
            {
                Schema s__539 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__540 = new InMemoryStore();
                Query q__541 = new Query(s__539, store__540).Where("age", ">=", "18");
                string actual___840 = q__541.ToSql().ToString();
                bool t___3688 = actual___840 == "SELECT * FROM users WHERE age >= 18";
                string fn__3680()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE age >= 18" + ") not (" + actual___840 + ")";
                }
                test___20.Assert(t___3688, (S1::Func<string>) fn__3680);
            }
            finally
            {
                test___20.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlSqlInjectionBlocked__842()
        {
            T::Test test___21 = new T::Test();
            try
            {
                Schema s__543 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__544 = new InMemoryStore();
                string bobby__545 = "Robert'); DROP TABLE users;--";
                Query q__546 = new Query(s__543, store__544).Where("name", "=", "Robert'); DROP TABLE users;--");
                string result__547 = q__546.ToSql().ToString();
                string actual___843 = result__547;
                bool t___3678 = actual___843 == "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'";
                string fn__3670()
                {
                    return "expected result == (" + "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'" + ") not (" + actual___843 + ")";
                }
                test___21.Assert(t___3678, (S1::Func<string>) fn__3670);
            }
            finally
            {
                test___21.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlOperatorNormalization__845()
        {
            T::Test test___22 = new T::Test();
            try
            {
                Schema s__549 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__550 = new InMemoryStore();
                Query q__551 = new Query(s__549, store__550).Where("name", "==", "Alice");
                string actual___846 = q__551.ToSql().ToString();
                bool t___3668 = actual___846 == "SELECT * FROM users WHERE name = 'Alice'";
                string fn__3660()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual___846 + ")";
                }
                test___22.Assert(t___3668, (S1::Func<string>) fn__3660);
            }
            finally
            {
                test___22.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlInvalidOperatorFallback__848()
        {
            T::Test test___23 = new T::Test();
            try
            {
                Schema s__553 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__554 = new InMemoryStore();
                Query q__555 = new Query(s__553, store__554).Where("name", "LIKE", "Alice");
                string actual___849 = q__555.ToSql().ToString();
                bool t___3658 = actual___849 == "SELECT * FROM users WHERE name = 'Alice'";
                string fn__3650()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Alice'" + ") not (" + actual___849 + ")";
                }
                test___23.Assert(t___3658, (S1::Func<string>) fn__3650);
            }
            finally
            {
                test___23.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlMultipleWhere__851()
        {
            T::Test test___24 = new T::Test();
            try
            {
                Schema s__557 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__558 = new InMemoryStore();
                Query q__559 = new Query(s__557, store__558).Where("age", ">=", "18").Where("age", "<", "30");
                string actual___852 = q__559.ToSql().ToString();
                bool t___3648 = actual___852 == "SELECT * FROM users WHERE age >= 18 AND age < 30";
                string fn__3639()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE age >= 18 AND age < 30" + ") not (" + actual___852 + ")";
                }
                test___24.Assert(t___3648, (S1::Func<string>) fn__3639);
            }
            finally
            {
                test___24.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlOrderBy__854()
        {
            T::Test test___25 = new T::Test();
            try
            {
                Schema s__561 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__562 = new InMemoryStore();
                Query q__563 = new Query(s__561, store__562).OrderBy("name", "asc");
                string actual___855 = q__563.ToSql().ToString();
                bool t___3637 = actual___855 == "SELECT * FROM users ORDER BY name ASC";
                string fn__3629()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users ORDER BY name ASC" + ") not (" + actual___855 + ")";
                }
                test___25.Assert(t___3637, (S1::Func<string>) fn__3629);
            }
            finally
            {
                test___25.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlOrderByDesc__857()
        {
            T::Test test___26 = new T::Test();
            try
            {
                Schema s__565 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__566 = new InMemoryStore();
                Query q__567 = new Query(s__565, store__566).OrderBy("age", "desc");
                string actual___858 = q__567.ToSql().ToString();
                bool t___3627 = actual___858 == "SELECT * FROM users ORDER BY age DESC";
                string fn__3619()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users ORDER BY age DESC" + ") not (" + actual___858 + ")";
                }
                test___26.Assert(t___3627, (S1::Func<string>) fn__3619);
            }
            finally
            {
                test___26.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlLimit__860()
        {
            T::Test test___27 = new T::Test();
            try
            {
                Schema s__569 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__570 = new InMemoryStore();
                Query q__571 = new Query(s__569, store__570).Limit(10);
                string actual___861 = q__571.ToSql().ToString();
                bool t___3617 = actual___861 == "SELECT * FROM users LIMIT 10";
                string fn__3609()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users LIMIT 10" + ") not (" + actual___861 + ")";
                }
                test___27.Assert(t___3617, (S1::Func<string>) fn__3609);
            }
            finally
            {
                test___27.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlOffset__863()
        {
            T::Test test___28 = new T::Test();
            try
            {
                Schema s__573 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__574 = new InMemoryStore();
                Query q__575 = new Query(s__573, store__574).Offset(5);
                string actual___864 = q__575.ToSql().ToString();
                bool t___3607 = actual___864 == "SELECT * FROM users OFFSET 5";
                string fn__3599()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users OFFSET 5" + ") not (" + actual___864 + ")";
                }
                test___28.Assert(t___3607, (S1::Func<string>) fn__3599);
            }
            finally
            {
                test___28.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlComplexQuery__866()
        {
            T::Test test___29 = new T::Test();
            try
            {
                Schema s__577 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false), O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__578 = new InMemoryStore();
                Query q__579 = new Query(s__577, store__578).Select(C::Listed.CreateReadOnlyList<string>("name", "age")).Where("age", ">=", "18").OrderBy("age", "desc").Limit(10).Offset(20);
                string actual___867 = q__579.ToSql().ToString();
                bool t___3597 = actual___867 == "SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20";
                string fn__3585()
                {
                    return "expected q.toSql().toString() == (" + "SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10 OFFSET 20" + ") not (" + actual___867 + ")";
                }
                test___29.Assert(t___3597, (S1::Func<string>) fn__3585);
            }
            finally
            {
                test___29.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlUnicodeEscaping__869()
        {
            T::Test test___30 = new T::Test();
            try
            {
                Schema s__581 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__582 = new InMemoryStore();
                Query q__583 = new Query(s__581, store__582).Where("name", "=", "Hello 世界");
                string actual___870 = q__583.ToSql().ToString();
                bool t___3583 = actual___870 == "SELECT * FROM users WHERE name = 'Hello 世界'";
                string fn__3575()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'Hello 世界'" + ") not (" + actual___870 + ")";
                }
                test___30.Assert(t___3583, (S1::Func<string>) fn__3575);
            }
            finally
            {
                test___30.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlEmbeddedQuotes__872()
        {
            T::Test test___31 = new T::Test();
            try
            {
                Schema s__585 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__586 = new InMemoryStore();
                Query q__587 = new Query(s__585, store__586).Where("name", "=", "O'Brien");
                string actual___873 = q__587.ToSql().ToString();
                bool t___3573 = actual___873 == "SELECT * FROM users WHERE name = 'O''Brien'";
                string fn__3565()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = 'O''Brien'" + ") not (" + actual___873 + ")";
                }
                test___31.Assert(t___3573, (S1::Func<string>) fn__3565);
            }
            finally
            {
                test___31.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlEmptyString__875()
        {
            T::Test test___32 = new T::Test();
            try
            {
                Schema s__589 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__590 = new InMemoryStore();
                Query q__591 = new Query(s__589, store__590).Where("name", "=", "");
                string actual___876 = q__591.ToSql().ToString();
                bool t___3563 = actual___876 == "SELECT * FROM users WHERE name = ''";
                string fn__3555()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE name = ''" + ") not (" + actual___876 + ")";
                }
                test___32.Assert(t___3563, (S1::Func<string>) fn__3555);
            }
            finally
            {
                test___32.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlBasicInsert__878()
        {
            T::Test test___33 = new T::Test();
            try
            {
                Schema s__593 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false), O::OrmeryGlobal.Field("age", "Int", false, false)));
                G::IReadOnlyDictionary<string, string> vals__594 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"), new G::KeyValuePair<string, string>("age", "25")));
                S0::SqlFragment result__595 = O::OrmeryGlobal.ToInsertSql(s__593, vals__594);
                string actual___879 = result__595.ToString();
                bool t___3553 = actual___879 == "INSERT INTO users (name, age) VALUES ('Alice', 25)";
                string fn__3545()
                {
                    return "expected result.toString() == (" + "INSERT INTO users (name, age) VALUES ('Alice', 25)" + ") not (" + actual___879 + ")";
                }
                test___33.Assert(t___3553, (S1::Func<string>) fn__3545);
            }
            finally
            {
                test___33.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlInjectionBlocked__881()
        {
            T::Test test___34 = new T::Test();
            try
            {
                Schema s__597 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                G::IReadOnlyDictionary<string, string> vals__598 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Robert'); DROP TABLE users;--")));
                S0::SqlFragment result__599 = O::OrmeryGlobal.ToInsertSql(s__597, vals__598);
                string actual___882 = result__599.ToString();
                bool t___3543 = actual___882 == "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')";
                string fn__3536()
                {
                    return "expected result.toString() == (" + "INSERT INTO users (name) VALUES ('Robert''); DROP TABLE users;--')" + ") not (" + actual___882 + ")";
                }
                test___34.Assert(t___3543, (S1::Func<string>) fn__3536);
            }
            finally
            {
                test___34.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlQueryStandalone__884()
        {
            T::Test test___35 = new T::Test();
            try
            {
                Schema s__601 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false), O::OrmeryGlobal.Field("age", "Int", false, false)));
                S0::SqlFragment result__602 = O::OrmeryGlobal.ToSqlQuery(s__601, C::Listed.CreateReadOnlyList<string>("name"), C::Listed.CreateReadOnlyList<WhereClause>(new WhereClause("age", ">", "21")), C::Listed.CreateReadOnlyList<OrderClause>(new OrderClause("name", "asc")), 5, 0);
                string actual___885 = result__602.ToString();
                bool t___3534 = actual___885 == "SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5";
                string fn__3525()
                {
                    return "expected result.toString() == (" + "SELECT name FROM users WHERE age > 21 ORDER BY name ASC LIMIT 5" + ") not (" + actual___885 + ")";
                }
                test___35.Assert(t___3534, (S1::Func<string>) fn__3525);
            }
            finally
            {
                test___35.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlAdversarialFieldNameBlocked__887()
        {
            T::Test test___36 = new T::Test();
            try
            {
                Schema s__604 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__605 = new InMemoryStore();
                Query q__606 = new Query(s__604, store__605).Where("1=1; DROP TABLE users; --", "=", "Alice");
                string actual___888 = q__606.ToSql().ToString();
                bool t___3523 = actual___888 == "SELECT * FROM users";
                string fn__3515()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual___888 + ")";
                }
                test___36.Assert(t___3523, (S1::Func<string>) fn__3515);
            }
            finally
            {
                test___36.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlAdversarialSelectColumnBlocked__890()
        {
            T::Test test___37 = new T::Test();
            try
            {
                Schema s__608 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__609 = new InMemoryStore();
                Query q__610 = new Query(s__608, store__609).Select(C::Listed.CreateReadOnlyList<string>("name", "1; DROP TABLE users"));
                string actual___891 = q__610.ToSql().ToString();
                bool t___3513 = actual___891 == "SELECT name FROM users";
                string fn__3505()
                {
                    return "expected q.toSql().toString() == (" + "SELECT name FROM users" + ") not (" + actual___891 + ")";
                }
                test___37.Assert(t___3513, (S1::Func<string>) fn__3505);
            }
            finally
            {
                test___37.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlAdversarialOrderByBlocked__893()
        {
            T::Test test___38 = new T::Test();
            try
            {
                Schema s__612 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__613 = new InMemoryStore();
                Query q__614 = new Query(s__612, store__613).OrderBy("1; DROP TABLE users", "asc");
                string actual___894 = q__614.ToSql().ToString();
                bool t___3503 = actual___894 == "SELECT * FROM users";
                string fn__3495()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users" + ") not (" + actual___894 + ")";
                }
                test___38.Assert(t___3503, (S1::Func<string>) fn__3495);
            }
            finally
            {
                test___38.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void isValidIdentifierValidNames__896()
        {
            T::Test test___39 = new T::Test();
            try
            {
                bool t___3480 = O::OrmeryGlobal.IsValidIdentifier("users");
                string fn__3479()
                {
                    return "expected `-work/src//`.isValidIdentifier(\u0022users\u0022)";
                }
                test___39.Assert(t___3480, (S1::Func<string>) fn__3479);
                bool t___3483 = O::OrmeryGlobal.IsValidIdentifier("user_table");
                string fn__3478()
                {
                    return "expected `-work/src//`.isValidIdentifier(\u0022user_table\u0022)";
                }
                test___39.Assert(t___3483, (S1::Func<string>) fn__3478);
                bool t___3486 = O::OrmeryGlobal.IsValidIdentifier("Table1");
                string fn__3477()
                {
                    return "expected `-work/src//`.isValidIdentifier(\u0022Table1\u0022)";
                }
                test___39.Assert(t___3486, (S1::Func<string>) fn__3477);
                bool t___3489 = O::OrmeryGlobal.IsValidIdentifier("_private");
                string fn__3476()
                {
                    return "expected `-work/src//`.isValidIdentifier(\u0022_private\u0022)";
                }
                test___39.Assert(t___3489, (S1::Func<string>) fn__3476);
                bool t___3492 = O::OrmeryGlobal.IsValidIdentifier("a");
                string fn__3475()
                {
                    return "expected `-work/src//`.isValidIdentifier(\u0022a\u0022)";
                }
                test___39.Assert(t___3492, (S1::Func<string>) fn__3475);
            }
            finally
            {
                test___39.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void isValidIdentifierInvalidNames__897()
        {
            T::Test test___40 = new T::Test();
            try
            {
                bool t___3453 = !O::OrmeryGlobal.IsValidIdentifier("");
                string fn__3450()
                {
                    return "expected !`-work/src//`.isValidIdentifier(\u0022\u0022)";
                }
                test___40.Assert(t___3453, (S1::Func<string>) fn__3450);
                bool t___3457 = !O::OrmeryGlobal.IsValidIdentifier("users; DROP TABLE");
                string fn__3449()
                {
                    return "expected !`-work/src//`.isValidIdentifier(\u0022users; DROP TABLE\u0022)";
                }
                test___40.Assert(t___3457, (S1::Func<string>) fn__3449);
                bool t___3461 = !O::OrmeryGlobal.IsValidIdentifier("users--");
                string fn__3448()
                {
                    return "expected !`-work/src//`.isValidIdentifier(\u0022users--\u0022)";
                }
                test___40.Assert(t___3461, (S1::Func<string>) fn__3448);
                bool t___3465 = !O::OrmeryGlobal.IsValidIdentifier("ta ble");
                string fn__3447()
                {
                    return "expected !`-work/src//`.isValidIdentifier(\u0022ta ble\u0022)";
                }
                test___40.Assert(t___3465, (S1::Func<string>) fn__3447);
                bool t___3469 = !O::OrmeryGlobal.IsValidIdentifier("table.name");
                string fn__3446()
                {
                    return "expected !`-work/src//`.isValidIdentifier(\u0022table.name\u0022)";
                }
                test___40.Assert(t___3469, (S1::Func<string>) fn__3446);
                bool t___3473 = !O::OrmeryGlobal.IsValidIdentifier("Robert'); DROP TABLE users;--");
                string fn__3445()
                {
                    return "expected !`-work/src//`.isValidIdentifier(\u0022Robert'); DROP TABLE users;--\u0022)";
                }
                test___40.Assert(t___3473, (S1::Func<string>) fn__3445);
            }
            finally
            {
                test___40.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlNonNumericIntValueProducesAlwaysFalse__898()
        {
            T::Test test___41 = new T::Test();
            try
            {
                Schema s__618 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__619 = new InMemoryStore();
                Query q__620 = new Query(s__618, store__619).Where("age", "=", "admin");
                string actual___899 = q__620.ToSql().ToString();
                bool t___3443 = actual___899 == "SELECT * FROM users WHERE 1 = 0";
                string fn__3435()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users WHERE 1 = 0" + ") not (" + actual___899 + ")";
                }
                test___41.Assert(t___3443, (S1::Func<string>) fn__3435);
            }
            finally
            {
                test___41.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void inMemoryNonNumericIntValueMatchesNothing__901()
        {
            T::Test test___42 = new T::Test();
            try
            {
                Schema s__622 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false), O::OrmeryGlobal.Field("age", "Int", false, false)));
                InMemoryStore store__623 = new InMemoryStore();
                store__623.Insert("users", C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"), new G::KeyValuePair<string, string>("age", "0"))));
                G::IReadOnlyList<Record> results__624 = new Query(s__622, store__623).Where("age", "=", "admin").All();
                int actual___902 = results__624.Count;
                bool t___3433 = actual___902 == 0;
                string fn__3421()
                {
                    return "expected results.length == (" + S1::Convert.ToString(0) + ") not (" + S1::Convert.ToString(actual___902) + ")";
                }
                test___42.Assert(t___3433, (S1::Func<string>) fn__3421);
            }
            finally
            {
                test___42.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toSqlLimitZeroEmitsLimit0__904()
        {
            T::Test test___43 = new T::Test();
            try
            {
                Schema s__626 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__627 = new InMemoryStore();
                Query q__628 = new Query(s__626, store__627).Limit(0);
                string actual___905 = q__628.ToSql().ToString();
                bool t___3417 = actual___905 == "SELECT * FROM users LIMIT 0";
                string fn__3409()
                {
                    return "expected q.toSql().toString() == (" + "SELECT * FROM users LIMIT 0" + ") not (" + actual___905 + ")";
                }
                test___43.Assert(t___3417, (S1::Func<string>) fn__3409);
            }
            finally
            {
                test___43.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void inMemoryLimitZeroReturnsEmpty__907()
        {
            T::Test test___44 = new T::Test();
            try
            {
                Schema s__630 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__631 = new InMemoryStore();
                store__631.Insert("users", C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"))));
                G::IReadOnlyList<Record> results__632 = new Query(s__630, store__631).Limit(0).All();
                int actual___908 = results__632.Count;
                bool t___3407 = actual___908 == 0;
                string fn__3396()
                {
                    return "expected results.length == (" + S1::Convert.ToString(0) + ") not (" + S1::Convert.ToString(actual___908) + ")";
                }
                test___44.Assert(t___3407, (S1::Func<string>) fn__3396);
            }
            finally
            {
                test___44.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void inMemoryNegativeLimitClampedToZero__910()
        {
            T::Test test___45 = new T::Test();
            try
            {
                Schema s__634 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                InMemoryStore store__635 = new InMemoryStore();
                store__635.Insert("users", C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"))));
                G::IReadOnlyList<Record> results__636 = new Query(s__634, store__635).Limit(-5).All();
                int actual___911 = results__636.Count;
                bool t___3392 = actual___911 == 0;
                string fn__3381()
                {
                    return "expected results.length == (" + S1::Convert.ToString(0) + ") not (" + S1::Convert.ToString(actual___911) + ")";
                }
                test___45.Assert(t___3392, (S1::Func<string>) fn__3381);
            }
            finally
            {
                test___45.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlNoMatchingFieldsReturnsEmpty__913()
        {
            T::Test test___46 = new T::Test();
            try
            {
                Schema s__638 = O::OrmeryGlobal.Schema("users", C::Listed.CreateReadOnlyList<Field>(O::OrmeryGlobal.Field("name", "String", false, false)));
                G::IReadOnlyDictionary<string, string> vals__639 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("nonexistent", "value")));
                S0::SqlFragment result__640 = O::OrmeryGlobal.ToInsertSql(s__638, vals__639);
                string actual___914 = result__640.ToString();
                bool t___3377 = actual___914 == "";
                string fn__3370()
                {
                    return "expected result.toString() == (" + "" + ") not (" + actual___914 + ")";
                }
                test___46.Assert(t___3377, (S1::Func<string>) fn__3370);
            }
            finally
            {
                test___46.SoftFailToHard();
            }
        }
    }
}
