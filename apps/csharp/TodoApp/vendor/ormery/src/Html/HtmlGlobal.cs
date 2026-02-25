using C0 = Ormery.Core;
using C1 = TemperLang.Core;
using S = System;
using G = System.Collections.Generic;
using R = TemperLang.Std.Regex;
namespace Ormery.Html
{
    public static class HtmlGlobal
    {
        internal static bool t___3449;
        internal static bool t___3450;
        internal static bool t___3451;
        internal static bool t___3452;
        internal static bool t___3453;
        internal static bool t___3454;
        internal static bool t___3455;
        internal static bool t___3461;
        internal static bool t___3462;
        internal static bool t___3463;
        internal static bool t___3464;
        internal static G::IReadOnlyList<string> strs__375;
        internal static G::IDictionary<string, string> mb__376;
        internal static int i__377;
        internal static int n__378;
        internal static G::IReadOnlyDictionary<string, string> return__374;
        internal static G::IReadOnlyDictionary<string, string> htmlNamedCharacters__267;
        internal static HtmlCodec return__373;
        public static C0::ICodec HtmlCodec;
        internal static string htmlStateStr__260(int x__379)
        {
            string return__129;
            if (x__379 == 0)
            {
                return__129 = "Pcdata";
            }
            else if (x__379 == 1)
            {
                return__129 = "OName";
            }
            else if (x__379 == 2)
            {
                return__129 = "CName";
            }
            else if (x__379 == 3)
            {
                return__129 = "BeforeAttr";
            }
            else if (x__379 == 4)
            {
                return__129 = "BeforeEq";
            }
            else if (x__379 == 5)
            {
                return__129 = "BeforeValue";
            }
            else if (x__379 == 6)
            {
                return__129 = "Attr";
            }
            else if (x__379 == 7)
            {
                return__129 = "AfterAttr";
            }
            else if (x__379 == 8)
            {
                return__129 = "SpecialBody";
            }
            else
            {
                return__129 = S::Convert.ToString(x__379);
            }
            return return__129;
        }
        internal static string tagStateStr__261(int x__381)
        {
            return S::Convert.ToString(x__381);
        }
        internal static string attribStateStr__262(int x__383)
        {
            string return__131;
            if (x__383 == 0)
            {
                return__131 = "Generic";
            }
            else if (x__383 == 1)
            {
                return__131 = "Css";
            }
            else if (x__383 == 2)
            {
                return__131 = "Js";
            }
            else if (x__383 == 3)
            {
                return__131 = "Url";
            }
            else if (x__383 == 4)
            {
                return__131 = "Urls";
            }
            else
            {
                return__131 = S::Convert.ToString(x__383);
            }
            return return__131;
        }
        internal static string delimStateStr__263(int x__385)
        {
            string return__132;
            if (x__385 == 0)
            {
                return__132 = "Uq";
            }
            else if (x__385 == 1)
            {
                return__132 = "Sq";
            }
            else if (x__385 == 2)
            {
                return__132 = "Dq";
            }
            else
            {
                return__132 = S::Convert.ToString(x__385);
            }
            return return__132;
        }
        internal static string urlStateStr__265(int x__456)
        {
            string return__137;
            if (x__456 == 0)
            {
                return__137 = "Start";
            }
            else if (x__456 == 1)
            {
                return__137 = "BeforeQuery";
            }
            else if (x__456 == 2)
            {
                return__137 = "Query";
            }
            else if (x__456 == 3)
            {
                return__137 = "Fragment";
            }
            else
            {
                return__137 = S::Convert.ToString(x__456);
            }
            return return__137;
        }
        internal static C0::AfterPropagate<UrlEscaperContext> urlPropagateContext__266(C0::AutoescState<UrlEscaperContext> before__458, string ? literalPart__459)
        {
            C0::AfterPropagate<UrlEscaperContext> return__138;
            R::CodeSet t___5455;
            R::CodeSet t___5459;
            R::Repeat t___5464;
            R::Or t___5482;
            R::Repeat t___5488;
            string t___5493;
            int t___5495;
            C0::AutoescState<UrlEscaperContext> t___5498;
            string t___5502;
            int t___5504;
            C0::AutoescState<UrlEscaperContext> t___5507;
            string t___5511;
            int t___5513;
            C0::AutoescState<UrlEscaperContext> t___5516;
            string t___5520;
            int t___5522;
            string t___5526;
            int t___5528;
            C0::AutoescState<UrlEscaperContext> t___5531;
            string t___5535;
            int t___5537;
            C0::AutoescState<UrlEscaperContext> t___5540;
            string t___5544;
            int t___5546;
            string t___5550;
            int t___5552;
            C0::AutoescState<UrlEscaperContext> t___5555;
            R::Match ? t___3204;
            R::Match ? t___3215;
            R::Match ? t___3226;
            R::Match ? t___3237;
            R::Match ? t___3245;
            R::Match ? t___3256;
            R::Match ? t___3267;
            R::Match ? t___3275;
            {
                {
                    UrlEscaperContext contextBefore__461 = before__458.Context;
                    t___5455 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("#")), false);
                    R::Regex pattern0__462 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5455)).Compiled();
                    t___5459 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("?")), false);
                    R::Regex pattern1__463 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5459)).Compiled();
                    t___5464 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("#")), true), 1, null, false);
                    R::Regex pattern2__464 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5464)).Compiled();
                    t___5482 = new R::Or(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints(":"), new R::CodePoints("#"), new R::CodePoints("?")), true), 0, null, false), new R::CodePoints(":"))), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" "), new R::CodePoints(":"), new R::CodePoints("#"), new R::CodePoints("?")), true)));
                    R::Regex pattern3__465 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5482)).Compiled();
                    t___5488 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("?"), new R::CodePoints("#")), true), 1, null, false);
                    R::Regex pattern4__466 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5488)).Compiled();
                    if (!(literalPart__459 == null))
                    {
                        string literalPart___992 = literalPart__459!;
                        if (contextBefore__461.UrlState == 0)
                        {
                            R::Match ? match__467;
                            try
                            {
                                t___3204 = pattern3__465.Find(literalPart___992);
                                match__467 = t___3204;
                            }
                            catch
                            {
                                match__467 = null;
                            }
                            if (!(match__467 == null))
                            {
                                R::Match match___993 = match__467!;
                                t___5493 = match___993.Full.Value;
                                t___5495 = match___993.Full.End;
                                t___5498 = new C0::AutoescState<UrlEscaperContext>(new UrlEscaperContext(1), before__458.Subsidiary);
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5493, t___5495, t___5498);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 0)
                        {
                            R::Match ? match__468;
                            try
                            {
                                t___3215 = pattern1__463.Find(literalPart___992);
                                match__468 = t___3215;
                            }
                            catch
                            {
                                match__468 = null;
                            }
                            if (!(match__468 == null))
                            {
                                R::Match match___994 = match__468!;
                                t___5502 = match___994.Full.Value;
                                t___5504 = match___994.Full.End;
                                t___5507 = new C0::AutoescState<UrlEscaperContext>(new UrlEscaperContext(2), before__458.Subsidiary);
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5502, t___5504, t___5507);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 0)
                        {
                            R::Match ? match__469;
                            try
                            {
                                t___3226 = pattern0__462.Find(literalPart___992);
                                match__469 = t___3226;
                            }
                            catch
                            {
                                match__469 = null;
                            }
                            if (!(match__469 == null))
                            {
                                R::Match match___995 = match__469!;
                                t___5511 = match___995.Full.Value;
                                t___5513 = match___995.Full.End;
                                t___5516 = new C0::AutoescState<UrlEscaperContext>(new UrlEscaperContext(3), before__458.Subsidiary);
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5511, t___5513, t___5516);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 1)
                        {
                            R::Match ? match__470;
                            try
                            {
                                t___3237 = pattern4__466.Find(literalPart___992);
                                match__470 = t___3237;
                            }
                            catch
                            {
                                match__470 = null;
                            }
                            if (!(match__470 == null))
                            {
                                R::Match match___996 = match__470!;
                                t___5520 = match___996.Full.Value;
                                t___5522 = match___996.Full.End;
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5520, t___5522, before__458);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 1)
                        {
                            R::Match ? match__471;
                            try
                            {
                                t___3245 = pattern1__463.Find(literalPart___992);
                                match__471 = t___3245;
                            }
                            catch
                            {
                                match__471 = null;
                            }
                            if (!(match__471 == null))
                            {
                                R::Match match___997 = match__471!;
                                t___5526 = match___997.Full.Value;
                                t___5528 = match___997.Full.End;
                                t___5531 = new C0::AutoescState<UrlEscaperContext>(new UrlEscaperContext(2), before__458.Subsidiary);
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5526, t___5528, t___5531);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 1)
                        {
                            R::Match ? match__472;
                            try
                            {
                                t___3256 = pattern0__462.Find(literalPart___992);
                                match__472 = t___3256;
                            }
                            catch
                            {
                                match__472 = null;
                            }
                            if (!(match__472 == null))
                            {
                                R::Match match___998 = match__472!;
                                t___5535 = match___998.Full.Value;
                                t___5537 = match___998.Full.End;
                                t___5540 = new C0::AutoescState<UrlEscaperContext>(new UrlEscaperContext(3), before__458.Subsidiary);
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5535, t___5537, t___5540);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 2)
                        {
                            R::Match ? match__473;
                            try
                            {
                                t___3267 = pattern2__464.Find(literalPart___992);
                                match__473 = t___3267;
                            }
                            catch
                            {
                                match__473 = null;
                            }
                            if (!(match__473 == null))
                            {
                                R::Match match___999 = match__473!;
                                t___5544 = match___999.Full.Value;
                                t___5546 = match___999.Full.End;
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5544, t___5546, before__458);
                                goto fn__460;
                            }
                        }
                        if (contextBefore__461.UrlState == 2)
                        {
                            R::Match ? match__474;
                            try
                            {
                                t___3275 = pattern0__462.Find(literalPart___992);
                                match__474 = t___3275;
                            }
                            catch
                            {
                                match__474 = null;
                            }
                            if (!(match__474 == null))
                            {
                                R::Match match___1000 = match__474!;
                                t___5550 = match___1000.Full.Value;
                                t___5552 = match___1000.Full.End;
                                t___5555 = new C0::AutoescState<UrlEscaperContext>(new UrlEscaperContext(3), before__458.Subsidiary);
                                return__138 = new C0::AfterPropagate<UrlEscaperContext>(t___5550, t___5552, t___5555);
                                goto fn__460;
                            }
                        }
                    }
                    if (literalPart__459 == null)
                    {
                        return__138 = new C0::AfterPropagate<UrlEscaperContext>("", 0, before__458);
                        goto fn__460;
                    }
                    throw new S::Exception();
                }
                fn__460:
                {
                }
            }
            return return__138;
        }
        internal static UrlContextPropagator urlContextPropagator__268;
        internal static R::Regex protocolAllowList__293;
        internal static SafeUrl fallbackSafeUrl__294;
        internal static G::IList<bool> lb__732;
        internal static int i__733;
        internal static G::IReadOnlyList<bool> urlQuerySafe__296;
        internal static G::IList<bool> lb__734;
        internal static int i__735;
        internal static G::IReadOnlyList<bool> urlSafe__297;
        internal static HtmlUrlPartUrlEscaper return__749;
        internal static HtmlUrlPartUrlEscaper htmlUrlPartUrlEscaper__298;
        internal static HtmlProtocolFilteringUrlEscaper return__731;
        internal static HtmlProtocolFilteringUrlEscaper htmlProtocolFilteringUrlEscaper__295;
        internal static HtmlAsIfQueryUrlEscaper return__763;
        internal static HtmlAsIfQueryUrlEscaper htmlAsIfQueryUrlEscaper__299;
        internal static C0::AfterPropagate<HtmlEscaperContext> htmlPropagateContext__264(C0::AutoescState<HtmlEscaperContext> before__387, string ? literalPart__388)
        {
            C0::AfterPropagate<HtmlEscaperContext> return__133;
            R::CodePoints t___4745;
            R::Sequence t___4754;
            R::CodePoints t___4757;
            R::Sequence t___4766;
            R::CodePoints t___4769;
            R::CodeSet t___4777;
            R::CodeSet t___4785;
            R::Sequence t___4806;
            R::Or t___4832;
            R::Sequence t___4844;
            R::CodeSet t___4852;
            R::CodeSet t___4857;
            R::CodePoints t___4860;
            R::CodePoints t___4863;
            R::CodePoints t___4866;
            R::CodePoints t___4869;
            R::CodePoints t___4872;
            R::Sequence t___4915;
            R::Sequence t___4932;
            R::Sequence t___4950;
            R::Repeat t___4958;
            R::Repeat t___4963;
            R::Repeat t___4968;
            R::Repeat t___4974;
            R::Repeat t___4984;
            R::Repeat t___4994;
            R::CodeSet t___4998;
            R::Sequence t___5008;
            R::Sequence t___5021;
            string t___5030;
            int t___5032;
            C0::AutoescState<HtmlEscaperContext> t___5038;
            string t___5046;
            int t___5048;
            C0::AutoescState<HtmlEscaperContext> t___5054;
            int t___5058;
            int t___5062;
            string t___5066;
            int t___5068;
            string t___5072;
            int t___5074;
            string t___5078;
            int t___5080;
            string t___5084;
            int t___5086;
            string t___5090;
            int t___5092;
            C0::AutoescState<HtmlEscaperContext> t___5098;
            string t___5102;
            int t___5104;
            C0::AutoescState<HtmlEscaperContext> t___5112;
            string t___5116;
            int t___5118;
            C0::AutoescState<HtmlEscaperContext> t___5124;
            string t___5128;
            int t___5130;
            string t___5134;
            int t___5136;
            C0::AfterPropagate<HtmlEscaperContext> t___5152;
            HtmlUrlDelegate t___5153;
            C0::AfterPropagate<HtmlEscaperContext> t___5169;
            HtmlUrlDelegate t___5170;
            C0::AfterPropagate<HtmlEscaperContext> t___5182;
            HtmlUrlDelegate t___5183;
            C0::AfterPropagate<HtmlEscaperContext> t___5199;
            HtmlCssDelegate t___5200;
            C0::AfterPropagate<HtmlEscaperContext> t___5212;
            HtmlJsDelegate t___5213;
            string t___5217;
            int t___5219;
            C0::AutoescState<HtmlEscaperContext> t___5225;
            string t___5229;
            int t___5231;
            string t___5235;
            int t___5237;
            C0::AutoescState<HtmlEscaperContext> t___5243;
            C0::AutoescState<HtmlEscaperContext> t___5251;
            string t___5255;
            int t___5257;
            C0::AutoescState<HtmlEscaperContext> t___5262;
            string t___5266;
            int t___5268;
            C0::AutoescState<HtmlEscaperContext> t___5273;
            C0::AutoescState<HtmlEscaperContext> t___5280;
            int t___5282;
            C0::AutoescState<HtmlEscaperContext> t___5287;
            int t___5290;
            C0::AutoescState<HtmlEscaperContext> t___5295;
            int t___5298;
            string t___5300;
            int t___5302;
            C0::AutoescState<HtmlEscaperContext> t___5307;
            int t___5310;
            string t___5312;
            int t___5314;
            C0::AutoescState<HtmlEscaperContext> t___5319;
            int t___5322;
            C0::AfterPropagate<HtmlEscaperContext> t___5328;
            HtmlUrlDelegate t___5329;
            int t___5332;
            C0::AfterPropagate<HtmlEscaperContext> t___5337;
            int t___5340;
            C0::AfterPropagate<HtmlEscaperContext> t___5343;
            int t___5346;
            C0::AfterPropagate<HtmlEscaperContext> t___5351;
            int t___5354;
            C0::AfterPropagate<HtmlEscaperContext> t___5359;
            int t___5362;
            int t___5364;
            C0::AfterPropagate<HtmlEscaperContext> t___5365;
            int t___5368;
            C0::AutoescState<HtmlEscaperContext> t___5374;
            C0::AfterPropagate<HtmlEscaperContext> t___5383;
            string t___5387;
            int t___5389;
            C0::AutoescState<HtmlEscaperContext> t___5395;
            R::Match ? t___2619;
            R::Match t___2625;
            R::Match ? t___2626;
            R::Match ? t___2639;
            R::Match t___2645;
            R::Match ? t___2646;
            R::Match ? t___2659;
            R::Match ? t___2665;
            R::Match ? t___2671;
            R::Match ? t___2679;
            R::Match ? t___2687;
            R::Match ? t___2695;
            R::Match ? t___2703;
            R::Match ? t___2717;
            R::Match t___2725;
            R::Match ? t___2726;
            R::Match ? t___2736;
            R::Match ? t___2750;
            R::Match ? t___2758;
            R::Match ? t___2766;
            R::Match t___2772;
            R::Match ? t___2773;
            R::Match ? t___2787;
            R::Match t___2793;
            R::Match ? t___2794;
            R::Match ? t___2808;
            R::Match ? t___2823;
            R::Match t___2829;
            R::Match ? t___2830;
            R::Match ? t___2844;
            R::Match ? t___2859;
            R::Match ? t___2873;
            R::Match ? t___2881;
            R::Match t___2895;
            R::Match ? t___2896;
            R::Match ? t___2906;
            R::Match ? t___2919;
            R::Match t___2932;
            R::Match ? t___2933;
            bool t___2942;
            bool t___2952;
            R::Match t___2953;
            R::Match ? t___2954;
            bool t___2964;
            R::Match ? t___2965;
            bool t___2979;
            R::Match ? t___2980;
            bool t___2994;
            R::Match ? t___2995;
            bool t___3007;
            R::Match ? t___3008;
            bool t___3018;
            R::Match ? t___3019;
            bool t___3027;
            R::Match ? t___3028;
            bool t___3038;
            R::Match ? t___3039;
            bool t___3049;
            bool t___3050;
            bool t___3053;
            bool t___3058;
            R::Match ? t___3077;
            {
                {
                    HtmlEscaperContext contextBefore__390 = before__387.Context;
                    t___4745 = new R::CodePoints("\u0022");
                    R::Regex pattern0__391 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4745)).Compiled();
                    t___4754 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::CodePoints("\u0022"), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("\u0022")), true), 0, null, false), new R::Repeat(new R::CodePoints("\u0022"), 0, 1, false)));
                    R::Regex pattern1__392 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4754)).Compiled();
                    t___4757 = new R::CodePoints("'");
                    R::Regex pattern2__393 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4757)).Compiled();
                    t___4766 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::CodePoints("'"), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("'")), true), 0, null, false), new R::Repeat(new R::CodePoints("'"), 0, 1, false)));
                    R::Regex pattern3__394 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4766)).Compiled();
                    t___4769 = new R::CodePoints(">");
                    R::Regex pattern4__395 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4769)).Compiled();
                    t___4777 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints(">"), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" ")), false);
                    R::Regex pattern5__396 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4777)).Compiled();
                    t___4785 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::ICodePart>(new R::CodeRange(65, 90), new R::CodeRange(97, 122), new R::CodeRange(48, 57), new R::CodeRange(58, 58), new R::CodePoints("-")), false);
                    R::Regex pattern6__397 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4785)).Compiled();
                    t___4806 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::CodeSet>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("S"), new R::CodePoints("s")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("R"), new R::CodePoints("r")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("C"), new R::CodePoints("c")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("S"), new R::CodePoints("s")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("E"), new R::CodePoints("e")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("T"), new R::CodePoints("t")), false)));
                    R::Regex pattern7__398 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4806)).Compiled();
                    t___4832 = new R::Or(C1::Listed.CreateReadOnlyList<R::Sequence>(new R::Sequence(C1::Listed.CreateReadOnlyList<R::CodeSet>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("S"), new R::CodePoints("s")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("R"), new R::CodePoints("r")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("C"), new R::CodePoints("c")), false))), new R::Sequence(C1::Listed.CreateReadOnlyList<R::CodeSet>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("H"), new R::CodePoints("h")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("R"), new R::CodePoints("r")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("E"), new R::CodePoints("e")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("F"), new R::CodePoints("f")), false)))));
                    R::Regex pattern8__399 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4832)).Compiled();
                    t___4844 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" ")), false), 0, null, false), new R::Repeat(new R::CodePoints("/"), 0, 1, false), new R::CodePoints(">")));
                    R::Regex pattern9__400 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4844)).Compiled();
                    t___4852 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints(">"), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" ")), true);
                    R::Regex pattern10__401 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4852)).Compiled();
                    t___4857 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodeRange>(new R::CodeRange(97, 122), new R::CodeRange(65, 90)), false);
                    R::Regex pattern11__402 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4857)).Compiled();
                    t___4860 = new R::CodePoints(",");
                    R::Regex pattern12__403 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4860)).Compiled();
                    t___4863 = new R::CodePoints("<");
                    R::Regex pattern13__404 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4863)).Compiled();
                    t___4866 = new R::CodePoints("</");
                    R::Regex pattern14__405 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4866)).Compiled();
                    t___4869 = new R::CodePoints("=");
                    R::Regex pattern15__406 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4869)).Compiled();
                    t___4872 = new R::CodePoints(">");
                    R::Regex pattern16__407 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4872)).Compiled();
                    t___4915 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("D"), new R::CodePoints("d")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("A"), new R::CodePoints("a")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("T"), new R::CodePoints("t")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("A"), new R::CodePoints("a")), false), new R::CodePoints("-"), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("="), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" "), new R::CodePoints(">")), true), 0, null, false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("U"), new R::CodePoints("u")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("R"), new R::CodePoints("r")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("L"), new R::CodePoints("l"), new R::CodePoints("I"), new R::CodePoints("i")), false), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("="), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" "), new R::CodePoints(">")), true), 0, null, false)));
                    R::Regex pattern17__408 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4915)).Compiled();
                    t___4932 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("O"), new R::CodePoints("o")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("N"), new R::CodePoints("n")), false), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("="), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" "), new R::CodePoints(">")), true), 0, null, false)));
                    R::Regex pattern18__409 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4932)).Compiled();
                    t___4950 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::CodeSet>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("S"), new R::CodePoints("s")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("T"), new R::CodePoints("t")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("Y"), new R::CodePoints("y")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("L"), new R::CodePoints("l")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("E"), new R::CodePoints("e")), false)));
                    R::Regex pattern19__410 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4950)).Compiled();
                    t___4958 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" ")), false), 1, null, false);
                    R::Regex pattern20__411 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4958)).Compiled();
                    t___4963 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("\u0022")), true), 1, null, false);
                    R::Regex pattern21__412 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4963)).Compiled();
                    t___4968 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("'")), true), 1, null, false);
                    R::Regex pattern22__413 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4968)).Compiled();
                    t___4974 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("<"), new R::CodePoints(">")), true), 1, null, false);
                    R::Regex pattern23__414 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4974)).Compiled();
                    t___4984 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("="), new R::CodePoints(">"), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" ")), true), 1, null, false);
                    R::Regex pattern24__415 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4984)).Compiled();
                    t___4994 = new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints(">"), new R::CodePoints("\t"), new R::CodePoints("\r"), new R::CodePoints("\n"), new R::CodePoints(" "), new R::CodePoints("\u0022")), true), 1, null, false);
                    R::Regex pattern25__416 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4994)).Compiled();
                    t___4998 = new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints(">")), true);
                    R::Regex pattern26__417 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___4998)).Compiled();
                    t___5008 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::ICodePart>(new R::CodeRange(97, 122), new R::CodeRange(65, 90), new R::CodeRange(48, 57), new R::CodePoints("-")), false), 1, null, false), new R::CodePoints(":")));
                    R::Regex pattern27__418 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5008)).Compiled();
                    t___5021 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodeRange>(new R::CodeRange(97, 122), new R::CodeRange(65, 90)), false), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::ICodePart>(new R::CodeRange(97, 122), new R::CodeRange(65, 90), new R::CodeRange(48, 57), new R::CodeRange(58, 58), new R::CodePoints("-")), false), 0, null, false)));
                    R::Regex pattern28__419 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, t___5021)).Compiled();
                    if (!(literalPart__388 == null))
                    {
                        string literalPart___1003 = literalPart__388!;
                        if (contextBefore__390.HtmlState == 0)
                        {
                            R::Match ? match__420;
                            try
                            {
                                t___2619 = pattern14__405.Find(literalPart___1003);
                                match__420 = t___2619;
                            }
                            catch
                            {
                                match__420 = null;
                            }
                            if (!(match__420 == null))
                            {
                                R::Match match___1004 = match__420!;
                                try
                                {
                                    t___2625 = pattern11__402.Find(C1::StringUtil.Slice(literalPart___1003, match___1004.Full.End, literalPart___1003.Length));
                                    t___2626 = t___2625;
                                }
                                catch
                                {
                                    t___2626 = null;
                                }
                                if (!(t___2626 == null))
                                {
                                    t___5030 = match___1004.Full.Value;
                                    t___5032 = match___1004.Full.End;
                                    t___5038 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(2, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                    return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5030, t___5032, t___5038);
                                    goto fn__389;
                                }
                            }
                        }
                        if (contextBefore__390.HtmlState == 0)
                        {
                            R::Match ? match__421;
                            try
                            {
                                t___2639 = pattern13__404.Find(literalPart___1003);
                                match__421 = t___2639;
                            }
                            catch
                            {
                                match__421 = null;
                            }
                            if (!(match__421 == null))
                            {
                                R::Match match___1005 = match__421!;
                                try
                                {
                                    t___2645 = pattern11__402.Find(C1::StringUtil.Slice(literalPart___1003, match___1005.Full.End, literalPart___1003.Length));
                                    t___2646 = t___2645;
                                }
                                catch
                                {
                                    t___2646 = null;
                                }
                                if (!(t___2646 == null))
                                {
                                    t___5046 = match___1005.Full.Value;
                                    t___5048 = match___1005.Full.End;
                                    t___5054 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(1, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                    return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5046, t___5048, t___5054);
                                    goto fn__389;
                                }
                            }
                        }
                        if (contextBefore__390.HtmlState == 0)
                        {
                            R::Match ? match__422;
                            try
                            {
                                t___2659 = pattern13__404.Find(literalPart___1003);
                                match__422 = t___2659;
                            }
                            catch
                            {
                                match__422 = null;
                            }
                            if (!(match__422 == null))
                            {
                                t___5058 = (match__422!).Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>("&lt;", t___5058, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 0)
                        {
                            R::Match ? match__423;
                            try
                            {
                                t___2665 = pattern16__407.Find(literalPart___1003);
                                match__423 = t___2665;
                            }
                            catch
                            {
                                match__423 = null;
                            }
                            if (!(match__423 == null))
                            {
                                t___5062 = (match__423!).Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>("&gt;", t___5062, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 0)
                        {
                            R::Match ? match__424;
                            try
                            {
                                t___2671 = pattern23__414.Find(literalPart___1003);
                                match__424 = t___2671;
                            }
                            catch
                            {
                                match__424 = null;
                            }
                            if (!(match__424 == null))
                            {
                                R::Match match___1008 = match__424!;
                                t___5066 = match___1008.Full.Value;
                                t___5068 = match___1008.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5066, t___5068, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 2)
                        {
                            R::Match ? match__425;
                            try
                            {
                                t___2679 = pattern1__392.Find(literalPart___1003);
                                match__425 = t___2679;
                            }
                            catch
                            {
                                match__425 = null;
                            }
                            if (!(match__425 == null))
                            {
                                R::Match match___1009 = match__425!;
                                t___5072 = match___1009.Full.Value;
                                t___5074 = match___1009.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5072, t___5074, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 2)
                        {
                            R::Match ? match__426;
                            try
                            {
                                t___2687 = pattern3__394.Find(literalPart___1003);
                                match__426 = t___2687;
                            }
                            catch
                            {
                                match__426 = null;
                            }
                            if (!(match__426 == null))
                            {
                                R::Match match___1010 = match__426!;
                                t___5078 = match___1010.Full.Value;
                                t___5080 = match___1010.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5078, t___5080, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 2)
                        {
                            R::Match ? match__427;
                            try
                            {
                                t___2695 = pattern26__417.Find(literalPart___1003);
                                match__427 = t___2695;
                            }
                            catch
                            {
                                match__427 = null;
                            }
                            if (!(match__427 == null))
                            {
                                R::Match match___1011 = match__427!;
                                t___5084 = match___1011.Full.Value;
                                t___5086 = match___1011.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5084, t___5086, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 2)
                        {
                            R::Match ? match__428;
                            try
                            {
                                t___2703 = pattern16__407.Find(literalPart___1003);
                                match__428 = t___2703;
                            }
                            catch
                            {
                                match__428 = null;
                            }
                            if (!(match__428 == null))
                            {
                                R::Match match___1012 = match__428!;
                                t___5090 = match___1012.Full.Value;
                                t___5092 = match___1012.Full.End;
                                t___5098 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(0, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5090, t___5092, t___5098);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 1)
                        {
                            R::Match ? match__429;
                            try
                            {
                                t___2717 = pattern28__419.Find(literalPart___1003);
                                match__429 = t___2717;
                            }
                            catch
                            {
                                match__429 = null;
                            }
                            if (!(match__429 == null))
                            {
                                R::Match match___1013 = match__429!;
                                t___5102 = match___1013.Full.Value;
                                t___5104 = match___1013.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5102, t___5104, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 1)
                        {
                            try
                            {
                                t___2725 = pattern4__395.Find(literalPart___1003);
                                t___2726 = t___2725;
                            }
                            catch
                            {
                                t___2726 = null;
                            }
                            if (!(t___2726 == null))
                            {
                                t___5112 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(3, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, t___5112);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 1)
                        {
                            R::Match ? match__430;
                            try
                            {
                                t___2736 = pattern20__411.Find(literalPart___1003);
                                match__430 = t___2736;
                            }
                            catch
                            {
                                match__430 = null;
                            }
                            if (!(match__430 == null))
                            {
                                R::Match match___1014 = match__430!;
                                t___5116 = match___1014.Full.Value;
                                t___5118 = match___1014.Full.End;
                                t___5124 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(3, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5116, t___5118, t___5124);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__431;
                            try
                            {
                                t___2750 = pattern20__411.Find(literalPart___1003);
                                match__431 = t___2750;
                            }
                            catch
                            {
                                match__431 = null;
                            }
                            if (!(match__431 == null))
                            {
                                R::Match match___1015 = match__431!;
                                t___5128 = match___1015.Full.Value;
                                t___5130 = match___1015.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5128, t___5130, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__432;
                            try
                            {
                                t___2758 = pattern27__418.Find(literalPart___1003);
                                match__432 = t___2758;
                            }
                            catch
                            {
                                match__432 = null;
                            }
                            if (!(match__432 == null))
                            {
                                R::Match match___1016 = match__432!;
                                t___5134 = match___1016.Full.Value;
                                t___5136 = match___1016.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5134, t___5136, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__433;
                            try
                            {
                                t___2766 = pattern7__398.Find(literalPart___1003);
                                match__433 = t___2766;
                            }
                            catch
                            {
                                match__433 = null;
                            }
                            if (!(match__433 == null))
                            {
                                R::Match match___1017 = match__433!;
                                try
                                {
                                    t___2772 = pattern6__397.Find(C1::StringUtil.Slice(literalPart___1003, match___1017.Full.End, literalPart___1003.Length));
                                    t___2773 = t___2772;
                                }
                                catch
                                {
                                    t___2773 = null;
                                }
                                if (t___2773 == null)
                                {
                                    t___5152 = new C0::AfterPropagate<HtmlEscaperContext>(match___1017.Full.Value, match___1017.Full.End, new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(4, contextBefore__390.TagState, 4, contextBefore__390.DelimState), before__387.Subsidiary));
                                    t___5153 = new HtmlUrlDelegate();
                                    return__133 = t___5152.Push(t___5153, HtmlCodec);
                                    goto fn__389;
                                }
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__434;
                            try
                            {
                                t___2787 = pattern8__399.Find(literalPart___1003);
                                match__434 = t___2787;
                            }
                            catch
                            {
                                match__434 = null;
                            }
                            if (!(match__434 == null))
                            {
                                R::Match match___1018 = match__434!;
                                try
                                {
                                    t___2793 = pattern6__397.Find(C1::StringUtil.Slice(literalPart___1003, match___1018.Full.End, literalPart___1003.Length));
                                    t___2794 = t___2793;
                                }
                                catch
                                {
                                    t___2794 = null;
                                }
                                if (t___2794 == null)
                                {
                                    t___5169 = new C0::AfterPropagate<HtmlEscaperContext>(match___1018.Full.Value, match___1018.Full.End, new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(4, contextBefore__390.TagState, 3, contextBefore__390.DelimState), before__387.Subsidiary));
                                    t___5170 = new HtmlUrlDelegate();
                                    return__133 = t___5169.Push(t___5170, HtmlCodec);
                                    goto fn__389;
                                }
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__435;
                            try
                            {
                                t___2808 = pattern17__408.Find(literalPart___1003);
                                match__435 = t___2808;
                            }
                            catch
                            {
                                match__435 = null;
                            }
                            if (!(match__435 == null))
                            {
                                R::Match match___1019 = match__435!;
                                t___5182 = new C0::AfterPropagate<HtmlEscaperContext>(match___1019.Full.Value, match___1019.Full.End, new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(4, contextBefore__390.TagState, 3, contextBefore__390.DelimState), before__387.Subsidiary));
                                t___5183 = new HtmlUrlDelegate();
                                return__133 = t___5182.Push(t___5183, HtmlCodec);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__436;
                            try
                            {
                                t___2823 = pattern19__410.Find(literalPart___1003);
                                match__436 = t___2823;
                            }
                            catch
                            {
                                match__436 = null;
                            }
                            if (!(match__436 == null))
                            {
                                R::Match match___1020 = match__436!;
                                try
                                {
                                    t___2829 = pattern6__397.Find(C1::StringUtil.Slice(literalPart___1003, match___1020.Full.End, literalPart___1003.Length));
                                    t___2830 = t___2829;
                                }
                                catch
                                {
                                    t___2830 = null;
                                }
                                if (t___2830 == null)
                                {
                                    t___5199 = new C0::AfterPropagate<HtmlEscaperContext>(match___1020.Full.Value, match___1020.Full.End, new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(4, contextBefore__390.TagState, 1, contextBefore__390.DelimState), before__387.Subsidiary));
                                    t___5200 = new HtmlCssDelegate();
                                    return__133 = t___5199.Push(t___5200, HtmlCodec);
                                    goto fn__389;
                                }
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__437;
                            try
                            {
                                t___2844 = pattern18__409.Find(literalPart___1003);
                                match__437 = t___2844;
                            }
                            catch
                            {
                                match__437 = null;
                            }
                            if (!(match__437 == null))
                            {
                                R::Match match___1021 = match__437!;
                                t___5212 = new C0::AfterPropagate<HtmlEscaperContext>(match___1021.Full.Value, match___1021.Full.End, new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(4, contextBefore__390.TagState, 2, contextBefore__390.DelimState), before__387.Subsidiary));
                                t___5213 = new HtmlJsDelegate();
                                return__133 = t___5212.Push(t___5213, HtmlCodec);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__438;
                            try
                            {
                                t___2859 = pattern24__415.Find(literalPart___1003);
                                match__438 = t___2859;
                            }
                            catch
                            {
                                match__438 = null;
                            }
                            if (!(match__438 == null))
                            {
                                R::Match match___1022 = match__438!;
                                t___5217 = match___1022.Full.Value;
                                t___5219 = match___1022.Full.End;
                                t___5225 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(4, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5217, t___5219, t___5225);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 4)
                        {
                            R::Match ? match__439;
                            try
                            {
                                t___2873 = pattern20__411.Find(literalPart___1003);
                                match__439 = t___2873;
                            }
                            catch
                            {
                                match__439 = null;
                            }
                            if (!(match__439 == null))
                            {
                                R::Match match___1023 = match__439!;
                                t___5229 = match___1023.Full.Value;
                                t___5231 = match___1023.Full.End;
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5229, t___5231, before__387);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 4)
                        {
                            R::Match ? match__440;
                            try
                            {
                                t___2881 = pattern15__406.Find(literalPart___1003);
                                match__440 = t___2881;
                            }
                            catch
                            {
                                match__440 = null;
                            }
                            if (!(match__440 == null))
                            {
                                R::Match match___1024 = match__440!;
                                t___5235 = match___1024.Full.Value;
                                t___5237 = match___1024.Full.End;
                                t___5243 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(5, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5235, t___5237, t___5243);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 4)
                        {
                            try
                            {
                                t___2895 = pattern9__400.Find(literalPart___1003);
                                t___2896 = t___2895;
                            }
                            catch
                            {
                                t___2896 = null;
                            }
                            if (!(t___2896 == null))
                            {
                                t___5251 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(7, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, t___5251);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 5)
                        {
                            R::Match ? match__441;
                            try
                            {
                                t___2906 = pattern0__391.Find(literalPart___1003);
                                match__441 = t___2906;
                            }
                            catch
                            {
                                match__441 = null;
                            }
                            if (!(match__441 == null))
                            {
                                R::Match match___1025 = match__441!;
                                t___5255 = match___1025.Full.Value;
                                t___5257 = match___1025.Full.End;
                                t___5262 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(6, contextBefore__390.TagState, contextBefore__390.AttribState, 2), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5255, t___5257, t___5262);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 5)
                        {
                            R::Match ? match__442;
                            try
                            {
                                t___2919 = pattern2__393.Find(literalPart___1003);
                                match__442 = t___2919;
                            }
                            catch
                            {
                                match__442 = null;
                            }
                            if (!(match__442 == null))
                            {
                                R::Match match___1026 = match__442!;
                                t___5266 = match___1026.Full.Value;
                                t___5268 = match___1026.Full.End;
                                t___5273 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(6, contextBefore__390.TagState, contextBefore__390.AttribState, 1), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5266, t___5268, t___5273);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 5)
                        {
                            try
                            {
                                t___2932 = pattern10__401.Find(literalPart___1003);
                                t___2933 = t___2932;
                            }
                            catch
                            {
                                t___2933 = null;
                            }
                            if (!(t___2933 == null))
                            {
                                t___5280 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(6, contextBefore__390.TagState, contextBefore__390.AttribState, 0), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>("\u0022", 0, t___5280);
                                goto fn__389;
                            }
                        }
                    }
                    if (literalPart__388 == null)
                    {
                        t___5282 = contextBefore__390.HtmlState;
                        t___2942 = t___5282 == 5;
                    }
                    else
                    {
                        t___2942 = false;
                    }
                    if (t___2942)
                    {
                        t___5287 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(6, contextBefore__390.TagState, contextBefore__390.AttribState, 0), before__387.Subsidiary);
                        return__133 = new C0::AfterPropagate<HtmlEscaperContext>("\u0022", 0, t___5287);
                        goto fn__389;
                    }
                    if (!(literalPart__388 == null))
                    {
                        string literalPart___1028 = literalPart__388!;
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5290 = contextBefore__390.DelimState;
                            t___2952 = t___5290 == 0;
                        }
                        else
                        {
                            t___2952 = false;
                        }
                        if (t___2952)
                        {
                            try
                            {
                                t___2953 = pattern5__396.Find(literalPart___1028);
                                t___2954 = t___2953;
                            }
                            catch
                            {
                                t___2954 = null;
                            }
                            if (!(t___2954 == null))
                            {
                                t___5295 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(7, contextBefore__390.TagState, contextBefore__390.AttribState, 0), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>("\u0022", 0, t___5295);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5298 = contextBefore__390.DelimState;
                            t___2964 = t___5298 == 2;
                        }
                        else
                        {
                            t___2964 = false;
                        }
                        if (t___2964)
                        {
                            R::Match ? match__443;
                            try
                            {
                                t___2965 = pattern0__391.Find(literalPart___1028);
                                match__443 = t___2965;
                            }
                            catch
                            {
                                match__443 = null;
                            }
                            if (!(match__443 == null))
                            {
                                R::Match match___1029 = match__443!;
                                t___5300 = match___1029.Full.Value;
                                t___5302 = match___1029.Full.End;
                                t___5307 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(7, contextBefore__390.TagState, contextBefore__390.AttribState, 0), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5300, t___5302, t___5307);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5310 = contextBefore__390.DelimState;
                            t___2979 = t___5310 == 1;
                        }
                        else
                        {
                            t___2979 = false;
                        }
                        if (t___2979)
                        {
                            R::Match ? match__444;
                            try
                            {
                                t___2980 = pattern2__393.Find(literalPart___1028);
                                match__444 = t___2980;
                            }
                            catch
                            {
                                match__444 = null;
                            }
                            if (!(match__444 == null))
                            {
                                R::Match match___1030 = match__444!;
                                t___5312 = match___1030.Full.Value;
                                t___5314 = match___1030.Full.End;
                                t___5319 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(7, contextBefore__390.TagState, contextBefore__390.AttribState, 0), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5312, t___5314, t___5319);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5322 = contextBefore__390.AttribState;
                            t___2994 = t___5322 == 4;
                        }
                        else
                        {
                            t___2994 = false;
                        }
                        if (t___2994)
                        {
                            R::Match ? match__445;
                            try
                            {
                                t___2995 = pattern12__403.Find(literalPart___1028);
                                match__445 = t___2995;
                            }
                            catch
                            {
                                match__445 = null;
                            }
                            if (!(match__445 == null))
                            {
                                R::Match match___1031 = match__445!;
                                t___5328 = new C0::AfterPropagate<HtmlEscaperContext>(match___1031.Full.Value, match___1031.Full.End, before__387).Pop();
                                t___5329 = new HtmlUrlDelegate();
                                return__133 = t___5328.Push(t___5329, HtmlCodec);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5332 = contextBefore__390.DelimState;
                            t___3007 = t___5332 == 0;
                        }
                        else
                        {
                            t___3007 = false;
                        }
                        if (t___3007)
                        {
                            R::Match ? match__446;
                            try
                            {
                                t___3008 = pattern25__416.Find(literalPart___1028);
                                match__446 = t___3008;
                            }
                            catch
                            {
                                match__446 = null;
                            }
                            if (!(match__446 == null))
                            {
                                R::Match match___1032 = match__446!;
                                t___5337 = new C0::AfterPropagate<HtmlEscaperContext>(match___1032.Full.Value, match___1032.Full.End, before__387);
                                return__133 = t___5337.Feed(false);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5340 = contextBefore__390.DelimState;
                            t___3018 = t___5340 == 0;
                        }
                        else
                        {
                            t___3018 = false;
                        }
                        if (t___3018)
                        {
                            R::Match ? match__447;
                            try
                            {
                                t___3019 = pattern0__391.Find(literalPart___1028);
                                match__447 = t___3019;
                            }
                            catch
                            {
                                match__447 = null;
                            }
                            if (!(match__447 == null))
                            {
                                t___5343 = new C0::AfterPropagate<HtmlEscaperContext>("&#34;", (match__447!).Full.End, before__387);
                                return__133 = t___5343.Feed(false);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5346 = contextBefore__390.DelimState;
                            t___3027 = t___5346 == 2;
                        }
                        else
                        {
                            t___3027 = false;
                        }
                        if (t___3027)
                        {
                            R::Match ? match__448;
                            try
                            {
                                t___3028 = pattern21__412.Find(literalPart___1028);
                                match__448 = t___3028;
                            }
                            catch
                            {
                                match__448 = null;
                            }
                            if (!(match__448 == null))
                            {
                                R::Match match___1034 = match__448!;
                                t___5351 = new C0::AfterPropagate<HtmlEscaperContext>(match___1034.Full.Value, match___1034.Full.End, before__387);
                                return__133 = t___5351.Feed(false);
                                goto fn__389;
                            }
                        }
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5354 = contextBefore__390.DelimState;
                            t___3038 = t___5354 == 1;
                        }
                        else
                        {
                            t___3038 = false;
                        }
                        if (t___3038)
                        {
                            R::Match ? match__449;
                            try
                            {
                                t___3039 = pattern22__413.Find(literalPart___1028);
                                match__449 = t___3039;
                            }
                            catch
                            {
                                match__449 = null;
                            }
                            if (!(match__449 == null))
                            {
                                R::Match match___1035 = match__449!;
                                t___5359 = new C0::AfterPropagate<HtmlEscaperContext>(match___1035.Full.Value, match___1035.Full.End, before__387);
                                return__133 = t___5359.Feed(false);
                                goto fn__389;
                            }
                        }
                    }
                    if (literalPart__388 == null)
                    {
                        if (contextBefore__390.HtmlState == 6)
                        {
                            t___5362 = contextBefore__390.AttribState;
                            t___3049 = t___5362 == 0;
                        }
                        else
                        {
                            t___3049 = false;
                        }
                        t___3050 = t___3049;
                    }
                    else
                    {
                        t___3050 = false;
                    }
                    if (t___3050)
                    {
                        return__133 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, before__387);
                        goto fn__389;
                    }
                    if (literalPart__388 == null)
                    {
                        t___5364 = contextBefore__390.HtmlState;
                        t___3053 = t___5364 == 6;
                    }
                    else
                    {
                        t___3053 = false;
                    }
                    if (t___3053)
                    {
                        t___5365 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, before__387);
                        return__133 = t___5365.Feed(true);
                        goto fn__389;
                    }
                    if (contextBefore__390.HtmlState == 7)
                    {
                        t___5368 = contextBefore__390.AttribState;
                        t___3058 = t___5368 == 0;
                    }
                    else
                    {
                        t___3058 = false;
                    }
                    if (t___3058)
                    {
                        t___5374 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(3, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                        return__133 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, t___5374);
                        goto fn__389;
                    }
                    if (contextBefore__390.HtmlState == 7)
                    {
                        t___5383 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(3, contextBefore__390.TagState, 0, contextBefore__390.DelimState), before__387.Subsidiary));
                        return__133 = t___5383.Pop();
                        goto fn__389;
                    }
                    if (!(literalPart__388 == null))
                    {
                        string literalPart___1038 = literalPart__388!;
                        if (contextBefore__390.HtmlState == 3)
                        {
                            R::Match ? match__450;
                            try
                            {
                                t___3077 = pattern16__407.Find(literalPart___1038);
                                match__450 = t___3077;
                            }
                            catch
                            {
                                match__450 = null;
                            }
                            if (!(match__450 == null))
                            {
                                R::Match match___1039 = match__450!;
                                t___5387 = match___1039.Full.Value;
                                t___5389 = match___1039.Full.End;
                                t___5395 = new C0::AutoescState<HtmlEscaperContext>(new HtmlEscaperContext(0, contextBefore__390.TagState, contextBefore__390.AttribState, contextBefore__390.DelimState), before__387.Subsidiary);
                                return__133 = new C0::AfterPropagate<HtmlEscaperContext>(t___5387, t___5389, t___5395);
                                goto fn__389;
                            }
                        }
                    }
                    if (literalPart__388 == null)
                    {
                        return__133 = new C0::AfterPropagate<HtmlEscaperContext>("", 0, before__387);
                        goto fn__389;
                    }
                    throw new S::Exception();
                }
                fn__389:
                {
                }
            }
            return return__133;
        }
        internal static HtmlPcdataEscaper return__593;
        internal static HtmlPcdataEscaper htmlPcdataEscaper__291;
        internal static OutputHtmlSpaceEscaper return__572;
        internal static OutputHtmlSpaceEscaper outputHtmlSpaceEscaper__290;
        internal static HtmlAttributeEscaper return__614;
        internal static HtmlAttributeEscaper htmlAttributeEscaper__292;
        public static IHtmlEscaper PickHtmlEscaper(C0::AutoescState<HtmlEscaperContext> stateBefore__615)
        {
            IHtmlEscaper return__193;
            bool t___2297;
            bool t___2298;
            bool t___2299;
            bool t___2300;
            IHtmlDelegate t___2305;
            IHtmlEscaper escaper__617;
            int t___4717 = stateBefore__615.Context.HtmlState;
            if (t___4717 == 0)
            {
                escaper__617 = htmlPcdataEscaper__291;
            }
            else
            {
                if (t___4717 == 1)
                {
                    t___2300 = true;
                }
                else
                {
                    if (t___4717 == 2)
                    {
                        t___2299 = true;
                    }
                    else
                    {
                        if (t___4717 == 3)
                        {
                            t___2298 = true;
                        }
                        else
                        {
                            if (t___4717 == 4)
                            {
                                t___2297 = true;
                            }
                            else
                            {
                                t___2297 = t___4717 == 7;
                            }
                            t___2298 = t___2297;
                        }
                        t___2299 = t___2298;
                    }
                    t___2300 = t___2299;
                }
                if (t___2300)
                {
                    escaper__617 = outputHtmlSpaceEscaper__290;
                }
                else if (t___4717 == 5) throw new S::Exception();
                else if (t___4717 == 6)
                {
                    escaper__617 = htmlAttributeEscaper__292;
                }
                else if (t___4717 == 8)
                {
                    escaper__617 = outputHtmlSpaceEscaper__290;
                }
                else throw new S::Exception();
            }
            C0::Subsidiary ? subsidiary__618 = stateBefore__615.Subsidiary;
            if (!(subsidiary__618 == null))
            {
                C0::Subsidiary subsidiary___1040 = subsidiary__618!;
                t___2305 = (IHtmlDelegate) subsidiary___1040.Delegate;
                IHtmlDelegate delegate__619 = t___2305;
                return__193 = delegate__619.Escaper(escaper__617);
            }
            else
            {
                return__193 = escaper__617;
            }
            return return__193;
        }
        static HtmlGlobal()
        {
            strs__375 = C1::Listed.CreateReadOnlyList<string>("AElig", "Æ", "AElig;", "Æ", "AMP", "&", "AMP;", "&", "Aacute", "Á", "Aacute;", "Á", "Abreve;", "Ă", "Acirc", "Â", "Acirc;", "Â", "Acy;", "А", "Afr;", "\ud835\udd04", "Agrave", "À", "Agrave;", "À", "Alpha;", "Α", "Amacr;", "Ā", "And;", "⩓", "Aogon;", "Ą", "Aopf;", "\ud835\udd38", "ApplyFunction;", "⁡", "Aring", "Å", "Aring;", "Å", "Ascr;", "\ud835\udc9c", "Assign;", "≔", "Atilde", "Ã", "Atilde;", "Ã", "Auml", "Ä", "Auml;", "Ä", "Backslash;", "∖", "Barv;", "⫧", "Barwed;", "⌆", "Bcy;", "Б", "Because;", "∵", "Bernoullis;", "ℬ", "Beta;", "Β", "Bfr;", "\ud835\udd05", "Bopf;", "\ud835\udd39", "Breve;", "˘", "Bscr;", "ℬ", "Bumpeq;", "≎", "CHcy;", "Ч", "COPY", "©", "COPY;", "©", "Cacute;", "Ć", "Cap;", "⋒", "CapitalDifferentialD;", "ⅅ", "Cayleys;", "ℭ", "Ccaron;", "Č", "Ccedil", "Ç", "Ccedil;", "Ç", "Ccirc;", "Ĉ", "Cconint;", "∰", "Cdot;", "Ċ", "Cedilla;", "¸", "CenterDot;", "·", "Cfr;", "ℭ", "Chi;", "Χ", "CircleDot;", "⊙", "CircleMinus;", "⊖", "CirclePlus;", "⊕", "CircleTimes;", "⊗", "ClockwiseContourIntegral;", "∲", "CloseCurlyDoubleQuote;", "”", "CloseCurlyQuote;", "’", "Colon;", "∷", "Colone;", "⩴", "Congruent;", "≡", "Conint;", "∯", "ContourIntegral;", "∮", "Copf;", "ℂ", "Coproduct;", "∐", "CounterClockwiseContourIntegral;", "∳", "Cross;", "⨯", "Cscr;", "\ud835\udc9e", "Cup;", "⋓", "CupCap;", "≍", "DD;", "ⅅ", "DDotrahd;", "⤑", "DJcy;", "Ђ", "DScy;", "Ѕ", "DZcy;", "Џ", "Dagger;", "‡", "Darr;", "↡", "Dashv;", "⫤", "Dcaron;", "Ď", "Dcy;", "Д", "Del;", "∇", "Delta;", "Δ", "Dfr;", "\ud835\udd07", "DiacriticalAcute;", "´", "DiacriticalDot;", "˙", "DiacriticalDoubleAcute;", "˝", "DiacriticalGrave;", "`", "DiacriticalTilde;", "˜", "Diamond;", "⋄", "DifferentialD;", "ⅆ", "Dopf;", "\ud835\udd3b", "Dot;", "¨", "DotDot;", "⃜", "DotEqual;", "≐", "DoubleContourIntegral;", "∯", "DoubleDot;", "¨", "DoubleDownArrow;", "⇓", "DoubleLeftArrow;", "⇐", "DoubleLeftRightArrow;", "⇔", "DoubleLeftTee;", "⫤", "DoubleLongLeftArrow;", "⟸", "DoubleLongLeftRightArrow;", "⟺", "DoubleLongRightArrow;", "⟹", "DoubleRightArrow;", "⇒", "DoubleRightTee;", "⊨", "DoubleUpArrow;", "⇑", "DoubleUpDownArrow;", "⇕", "DoubleVerticalBar;", "∥", "DownArrow;", "↓", "DownArrowBar;", "⤓", "DownArrowUpArrow;", "⇵", "DownBreve;", "̑", "DownLeftRightVector;", "⥐", "DownLeftTeeVector;", "⥞", "DownLeftVector;", "↽", "DownLeftVectorBar;", "⥖", "DownRightTeeVector;", "⥟", "DownRightVector;", "⇁", "DownRightVectorBar;", "⥗", "DownTee;", "⊤", "DownTeeArrow;", "↧", "Downarrow;", "⇓", "Dscr;", "\ud835\udc9f", "Dstrok;", "Đ", "ENG;", "Ŋ", "ETH", "Ð", "ETH;", "Ð", "Eacute", "É", "Eacute;", "É", "Ecaron;", "Ě", "Ecirc", "Ê", "Ecirc;", "Ê", "Ecy;", "Э", "Edot;", "Ė", "Efr;", "\ud835\udd08", "Egrave", "È", "Egrave;", "È", "Element;", "∈", "Emacr;", "Ē", "EmptySmallSquare;", "◻", "EmptyVerySmallSquare;", "▫", "Eogon;", "Ę", "Eopf;", "\ud835\udd3c", "Epsilon;", "Ε", "Equal;", "⩵", "EqualTilde;", "≂", "Equilibrium;", "⇌", "Escr;", "ℰ", "Esim;", "⩳", "Eta;", "Η", "Euml", "Ë", "Euml;", "Ë", "Exists;", "∃", "ExponentialE;", "ⅇ", "Fcy;", "Ф", "Ffr;", "\ud835\udd09", "FilledSmallSquare;", "◼", "FilledVerySmallSquare;", "▪", "Fopf;", "\ud835\udd3d", "ForAll;", "∀", "Fouriertrf;", "ℱ", "Fscr;", "ℱ", "GJcy;", "Ѓ", "GT", ">", "GT;", ">", "Gamma;", "Γ", "Gammad;", "Ϝ", "Gbreve;", "Ğ", "Gcedil;", "Ģ", "Gcirc;", "Ĝ", "Gcy;", "Г", "Gdot;", "Ġ", "Gfr;", "\ud835\udd0a", "Gg;", "⋙", "Gopf;", "\ud835\udd3e", "GreaterEqual;", "≥", "GreaterEqualLess;", "⋛", "GreaterFullEqual;", "≧", "GreaterGreater;", "⪢", "GreaterLess;", "≷", "GreaterSlantEqual;", "⩾", "GreaterTilde;", "≳", "Gscr;", "\ud835\udca2", "Gt;", "≫", "HARDcy;", "Ъ", "Hacek;", "ˇ", "Hat;", "^", "Hcirc;", "Ĥ", "Hfr;", "ℌ", "HilbertSpace;", "ℋ", "Hopf;", "ℍ", "HorizontalLine;", "─", "Hscr;", "ℋ", "Hstrok;", "Ħ", "HumpDownHump;", "≎", "HumpEqual;", "≏", "IEcy;", "Е", "IJlig;", "Ĳ", "IOcy;", "Ё", "Iacute", "Í", "Iacute;", "Í", "Icirc", "Î", "Icirc;", "Î", "Icy;", "И", "Idot;", "İ", "Ifr;", "ℑ", "Igrave", "Ì", "Igrave;", "Ì", "Im;", "ℑ", "Imacr;", "Ī", "ImaginaryI;", "ⅈ", "Implies;", "⇒", "Int;", "∬", "Integral;", "∫", "Intersection;", "⋂", "InvisibleComma;", "⁣", "InvisibleTimes;", "⁢", "Iogon;", "Į", "Iopf;", "\ud835\udd40", "Iota;", "Ι", "Iscr;", "ℐ", "Itilde;", "Ĩ", "Iukcy;", "І", "Iuml", "Ï", "Iuml;", "Ï", "Jcirc;", "Ĵ", "Jcy;", "Й", "Jfr;", "\ud835\udd0d", "Jopf;", "\ud835\udd41", "Jscr;", "\ud835\udca5", "Jsercy;", "Ј", "Jukcy;", "Є", "KHcy;", "Х", "KJcy;", "Ќ", "Kappa;", "Κ", "Kcedil;", "Ķ", "Kcy;", "К", "Kfr;", "\ud835\udd0e", "Kopf;", "\ud835\udd42", "Kscr;", "\ud835\udca6", "LJcy;", "Љ", "LT", "<", "LT;", "<", "Lacute;", "Ĺ", "Lambda;", "Λ", "Lang;", "⟪", "Laplacetrf;", "ℒ", "Larr;", "↞", "Lcaron;", "Ľ", "Lcedil;", "Ļ", "Lcy;", "Л", "LeftAngleBracket;", "⟨", "LeftArrow;", "←", "LeftArrowBar;", "⇤", "LeftArrowRightArrow;", "⇆", "LeftCeiling;", "⌈", "LeftDoubleBracket;", "⟦", "LeftDownTeeVector;", "⥡", "LeftDownVector;", "⇃", "LeftDownVectorBar;", "⥙", "LeftFloor;", "⌊", "LeftRightArrow;", "↔", "LeftRightVector;", "⥎", "LeftTee;", "⊣", "LeftTeeArrow;", "↤", "LeftTeeVector;", "⥚", "LeftTriangle;", "⊲", "LeftTriangleBar;", "⧏", "LeftTriangleEqual;", "⊴", "LeftUpDownVector;", "⥑", "LeftUpTeeVector;", "⥠", "LeftUpVector;", "↿", "LeftUpVectorBar;", "⥘", "LeftVector;", "↼", "LeftVectorBar;", "⥒", "Leftarrow;", "⇐", "Leftrightarrow;", "⇔", "LessEqualGreater;", "⋚", "LessFullEqual;", "≦", "LessGreater;", "≶", "LessLess;", "⪡", "LessSlantEqual;", "⩽", "LessTilde;", "≲", "Lfr;", "\ud835\udd0f", "Ll;", "⋘", "Lleftarrow;", "⇚", "Lmidot;", "Ŀ", "LongLeftArrow;", "⟵", "LongLeftRightArrow;", "⟷", "LongRightArrow;", "⟶", "Longleftarrow;", "⟸", "Longleftrightarrow;", "⟺", "Longrightarrow;", "⟹", "Lopf;", "\ud835\udd43", "LowerLeftArrow;", "↙", "LowerRightArrow;", "↘", "Lscr;", "ℒ", "Lsh;", "↰", "Lstrok;", "Ł", "Lt;", "≪", "Map;", "⤅", "Mcy;", "М", "MediumSpace;", " ", "Mellintrf;", "ℳ", "Mfr;", "\ud835\udd10", "MinusPlus;", "∓", "Mopf;", "\ud835\udd44", "Mscr;", "ℳ", "Mu;", "Μ", "NJcy;", "Њ", "Nacute;", "Ń", "Ncaron;", "Ň", "Ncedil;", "Ņ", "Ncy;", "Н", "NegativeMediumSpace;", "​", "NegativeThickSpace;", "​", "NegativeThinSpace;", "​", "NegativeVeryThinSpace;", "​", "NestedGreaterGreater;", "≫", "NestedLessLess;", "≪", "NewLine;", "\n", "Nfr;", "\ud835\udd11", "NoBreak;", "⁠", "NonBreakingSpace;", " ", "Nopf;", "ℕ", "Not;", "⫬", "NotCongruent;", "≢", "NotCupCap;", "≭", "NotDoubleVerticalBar;", "∦", "NotElement;", "∉", "NotEqual;", "≠", "NotEqualTilde;", "≂̸", "NotExists;", "∄", "NotGreater;", "≯", "NotGreaterEqual;", "≱", "NotGreaterFullEqual;", "≧̸", "NotGreaterGreater;", "≫̸", "NotGreaterLess;", "≹", "NotGreaterSlantEqual;", "⩾̸", "NotGreaterTilde;", "≵", "NotHumpDownHump;", "≎̸", "NotHumpEqual;", "≏̸", "NotLeftTriangle;", "⋪", "NotLeftTriangleBar;", "⧏̸", "NotLeftTriangleEqual;", "⋬", "NotLess;", "≮", "NotLessEqual;", "≰", "NotLessGreater;", "≸", "NotLessLess;", "≪̸", "NotLessSlantEqual;", "⩽̸", "NotLessTilde;", "≴", "NotNestedGreaterGreater;", "⪢̸", "NotNestedLessLess;", "⪡̸", "NotPrecedes;", "⊀", "NotPrecedesEqual;", "⪯̸", "NotPrecedesSlantEqual;", "⋠", "NotReverseElement;", "∌", "NotRightTriangle;", "⋫", "NotRightTriangleBar;", "⧐̸", "NotRightTriangleEqual;", "⋭", "NotSquareSubset;", "⊏̸", "NotSquareSubsetEqual;", "⋢", "NotSquareSuperset;", "⊐̸", "NotSquareSupersetEqual;", "⋣", "NotSubset;", "⊂⃒", "NotSubsetEqual;", "⊈", "NotSucceeds;", "⊁", "NotSucceedsEqual;", "⪰̸", "NotSucceedsSlantEqual;", "⋡", "NotSucceedsTilde;", "≿̸", "NotSuperset;", "⊃⃒", "NotSupersetEqual;", "⊉", "NotTilde;", "≁", "NotTildeEqual;", "≄", "NotTildeFullEqual;", "≇", "NotTildeTilde;", "≉", "NotVerticalBar;", "∤", "Nscr;", "\ud835\udca9", "Ntilde", "Ñ", "Ntilde;", "Ñ", "Nu;", "Ν", "OElig;", "Œ", "Oacute", "Ó", "Oacute;", "Ó", "Ocirc", "Ô", "Ocirc;", "Ô", "Ocy;", "О", "Odblac;", "Ő", "Ofr;", "\ud835\udd12", "Ograve", "Ò", "Ograve;", "Ò", "Omacr;", "Ō", "Omega;", "Ω", "Omicron;", "Ο", "Oopf;", "\ud835\udd46", "OpenCurlyDoubleQuote;", "“", "OpenCurlyQuote;", "‘", "Or;", "⩔", "Oscr;", "\ud835\udcaa", "Oslash", "Ø", "Oslash;", "Ø", "Otilde", "Õ", "Otilde;", "Õ", "Otimes;", "⨷", "Ouml", "Ö", "Ouml;", "Ö", "OverBar;", "‾", "OverBrace;", "⏞", "OverBracket;", "⎴", "OverParenthesis;", "⏜", "PartialD;", "∂", "Pcy;", "П", "Pfr;", "\ud835\udd13", "Phi;", "Φ", "Pi;", "Π", "PlusMinus;", "±", "Poincareplane;", "ℌ", "Popf;", "ℙ", "Pr;", "⪻", "Precedes;", "≺", "PrecedesEqual;", "⪯", "PrecedesSlantEqual;", "≼", "PrecedesTilde;", "≾", "Prime;", "″", "Product;", "∏", "Proportion;", "∷", "Proportional;", "∝", "Pscr;", "\ud835\udcab", "Psi;", "Ψ", "QUOT", "\u0022", "QUOT;", "\u0022", "Qfr;", "\ud835\udd14", "Qopf;", "ℚ", "Qscr;", "\ud835\udcac", "RBarr;", "⤐", "REG", "®", "REG;", "®", "Racute;", "Ŕ", "Rang;", "⟫", "Rarr;", "↠", "Rarrtl;", "⤖", "Rcaron;", "Ř", "Rcedil;", "Ŗ", "Rcy;", "Р", "Re;", "ℜ", "ReverseElement;", "∋", "ReverseEquilibrium;", "⇋", "ReverseUpEquilibrium;", "⥯", "Rfr;", "ℜ", "Rho;", "Ρ", "RightAngleBracket;", "⟩", "RightArrow;", "→", "RightArrowBar;", "⇥", "RightArrowLeftArrow;", "⇄", "RightCeiling;", "⌉", "RightDoubleBracket;", "⟧", "RightDownTeeVector;", "⥝", "RightDownVector;", "⇂", "RightDownVectorBar;", "⥕", "RightFloor;", "⌋", "RightTee;", "⊢", "RightTeeArrow;", "↦", "RightTeeVector;", "⥛", "RightTriangle;", "⊳", "RightTriangleBar;", "⧐", "RightTriangleEqual;", "⊵", "RightUpDownVector;", "⥏", "RightUpTeeVector;", "⥜", "RightUpVector;", "↾", "RightUpVectorBar;", "⥔", "RightVector;", "⇀", "RightVectorBar;", "⥓", "Rightarrow;", "⇒", "Ropf;", "ℝ", "RoundImplies;", "⥰", "Rrightarrow;", "⇛", "Rscr;", "ℛ", "Rsh;", "↱", "RuleDelayed;", "⧴", "SHCHcy;", "Щ", "SHcy;", "Ш", "SOFTcy;", "Ь", "Sacute;", "Ś", "Sc;", "⪼", "Scaron;", "Š", "Scedil;", "Ş", "Scirc;", "Ŝ", "Scy;", "С", "Sfr;", "\ud835\udd16", "ShortDownArrow;", "↓", "ShortLeftArrow;", "←", "ShortRightArrow;", "→", "ShortUpArrow;", "↑", "Sigma;", "Σ", "SmallCircle;", "∘", "Sopf;", "\ud835\udd4a", "Sqrt;", "√", "Square;", "□", "SquareIntersection;", "⊓", "SquareSubset;", "⊏", "SquareSubsetEqual;", "⊑", "SquareSuperset;", "⊐", "SquareSupersetEqual;", "⊒", "SquareUnion;", "⊔", "Sscr;", "\ud835\udcae", "Star;", "⋆", "Sub;", "⋐", "Subset;", "⋐", "SubsetEqual;", "⊆", "Succeeds;", "≻", "SucceedsEqual;", "⪰", "SucceedsSlantEqual;", "≽", "SucceedsTilde;", "≿", "SuchThat;", "∋", "Sum;", "∑", "Sup;", "⋑", "Superset;", "⊃", "SupersetEqual;", "⊇", "Supset;", "⋑", "THORN", "Þ", "THORN;", "Þ", "TRADE;", "™", "TSHcy;", "Ћ", "TScy;", "Ц", "Tab;", "\t", "Tau;", "Τ", "Tcaron;", "Ť", "Tcedil;", "Ţ", "Tcy;", "Т", "Tfr;", "\ud835\udd17", "Therefore;", "∴", "Theta;", "Θ", "ThickSpace;", "  ", "ThinSpace;", " ", "Tilde;", "∼", "TildeEqual;", "≃", "TildeFullEqual;", "≅", "TildeTilde;", "≈", "Topf;", "\ud835\udd4b", "TripleDot;", "⃛", "Tscr;", "\ud835\udcaf", "Tstrok;", "Ŧ", "Uacute", "Ú", "Uacute;", "Ú", "Uarr;", "↟", "Uarrocir;", "⥉", "Ubrcy;", "Ў", "Ubreve;", "Ŭ", "Ucirc", "Û", "Ucirc;", "Û", "Ucy;", "У", "Udblac;", "Ű", "Ufr;", "\ud835\udd18", "Ugrave", "Ù", "Ugrave;", "Ù", "Umacr;", "Ū", "UnderBar;", "_", "UnderBrace;", "⏟", "UnderBracket;", "⎵", "UnderParenthesis;", "⏝", "Union;", "⋃", "UnionPlus;", "⊎", "Uogon;", "Ų", "Uopf;", "\ud835\udd4c", "UpArrow;", "↑", "UpArrowBar;", "⤒", "UpArrowDownArrow;", "⇅", "UpDownArrow;", "↕", "UpEquilibrium;", "⥮", "UpTee;", "⊥", "UpTeeArrow;", "↥", "Uparrow;", "⇑", "Updownarrow;", "⇕", "UpperLeftArrow;", "↖", "UpperRightArrow;", "↗", "Upsi;", "ϒ", "Upsilon;", "Υ", "Uring;", "Ů", "Uscr;", "\ud835\udcb0", "Utilde;", "Ũ", "Uuml", "Ü", "Uuml;", "Ü", "VDash;", "⊫", "Vbar;", "⫫", "Vcy;", "В", "Vdash;", "⊩", "Vdashl;", "⫦", "Vee;", "⋁", "Verbar;", "‖", "Vert;", "‖", "VerticalBar;", "∣", "VerticalLine;", "|", "VerticalSeparator;", "❘", "VerticalTilde;", "≀", "VeryThinSpace;", " ", "Vfr;", "\ud835\udd19", "Vopf;", "\ud835\udd4d", "Vscr;", "\ud835\udcb1", "Vvdash;", "⊪", "Wcirc;", "Ŵ", "Wedge;", "⋀", "Wfr;", "\ud835\udd1a", "Wopf;", "\ud835\udd4e", "Wscr;", "\ud835\udcb2", "Xfr;", "\ud835\udd1b", "Xi;", "Ξ", "Xopf;", "\ud835\udd4f", "Xscr;", "\ud835\udcb3", "YAcy;", "Я", "YIcy;", "Ї", "YUcy;", "Ю", "Yacute", "Ý", "Yacute;", "Ý", "Ycirc;", "Ŷ", "Ycy;", "Ы", "Yfr;", "\ud835\udd1c", "Yopf;", "\ud835\udd50", "Yscr;", "\ud835\udcb4", "Yuml;", "Ÿ", "ZHcy;", "Ж", "Zacute;", "Ź", "Zcaron;", "Ž", "Zcy;", "З", "Zdot;", "Ż", "ZeroWidthSpace;", "​", "Zeta;", "Ζ", "Zfr;", "ℨ", "Zopf;", "ℤ", "Zscr;", "\ud835\udcb5", "aacute", "á", "aacute;", "á", "abreve;", "ă", "ac;", "∾", "acE;", "∾̳", "acd;", "∿", "acirc", "â", "acirc;", "â", "acute", "´", "acute;", "´", "acy;", "а", "aelig", "æ", "aelig;", "æ", "af;", "⁡", "afr;", "\ud835\udd1e", "agrave", "à", "agrave;", "à", "alefsym;", "ℵ", "aleph;", "ℵ", "alpha;", "α", "amacr;", "ā", "amalg;", "⨿", "amp", "&", "amp;", "&", "and;", "∧", "andand;", "⩕", "andd;", "⩜", "andslope;", "⩘", "andv;", "⩚", "ang;", "∠", "ange;", "⦤", "angle;", "∠", "angmsd;", "∡", "angmsdaa;", "⦨", "angmsdab;", "⦩", "angmsdac;", "⦪", "angmsdad;", "⦫", "angmsdae;", "⦬", "angmsdaf;", "⦭", "angmsdag;", "⦮", "angmsdah;", "⦯", "angrt;", "∟", "angrtvb;", "⊾", "angrtvbd;", "⦝", "angsph;", "∢", "angst;", "Å", "angzarr;", "⍼", "aogon;", "ą", "aopf;", "\ud835\udd52", "ap;", "≈", "apE;", "⩰", "apacir;", "⩯", "ape;", "≊", "apid;", "≋", "apos;", "'", "approx;", "≈", "approxeq;", "≊", "aring", "å", "aring;", "å", "ascr;", "\ud835\udcb6", "ast;", "*", "asymp;", "≈", "asympeq;", "≍", "atilde", "ã", "atilde;", "ã", "auml", "ä", "auml;", "ä", "awconint;", "∳", "awint;", "⨑", "bNot;", "⫭", "backcong;", "≌", "backepsilon;", "϶", "backprime;", "‵", "backsim;", "∽", "backsimeq;", "⋍", "barvee;", "⊽", "barwed;", "⌅", "barwedge;", "⌅", "bbrk;", "⎵", "bbrktbrk;", "⎶", "bcong;", "≌", "bcy;", "б", "bdquo;", "„", "becaus;", "∵", "because;", "∵", "bemptyv;", "⦰", "bepsi;", "϶", "bernou;", "ℬ", "beta;", "β", "beth;", "ℶ", "between;", "≬", "bfr;", "\ud835\udd1f", "bigcap;", "⋂", "bigcirc;", "◯", "bigcup;", "⋃", "bigodot;", "⨀", "bigoplus;", "⨁", "bigotimes;", "⨂", "bigsqcup;", "⨆", "bigstar;", "★", "bigtriangledown;", "▽", "bigtriangleup;", "△", "biguplus;", "⨄", "bigvee;", "⋁", "bigwedge;", "⋀", "bkarow;", "⤍", "blacklozenge;", "⧫", "blacksquare;", "▪", "blacktriangle;", "▴", "blacktriangledown;", "▾", "blacktriangleleft;", "◂", "blacktriangleright;", "▸", "blank;", "␣", "blk12;", "▒", "blk14;", "░", "blk34;", "▓", "block;", "█", "bne;", "=⃥", "bnequiv;", "≡⃥", "bnot;", "⌐", "bopf;", "\ud835\udd53", "bot;", "⊥", "bottom;", "⊥", "bowtie;", "⋈", "boxDL;", "╗", "boxDR;", "╔", "boxDl;", "╖", "boxDr;", "╓", "boxH;", "═", "boxHD;", "╦", "boxHU;", "╩", "boxHd;", "╤", "boxHu;", "╧", "boxUL;", "╝", "boxUR;", "╚", "boxUl;", "╜", "boxUr;", "╙", "boxV;", "║", "boxVH;", "╬", "boxVL;", "╣", "boxVR;", "╠", "boxVh;", "╫", "boxVl;", "╢", "boxVr;", "╟", "boxbox;", "⧉", "boxdL;", "╕", "boxdR;", "╒", "boxdl;", "┐", "boxdr;", "┌", "boxh;", "─", "boxhD;", "╥", "boxhU;", "╨", "boxhd;", "┬", "boxhu;", "┴", "boxminus;", "⊟", "boxplus;", "⊞", "boxtimes;", "⊠", "boxuL;", "╛", "boxuR;", "╘", "boxul;", "┘", "boxur;", "└", "boxv;", "│", "boxvH;", "╪", "boxvL;", "╡", "boxvR;", "╞", "boxvh;", "┼", "boxvl;", "┤", "boxvr;", "├", "bprime;", "‵", "breve;", "˘", "brvbar", "¦", "brvbar;", "¦", "bscr;", "\ud835\udcb7", "bsemi;", "⁏", "bsim;", "∽", "bsime;", "⋍", "bsol;", "\\", "bsolb;", "⧅", "bsolhsub;", "⟈", "bull;", "•", "bullet;", "•", "bump;", "≎", "bumpE;", "⪮", "bumpe;", "≏", "bumpeq;", "≏", "cacute;", "ć", "cap;", "∩", "capand;", "⩄", "capbrcup;", "⩉", "capcap;", "⩋", "capcup;", "⩇", "capdot;", "⩀", "caps;", "∩︀", "caret;", "⁁", "caron;", "ˇ", "ccaps;", "⩍", "ccaron;", "č", "ccedil", "ç", "ccedil;", "ç", "ccirc;", "ĉ", "ccups;", "⩌", "ccupssm;", "⩐", "cdot;", "ċ", "cedil", "¸", "cedil;", "¸", "cemptyv;", "⦲", "cent", "¢", "cent;", "¢", "centerdot;", "·", "cfr;", "\ud835\udd20", "chcy;", "ч", "check;", "✓", "checkmark;", "✓", "chi;", "χ", "cir;", "○", "cirE;", "⧃", "circ;", "ˆ", "circeq;", "≗", "circlearrowleft;", "↺", "circlearrowright;", "↻", "circledR;", "®", "circledS;", "Ⓢ", "circledast;", "⊛", "circledcirc;", "⊚", "circleddash;", "⊝", "cire;", "≗", "cirfnint;", "⨐", "cirmid;", "⫯", "cirscir;", "⧂", "clubs;", "♣", "clubsuit;", "♣", "colon;", ":", "colone;", "≔", "coloneq;", "≔", "comma;", ",", "commat;", "@", "comp;", "∁", "compfn;", "∘", "complement;", "∁", "complexes;", "ℂ", "cong;", "≅", "congdot;", "⩭", "conint;", "∮", "copf;", "\ud835\udd54", "coprod;", "∐", "copy", "©", "copy;", "©", "copysr;", "℗", "crarr;", "↵", "cross;", "✗", "cscr;", "\ud835\udcb8", "csub;", "⫏", "csube;", "⫑", "csup;", "⫐", "csupe;", "⫒", "ctdot;", "⋯", "cudarrl;", "⤸", "cudarrr;", "⤵", "cuepr;", "⋞", "cuesc;", "⋟", "cularr;", "↶", "cularrp;", "⤽", "cup;", "∪", "cupbrcap;", "⩈", "cupcap;", "⩆", "cupcup;", "⩊", "cupdot;", "⊍", "cupor;", "⩅", "cups;", "∪︀", "curarr;", "↷", "curarrm;", "⤼", "curlyeqprec;", "⋞", "curlyeqsucc;", "⋟", "curlyvee;", "⋎", "curlywedge;", "⋏", "curren", "¤", "curren;", "¤", "curvearrowleft;", "↶", "curvearrowright;", "↷", "cuvee;", "⋎", "cuwed;", "⋏", "cwconint;", "∲", "cwint;", "∱", "cylcty;", "⌭", "dArr;", "⇓", "dHar;", "⥥", "dagger;", "†", "daleth;", "ℸ", "darr;", "↓", "dash;", "‐", "dashv;", "⊣", "dbkarow;", "⤏", "dblac;", "˝", "dcaron;", "ď", "dcy;", "д", "dd;", "ⅆ", "ddagger;", "‡", "ddarr;", "⇊", "ddotseq;", "⩷", "deg", "°", "deg;", "°", "delta;", "δ", "demptyv;", "⦱", "dfisht;", "⥿", "dfr;", "\ud835\udd21", "dharl;", "⇃", "dharr;", "⇂", "diam;", "⋄", "diamond;", "⋄", "diamondsuit;", "♦", "diams;", "♦", "die;", "¨", "digamma;", "ϝ", "disin;", "⋲", "div;", "÷", "divide", "÷", "divide;", "÷", "divideontimes;", "⋇", "divonx;", "⋇", "djcy;", "ђ", "dlcorn;", "⌞", "dlcrop;", "⌍", "dollar;", "\u0024", "dopf;", "\ud835\udd55", "dot;", "˙", "doteq;", "≐", "doteqdot;", "≑", "dotminus;", "∸", "dotplus;", "∔", "dotsquare;", "⊡", "doublebarwedge;", "⌆", "downarrow;", "↓", "downdownarrows;", "⇊", "downharpoonleft;", "⇃", "downharpoonright;", "⇂", "drbkarow;", "⤐", "drcorn;", "⌟", "drcrop;", "⌌", "dscr;", "\ud835\udcb9", "dscy;", "ѕ", "dsol;", "⧶", "dstrok;", "đ", "dtdot;", "⋱", "dtri;", "▿", "dtrif;", "▾", "duarr;", "⇵", "duhar;", "⥯", "dwangle;", "⦦", "dzcy;", "џ", "dzigrarr;", "⟿", "eDDot;", "⩷", "eDot;", "≑", "eacute", "é", "eacute;", "é", "easter;", "⩮", "ecaron;", "ě", "ecir;", "≖", "ecirc", "ê", "ecirc;", "ê", "ecolon;", "≕", "ecy;", "э", "edot;", "ė", "ee;", "ⅇ", "efDot;", "≒", "efr;", "\ud835\udd22", "eg;", "⪚", "egrave", "è", "egrave;", "è", "egs;", "⪖", "egsdot;", "⪘", "el;", "⪙", "elinters;", "⏧", "ell;", "ℓ", "els;", "⪕", "elsdot;", "⪗", "emacr;", "ē", "empty;", "∅", "emptyset;", "∅", "emptyv;", "∅", "emsp13;", " ", "emsp14;", " ", "emsp;", " ", "eng;", "ŋ", "ensp;", " ", "eogon;", "ę", "eopf;", "\ud835\udd56", "epar;", "⋕", "eparsl;", "⧣", "eplus;", "⩱", "epsi;", "ε", "epsilon;", "ε", "epsiv;", "ϵ", "eqcirc;", "≖", "eqcolon;", "≕", "eqsim;", "≂", "eqslantgtr;", "⪖", "eqslantless;", "⪕", "equals;", "=", "equest;", "≟", "equiv;", "≡", "equivDD;", "⩸", "eqvparsl;", "⧥", "erDot;", "≓", "erarr;", "⥱", "escr;", "ℯ", "esdot;", "≐", "esim;", "≂", "eta;", "η", "eth", "ð", "eth;", "ð", "euml", "ë", "euml;", "ë", "euro;", "€", "excl;", "!", "exist;", "∃", "expectation;", "ℰ", "exponentiale;", "ⅇ", "fallingdotseq;", "≒", "fcy;", "ф", "female;", "♀", "ffilig;", "ﬃ", "fflig;", "ﬀ", "ffllig;", "ﬄ", "ffr;", "\ud835\udd23", "filig;", "ﬁ", "fjlig;", "fj", "flat;", "♭", "fllig;", "ﬂ", "fltns;", "▱", "fnof;", "ƒ", "fopf;", "\ud835\udd57", "forall;", "∀", "fork;", "⋔", "forkv;", "⫙", "fpartint;", "⨍", "frac12", "½", "frac12;", "½", "frac13;", "⅓", "frac14", "¼", "frac14;", "¼", "frac15;", "⅕", "frac16;", "⅙", "frac18;", "⅛", "frac23;", "⅔", "frac25;", "⅖", "frac34", "¾", "frac34;", "¾", "frac35;", "⅗", "frac38;", "⅜", "frac45;", "⅘", "frac56;", "⅚", "frac58;", "⅝", "frac78;", "⅞", "frasl;", "⁄", "frown;", "⌢", "fscr;", "\ud835\udcbb", "gE;", "≧", "gEl;", "⪌", "gacute;", "ǵ", "gamma;", "γ", "gammad;", "ϝ", "gap;", "⪆", "gbreve;", "ğ", "gcirc;", "ĝ", "gcy;", "г", "gdot;", "ġ", "ge;", "≥", "gel;", "⋛", "geq;", "≥", "geqq;", "≧", "geqslant;", "⩾", "ges;", "⩾", "gescc;", "⪩", "gesdot;", "⪀", "gesdoto;", "⪂", "gesdotol;", "⪄", "gesl;", "⋛︀", "gesles;", "⪔", "gfr;", "\ud835\udd24", "gg;", "≫", "ggg;", "⋙", "gimel;", "ℷ", "gjcy;", "ѓ", "gl;", "≷", "glE;", "⪒", "gla;", "⪥", "glj;", "⪤", "gnE;", "≩", "gnap;", "⪊", "gnapprox;", "⪊", "gne;", "⪈", "gneq;", "⪈", "gneqq;", "≩", "gnsim;", "⋧", "gopf;", "\ud835\udd58", "grave;", "`", "gscr;", "ℊ", "gsim;", "≳", "gsime;", "⪎", "gsiml;", "⪐", "gt", ">", "gt;", ">", "gtcc;", "⪧", "gtcir;", "⩺", "gtdot;", "⋗", "gtlPar;", "⦕", "gtquest;", "⩼", "gtrapprox;", "⪆", "gtrarr;", "⥸", "gtrdot;", "⋗", "gtreqless;", "⋛", "gtreqqless;", "⪌", "gtrless;", "≷", "gtrsim;", "≳", "gvertneqq;", "≩︀", "gvnE;", "≩︀", "hArr;", "⇔", "hairsp;", " ", "half;", "½", "hamilt;", "ℋ", "hardcy;", "ъ", "harr;", "↔", "harrcir;", "⥈", "harrw;", "↭", "hbar;", "ℏ", "hcirc;", "ĥ", "hearts;", "♥", "heartsuit;", "♥", "hellip;", "…", "hercon;", "⊹", "hfr;", "\ud835\udd25", "hksearow;", "⤥", "hkswarow;", "⤦", "hoarr;", "⇿", "homtht;", "∻", "hookleftarrow;", "↩", "hookrightarrow;", "↪", "hopf;", "\ud835\udd59", "horbar;", "―", "hscr;", "\ud835\udcbd", "hslash;", "ℏ", "hstrok;", "ħ", "hybull;", "⁃", "hyphen;", "‐", "iacute", "í", "iacute;", "í", "ic;", "⁣", "icirc", "î", "icirc;", "î", "icy;", "и", "iecy;", "е", "iexcl", "¡", "iexcl;", "¡", "iff;", "⇔", "ifr;", "\ud835\udd26", "igrave", "ì", "igrave;", "ì", "ii;", "ⅈ", "iiiint;", "⨌", "iiint;", "∭", "iinfin;", "⧜", "iiota;", "℩", "ijlig;", "ĳ", "imacr;", "ī", "image;", "ℑ", "imagline;", "ℐ", "imagpart;", "ℑ", "imath;", "ı", "imof;", "⊷", "imped;", "Ƶ", "in;", "∈", "incare;", "℅", "infin;", "∞", "infintie;", "⧝", "inodot;", "ı", "int;", "∫", "intcal;", "⊺", "integers;", "ℤ", "intercal;", "⊺", "intlarhk;", "⨗", "intprod;", "⨼", "iocy;", "ё", "iogon;", "į", "iopf;", "\ud835\udd5a", "iota;", "ι", "iprod;", "⨼", "iquest", "¿", "iquest;", "¿", "iscr;", "\ud835\udcbe", "isin;", "∈", "isinE;", "⋹", "isindot;", "⋵", "isins;", "⋴", "isinsv;", "⋳", "isinv;", "∈", "it;", "⁢", "itilde;", "ĩ", "iukcy;", "і", "iuml", "ï", "iuml;", "ï", "jcirc;", "ĵ", "jcy;", "й", "jfr;", "\ud835\udd27", "jmath;", "ȷ", "jopf;", "\ud835\udd5b", "jscr;", "\ud835\udcbf", "jsercy;", "ј", "jukcy;", "є", "kappa;", "κ", "kappav;", "ϰ", "kcedil;", "ķ", "kcy;", "к", "kfr;", "\ud835\udd28", "kgreen;", "ĸ", "khcy;", "х", "kjcy;", "ќ", "kopf;", "\ud835\udd5c", "kscr;", "\ud835\udcc0", "lAarr;", "⇚", "lArr;", "⇐", "lAtail;", "⤛", "lBarr;", "⤎", "lE;", "≦", "lEg;", "⪋", "lHar;", "⥢", "lacute;", "ĺ", "laemptyv;", "⦴", "lagran;", "ℒ", "lambda;", "λ", "lang;", "⟨", "langd;", "⦑", "langle;", "⟨", "lap;", "⪅", "laquo", "«", "laquo;", "«", "larr;", "←", "larrb;", "⇤", "larrbfs;", "⤟", "larrfs;", "⤝", "larrhk;", "↩", "larrlp;", "↫", "larrpl;", "⤹", "larrsim;", "⥳", "larrtl;", "↢", "lat;", "⪫", "latail;", "⤙", "late;", "⪭", "lates;", "⪭︀", "lbarr;", "⤌", "lbbrk;", "❲", "lbrace;", "{", "lbrack;", "[", "lbrke;", "⦋", "lbrksld;", "⦏", "lbrkslu;", "⦍", "lcaron;", "ľ", "lcedil;", "ļ", "lceil;", "⌈", "lcub;", "{", "lcy;", "л", "ldca;", "⤶", "ldquo;", "“", "ldquor;", "„", "ldrdhar;", "⥧", "ldrushar;", "⥋", "ldsh;", "↲", "le;", "≤", "leftarrow;", "←", "leftarrowtail;", "↢", "leftharpoondown;", "↽", "leftharpoonup;", "↼", "leftleftarrows;", "⇇", "leftrightarrow;", "↔", "leftrightarrows;", "⇆", "leftrightharpoons;", "⇋", "leftrightsquigarrow;", "↭", "leftthreetimes;", "⋋", "leg;", "⋚", "leq;", "≤", "leqq;", "≦", "leqslant;", "⩽", "les;", "⩽", "lescc;", "⪨", "lesdot;", "⩿", "lesdoto;", "⪁", "lesdotor;", "⪃", "lesg;", "⋚︀", "lesges;", "⪓", "lessapprox;", "⪅", "lessdot;", "⋖", "lesseqgtr;", "⋚", "lesseqqgtr;", "⪋", "lessgtr;", "≶", "lesssim;", "≲", "lfisht;", "⥼", "lfloor;", "⌊", "lfr;", "\ud835\udd29", "lg;", "≶", "lgE;", "⪑", "lhard;", "↽", "lharu;", "↼", "lharul;", "⥪", "lhblk;", "▄", "ljcy;", "љ", "ll;", "≪", "llarr;", "⇇", "llcorner;", "⌞", "llhard;", "⥫", "lltri;", "◺", "lmidot;", "ŀ", "lmoust;", "⎰", "lmoustache;", "⎰", "lnE;", "≨", "lnap;", "⪉", "lnapprox;", "⪉", "lne;", "⪇", "lneq;", "⪇", "lneqq;", "≨", "lnsim;", "⋦", "loang;", "⟬", "loarr;", "⇽", "lobrk;", "⟦", "longleftarrow;", "⟵", "longleftrightarrow;", "⟷", "longmapsto;", "⟼", "longrightarrow;", "⟶", "looparrowleft;", "↫", "looparrowright;", "↬", "lopar;", "⦅", "lopf;", "\ud835\udd5d", "loplus;", "⨭", "lotimes;", "⨴", "lowast;", "∗", "lowbar;", "_", "loz;", "◊", "lozenge;", "◊", "lozf;", "⧫", "lpar;", "(", "lparlt;", "⦓", "lrarr;", "⇆", "lrcorner;", "⌟", "lrhar;", "⇋", "lrhard;", "⥭", "lrm;", "‎", "lrtri;", "⊿", "lsaquo;", "‹", "lscr;", "\ud835\udcc1", "lsh;", "↰", "lsim;", "≲", "lsime;", "⪍", "lsimg;", "⪏", "lsqb;", "[", "lsquo;", "‘", "lsquor;", "‚", "lstrok;", "ł", "lt", "<", "lt;", "<", "ltcc;", "⪦", "ltcir;", "⩹", "ltdot;", "⋖", "lthree;", "⋋", "ltimes;", "⋉", "ltlarr;", "⥶", "ltquest;", "⩻", "ltrPar;", "⦖", "ltri;", "◃", "ltrie;", "⊴", "ltrif;", "◂", "lurdshar;", "⥊", "luruhar;", "⥦", "lvertneqq;", "≨︀", "lvnE;", "≨︀", "mDDot;", "∺", "macr", "¯", "macr;", "¯", "male;", "♂", "malt;", "✠", "maltese;", "✠", "map;", "↦", "mapsto;", "↦", "mapstodown;", "↧", "mapstoleft;", "↤", "mapstoup;", "↥", "marker;", "▮", "mcomma;", "⨩", "mcy;", "м", "mdash;", "—", "measuredangle;", "∡", "mfr;", "\ud835\udd2a", "mho;", "℧", "micro", "µ", "micro;", "µ", "mid;", "∣", "midast;", "*", "midcir;", "⫰", "middot", "·", "middot;", "·", "minus;", "−", "minusb;", "⊟", "minusd;", "∸", "minusdu;", "⨪", "mlcp;", "⫛", "mldr;", "…", "mnplus;", "∓", "models;", "⊧", "mopf;", "\ud835\udd5e", "mp;", "∓", "mscr;", "\ud835\udcc2", "mstpos;", "∾", "mu;", "μ", "multimap;", "⊸", "mumap;", "⊸", "nGg;", "⋙̸", "nGt;", "≫⃒", "nGtv;", "≫̸", "nLeftarrow;", "⇍", "nLeftrightarrow;", "⇎", "nLl;", "⋘̸", "nLt;", "≪⃒", "nLtv;", "≪̸", "nRightarrow;", "⇏", "nVDash;", "⊯", "nVdash;", "⊮", "nabla;", "∇", "nacute;", "ń", "nang;", "∠⃒", "nap;", "≉", "napE;", "⩰̸", "napid;", "≋̸", "napos;", "ŉ", "napprox;", "≉", "natur;", "♮", "natural;", "♮", "naturals;", "ℕ", "nbsp", " ", "nbsp;", " ", "nbump;", "≎̸", "nbumpe;", "≏̸", "ncap;", "⩃", "ncaron;", "ň", "ncedil;", "ņ", "ncong;", "≇", "ncongdot;", "⩭̸", "ncup;", "⩂", "ncy;", "н", "ndash;", "–", "ne;", "≠", "neArr;", "⇗", "nearhk;", "⤤", "nearr;", "↗", "nearrow;", "↗", "nedot;", "≐̸", "nequiv;", "≢", "nesear;", "⤨", "nesim;", "≂̸", "nexist;", "∄", "nexists;", "∄", "nfr;", "\ud835\udd2b", "ngE;", "≧̸", "nge;", "≱", "ngeq;", "≱", "ngeqq;", "≧̸", "ngeqslant;", "⩾̸", "nges;", "⩾̸", "ngsim;", "≵", "ngt;", "≯", "ngtr;", "≯", "nhArr;", "⇎", "nharr;", "↮", "nhpar;", "⫲", "ni;", "∋", "nis;", "⋼", "nisd;", "⋺", "niv;", "∋", "njcy;", "њ", "nlArr;", "⇍", "nlE;", "≦̸", "nlarr;", "↚", "nldr;", "‥", "nle;", "≰", "nleftarrow;", "↚", "nleftrightarrow;", "↮", "nleq;", "≰", "nleqq;", "≦̸", "nleqslant;", "⩽̸", "nles;", "⩽̸", "nless;", "≮", "nlsim;", "≴", "nlt;", "≮", "nltri;", "⋪", "nltrie;", "⋬", "nmid;", "∤", "nopf;", "\ud835\udd5f", "not", "¬", "not;", "¬", "notin;", "∉", "notinE;", "⋹̸", "notindot;", "⋵̸", "notinva;", "∉", "notinvb;", "⋷", "notinvc;", "⋶", "notni;", "∌", "notniva;", "∌", "notnivb;", "⋾", "notnivc;", "⋽", "npar;", "∦", "nparallel;", "∦", "nparsl;", "⫽⃥", "npart;", "∂̸", "npolint;", "⨔", "npr;", "⊀", "nprcue;", "⋠", "npre;", "⪯̸", "nprec;", "⊀", "npreceq;", "⪯̸", "nrArr;", "⇏", "nrarr;", "↛", "nrarrc;", "⤳̸", "nrarrw;", "↝̸", "nrightarrow;", "↛", "nrtri;", "⋫", "nrtrie;", "⋭", "nsc;", "⊁", "nsccue;", "⋡", "nsce;", "⪰̸", "nscr;", "\ud835\udcc3", "nshortmid;", "∤", "nshortparallel;", "∦", "nsim;", "≁", "nsime;", "≄", "nsimeq;", "≄", "nsmid;", "∤", "nspar;", "∦", "nsqsube;", "⋢", "nsqsupe;", "⋣", "nsub;", "⊄", "nsubE;", "⫅̸", "nsube;", "⊈", "nsubset;", "⊂⃒", "nsubseteq;", "⊈", "nsubseteqq;", "⫅̸", "nsucc;", "⊁", "nsucceq;", "⪰̸", "nsup;", "⊅", "nsupE;", "⫆̸", "nsupe;", "⊉", "nsupset;", "⊃⃒", "nsupseteq;", "⊉", "nsupseteqq;", "⫆̸", "ntgl;", "≹", "ntilde", "ñ", "ntilde;", "ñ", "ntlg;", "≸", "ntriangleleft;", "⋪", "ntrianglelefteq;", "⋬", "ntriangleright;", "⋫", "ntrianglerighteq;", "⋭", "nu;", "ν", "num;", "#", "numero;", "№", "numsp;", " ", "nvDash;", "⊭", "nvHarr;", "⤄", "nvap;", "≍⃒", "nvdash;", "⊬", "nvge;", "≥⃒", "nvgt;", ">⃒", "nvinfin;", "⧞", "nvlArr;", "⤂", "nvle;", "≤⃒", "nvlt;", "<⃒", "nvltrie;", "⊴⃒", "nvrArr;", "⤃", "nvrtrie;", "⊵⃒", "nvsim;", "∼⃒", "nwArr;", "⇖", "nwarhk;", "⤣", "nwarr;", "↖", "nwarrow;", "↖", "nwnear;", "⤧", "oS;", "Ⓢ", "oacute", "ó", "oacute;", "ó", "oast;", "⊛", "ocir;", "⊚", "ocirc", "ô", "ocirc;", "ô", "ocy;", "о", "odash;", "⊝", "odblac;", "ő", "odiv;", "⨸", "odot;", "⊙", "odsold;", "⦼", "oelig;", "œ", "ofcir;", "⦿", "ofr;", "\ud835\udd2c", "ogon;", "˛", "ograve", "ò", "ograve;", "ò", "ogt;", "⧁", "ohbar;", "⦵", "ohm;", "Ω", "oint;", "∮", "olarr;", "↺", "olcir;", "⦾", "olcross;", "⦻", "oline;", "‾", "olt;", "⧀", "omacr;", "ō", "omega;", "ω", "omicron;", "ο", "omid;", "⦶", "ominus;", "⊖", "oopf;", "\ud835\udd60", "opar;", "⦷", "operp;", "⦹", "oplus;", "⊕", "or;", "∨", "orarr;", "↻", "ord;", "⩝", "order;", "ℴ", "orderof;", "ℴ", "ordf", "ª", "ordf;", "ª", "ordm", "º", "ordm;", "º", "origof;", "⊶", "oror;", "⩖", "orslope;", "⩗", "orv;", "⩛", "oscr;", "ℴ", "oslash", "ø", "oslash;", "ø", "osol;", "⊘", "otilde", "õ", "otilde;", "õ", "otimes;", "⊗", "otimesas;", "⨶", "ouml", "ö", "ouml;", "ö", "ovbar;", "⌽", "par;", "∥", "para", "¶", "para;", "¶", "parallel;", "∥", "parsim;", "⫳", "parsl;", "⫽", "part;", "∂", "pcy;", "п", "percnt;", "%", "period;", ".", "permil;", "‰", "perp;", "⊥", "pertenk;", "‱", "pfr;", "\ud835\udd2d", "phi;", "φ", "phiv;", "ϕ", "phmmat;", "ℳ", "phone;", "☎", "pi;", "π", "pitchfork;", "⋔", "piv;", "ϖ", "planck;", "ℏ", "planckh;", "ℎ", "plankv;", "ℏ", "plus;", "+", "plusacir;", "⨣", "plusb;", "⊞", "pluscir;", "⨢", "plusdo;", "∔", "plusdu;", "⨥", "pluse;", "⩲", "plusmn", "±", "plusmn;", "±", "plussim;", "⨦", "plustwo;", "⨧", "pm;", "±", "pointint;", "⨕", "popf;", "\ud835\udd61", "pound", "£", "pound;", "£", "pr;", "≺", "prE;", "⪳", "prap;", "⪷", "prcue;", "≼", "pre;", "⪯", "prec;", "≺", "precapprox;", "⪷", "preccurlyeq;", "≼", "preceq;", "⪯", "precnapprox;", "⪹", "precneqq;", "⪵", "precnsim;", "⋨", "precsim;", "≾", "prime;", "′", "primes;", "ℙ", "prnE;", "⪵", "prnap;", "⪹", "prnsim;", "⋨", "prod;", "∏", "profalar;", "⌮", "profline;", "⌒", "profsurf;", "⌓", "prop;", "∝", "propto;", "∝", "prsim;", "≾", "prurel;", "⊰", "pscr;", "\ud835\udcc5", "psi;", "ψ", "puncsp;", " ", "qfr;", "\ud835\udd2e", "qint;", "⨌", "qopf;", "\ud835\udd62", "qprime;", "⁗", "qscr;", "\ud835\udcc6", "quaternions;", "ℍ", "quatint;", "⨖", "quest;", "?", "questeq;", "≟", "quot", "\u0022", "quot;", "\u0022", "rAarr;", "⇛", "rArr;", "⇒", "rAtail;", "⤜", "rBarr;", "⤏", "rHar;", "⥤", "race;", "∽̱", "racute;", "ŕ", "radic;", "√", "raemptyv;", "⦳", "rang;", "⟩", "rangd;", "⦒", "range;", "⦥", "rangle;", "⟩", "raquo", "»", "raquo;", "»", "rarr;", "→", "rarrap;", "⥵", "rarrb;", "⇥", "rarrbfs;", "⤠", "rarrc;", "⤳", "rarrfs;", "⤞", "rarrhk;", "↪", "rarrlp;", "↬", "rarrpl;", "⥅", "rarrsim;", "⥴", "rarrtl;", "↣", "rarrw;", "↝", "ratail;", "⤚", "ratio;", "∶", "rationals;", "ℚ", "rbarr;", "⤍", "rbbrk;", "❳", "rbrace;", "}", "rbrack;", "]", "rbrke;", "⦌", "rbrksld;", "⦎", "rbrkslu;", "⦐", "rcaron;", "ř", "rcedil;", "ŗ", "rceil;", "⌉", "rcub;", "}", "rcy;", "р", "rdca;", "⤷", "rdldhar;", "⥩", "rdquo;", "”", "rdquor;", "”", "rdsh;", "↳", "real;", "ℜ", "realine;", "ℛ", "realpart;", "ℜ", "reals;", "ℝ", "rect;", "▭", "reg", "®", "reg;", "®", "rfisht;", "⥽", "rfloor;", "⌋", "rfr;", "\ud835\udd2f", "rhard;", "⇁", "rharu;", "⇀", "rharul;", "⥬", "rho;", "ρ", "rhov;", "ϱ", "rightarrow;", "→", "rightarrowtail;", "↣", "rightharpoondown;", "⇁", "rightharpoonup;", "⇀", "rightleftarrows;", "⇄", "rightleftharpoons;", "⇌", "rightrightarrows;", "⇉", "rightsquigarrow;", "↝", "rightthreetimes;", "⋌", "ring;", "˚", "risingdotseq;", "≓", "rlarr;", "⇄", "rlhar;", "⇌", "rlm;", "‏", "rmoust;", "⎱", "rmoustache;", "⎱", "rnmid;", "⫮", "roang;", "⟭", "roarr;", "⇾", "robrk;", "⟧", "ropar;", "⦆", "ropf;", "\ud835\udd63", "roplus;", "⨮", "rotimes;", "⨵", "rpar;", ")", "rpargt;", "⦔", "rppolint;", "⨒", "rrarr;", "⇉", "rsaquo;", "›", "rscr;", "\ud835\udcc7", "rsh;", "↱", "rsqb;", "]", "rsquo;", "’", "rsquor;", "’", "rthree;", "⋌", "rtimes;", "⋊", "rtri;", "▹", "rtrie;", "⊵", "rtrif;", "▸", "rtriltri;", "⧎", "ruluhar;", "⥨", "rx;", "℞", "sacute;", "ś", "sbquo;", "‚", "sc;", "≻", "scE;", "⪴", "scap;", "⪸", "scaron;", "š", "sccue;", "≽", "sce;", "⪰", "scedil;", "ş", "scirc;", "ŝ", "scnE;", "⪶", "scnap;", "⪺", "scnsim;", "⋩", "scpolint;", "⨓", "scsim;", "≿", "scy;", "с", "sdot;", "⋅", "sdotb;", "⊡", "sdote;", "⩦", "seArr;", "⇘", "searhk;", "⤥", "searr;", "↘", "searrow;", "↘", "sect", "§", "sect;", "§", "semi;", ";", "seswar;", "⤩", "setminus;", "∖", "setmn;", "∖", "sext;", "✶", "sfr;", "\ud835\udd30", "sfrown;", "⌢", "sharp;", "♯", "shchcy;", "щ", "shcy;", "ш", "shortmid;", "∣", "shortparallel;", "∥", "shy", "­", "shy;", "­", "sigma;", "σ", "sigmaf;", "ς", "sigmav;", "ς", "sim;", "∼", "simdot;", "⩪", "sime;", "≃", "simeq;", "≃", "simg;", "⪞", "simgE;", "⪠", "siml;", "⪝", "simlE;", "⪟", "simne;", "≆", "simplus;", "⨤", "simrarr;", "⥲", "slarr;", "←", "smallsetminus;", "∖", "smashp;", "⨳", "smeparsl;", "⧤", "smid;", "∣", "smile;", "⌣", "smt;", "⪪", "smte;", "⪬", "smtes;", "⪬︀", "softcy;", "ь", "sol;", "/", "solb;", "⧄", "solbar;", "⌿", "sopf;", "\ud835\udd64", "spades;", "♠", "spadesuit;", "♠", "spar;", "∥", "sqcap;", "⊓", "sqcaps;", "⊓︀", "sqcup;", "⊔", "sqcups;", "⊔︀", "sqsub;", "⊏", "sqsube;", "⊑", "sqsubset;", "⊏", "sqsubseteq;", "⊑", "sqsup;", "⊐", "sqsupe;", "⊒", "sqsupset;", "⊐", "sqsupseteq;", "⊒", "squ;", "□", "square;", "□", "squarf;", "▪", "squf;", "▪", "srarr;", "→", "sscr;", "\ud835\udcc8", "ssetmn;", "∖", "ssmile;", "⌣", "sstarf;", "⋆", "star;", "☆", "starf;", "★", "straightepsilon;", "ϵ", "straightphi;", "ϕ", "strns;", "¯", "sub;", "⊂", "subE;", "⫅", "subdot;", "⪽", "sube;", "⊆", "subedot;", "⫃", "submult;", "⫁", "subnE;", "⫋", "subne;", "⊊", "subplus;", "⪿", "subrarr;", "⥹", "subset;", "⊂", "subseteq;", "⊆", "subseteqq;", "⫅", "subsetneq;", "⊊", "subsetneqq;", "⫋", "subsim;", "⫇", "subsub;", "⫕", "subsup;", "⫓", "succ;", "≻", "succapprox;", "⪸", "succcurlyeq;", "≽", "succeq;", "⪰", "succnapprox;", "⪺", "succneqq;", "⪶", "succnsim;", "⋩", "succsim;", "≿", "sum;", "∑", "sung;", "♪", "sup1", "¹", "sup1;", "¹", "sup2", "²", "sup2;", "²", "sup3", "³", "sup3;", "³", "sup;", "⊃", "supE;", "⫆", "supdot;", "⪾", "supdsub;", "⫘", "supe;", "⊇", "supedot;", "⫄", "suphsol;", "⟉", "suphsub;", "⫗", "suplarr;", "⥻", "supmult;", "⫂", "supnE;", "⫌", "supne;", "⊋", "supplus;", "⫀", "supset;", "⊃", "supseteq;", "⊇", "supseteqq;", "⫆", "supsetneq;", "⊋", "supsetneqq;", "⫌", "supsim;", "⫈", "supsub;", "⫔", "supsup;", "⫖", "swArr;", "⇙", "swarhk;", "⤦", "swarr;", "↙", "swarrow;", "↙", "swnwar;", "⤪", "szlig", "ß", "szlig;", "ß", "target;", "⌖", "tau;", "τ", "tbrk;", "⎴", "tcaron;", "ť", "tcedil;", "ţ", "tcy;", "т", "tdot;", "⃛", "telrec;", "⌕", "tfr;", "\ud835\udd31", "there4;", "∴", "therefore;", "∴", "theta;", "θ", "thetasym;", "ϑ", "thetav;", "ϑ", "thickapprox;", "≈", "thicksim;", "∼", "thinsp;", " ", "thkap;", "≈", "thksim;", "∼", "thorn", "þ", "thorn;", "þ", "tilde;", "˜", "times", "×", "times;", "×", "timesb;", "⊠", "timesbar;", "⨱", "timesd;", "⨰", "tint;", "∭", "toea;", "⤨", "top;", "⊤", "topbot;", "⌶", "topcir;", "⫱", "topf;", "\ud835\udd65", "topfork;", "⫚", "tosa;", "⤩", "tprime;", "‴", "trade;", "™", "triangle;", "▵", "triangledown;", "▿", "triangleleft;", "◃", "trianglelefteq;", "⊴", "triangleq;", "≜", "triangleright;", "▹", "trianglerighteq;", "⊵", "tridot;", "◬", "trie;", "≜", "triminus;", "⨺", "triplus;", "⨹", "trisb;", "⧍", "tritime;", "⨻", "trpezium;", "⏢", "tscr;", "\ud835\udcc9", "tscy;", "ц", "tshcy;", "ћ", "tstrok;", "ŧ", "twixt;", "≬", "twoheadleftarrow;", "↞", "twoheadrightarrow;", "↠", "uArr;", "⇑", "uHar;", "⥣", "uacute", "ú", "uacute;", "ú", "uarr;", "↑", "ubrcy;", "ў", "ubreve;", "ŭ", "ucirc", "û", "ucirc;", "û", "ucy;", "у", "udarr;", "⇅", "udblac;", "ű", "udhar;", "⥮", "ufisht;", "⥾", "ufr;", "\ud835\udd32", "ugrave", "ù", "ugrave;", "ù", "uharl;", "↿", "uharr;", "↾", "uhblk;", "▀", "ulcorn;", "⌜", "ulcorner;", "⌜", "ulcrop;", "⌏", "ultri;", "◸", "umacr;", "ū", "uml", "¨", "uml;", "¨", "uogon;", "ų", "uopf;", "\ud835\udd66", "uparrow;", "↑", "updownarrow;", "↕", "upharpoonleft;", "↿", "upharpoonright;", "↾", "uplus;", "⊎", "upsi;", "υ", "upsih;", "ϒ", "upsilon;", "υ", "upuparrows;", "⇈", "urcorn;", "⌝", "urcorner;", "⌝", "urcrop;", "⌎", "uring;", "ů", "urtri;", "◹", "uscr;", "\ud835\udcca", "utdot;", "⋰", "utilde;", "ũ", "utri;", "▵", "utrif;", "▴", "uuarr;", "⇈", "uuml", "ü", "uuml;", "ü", "uwangle;", "⦧", "vArr;", "⇕", "vBar;", "⫨", "vBarv;", "⫩", "vDash;", "⊨", "vangrt;", "⦜", "varepsilon;", "ϵ", "varkappa;", "ϰ", "varnothing;", "∅", "varphi;", "ϕ", "varpi;", "ϖ", "varpropto;", "∝", "varr;", "↕", "varrho;", "ϱ", "varsigma;", "ς", "varsubsetneq;", "⊊︀", "varsubsetneqq;", "⫋︀", "varsupsetneq;", "⊋︀", "varsupsetneqq;", "⫌︀", "vartheta;", "ϑ", "vartriangleleft;", "⊲", "vartriangleright;", "⊳", "vcy;", "в", "vdash;", "⊢", "vee;", "∨", "veebar;", "⊻", "veeeq;", "≚", "vellip;", "⋮", "verbar;", "|", "vert;", "|", "vfr;", "\ud835\udd33", "vltri;", "⊲", "vnsub;", "⊂⃒", "vnsup;", "⊃⃒", "vopf;", "\ud835\udd67", "vprop;", "∝", "vrtri;", "⊳", "vscr;", "\ud835\udccb", "vsubnE;", "⫋︀", "vsubne;", "⊊︀", "vsupnE;", "⫌︀", "vsupne;", "⊋︀", "vzigzag;", "⦚", "wcirc;", "ŵ", "wedbar;", "⩟", "wedge;", "∧", "wedgeq;", "≙", "weierp;", "℘", "wfr;", "\ud835\udd34", "wopf;", "\ud835\udd68", "wp;", "℘", "wr;", "≀", "wreath;", "≀", "wscr;", "\ud835\udccc", "xcap;", "⋂", "xcirc;", "◯", "xcup;", "⋃", "xdtri;", "▽", "xfr;", "\ud835\udd35", "xhArr;", "⟺", "xharr;", "⟷", "xi;", "ξ", "xlArr;", "⟸", "xlarr;", "⟵", "xmap;", "⟼", "xnis;", "⋻", "xodot;", "⨀", "xopf;", "\ud835\udd69", "xoplus;", "⨁", "xotime;", "⨂", "xrArr;", "⟹", "xrarr;", "⟶", "xscr;", "\ud835\udccd", "xsqcup;", "⨆", "xuplus;", "⨄", "xutri;", "△", "xvee;", "⋁", "xwedge;", "⋀", "yacute", "ý", "yacute;", "ý", "yacy;", "я", "ycirc;", "ŷ", "ycy;", "ы", "yen", "¥", "yen;", "¥", "yfr;", "\ud835\udd36", "yicy;", "ї", "yopf;", "\ud835\udd6a", "yscr;", "\ud835\udcce", "yucy;", "ю", "yuml", "ÿ", "yuml;", "ÿ", "zacute;", "ź", "zcaron;", "ž", "zcy;", "з", "zdot;", "ż", "zeetrf;", "ℨ", "zeta;", "ζ", "zfr;", "\ud835\udd37", "zhcy;", "ж", "zigrarr;", "⇝", "zopf;", "\ud835\udd6b", "zscr;", "\ud835\udccf", "zwj;", "‍", "zwnj;", "‌");
            mb__376 = new C1::OrderedDictionary<string, string>();
            i__377 = 0;
            n__378 = strs__375.Count;
            while (i__377 < n__378)
            {
                mb__376[strs__375[i__377]] = strs__375[i__377 + 1];
                i__377 = i__377 + 2;
            }
            return__374 = C1::Mapped.ToMap(mb__376);
            htmlNamedCharacters__267 = return__374;
            return__373 = new HtmlCodec();
            HtmlCodec = return__373;
            urlContextPropagator__268 = new UrlContextPropagator();
            protocolAllowList__293 = new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(R::RegexGlobal.Begin, new R::Or(C1::Listed.CreateReadOnlyList<R::Sequence>(new R::Sequence(C1::Listed.CreateReadOnlyList<R::IRegexNode>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("H"), new R::CodePoints("h")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("T"), new R::CodePoints("t")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("T"), new R::CodePoints("t")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("P"), new R::CodePoints("p")), false), new R::Repeat(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("S"), new R::CodePoints("s")), false), 0, 1, false))), new R::Sequence(C1::Listed.CreateReadOnlyList<R::CodeSet>(new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("M"), new R::CodePoints("m")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("A"), new R::CodePoints("a")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("I"), new R::CodePoints("i")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("L"), new R::CodePoints("l")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("T"), new R::CodePoints("t")), false), new R::CodeSet(C1::Listed.CreateReadOnlyList<R::CodePoints>(new R::CodePoints("O"), new R::CodePoints("o")), false))))), R::RegexGlobal.End)).Compiled();
            fallbackSafeUrl__294 = new SafeUrl("about:zz_Temper_zz#");
            lb__732 = new G::List<bool>();
            i__733 = 0;
            while (i__733 < 128)
            {
                if (i__733 == 47)
                {
                    t___3455 = true;
                }
                else
                {
                    if (i__733 == 46)
                    {
                        t___3454 = true;
                    }
                    else
                    {
                        if (i__733 == 45)
                        {
                            t___3453 = true;
                        }
                        else
                        {
                            if (i__733 == 95)
                            {
                                t___3452 = true;
                            }
                            else
                            {
                                if (48 <= i__733)
                                {
                                    t___3449 = i__733 <= 57;
                                }
                                else
                                {
                                    t___3449 = false;
                                }
                                if (t___3449)
                                {
                                    t___3451 = true;
                                }
                                else
                                {
                                    if (97 <= (i__733 | 32))
                                    {
                                        t___3450 = (i__733 | 32) <= 122;
                                    }
                                    else
                                    {
                                        t___3450 = false;
                                    }
                                    t___3451 = t___3450;
                                }
                                t___3452 = t___3451;
                            }
                            t___3453 = t___3452;
                        }
                        t___3454 = t___3453;
                    }
                    t___3455 = t___3454;
                }
                C1::Listed.Add(lb__732, t___3455);
                i__733 = i__733 + 1;
            }
            urlQuerySafe__296 = C1::Listed.ToReadOnlyList(lb__732);
            lb__734 = new G::List<bool>();
            i__735 = 0;
            while (i__735 < 128)
            {
                if (urlQuerySafe__296[i__735])
                {
                    t___3464 = true;
                }
                else
                {
                    if (i__735 == 58)
                    {
                        t___3463 = true;
                    }
                    else
                    {
                        if (i__735 == 63)
                        {
                            t___3462 = true;
                        }
                        else
                        {
                            if (i__735 == 35)
                            {
                                t___3461 = true;
                            }
                            else
                            {
                                t___3461 = i__735 == 38;
                            }
                            t___3462 = t___3461;
                        }
                        t___3463 = t___3462;
                    }
                    t___3464 = t___3463;
                }
                C1::Listed.Add(lb__734, t___3464);
                i__735 = i__735 + 1;
            }
            urlSafe__297 = C1::Listed.ToReadOnlyList(lb__734);
            return__749 = new HtmlUrlPartUrlEscaper();
            htmlUrlPartUrlEscaper__298 = return__749;
            return__731 = new HtmlProtocolFilteringUrlEscaper();
            htmlProtocolFilteringUrlEscaper__295 = return__731;
            return__763 = new HtmlAsIfQueryUrlEscaper();
            htmlAsIfQueryUrlEscaper__299 = return__763;
            return__593 = new HtmlPcdataEscaper();
            htmlPcdataEscaper__291 = return__593;
            return__572 = new OutputHtmlSpaceEscaper();
            outputHtmlSpaceEscaper__290 = return__572;
            return__614 = new HtmlAttributeEscaper();
            htmlAttributeEscaper__292 = return__614;
        }
    }
}
