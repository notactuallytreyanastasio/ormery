using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class CodePoints: ICodePart
    {
        readonly string value__187;
        public CodePoints(string value__189)
        {
            this.value__187 = value__189;
        }
        public string Value
        {
            get
            {
                return this.value__187;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1311)
        {
            return IRegexNode.FoundDefault(this, text___1311);
        }
        public Match Find(string text___1313)
        {
            return IRegexNode.FindDefault(this, text___1313);
        }
        public string Replace(string text___1315, S::Func<Match, string> format___1316)
        {
            return IRegexNode.ReplaceDefault(this, text___1315, (S::Func<Match, string>) format___1316);
        }
        public G::IReadOnlyList<string> Split(string text___1318)
        {
            return IRegexNode.SplitDefault(this, text___1318);
        }
    }
}
