using S = System;
using T = System.Text;
namespace Ormery.Sql
{
    public class SqlInt64: ISqlPart
    {
        readonly long value__205;
        public void FormatTo(T::StringBuilder builder__207)
        {
            string t___1171 = S::Convert.ToString(this.value__205);
            builder__207.Append(t___1171);
        }
        public SqlInt64(long value__210)
        {
            this.value__205 = value__210;
        }
        public long Value
        {
            get
            {
                return this.value__205;
            }
        }
    }
}
