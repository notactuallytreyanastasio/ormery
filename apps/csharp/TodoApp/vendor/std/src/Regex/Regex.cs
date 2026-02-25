using S = System;
using G = System.Collections.Generic;
using R = TemperLang.Std.Regex;
namespace TemperLang.Std.Regex
{
    public class Regex
    {
        readonly IRegexNode data__260;
        public Regex(IRegexNode data__262)
        {
            IRegexNode t___419 = data__262;
            this.data__260 = t___419;
            string formatted__264 = RegexFormatter.RegexFormat(data__262);
            object t___1156 = R::RegexSupport.CompileFormatted(data__262, formatted__264);
            this.compiled__279 = t___1156;
        }
        public bool Found(string text__266)
        {
            return R::RegexSupport.CompiledFound(this, this.compiled__279, text__266);
        }
        public Match Find(string text__269, int ? begin = null)
        {
            int begin__270;
            if (begin == null)
            {
                begin__270 = 0;
            }
            else
            {
                begin__270 = begin.Value;
            }
            return R::RegexSupport.CompiledFind(this, this.compiled__279, text__269, begin__270, R::RegexGlobal.regexRefs__162);
        }
        public string Replace(string text__273, S::Func<Match, string> format__274)
        {
            return R::RegexSupport.CompiledReplace(this, this.compiled__279, text__273, (S::Func<Match, string>) format__274, R::RegexGlobal.regexRefs__162);
        }
        public G::IReadOnlyList<string> Split(string text__277)
        {
            return R::RegexSupport.CompiledSplit(this, this.compiled__279, text__277, R::RegexGlobal.regexRefs__162);
        }
        readonly object compiled__279;
        public IRegexNode Data
        {
            get
            {
                return this.data__260;
            }
        }
    }
}
