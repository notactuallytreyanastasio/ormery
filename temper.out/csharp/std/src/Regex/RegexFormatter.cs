using S = System;
using T = System.Text;
using C = TemperLang.Core;
using R = TemperLang.Std.Regex;
namespace TemperLang.Std.Regex
{
    class RegexFormatter
    {
        readonly T::StringBuilder out__301;
        public static string RegexFormat(IRegexNode data__307)
        {
            return new RegexFormatter().Format(data__307);
        }
        public string Format(IRegexNode regex__310)
        {
            this.PushRegex(regex__310);
            return this.out__301.ToString();
        }
        void PushRegex(IRegexNode regex__313)
        {
            Capture t___884;
            CodePoints t___885;
            CodeRange t___886;
            CodeSet t___887;
            Or t___888;
            Repeat t___889;
            Sequence t___890;
            if (regex__313 is Capture)
            {
                t___884 = (Capture) regex__313;
                this.PushCapture(t___884);
            }
            else if (regex__313 is CodePoints)
            {
                t___885 = (CodePoints) regex__313;
                this.PushCodePoints(t___885, false);
            }
            else if (regex__313 is CodeRange)
            {
                t___886 = (CodeRange) regex__313;
                this.PushCodeRange(t___886);
            }
            else if (regex__313 is CodeSet)
            {
                t___887 = (CodeSet) regex__313;
                this.PushCodeSet(t___887);
            }
            else if (regex__313 is Or)
            {
                t___888 = (Or) regex__313;
                this.PushOr(t___888);
            }
            else if (regex__313 is Repeat)
            {
                t___889 = (Repeat) regex__313;
                this.PushRepeat(t___889);
            }
            else if (regex__313 is Sequence)
            {
                t___890 = (Sequence) regex__313;
                this.PushSequence(t___890);
            }
            else if (regex__313 == R::RegexGlobal.Begin) this.out__301.Append("^");
            else if (regex__313 == R::RegexGlobal.Dot) this.out__301.Append(".");
            else if (regex__313 == R::RegexGlobal.End) this.out__301.Append("\u0024");
            else if (regex__313 == R::RegexGlobal.WordBoundary) this.out__301.Append("\\b");
            else if (regex__313 == R::RegexGlobal.Digit) this.out__301.Append("\\d");
            else if (regex__313 == R::RegexGlobal.Space) this.out__301.Append("\\s");
            else if (regex__313 == R::RegexGlobal.Word) this.out__301.Append("\\w");
        }
        void PushCapture(Capture capture__316)
        {
            this.out__301.Append("(");
            T::StringBuilder t___858 = this.out__301;
            string t___1242 = capture__316.Name;
            this.PushCaptureName(t___858, t___1242);
            IRegexNode t___1244 = capture__316.Item;
            this.PushRegex(t___1244);
            this.out__301.Append(")");
        }
        void PushCaptureName(T::StringBuilder out__319, string name__320)
        {
            out__319.Append("?<" + name__320 + ">");
        }
        void PushCode(int code__323, bool insideCodeSet__324)
        {
            bool t___846;
            bool t___847;
            string t___848;
            string t___850;
            bool t___851;
            bool t___852;
            bool t___853;
            bool t___854;
            string t___855;
            {
                {
                    string specialEscape__326;
                    if (code__323 == Codes.CarriageReturn)
                    {
                        specialEscape__326 = "r";
                    }
                    else if (code__323 == Codes.Newline)
                    {
                        specialEscape__326 = "n";
                    }
                    else if (code__323 == Codes.Tab)
                    {
                        specialEscape__326 = "t";
                    }
                    else
                    {
                        specialEscape__326 = "";
                    }
                    if (specialEscape__326 != "")
                    {
                        this.out__301.Append("\\");
                        this.out__301.Append(specialEscape__326);
                        goto fn__325;
                    }
                    if (code__323 <= 127)
                    {
                        int escapeNeed__327 = R::RegexGlobal.escapeNeeds__163[code__323];
                        if (escapeNeed__327 == 2)
                        {
                            t___847 = true;
                        }
                        else
                        {
                            if (insideCodeSet__324)
                            {
                                t___846 = code__323 == Codes.Dash;
                            }
                            else
                            {
                                t___846 = false;
                            }
                            t___847 = t___846;
                        }
                        if (t___847)
                        {
                            this.out__301.Append("\\");
                            t___848 = C::Core.StringFromCodePoint(code__323);
                            this.out__301.Append(t___848);
                            goto fn__325;
                        }
                        else if (escapeNeed__327 == 0)
                        {
                            t___850 = C::Core.StringFromCodePoint(code__323);
                            this.out__301.Append(t___850);
                            goto fn__325;
                        }
                    }
                    if (code__323 >= Codes.SupplementalMin)
                    {
                        t___854 = true;
                    }
                    else
                    {
                        if (code__323 > Codes.HighControlMax)
                        {
                            if (Codes.SurrogateMin <= code__323)
                            {
                                t___851 = code__323 <= Codes.SurrogateMax;
                            }
                            else
                            {
                                t___851 = false;
                            }
                            if (t___851)
                            {
                                t___852 = true;
                            }
                            else
                            {
                                t___852 = code__323 == Codes.Uint16_Max;
                            }
                            t___853 = !t___852;
                        }
                        else
                        {
                            t___853 = false;
                        }
                        t___854 = t___853;
                    }
                    if (t___854)
                    {
                        t___855 = C::Core.StringFromCodePoint(code__323);
                        this.out__301.Append(t___855);
                    }
                    else R::RegexSupport.PushCodeTo(this, this.out__301, code__323, insideCodeSet__324);
                }
                fn__325:
                {
                }
            }
        }
        void PushCodePoints(CodePoints codePoints__334, bool insideCodeSet__335)
        {
            int t___1229;
            int t___1231;
            string value__337 = codePoints__334.Value;
            int index__338 = 0;
            while (true)
            {
                if (!C::StringUtil.HasIndex(value__337, index__338)) break;
                t___1229 = C::StringUtil.Get(value__337, index__338);
                this.PushCode(t___1229, insideCodeSet__335);
                t___1231 = C::StringUtil.Next(value__337, index__338);
                index__338 = t___1231;
            }
        }
        void PushCodeRange(CodeRange codeRange__340)
        {
            this.out__301.Append("[");
            this.PushCodeRangeUnwrapped(codeRange__340);
            this.out__301.Append("]");
        }
        void PushCodeRangeUnwrapped(CodeRange codeRange__343)
        {
            int t___1219 = codeRange__343.Min;
            this.PushCode(t___1219, true);
            this.out__301.Append("-");
            int t___1222 = codeRange__343.Max;
            this.PushCode(t___1222, true);
        }
        void PushCodeSet(CodeSet codeSet__346)
        {
            int t___1213;
            ICodePart t___1215;
            CodeSet t___831;
            IRegexNode adjusted__348 = R::RegexSupport.AdjustCodeSet(this, codeSet__346, R::RegexGlobal.regexRefs__162);
            if (adjusted__348 is CodeSet)
            {
                t___831 = (CodeSet) adjusted__348;
                this.out__301.Append("[");
                if (t___831.Negated) this.out__301.Append("^");
                int i__349 = 0;
                while (true)
                {
                    t___1213 = t___831.Items.Count;
                    if (!(i__349 < t___1213)) break;
                    t___1215 = t___831.Items[i__349];
                    this.PushCodeSetItem(t___1215);
                    i__349 = i__349 + 1;
                }
                this.out__301.Append("]");
            }
            else this.PushRegex(adjusted__348);
        }
        void PushCodeSetItem(ICodePart codePart__355)
        {
            CodePoints t___819;
            CodeRange t___820;
            ISpecialSet t___821;
            if (codePart__355 is CodePoints)
            {
                t___819 = (CodePoints) codePart__355;
                this.PushCodePoints(t___819, true);
            }
            else if (codePart__355 is CodeRange)
            {
                t___820 = (CodeRange) codePart__355;
                this.PushCodeRangeUnwrapped(t___820);
            }
            else if (codePart__355 is ISpecialSet)
            {
                t___821 = (ISpecialSet) codePart__355;
                this.PushRegex(t___821);
            }
        }
        void PushOr(Or or__358)
        {
            IRegexNode t___1192;
            int t___1195;
            IRegexNode t___1198;
            if (!(or__358.Items.Count == 0))
            {
                this.out__301.Append("(?:");
                t___1192 = or__358.Items[0];
                this.PushRegex(t___1192);
                int i__360 = 1;
                while (true)
                {
                    t___1195 = or__358.Items.Count;
                    if (!(i__360 < t___1195)) break;
                    this.out__301.Append("|");
                    t___1198 = or__358.Items[i__360];
                    this.PushRegex(t___1198);
                    i__360 = i__360 + 1;
                }
                this.out__301.Append(")");
            }
        }
        void PushRepeat(Repeat repeat__362)
        {
            string t___1180;
            string t___1183;
            bool t___796;
            bool t___797;
            bool t___798;
            this.out__301.Append("(?:");
            IRegexNode t___1172 = repeat__362.Item;
            this.PushRegex(t___1172);
            this.out__301.Append(")");
            int min__364 = repeat__362.Min;
            int ? max__365 = repeat__362.Max;
            if (min__364 == 0)
            {
                t___796 = max__365 == 1;
            }
            else
            {
                t___796 = false;
            }
            if (t___796) this.out__301.Append("?");
            else
            {
                if (min__364 == 0)
                {
                    t___797 = max__365 == null;
                }
                else
                {
                    t___797 = false;
                }
                if (t___797) this.out__301.Append("*");
                else
                {
                    if (min__364 == 1)
                    {
                        t___798 = max__365 == null;
                    }
                    else
                    {
                        t___798 = false;
                    }
                    if (t___798) this.out__301.Append("+");
                    else
                    {
                        t___1180 = S::Convert.ToString(min__364);
                        this.out__301.Append("{" + t___1180);
                        if (min__364 != max__365)
                        {
                            this.out__301.Append(",");
                            if (!(max__365 == null))
                            {
                                t___1183 = S::Convert.ToString(max__365.Value);
                                this.out__301.Append(t___1183);
                            }
                        }
                        this.out__301.Append("}");
                    }
                }
            }
            if (repeat__362.Reluctant) this.out__301.Append("?");
        }
        void PushSequence(Sequence sequence__367)
        {
            int t___1167;
            IRegexNode t___1169;
            int i__369 = 0;
            while (true)
            {
                t___1167 = sequence__367.Items.Count;
                if (!(i__369 < t___1167)) break;
                t___1169 = sequence__367.Items[i__369];
                this.PushRegex(t___1169);
                i__369 = i__369 + 1;
            }
        }
        public int ? MaxCode(ICodePart codePart__371)
        {
            int ? return__157;
            int t___1163;
            CodePoints t___784;
            if (codePart__371 is CodePoints)
            {
                t___784 = (CodePoints) codePart__371;
                string value__373 = t___784.Value;
                if (string.IsNullOrEmpty(value__373))
                {
                    return__157 = null;
                }
                else
                {
                    int max__374 = 0;
                    int index__375 = 0;
                    while (true)
                    {
                        if (!C::StringUtil.HasIndex(value__373, index__375)) break;
                        int next__376 = C::StringUtil.Get(value__373, index__375);
                        if (next__376 > max__374)
                        {
                            max__374 = next__376;
                        }
                        t___1163 = C::StringUtil.Next(value__373, index__375);
                        index__375 = t___1163;
                    }
                    return__157 = max__374;
                }
            }
            else if (codePart__371 is CodeRange)
            {
                return__157 = ((CodeRange) codePart__371).Max;
            }
            else if (codePart__371 == R::RegexGlobal.Digit)
            {
                return__157 = Codes.Digit9;
            }
            else if (codePart__371 == R::RegexGlobal.Space)
            {
                return__157 = Codes.Space;
            }
            else if (codePart__371 == R::RegexGlobal.Word)
            {
                return__157 = Codes.LowerZ;
            }
            else
            {
                return__157 = null;
            }
            return return__157;
        }
        public RegexFormatter()
        {
            T::StringBuilder t___1157 = new T::StringBuilder();
            this.out__301 = t___1157;
        }
    }
}
