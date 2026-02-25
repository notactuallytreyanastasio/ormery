using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Or: IRegexNode
    {
        readonly G::IReadOnlyList<IRegexNode> items__214;
        public Or(G::IReadOnlyList<IRegexNode> items__216)
        {
            this.items__214 = items__216;
        }
        public G::IReadOnlyList<IRegexNode> Items
        {
            get
            {
                return this.items__214;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1341)
        {
            return IRegexNode.FoundDefault(this, text___1341);
        }
        public Match Find(string text___1343)
        {
            return IRegexNode.FindDefault(this, text___1343);
        }
        public string Replace(string text___1345, S::Func<Match, string> format___1346)
        {
            return IRegexNode.ReplaceDefault(this, text___1345, (S::Func<Match, string>) format___1346);
        }
        public G::IReadOnlyList<string> Split(string text___1348)
        {
            return IRegexNode.SplitDefault(this, text___1348);
        }
    }
}
