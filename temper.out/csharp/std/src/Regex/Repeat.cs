using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public class Repeat: IRegexNode
    {
        readonly IRegexNode item__217;
        readonly int min__218;
        readonly int ? max__219;
        readonly bool reluctant__220;
        public Repeat(IRegexNode item__222, int min__223, int ? max__224, bool ? reluctant = null)
        {
            bool reluctant__225;
            if (reluctant == null)
            {
                reluctant__225 = false;
            }
            else
            {
                reluctant__225 = reluctant.Value;
            }
            this.item__217 = item__222;
            this.min__218 = min__223;
            this.max__219 = max__224;
            this.reluctant__220 = reluctant__225;
        }
        public IRegexNode Item
        {
            get
            {
                return this.item__217;
            }
        }
        public int Min
        {
            get
            {
                return this.min__218;
            }
        }
        public int ? Max
        {
            get
            {
                return this.max__219;
            }
        }
        public bool Reluctant
        {
            get
            {
                return this.reluctant__220;
            }
        }
        public Regex Compiled()
        {
            return IRegexNode.CompiledDefault(this);
        }
        public bool Found(string text___1351)
        {
            return IRegexNode.FoundDefault(this, text___1351);
        }
        public Match Find(string text___1353)
        {
            return IRegexNode.FindDefault(this, text___1353);
        }
        public string Replace(string text___1355, S::Func<Match, string> format___1356)
        {
            return IRegexNode.ReplaceDefault(this, text___1355, (S::Func<Match, string>) format___1356);
        }
        public G::IReadOnlyList<string> Split(string text___1358)
        {
            return IRegexNode.SplitDefault(this, text___1358);
        }
    }
}
