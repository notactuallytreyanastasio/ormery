using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Regex
{
    class RegexRefs
    {
        readonly CodePoints codePoints__251;
        readonly Group group__252;
        readonly Match match__253;
        readonly Or orObject__254;
        public RegexRefs(CodePoints ? codePoints = null, Group ? @group = null, Match ? match = null, Or ? orObject = null)
        {
            CodePoints t___1272;
            Group t___1273;
            G::IReadOnlyDictionary<string, Group> t___1275;
            Match t___1276;
            Or t___1277;
            CodePoints codePoints__256;
            if (codePoints == null)
            {
                t___1272 = new CodePoints("");
                codePoints__256 = t___1272;
            }
            else
            {
                codePoints__256 = codePoints!;
            }
            Group group__257;
            if (@group == null)
            {
                t___1273 = new Group("", "", 0, 0);
                group__257 = t___1273;
            }
            else
            {
                group__257 = @group!;
            }
            Match match__258;
            if (match == null)
            {
                t___1275 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, Group>>(new G::KeyValuePair<string, Group>("", group__257)));
                t___1276 = new Match(group__257, t___1275);
                match__258 = t___1276;
            }
            else
            {
                match__258 = match!;
            }
            Or orObject__259;
            if (orObject == null)
            {
                t___1277 = new Or(C::Listed.CreateReadOnlyList<IRegexNode>());
                orObject__259 = t___1277;
            }
            else
            {
                orObject__259 = orObject!;
            }
            this.codePoints__251 = codePoints__256;
            this.group__252 = group__257;
            this.match__253 = match__258;
            this.orObject__254 = orObject__259;
        }
        public CodePoints CodePoints
        {
            get
            {
                return this.codePoints__251;
            }
        }
        public Group Group
        {
            get
            {
                return this.group__252;
            }
        }
        public Match Match
        {
            get
            {
                return this.match__253;
            }
        }
        public Or OrObject
        {
            get
            {
                return this.orObject__254;
            }
        }
    }
}
