using S = System;
using G = System.Collections.Generic;
namespace TemperLang.Std.Regex
{
    public interface IRegexNode
    {
        protected static Regex CompiledDefault(IRegexNode this__42)
        {
            return new Regex(this__42);
        }
        Regex Compiled()
        {
            return CompiledDefault(this);
        }
        protected static bool FoundDefault(IRegexNode this__43, string text__170)
        {
            return this__43.Compiled().Found(text__170);
        }
        bool Found(string text__170)
        {
            return FoundDefault(this, text__170);
        }
        protected static Match FindDefault(IRegexNode this__44, string text__173)
        {
            return this__44.Compiled().Find(text__173);
        }
        Match Find(string text__173)
        {
            return FindDefault(this, text__173);
        }
        protected static string ReplaceDefault(IRegexNode this__45, string text__176, S::Func<Match, string> format__177)
        {
            return this__45.Compiled().Replace(text__176, (S::Func<Match, string>) format__177);
        }
        string Replace(string text__176, S::Func<Match, string> format__177)
        {
            return ReplaceDefault(this, text__176, format__177);
        }
        protected static G::IReadOnlyList<string> SplitDefault(IRegexNode this__46, string text__180)
        {
            return this__46.Compiled().Split(text__180);
        }
        G::IReadOnlyList<string> Split(string text__180)
        {
            return SplitDefault(this, text__180);
        }
    }
}
