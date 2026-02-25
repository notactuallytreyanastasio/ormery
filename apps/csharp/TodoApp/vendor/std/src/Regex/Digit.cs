using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Digit: ISpecialSet
    {
        public Digit()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1411)
        {
            return IRegexNode.FoundDefault(this, text___1411);
        }
        public Match Find(string text___1413)
        {
            return IRegexNode.FindDefault(this, text___1413);
        }
        public string Replace(string text___1415, S::Func<Match, string> format___1416)
        {
            return IRegexNode.ReplaceDefault(this, text___1415, (S::Func<Match, string>) format___1416);
        }
        public G::IReadOnlyList<string> Split(string text___1418)
        {
            return IRegexNode.SplitDefault(this, text___1418);
        }
    }
}
