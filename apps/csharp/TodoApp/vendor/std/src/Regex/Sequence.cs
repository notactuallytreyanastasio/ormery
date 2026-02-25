using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Sequence: IRegexNode
    {
        readonly G::IReadOnlyList<IRegexNode> items__234;
        public Sequence(G::IReadOnlyList<IRegexNode> items__236)
        {
            this.items__234 = items__236;
        }
        public G::IReadOnlyList<IRegexNode> Items
        {
            get
            {
                return this.items__234;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1361)
        {
            return IRegexNode.FoundDefault(this, text___1361);
        }
        public Match Find(string text___1363)
        {
            return IRegexNode.FindDefault(this, text___1363);
        }
        public string Replace(string text___1365, S::Func<Match, string> format___1366)
        {
            return IRegexNode.ReplaceDefault(this, text___1365, (S::Func<Match, string>) format___1366);
        }
        public G::IReadOnlyList<string> Split(string text___1368)
        {
            return IRegexNode.SplitDefault(this, text___1368);
        }
    }
}
