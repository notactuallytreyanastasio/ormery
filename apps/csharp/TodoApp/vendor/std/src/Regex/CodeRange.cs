using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class CodeRange: ICodePart
    {
        readonly int min__204;
        readonly int max__205;
        public CodeRange(int min__207, int max__208)
        {
            this.min__204 = min__207;
            this.max__205 = max__208;
        }
        public int Min
        {
            get
            {
                return this.min__204;
            }
        }
        public int Max
        {
            get
            {
                return this.max__205;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1321)
        {
            return IRegexNode.FoundDefault(this, text___1321);
        }
        public Match Find(string text___1323)
        {
            return IRegexNode.FindDefault(this, text___1323);
        }
        public string Replace(string text___1325, S::Func<Match, string> format___1326)
        {
            return IRegexNode.ReplaceDefault(this, text___1325, (S::Func<Match, string>) format___1326);
        }
        public G::IReadOnlyList<string> Split(string text___1328)
        {
            return IRegexNode.SplitDefault(this, text___1328);
        }
    }
}
