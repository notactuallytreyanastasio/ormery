using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class End: ISpecial
    {
        public End()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1391)
        {
            return IRegexNode.FoundDefault(this, text___1391);
        }
        public Match Find(string text___1393)
        {
            return IRegexNode.FindDefault(this, text___1393);
        }
        public string Replace(string text___1395, S::Func<Match, string> format___1396)
        {
            return IRegexNode.ReplaceDefault(this, text___1395, (S::Func<Match, string>) format___1396);
        }
        public G::IReadOnlyList<string> Split(string text___1398)
        {
            return IRegexNode.SplitDefault(this, text___1398);
        }
    }
}
