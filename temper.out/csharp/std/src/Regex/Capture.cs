using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Capture: IRegexNode
    {
        readonly string name__182;
        readonly IRegexNode item__183;
        public Capture(string name__185, IRegexNode item__186)
        {
            this.name__182 = name__185;
            this.item__183 = item__186;
        }
        public string Name
        {
            get
            {
                return this.name__182;
            }
        }
        public IRegexNode Item
        {
            get
            {
                return this.item__183;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1301)
        {
            return IRegexNode.FoundDefault(this, text___1301);
        }
        public Match Find(string text___1303)
        {
            return IRegexNode.FindDefault(this, text___1303);
        }
        public string Replace(string text___1305, S::Func<Match, string> format___1306)
        {
            return IRegexNode.ReplaceDefault(this, text___1305, (S::Func<Match, string>) format___1306);
        }
        public G::IReadOnlyList<string> Split(string text___1308)
        {
            return IRegexNode.SplitDefault(this, text___1308);
        }
    }
}
