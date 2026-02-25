using T = System.Text;
namespace Ormery.Sql
{
    public class SqlSource: ISqlPart
    {
        readonly string source__173;
        public void FormatTo(T::StringBuilder builder__175)
        {
            builder__175.Append(this.source__173);
        }
        public SqlSource(string source__178)
        {
            this.source__173 = source__178;
        }
        public string Source
        {
            get
            {
                return this.source__173;
            }
        }
    }
}
