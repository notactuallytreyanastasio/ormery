using G = System.Collections.Generic;
using T = System.Text;
namespace Ormery.Sql
{
    public class SqlFragment
    {
        readonly G::IReadOnlyList<ISqlPart> parts__161;
        public SqlSource ToSource()
        {
            return new SqlSource(this.ToString());
        }
        public string ToString()
        {
            int t___1186;
            T::StringBuilder builder__166 = new T::StringBuilder();
            int i__167 = 0;
            while (true)
            {
                t___1186 = this.parts__161.Count;
                if (!(i__167 < t___1186)) break;
                this.parts__161[i__167].FormatTo(builder__166);
                i__167 = i__167 + 1;
            }
            return builder__166.ToString();
        }
        public SqlFragment(G::IReadOnlyList<ISqlPart> parts__169)
        {
            this.parts__161 = parts__169;
        }
        public G::IReadOnlyList<ISqlPart> Parts
        {
            get
            {
                return this.parts__161;
            }
        }
    }
}
