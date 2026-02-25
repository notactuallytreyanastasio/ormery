using S = System;
using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Sql
{
    public class SqlDate: ISqlPart
    {
        readonly S::DateTime value__185;
        public void FormatTo(T::StringBuilder builder__187)
        {
            builder__187.Append("'");
            string s__189 = this.value__185.ToString("yyyy-MM-dd");
            void fn__1178(int c__190)
            {
                if (c__190 == 39) builder__187.Append("''");
                else C::StringUtil.AppendCodePoint(builder__187, c__190);
            }
            C::StringUtil.ForEach(s__189, (S::Action<int>) fn__1178);
            builder__187.Append("'");
        }
        public SqlDate(S::DateTime value__192)
        {
            this.value__185 = value__192;
        }
        public S::DateTime Value
        {
            get
            {
                return this.value__185;
            }
        }
    }
}
