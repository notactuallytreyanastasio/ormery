using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Match
    {
        readonly Group full__237;
        readonly G::IReadOnlyDictionary<string, Group> groups__238;
        public Match(Group full__240, G::IReadOnlyDictionary<string, Group> groups__241)
        {
            this.full__237 = full__240;
            this.groups__238 = groups__241;
        }
        public Group Full
        {
            get
            {
                return this.full__237;
            }
        }
        public G::IReadOnlyDictionary<string, Group> Groups
        {
            get
            {
                return this.groups__238;
            }
        }
    }
}
