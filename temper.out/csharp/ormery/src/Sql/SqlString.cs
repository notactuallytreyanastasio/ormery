using S = System;
using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Sql
{
    public class SqlString: ISqlPart
    {
        readonly string value__211;
        public void FormatTo(T::StringBuilder builder__213)
        {
            builder__213.Append("'");
            void fn__1166(int c__215)
            {
                if (c__215 == 39) builder__213.Append("''");
                else C::StringUtil.AppendCodePoint(builder__213, c__215);
            }
            C::StringUtil.ForEach(this.value__211, (S::Action<int>) fn__1166);
            builder__213.Append("'");
        }
        public SqlString(string value__217)
        {
            this.value__211 = value__217;
        }
        public string Value
        {
            get
            {
                return this.value__211;
            }
        }
    }
}
