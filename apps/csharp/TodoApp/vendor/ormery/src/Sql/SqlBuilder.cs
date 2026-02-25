using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace Ormery.Sql
{
    public class SqlBuilder
    {
        readonly G::IList<ISqlPart> buffer__94;
        public void AppendSafe(string sqlSource__96)
        {
            SqlSource t___1163 = new SqlSource(sqlSource__96);
            C::Listed.Add(this.buffer__94, t___1163);
        }
        public void AppendFragment(SqlFragment fragment__99)
        {
            G::IReadOnlyList<ISqlPart> t___1161 = fragment__99.Parts;
            C::Listed.AddAll(this.buffer__94, t___1161);
        }
        public void AppendPart(ISqlPart part__102)
        {
            C::Listed.Add(this.buffer__94, part__102);
        }
        public void AppendPartList(G::IReadOnlyList<ISqlPart> values__105)
        {
            void fn__1157(ISqlPart x__107)
            {
                this.AppendPart(x__107);
            }
            this.AppendList(values__105, (S::Action<ISqlPart>) fn__1157);
        }
        public void AppendBoolean(bool value__109)
        {
            SqlBoolean t___1154 = new SqlBoolean(value__109);
            C::Listed.Add(this.buffer__94, t___1154);
        }
        public void AppendBooleanList(G::IReadOnlyList<bool> values__112)
        {
            void fn__1151(bool x__114)
            {
                this.AppendBoolean(x__114);
            }
            this.AppendList(values__112, (S::Action<bool>) fn__1151);
        }
        public void AppendDate(S::DateTime value__116)
        {
            SqlDate t___1148 = new SqlDate(value__116);
            C::Listed.Add(this.buffer__94, t___1148);
        }
        public void AppendDateList(G::IReadOnlyList<S::DateTime> values__119)
        {
            void fn__1145(S::DateTime x__121)
            {
                this.AppendDate(x__121);
            }
            this.AppendList(values__119, (S::Action<S::DateTime>) fn__1145);
        }
        public void AppendFloat64(double value__123)
        {
            SqlFloat64 t___1142 = new SqlFloat64(value__123);
            C::Listed.Add(this.buffer__94, t___1142);
        }
        public void AppendFloat64_List(G::IReadOnlyList<double> values__126)
        {
            void fn__1139(double x__128)
            {
                this.AppendFloat64(x__128);
            }
            this.AppendList(values__126, (S::Action<double>) fn__1139);
        }
        public void AppendInt32(int value__130)
        {
            SqlInt32 t___1136 = new SqlInt32(value__130);
            C::Listed.Add(this.buffer__94, t___1136);
        }
        public void AppendInt32_List(G::IReadOnlyList<int> values__133)
        {
            void fn__1133(int x__135)
            {
                this.AppendInt32(x__135);
            }
            this.AppendList(values__133, (S::Action<int>) fn__1133);
        }
        public void AppendInt64(long value__137)
        {
            SqlInt64 t___1130 = new SqlInt64(value__137);
            C::Listed.Add(this.buffer__94, t___1130);
        }
        public void AppendInt64_List(G::IReadOnlyList<long> values__140)
        {
            void fn__1127(long x__142)
            {
                this.AppendInt64(x__142);
            }
            this.AppendList(values__140, (S::Action<long>) fn__1127);
        }
        public void AppendString(string value__144)
        {
            SqlString t___1124 = new SqlString(value__144);
            C::Listed.Add(this.buffer__94, t___1124);
        }
        public void AppendStringList(G::IReadOnlyList<string> values__147)
        {
            void fn__1121(string x__149)
            {
                this.AppendString(x__149);
            }
            this.AppendList(values__147, (S::Action<string>) fn__1121);
        }
        void AppendList<T__27>(G::IReadOnlyList<T__27> values__151, S::Action<T__27> appendValue__152)
        {
            int t___1116;
            T__27 t___1118;
            int i__154 = 0;
            while (true)
            {
                t___1116 = values__151.Count;
                if (!(i__154 < t___1116)) break;
                if (i__154 > 0) this.AppendSafe(", ");
                t___1118 = values__151[i__154];
                appendValue__152(t___1118);
                i__154 = i__154 + 1;
            }
        }
        public SqlFragment Accumulated
        {
            get
            {
                return new SqlFragment(C::Listed.ToReadOnlyList(this.buffer__94));
            }
        }
        public SqlBuilder()
        {
            G::IList<ISqlPart> t___1113 = new G::List<ISqlPart>();
            this.buffer__94 = t___1113;
        }
    }
}
