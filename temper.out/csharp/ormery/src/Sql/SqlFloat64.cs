using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Sql
{
    public class SqlFloat64: ISqlPart
    {
        readonly double value__193;
        public void FormatTo(T::StringBuilder builder__195)
        {
            string t___1175 = C::Float64.Format(this.value__193);
            builder__195.Append(t___1175);
        }
        public SqlFloat64(double value__198)
        {
            this.value__193 = value__198;
        }
        public double Value
        {
            get
            {
                return this.value__193;
            }
        }
    }
}
