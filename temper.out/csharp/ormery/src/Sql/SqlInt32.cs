using S = System;
using T = System.Text;
namespace Ormery.Sql
{
    public class SqlInt32: ISqlPart
    {
        readonly int value__199;
        public void FormatTo(T::StringBuilder builder__201)
        {
            string t___1173 = S::Convert.ToString(this.value__199);
            builder__201.Append(t___1173);
        }
        public SqlInt32(int value__204)
        {
            this.value__199 = value__204;
        }
        public int Value
        {
            get
            {
                return this.value__199;
            }
        }
    }
}
