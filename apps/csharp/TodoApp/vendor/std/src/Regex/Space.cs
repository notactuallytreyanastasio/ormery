using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Space: ISpecialSet
    {
        public Space()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1421)
        {
            return IRegexNode.FoundDefault(this, text___1421);
        }
        public Match Find(string text___1423)
        {
            return IRegexNode.FindDefault(this, text___1423);
        }
        public string Replace(string text___1425, S::Func<Match, string> format___1426)
        {
            return IRegexNode.ReplaceDefault(this, text___1425, (S::Func<Match, string>) format___1426);
        }
        public G::IReadOnlyList<string> Split(string text___1428)
        {
            return IRegexNode.SplitDefault(this, text___1428);
        }
    }
}
