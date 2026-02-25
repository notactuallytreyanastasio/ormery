using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    class Word: ISpecialSet
    {
        public Word()
        {
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1431)
        {
            return IRegexNode.FoundDefault(this, text___1431);
        }
        public Match Find(string text___1433)
        {
            return IRegexNode.FindDefault(this, text___1433);
        }
        public string Replace(string text___1435, S::Func<Match, string> format___1436)
        {
            return IRegexNode.ReplaceDefault(this, text___1435, (S::Func<Match, string>) format___1436);
        }
        public G::IReadOnlyList<string> Split(string text___1438)
        {
            return IRegexNode.SplitDefault(this, text___1438);
        }
    }
}
