using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace Ormery
{
    public class Record
    {
        readonly G::IReadOnlyDictionary<string, string> data__306;
        public string Get(string field__308)
        {
            return this.data__306[field__308];
        }
        public string GetOr(string field__311, string fallback__312)
        {
            return C::Mapped.GetOrDefault(this.data__306, field__311, fallback__312);
        }
        public bool Has(string field__315)
        {
            return C::Mapped.ContainsKey(this.data__306, field__315);
        }
        public int Id
        {
            get
            {
                string idStr__319;
                idStr__319 = this.data__306["id"];
                return C::Core.ToInt(idStr__319);
            }
        }
        public string Describe()
        {
            string fn__4061(string k__323, string v__324)
            {
                return k__323 + ": " + v__324;
            }
            G::IReadOnlyList<string> pairs__322 = C::Mapped.ToListWith(this.data__306, (S::Func<string, string, string>) fn__4061);
            string fn__4060(string s__326)
            {
                return s__326;
            }
            return C::Listed.Join(pairs__322, ", ", (S::Func<string, string>) fn__4060);
        }
        public Record(G::IReadOnlyDictionary<string, string> data__329)
        {
            this.data__306 = data__329;
        }
        public G::IReadOnlyDictionary<string, string> Data
        {
            get
            {
                return this.data__306;
            }
        }
    }
}
