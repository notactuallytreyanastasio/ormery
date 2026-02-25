using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class CodeSet: IRegexNode
    {
        readonly G::IReadOnlyList<ICodePart> items__209;
        readonly bool negated__210;
        public CodeSet(G::IReadOnlyList<ICodePart> items__212, bool ? negated = null)
        {
            bool negated__213;
            if (negated == null)
            {
                negated__213 = false;
            }
            else
            {
                negated__213 = negated.Value;
            }
            this.items__209 = items__212;
            this.negated__210 = negated__213;
        }
        public G::IReadOnlyList<ICodePart> Items
        {
            get
            {
                return this.items__209;
            }
        }
        public bool Negated
        {
            get
            {
                return this.negated__210;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1331)
        {
            return IRegexNode.FoundDefault(this, text___1331);
        }
        public Match Find(string text___1333)
        {
            return IRegexNode.FindDefault(this, text___1333);
        }
        public string Replace(string text___1335, S::Func<Match, string> format___1336)
        {
            return IRegexNode.ReplaceDefault(this, text___1335, (S::Func<Match, string>) format___1336);
        }
        public G::IReadOnlyList<string> Split(string text___1338)
        {
            return IRegexNode.SplitDefault(this, text___1338);
        }
    }
}
