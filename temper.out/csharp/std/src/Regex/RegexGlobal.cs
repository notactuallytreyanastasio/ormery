using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace TemperLang.Std.Regex
{
    public static class RegexGlobal
    {
        internal static ISpecial return__190;
        public static ISpecial Begin;
        internal static ISpecial return__192;
        public static ISpecial Dot;
        internal static ISpecial return__194;
        public static ISpecial End;
        internal static ISpecial return__196;
        public static ISpecial WordBoundary;
        internal static ISpecialSet return__198;
        public static ISpecialSet Digit;
        internal static ISpecialSet return__200;
        public static ISpecialSet Space;
        internal static ISpecialSet return__202;
        public static ISpecialSet Word;
        internal static G::IReadOnlyList<int> buildEscapeNeeds__161()
        {
            bool t___925;
            bool t___926;
            bool t___927;
            bool t___928;
            bool t___929;
            bool t___930;
            bool t___931;
            bool t___932;
            bool t___933;
            bool t___934;
            bool t___935;
            bool t___936;
            bool t___937;
            bool t___938;
            bool t___939;
            bool t___940;
            bool t___941;
            bool t___942;
            bool t___943;
            bool t___944;
            bool t___945;
            bool t___946;
            bool t___947;
            bool t___948;
            int t___949;
            G::IList<int> escapeNeeds__379 = new G::List<int>();
            int code__380 = 0;
            while (code__380 <= 127)
            {
                if (code__380 == Codes.Dash)
                {
                    t___932 = true;
                }
                else
                {
                    if (code__380 == Codes.Space)
                    {
                        t___931 = true;
                    }
                    else
                    {
                        if (code__380 == Codes.Underscore)
                        {
                            t___930 = true;
                        }
                        else
                        {
                            if (Codes.Digit0 <= code__380)
                            {
                                t___925 = code__380 <= Codes.Digit9;
                            }
                            else
                            {
                                t___925 = false;
                            }
                            if (t___925)
                            {
                                t___929 = true;
                            }
                            else
                            {
                                if (Codes.UpperA <= code__380)
                                {
                                    t___926 = code__380 <= Codes.UpperZ;
                                }
                                else
                                {
                                    t___926 = false;
                                }
                                if (t___926)
                                {
                                    t___928 = true;
                                }
                                else
                                {
                                    if (Codes.LowerA <= code__380)
                                    {
                                        t___927 = code__380 <= Codes.LowerZ;
                                    }
                                    else
                                    {
                                        t___927 = false;
                                    }
                                    t___928 = t___927;
                                }
                                t___929 = t___928;
                            }
                            t___930 = t___929;
                        }
                        t___931 = t___930;
                    }
                    t___932 = t___931;
                }
                if (t___932)
                {
                    t___949 = 0;
                }
                else
                {
                    if (code__380 == Codes.Ampersand)
                    {
                        t___948 = true;
                    }
                    else
                    {
                        if (code__380 == Codes.Backslash)
                        {
                            t___947 = true;
                        }
                        else
                        {
                            if (code__380 == Codes.Caret)
                            {
                                t___946 = true;
                            }
                            else
                            {
                                if (code__380 == Codes.CurlyLeft)
                                {
                                    t___945 = true;
                                }
                                else
                                {
                                    if (code__380 == Codes.CurlyRight)
                                    {
                                        t___944 = true;
                                    }
                                    else
                                    {
                                        if (code__380 == Codes.Dot)
                                        {
                                            t___943 = true;
                                        }
                                        else
                                        {
                                            if (code__380 == Codes.Peso)
                                            {
                                                t___942 = true;
                                            }
                                            else
                                            {
                                                if (code__380 == Codes.Pipe)
                                                {
                                                    t___941 = true;
                                                }
                                                else
                                                {
                                                    if (code__380 == Codes.Plus)
                                                    {
                                                        t___940 = true;
                                                    }
                                                    else
                                                    {
                                                        if (code__380 == Codes.Question)
                                                        {
                                                            t___939 = true;
                                                        }
                                                        else
                                                        {
                                                            if (code__380 == Codes.RoundLeft)
                                                            {
                                                                t___938 = true;
                                                            }
                                                            else
                                                            {
                                                                if (code__380 == Codes.RoundRight)
                                                                {
                                                                    t___937 = true;
                                                                }
                                                                else
                                                                {
                                                                    if (code__380 == Codes.Slash)
                                                                    {
                                                                        t___936 = true;
                                                                    }
                                                                    else
                                                                    {
                                                                        if (code__380 == Codes.SquareLeft)
                                                                        {
                                                                            t___935 = true;
                                                                        }
                                                                        else
                                                                        {
                                                                            if (code__380 == Codes.SquareRight)
                                                                            {
                                                                                t___934 = true;
                                                                            }
                                                                            else
                                                                            {
                                                                                if (code__380 == Codes.Star)
                                                                                {
                                                                                    t___933 = true;
                                                                                }
                                                                                else
                                                                                {
                                                                                    t___933 = code__380 == Codes.Tilde;
                                                                                }
                                                                                t___934 = t___933;
                                                                            }
                                                                            t___935 = t___934;
                                                                        }
                                                                        t___936 = t___935;
                                                                    }
                                                                    t___937 = t___936;
                                                                }
                                                                t___938 = t___937;
                                                            }
                                                            t___939 = t___938;
                                                        }
                                                        t___940 = t___939;
                                                    }
                                                    t___941 = t___940;
                                                }
                                                t___942 = t___941;
                                            }
                                            t___943 = t___942;
                                        }
                                        t___944 = t___943;
                                    }
                                    t___945 = t___944;
                                }
                                t___946 = t___945;
                            }
                            t___947 = t___946;
                        }
                        t___948 = t___947;
                    }
                    if (t___948)
                    {
                        t___949 = 2;
                    }
                    else
                    {
                        t___949 = 1;
                    }
                }
                C::Listed.Add(escapeNeeds__379, t___949);
                code__380 = code__380 + 1;
            }
            return C::Listed.ToReadOnlyList(escapeNeeds__379);
        }
        internal static G::IReadOnlyList<int> return__990;
        internal static G::IReadOnlyList<int> escapeNeeds__163;
        internal static RegexRefs return__991;
        internal static RegexRefs regexRefs__162;
        public static IRegexNode Entire(IRegexNode item__226)
        {
            return new Sequence(C::Listed.CreateReadOnlyList<IRegexNode>(Begin, item__226, End));
        }
        public static Repeat OneOrMore(IRegexNode item__228, bool ? reluctant = null)
        {
            bool reluctant__229;
            if (reluctant == null)
            {
                reluctant__229 = false;
            }
            else
            {
                reluctant__229 = reluctant.Value;
            }
            return new Repeat(item__228, 1, null, reluctant__229);
        }
        public static Repeat Optional(IRegexNode item__231, bool ? reluctant = null)
        {
            bool reluctant__232;
            if (reluctant == null)
            {
                reluctant__232 = false;
            }
            else
            {
                reluctant__232 = reluctant.Value;
            }
            return new Repeat(item__231, 0, 1, reluctant__232);
        }
        static RegexGlobal()
        {
            return__190 = new Begin();
            Begin = return__190;
            return__192 = new Dot();
            Dot = return__192;
            return__194 = new End();
            End = return__194;
            return__196 = new WordBoundary();
            WordBoundary = return__196;
            return__198 = new Digit();
            Digit = return__198;
            return__200 = new Space();
            Space = return__200;
            return__202 = new Word();
            Word = return__202;
            return__990 = buildEscapeNeeds__161();
            escapeNeeds__163 = return__990;
            return__991 = new RegexRefs();
            regexRefs__162 = return__991;
        }
    }
}
