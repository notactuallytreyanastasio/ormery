using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Begin: ISpecial
    {
        public Begin()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1371)
        {
            return IRegexNode.FoundDefault(this, text___1371);
        }
        public Match Find(string text___1373)
        {
            return IRegexNode.FindDefault(this, text___1373);
        }
        public string Replace(string text___1375, S::Func<Match, string> format___1376)
        {
            return IRegexNode.ReplaceDefault(this, text___1375, (S::Func<Match, string>) format___1376);
        }
        public G::IReadOnlyList<string> Split(string text___1378)
        {
            return IRegexNode.SplitDefault(this, text___1378);
        }
    }
}
