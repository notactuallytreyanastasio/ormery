#![allow(warnings)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
use crate::core::CodecTrait;
use crate::core::ContextualAutoescapingAccumulatorTrait;
use temper_std::regex::RegexNodeTrait;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            let mut t___3449: bool;
            let mut t___3450: bool;
            let mut t___3451: bool;
            let mut t___3452: bool;
            let mut t___3453: bool;
            let mut t___3454: bool;
            let mut t___3455: bool;
            let mut t___3461: bool;
            let mut t___3462: bool;
            let mut t___3463: bool;
            let mut t___3464: bool;
            OUTPUT_HTML_SPACE_ESCAPER__INSTANCE.set(OutputHtmlSpaceEscaper::new()).unwrap_or_else(| _ | panic!());
            HTML_PCDATA_ESCAPER__INSTANCE.set(HtmlPcdataEscaper::new()).unwrap_or_else(| _ | panic!());
            HTML_ATTRIBUTE_ESCAPER__INSTANCE.set(HtmlAttributeEscaper::new()).unwrap_or_else(| _ | panic!());
            HTML_PROTOCOL_FILTERING_URL_ESCAPER__INSTANCE.set(HtmlProtocolFilteringUrlEscaper::new()).unwrap_or_else(| _ | panic!());
            HTML_URL_PART_URL_ESCAPER__INSTANCE.set(HtmlUrlPartUrlEscaper::new()).unwrap_or_else(| _ | panic!());
            HTML_AS_IF_QUERY_URL_ESCAPER__INSTANCE.set(HtmlAsIfQueryUrlEscaper::new()).unwrap_or_else(| _ | panic!());
            let strs__375: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("AElig".to_string()), std::sync::Arc::new("Æ".to_string()), std::sync::Arc::new("AElig;".to_string()), std::sync::Arc::new("Æ".to_string()), std::sync::Arc::new("AMP".to_string()), std::sync::Arc::new("&".to_string()), std::sync::Arc::new("AMP;".to_string()), std::sync::Arc::new("&".to_string()), std::sync::Arc::new("Aacute".to_string()), std::sync::Arc::new("Á".to_string()), std::sync::Arc::new("Aacute;".to_string()), std::sync::Arc::new("Á".to_string()), std::sync::Arc::new("Abreve;".to_string()), std::sync::Arc::new("Ă".to_string()), std::sync::Arc::new("Acirc".to_string()), std::sync::Arc::new("Â".to_string()), std::sync::Arc::new("Acirc;".to_string()), std::sync::Arc::new("Â".to_string()), std::sync::Arc::new("Acy;".to_string()), std::sync::Arc::new("А".to_string()), std::sync::Arc::new("Afr;".to_string()), std::sync::Arc::new("𝔄".to_string()), std::sync::Arc::new("Agrave".to_string()), std::sync::Arc::new("À".to_string()), std::sync::Arc::new("Agrave;".to_string()), std::sync::Arc::new("À".to_string()), std::sync::Arc::new("Alpha;".to_string()), std::sync::Arc::new("Α".to_string()), std::sync::Arc::new("Amacr;".to_string()), std::sync::Arc::new("Ā".to_string()), std::sync::Arc::new("And;".to_string()), std::sync::Arc::new("\u{2a53}".to_string()), std::sync::Arc::new("Aogon;".to_string()), std::sync::Arc::new("Ą".to_string()), std::sync::Arc::new("Aopf;".to_string()), std::sync::Arc::new("𝔸".to_string()), std::sync::Arc::new("ApplyFunction;".to_string()), std::sync::Arc::new("\u{2061}".to_string()), std::sync::Arc::new("Aring".to_string()), std::sync::Arc::new("Å".to_string()), std::sync::Arc::new("Aring;".to_string()), std::sync::Arc::new("Å".to_string()), std::sync::Arc::new("Ascr;".to_string()), std::sync::Arc::new("𝒜".to_string()), std::sync::Arc::new("Assign;".to_string()), std::sync::Arc::new("\u{2254}".to_string()), std::sync::Arc::new("Atilde".to_string()), std::sync::Arc::new("Ã".to_string()), std::sync::Arc::new("Atilde;".to_string()), std::sync::Arc::new("Ã".to_string()), std::sync::Arc::new("Auml".to_string()), std::sync::Arc::new("Ä".to_string()), std::sync::Arc::new("Auml;".to_string()), std::sync::Arc::new("Ä".to_string()), std::sync::Arc::new("Backslash;".to_string()), std::sync::Arc::new("\u{2216}".to_string()), std::sync::Arc::new("Barv;".to_string()), std::sync::Arc::new("\u{2ae7}".to_string()), std::sync::Arc::new("Barwed;".to_string()), std::sync::Arc::new("\u{2306}".to_string()), std::sync::Arc::new("Bcy;".to_string()), std::sync::Arc::new("Б".to_string()), std::sync::Arc::new("Because;".to_string()), std::sync::Arc::new("\u{2235}".to_string()), std::sync::Arc::new("Bernoullis;".to_string()), std::sync::Arc::new("ℬ".to_string()), std::sync::Arc::new("Beta;".to_string()), std::sync::Arc::new("Β".to_string()), std::sync::Arc::new("Bfr;".to_string()), std::sync::Arc::new("𝔅".to_string()), std::sync::Arc::new("Bopf;".to_string()), std::sync::Arc::new("𝔹".to_string()), std::sync::Arc::new("Breve;".to_string()), std::sync::Arc::new("\u{2d8}".to_string()), std::sync::Arc::new("Bscr;".to_string()), std::sync::Arc::new("ℬ".to_string()), std::sync::Arc::new("Bumpeq;".to_string()), std::sync::Arc::new("\u{224e}".to_string()), std::sync::Arc::new("CHcy;".to_string()), std::sync::Arc::new("Ч".to_string()), std::sync::Arc::new("COPY".to_string()), std::sync::Arc::new("\u{a9}".to_string()), std::sync::Arc::new("COPY;".to_string()), std::sync::Arc::new("\u{a9}".to_string()), std::sync::Arc::new("Cacute;".to_string()), std::sync::Arc::new("Ć".to_string()), std::sync::Arc::new("Cap;".to_string()), std::sync::Arc::new("\u{22d2}".to_string()), std::sync::Arc::new("CapitalDifferentialD;".to_string()), std::sync::Arc::new("ⅅ".to_string()), std::sync::Arc::new("Cayleys;".to_string()), std::sync::Arc::new("ℭ".to_string()), std::sync::Arc::new("Ccaron;".to_string()), std::sync::Arc::new("Č".to_string()), std::sync::Arc::new("Ccedil".to_string()), std::sync::Arc::new("Ç".to_string()), std::sync::Arc::new("Ccedil;".to_string()), std::sync::Arc::new("Ç".to_string()), std::sync::Arc::new("Ccirc;".to_string()), std::sync::Arc::new("Ĉ".to_string()), std::sync::Arc::new("Cconint;".to_string()), std::sync::Arc::new("\u{2230}".to_string()), std::sync::Arc::new("Cdot;".to_string()), std::sync::Arc::new("Ċ".to_string()), std::sync::Arc::new("Cedilla;".to_string()), std::sync::Arc::new("\u{b8}".to_string()), std::sync::Arc::new("CenterDot;".to_string()), std::sync::Arc::new("\u{b7}".to_string()), std::sync::Arc::new("Cfr;".to_string()), std::sync::Arc::new("ℭ".to_string()), std::sync::Arc::new("Chi;".to_string()), std::sync::Arc::new("Χ".to_string()), std::sync::Arc::new("CircleDot;".to_string()), std::sync::Arc::new("\u{2299}".to_string()), std::sync::Arc::new("CircleMinus;".to_string()), std::sync::Arc::new("\u{2296}".to_string()), std::sync::Arc::new("CirclePlus;".to_string()), std::sync::Arc::new("\u{2295}".to_string()), std::sync::Arc::new("CircleTimes;".to_string()), std::sync::Arc::new("\u{2297}".to_string()), std::sync::Arc::new("ClockwiseContourIntegral;".to_string()), std::sync::Arc::new("\u{2232}".to_string()), std::sync::Arc::new("CloseCurlyDoubleQuote;".to_string()), std::sync::Arc::new("\u{201d}".to_string()), std::sync::Arc::new("CloseCurlyQuote;".to_string()), std::sync::Arc::new("\u{2019}".to_string()), std::sync::Arc::new("Colon;".to_string()), std::sync::Arc::new("\u{2237}".to_string()), std::sync::Arc::new("Colone;".to_string()), std::sync::Arc::new("\u{2a74}".to_string()), std::sync::Arc::new("Congruent;".to_string()), std::sync::Arc::new("\u{2261}".to_string()), std::sync::Arc::new("Conint;".to_string()), std::sync::Arc::new("\u{222f}".to_string()), std::sync::Arc::new("ContourIntegral;".to_string()), std::sync::Arc::new("\u{222e}".to_string()), std::sync::Arc::new("Copf;".to_string()), std::sync::Arc::new("ℂ".to_string()), std::sync::Arc::new("Coproduct;".to_string()), std::sync::Arc::new("\u{2210}".to_string()), std::sync::Arc::new("CounterClockwiseContourIntegral;".to_string()), std::sync::Arc::new("\u{2233}".to_string()), std::sync::Arc::new("Cross;".to_string()), std::sync::Arc::new("\u{2a2f}".to_string()), std::sync::Arc::new("Cscr;".to_string()), std::sync::Arc::new("𝒞".to_string()), std::sync::Arc::new("Cup;".to_string()), std::sync::Arc::new("\u{22d3}".to_string()), std::sync::Arc::new("CupCap;".to_string()), std::sync::Arc::new("\u{224d}".to_string()), std::sync::Arc::new("DD;".to_string()), std::sync::Arc::new("ⅅ".to_string()), std::sync::Arc::new("DDotrahd;".to_string()), std::sync::Arc::new("\u{2911}".to_string()), std::sync::Arc::new("DJcy;".to_string()), std::sync::Arc::new("Ђ".to_string()), std::sync::Arc::new("DScy;".to_string()), std::sync::Arc::new("Ѕ".to_string()), std::sync::Arc::new("DZcy;".to_string()), std::sync::Arc::new("Џ".to_string()), std::sync::Arc::new("Dagger;".to_string()), std::sync::Arc::new("\u{2021}".to_string()), std::sync::Arc::new("Darr;".to_string()), std::sync::Arc::new("\u{21a1}".to_string()), std::sync::Arc::new("Dashv;".to_string()), std::sync::Arc::new("\u{2ae4}".to_string()), std::sync::Arc::new("Dcaron;".to_string()), std::sync::Arc::new("Ď".to_string()), std::sync::Arc::new("Dcy;".to_string()), std::sync::Arc::new("Д".to_string()), std::sync::Arc::new("Del;".to_string()), std::sync::Arc::new("\u{2207}".to_string()), std::sync::Arc::new("Delta;".to_string()), std::sync::Arc::new("Δ".to_string()), std::sync::Arc::new("Dfr;".to_string()), std::sync::Arc::new("𝔇".to_string()), std::sync::Arc::new("DiacriticalAcute;".to_string()), std::sync::Arc::new("\u{b4}".to_string()), std::sync::Arc::new("DiacriticalDot;".to_string()), std::sync::Arc::new("\u{2d9}".to_string()), std::sync::Arc::new("DiacriticalDoubleAcute;".to_string()), std::sync::Arc::new("\u{2dd}".to_string()), std::sync::Arc::new("DiacriticalGrave;".to_string()), std::sync::Arc::new("`".to_string()), std::sync::Arc::new("DiacriticalTilde;".to_string()), std::sync::Arc::new("\u{2dc}".to_string()), std::sync::Arc::new("Diamond;".to_string()), std::sync::Arc::new("\u{22c4}".to_string()), std::sync::Arc::new("DifferentialD;".to_string()), std::sync::Arc::new("ⅆ".to_string()), std::sync::Arc::new("Dopf;".to_string()), std::sync::Arc::new("𝔻".to_string()), std::sync::Arc::new("Dot;".to_string()), std::sync::Arc::new("\u{a8}".to_string()), std::sync::Arc::new("DotDot;".to_string()), std::sync::Arc::new("\u{20dc}".to_string()), std::sync::Arc::new("DotEqual;".to_string()), std::sync::Arc::new("\u{2250}".to_string()), std::sync::Arc::new("DoubleContourIntegral;".to_string()), std::sync::Arc::new("\u{222f}".to_string()), std::sync::Arc::new("DoubleDot;".to_string()), std::sync::Arc::new("\u{a8}".to_string()), std::sync::Arc::new("DoubleDownArrow;".to_string()), std::sync::Arc::new("\u{21d3}".to_string()), std::sync::Arc::new("DoubleLeftArrow;".to_string()), std::sync::Arc::new("\u{21d0}".to_string()), std::sync::Arc::new("DoubleLeftRightArrow;".to_string()), std::sync::Arc::new("\u{21d4}".to_string()), std::sync::Arc::new("DoubleLeftTee;".to_string()), std::sync::Arc::new("\u{2ae4}".to_string()), std::sync::Arc::new("DoubleLongLeftArrow;".to_string()), std::sync::Arc::new("\u{27f8}".to_string()), std::sync::Arc::new("DoubleLongLeftRightArrow;".to_string()), std::sync::Arc::new("\u{27fa}".to_string()), std::sync::Arc::new("DoubleLongRightArrow;".to_string()), std::sync::Arc::new("\u{27f9}".to_string()), std::sync::Arc::new("DoubleRightArrow;".to_string()), std::sync::Arc::new("\u{21d2}".to_string()), std::sync::Arc::new("DoubleRightTee;".to_string()), std::sync::Arc::new("\u{22a8}".to_string()), std::sync::Arc::new("DoubleUpArrow;".to_string()), std::sync::Arc::new("\u{21d1}".to_string()), std::sync::Arc::new("DoubleUpDownArrow;".to_string()), std::sync::Arc::new("\u{21d5}".to_string()), std::sync::Arc::new("DoubleVerticalBar;".to_string()), std::sync::Arc::new("\u{2225}".to_string()), std::sync::Arc::new("DownArrow;".to_string()), std::sync::Arc::new("\u{2193}".to_string()), std::sync::Arc::new("DownArrowBar;".to_string()), std::sync::Arc::new("\u{2913}".to_string()), std::sync::Arc::new("DownArrowUpArrow;".to_string()), std::sync::Arc::new("\u{21f5}".to_string()), std::sync::Arc::new("DownBreve;".to_string()), std::sync::Arc::new("\u{311}".to_string()), std::sync::Arc::new("DownLeftRightVector;".to_string()), std::sync::Arc::new("\u{2950}".to_string()), std::sync::Arc::new("DownLeftTeeVector;".to_string()), std::sync::Arc::new("\u{295e}".to_string()), std::sync::Arc::new("DownLeftVector;".to_string()), std::sync::Arc::new("\u{21bd}".to_string()), std::sync::Arc::new("DownLeftVectorBar;".to_string()), std::sync::Arc::new("\u{2956}".to_string()), std::sync::Arc::new("DownRightTeeVector;".to_string()), std::sync::Arc::new("\u{295f}".to_string()), std::sync::Arc::new("DownRightVector;".to_string()), std::sync::Arc::new("\u{21c1}".to_string()), std::sync::Arc::new("DownRightVectorBar;".to_string()), std::sync::Arc::new("\u{2957}".to_string()), std::sync::Arc::new("DownTee;".to_string()), std::sync::Arc::new("\u{22a4}".to_string()), std::sync::Arc::new("DownTeeArrow;".to_string()), std::sync::Arc::new("\u{21a7}".to_string()), std::sync::Arc::new("Downarrow;".to_string()), std::sync::Arc::new("\u{21d3}".to_string()), std::sync::Arc::new("Dscr;".to_string()), std::sync::Arc::new("𝒟".to_string()), std::sync::Arc::new("Dstrok;".to_string()), std::sync::Arc::new("Đ".to_string()), std::sync::Arc::new("ENG;".to_string()), std::sync::Arc::new("Ŋ".to_string()), std::sync::Arc::new("ETH".to_string()), std::sync::Arc::new("Ð".to_string()), std::sync::Arc::new("ETH;".to_string()), std::sync::Arc::new("Ð".to_string()), std::sync::Arc::new("Eacute".to_string()), std::sync::Arc::new("É".to_string()), std::sync::Arc::new("Eacute;".to_string()), std::sync::Arc::new("É".to_string()), std::sync::Arc::new("Ecaron;".to_string()), std::sync::Arc::new("Ě".to_string()), std::sync::Arc::new("Ecirc".to_string()), std::sync::Arc::new("Ê".to_string()), std::sync::Arc::new("Ecirc;".to_string()), std::sync::Arc::new("Ê".to_string()), std::sync::Arc::new("Ecy;".to_string()), std::sync::Arc::new("Э".to_string()), std::sync::Arc::new("Edot;".to_string()), std::sync::Arc::new("Ė".to_string()), std::sync::Arc::new("Efr;".to_string()), std::sync::Arc::new("𝔈".to_string()), std::sync::Arc::new("Egrave".to_string()), std::sync::Arc::new("È".to_string()), std::sync::Arc::new("Egrave;".to_string()), std::sync::Arc::new("È".to_string()), std::sync::Arc::new("Element;".to_string()), std::sync::Arc::new("\u{2208}".to_string()), std::sync::Arc::new("Emacr;".to_string()), std::sync::Arc::new("Ē".to_string()), std::sync::Arc::new("EmptySmallSquare;".to_string()), std::sync::Arc::new("\u{25fb}".to_string()), std::sync::Arc::new("EmptyVerySmallSquare;".to_string()), std::sync::Arc::new("\u{25ab}".to_string()), std::sync::Arc::new("Eogon;".to_string()), std::sync::Arc::new("Ę".to_string()), std::sync::Arc::new("Eopf;".to_string()), std::sync::Arc::new("𝔼".to_string()), std::sync::Arc::new("Epsilon;".to_string()), std::sync::Arc::new("Ε".to_string()), std::sync::Arc::new("Equal;".to_string()), std::sync::Arc::new("\u{2a75}".to_string()), std::sync::Arc::new("EqualTilde;".to_string()), std::sync::Arc::new("\u{2242}".to_string()), std::sync::Arc::new("Equilibrium;".to_string()), std::sync::Arc::new("\u{21cc}".to_string()), std::sync::Arc::new("Escr;".to_string()), std::sync::Arc::new("ℰ".to_string()), std::sync::Arc::new("Esim;".to_string()), std::sync::Arc::new("\u{2a73}".to_string()), std::sync::Arc::new("Eta;".to_string()), std::sync::Arc::new("Η".to_string()), std::sync::Arc::new("Euml".to_string()), std::sync::Arc::new("Ë".to_string()), std::sync::Arc::new("Euml;".to_string()), std::sync::Arc::new("Ë".to_string()), std::sync::Arc::new("Exists;".to_string()), std::sync::Arc::new("\u{2203}".to_string()), std::sync::Arc::new("ExponentialE;".to_string()), std::sync::Arc::new("ⅇ".to_string()), std::sync::Arc::new("Fcy;".to_string()), std::sync::Arc::new("Ф".to_string()), std::sync::Arc::new("Ffr;".to_string()), std::sync::Arc::new("𝔉".to_string()), std::sync::Arc::new("FilledSmallSquare;".to_string()), std::sync::Arc::new("\u{25fc}".to_string()), std::sync::Arc::new("FilledVerySmallSquare;".to_string()), std::sync::Arc::new("\u{25aa}".to_string()), std::sync::Arc::new("Fopf;".to_string()), std::sync::Arc::new("𝔽".to_string()), std::sync::Arc::new("ForAll;".to_string()), std::sync::Arc::new("\u{2200}".to_string()), std::sync::Arc::new("Fouriertrf;".to_string()), std::sync::Arc::new("ℱ".to_string()), std::sync::Arc::new("Fscr;".to_string()), std::sync::Arc::new("ℱ".to_string()), std::sync::Arc::new("GJcy;".to_string()), std::sync::Arc::new("Ѓ".to_string()), std::sync::Arc::new("GT".to_string()), std::sync::Arc::new(">".to_string()), std::sync::Arc::new("GT;".to_string()), std::sync::Arc::new(">".to_string()), std::sync::Arc::new("Gamma;".to_string()), std::sync::Arc::new("Γ".to_string()), std::sync::Arc::new("Gammad;".to_string()), std::sync::Arc::new("Ϝ".to_string()), std::sync::Arc::new("Gbreve;".to_string()), std::sync::Arc::new("Ğ".to_string()), std::sync::Arc::new("Gcedil;".to_string()), std::sync::Arc::new("Ģ".to_string()), std::sync::Arc::new("Gcirc;".to_string()), std::sync::Arc::new("Ĝ".to_string()), std::sync::Arc::new("Gcy;".to_string()), std::sync::Arc::new("Г".to_string()), std::sync::Arc::new("Gdot;".to_string()), std::sync::Arc::new("Ġ".to_string()), std::sync::Arc::new("Gfr;".to_string()), std::sync::Arc::new("𝔊".to_string()), std::sync::Arc::new("Gg;".to_string()), std::sync::Arc::new("\u{22d9}".to_string()), std::sync::Arc::new("Gopf;".to_string()), std::sync::Arc::new("𝔾".to_string()), std::sync::Arc::new("GreaterEqual;".to_string()), std::sync::Arc::new("\u{2265}".to_string()), std::sync::Arc::new("GreaterEqualLess;".to_string()), std::sync::Arc::new("\u{22db}".to_string()), std::sync::Arc::new("GreaterFullEqual;".to_string()), std::sync::Arc::new("\u{2267}".to_string()), std::sync::Arc::new("GreaterGreater;".to_string()), std::sync::Arc::new("\u{2aa2}".to_string()), std::sync::Arc::new("GreaterLess;".to_string()), std::sync::Arc::new("\u{2277}".to_string()), std::sync::Arc::new("GreaterSlantEqual;".to_string()), std::sync::Arc::new("\u{2a7e}".to_string()), std::sync::Arc::new("GreaterTilde;".to_string()), std::sync::Arc::new("\u{2273}".to_string()), std::sync::Arc::new("Gscr;".to_string()), std::sync::Arc::new("𝒢".to_string()), std::sync::Arc::new("Gt;".to_string()), std::sync::Arc::new("\u{226b}".to_string()), std::sync::Arc::new("HARDcy;".to_string()), std::sync::Arc::new("Ъ".to_string()), std::sync::Arc::new("Hacek;".to_string()), std::sync::Arc::new("ˇ".to_string()), std::sync::Arc::new("Hat;".to_string()), std::sync::Arc::new("^".to_string()), std::sync::Arc::new("Hcirc;".to_string()), std::sync::Arc::new("Ĥ".to_string()), std::sync::Arc::new("Hfr;".to_string()), std::sync::Arc::new("ℌ".to_string()), std::sync::Arc::new("HilbertSpace;".to_string()), std::sync::Arc::new("ℋ".to_string()), std::sync::Arc::new("Hopf;".to_string()), std::sync::Arc::new("ℍ".to_string()), std::sync::Arc::new("HorizontalLine;".to_string()), std::sync::Arc::new("\u{2500}".to_string()), std::sync::Arc::new("Hscr;".to_string()), std::sync::Arc::new("ℋ".to_string()), std::sync::Arc::new("Hstrok;".to_string()), std::sync::Arc::new("Ħ".to_string()), std::sync::Arc::new("HumpDownHump;".to_string()), std::sync::Arc::new("\u{224e}".to_string()), std::sync::Arc::new("HumpEqual;".to_string()), std::sync::Arc::new("\u{224f}".to_string()), std::sync::Arc::new("IEcy;".to_string()), std::sync::Arc::new("Е".to_string()), std::sync::Arc::new("IJlig;".to_string()), std::sync::Arc::new("Ĳ".to_string()), std::sync::Arc::new("IOcy;".to_string()), std::sync::Arc::new("Ё".to_string()), std::sync::Arc::new("Iacute".to_string()), std::sync::Arc::new("Í".to_string()), std::sync::Arc::new("Iacute;".to_string()), std::sync::Arc::new("Í".to_string()), std::sync::Arc::new("Icirc".to_string()), std::sync::Arc::new("Î".to_string()), std::sync::Arc::new("Icirc;".to_string()), std::sync::Arc::new("Î".to_string()), std::sync::Arc::new("Icy;".to_string()), std::sync::Arc::new("И".to_string()), std::sync::Arc::new("Idot;".to_string()), std::sync::Arc::new("İ".to_string()), std::sync::Arc::new("Ifr;".to_string()), std::sync::Arc::new("ℑ".to_string()), std::sync::Arc::new("Igrave".to_string()), std::sync::Arc::new("Ì".to_string()), std::sync::Arc::new("Igrave;".to_string()), std::sync::Arc::new("Ì".to_string()), std::sync::Arc::new("Im;".to_string()), std::sync::Arc::new("ℑ".to_string()), std::sync::Arc::new("Imacr;".to_string()), std::sync::Arc::new("Ī".to_string()), std::sync::Arc::new("ImaginaryI;".to_string()), std::sync::Arc::new("ⅈ".to_string()), std::sync::Arc::new("Implies;".to_string()), std::sync::Arc::new("\u{21d2}".to_string()), std::sync::Arc::new("Int;".to_string()), std::sync::Arc::new("\u{222c}".to_string()), std::sync::Arc::new("Integral;".to_string()), std::sync::Arc::new("\u{222b}".to_string()), std::sync::Arc::new("Intersection;".to_string()), std::sync::Arc::new("\u{22c2}".to_string()), std::sync::Arc::new("InvisibleComma;".to_string()), std::sync::Arc::new("\u{2063}".to_string()), std::sync::Arc::new("InvisibleTimes;".to_string()), std::sync::Arc::new("\u{2062}".to_string()), std::sync::Arc::new("Iogon;".to_string()), std::sync::Arc::new("Į".to_string()), std::sync::Arc::new("Iopf;".to_string()), std::sync::Arc::new("𝕀".to_string()), std::sync::Arc::new("Iota;".to_string()), std::sync::Arc::new("Ι".to_string()), std::sync::Arc::new("Iscr;".to_string()), std::sync::Arc::new("ℐ".to_string()), std::sync::Arc::new("Itilde;".to_string()), std::sync::Arc::new("Ĩ".to_string()), std::sync::Arc::new("Iukcy;".to_string()), std::sync::Arc::new("І".to_string()), std::sync::Arc::new("Iuml".to_string()), std::sync::Arc::new("Ï".to_string()), std::sync::Arc::new("Iuml;".to_string()), std::sync::Arc::new("Ï".to_string()), std::sync::Arc::new("Jcirc;".to_string()), std::sync::Arc::new("Ĵ".to_string()), std::sync::Arc::new("Jcy;".to_string()), std::sync::Arc::new("Й".to_string()), std::sync::Arc::new("Jfr;".to_string()), std::sync::Arc::new("𝔍".to_string()), std::sync::Arc::new("Jopf;".to_string()), std::sync::Arc::new("𝕁".to_string()), std::sync::Arc::new("Jscr;".to_string()), std::sync::Arc::new("𝒥".to_string()), std::sync::Arc::new("Jsercy;".to_string()), std::sync::Arc::new("Ј".to_string()), std::sync::Arc::new("Jukcy;".to_string()), std::sync::Arc::new("Є".to_string()), std::sync::Arc::new("KHcy;".to_string()), std::sync::Arc::new("Х".to_string()), std::sync::Arc::new("KJcy;".to_string()), std::sync::Arc::new("Ќ".to_string()), std::sync::Arc::new("Kappa;".to_string()), std::sync::Arc::new("Κ".to_string()), std::sync::Arc::new("Kcedil;".to_string()), std::sync::Arc::new("Ķ".to_string()), std::sync::Arc::new("Kcy;".to_string()), std::sync::Arc::new("К".to_string()), std::sync::Arc::new("Kfr;".to_string()), std::sync::Arc::new("𝔎".to_string()), std::sync::Arc::new("Kopf;".to_string()), std::sync::Arc::new("𝕂".to_string()), std::sync::Arc::new("Kscr;".to_string()), std::sync::Arc::new("𝒦".to_string()), std::sync::Arc::new("LJcy;".to_string()), std::sync::Arc::new("Љ".to_string()), std::sync::Arc::new("LT".to_string()), std::sync::Arc::new("<".to_string()), std::sync::Arc::new("LT;".to_string()), std::sync::Arc::new("<".to_string()), std::sync::Arc::new("Lacute;".to_string()), std::sync::Arc::new("Ĺ".to_string()), std::sync::Arc::new("Lambda;".to_string()), std::sync::Arc::new("Λ".to_string()), std::sync::Arc::new("Lang;".to_string()), std::sync::Arc::new("\u{27ea}".to_string()), std::sync::Arc::new("Laplacetrf;".to_string()), std::sync::Arc::new("ℒ".to_string()), std::sync::Arc::new("Larr;".to_string()), std::sync::Arc::new("\u{219e}".to_string()), std::sync::Arc::new("Lcaron;".to_string()), std::sync::Arc::new("Ľ".to_string()), std::sync::Arc::new("Lcedil;".to_string()), std::sync::Arc::new("Ļ".to_string()), std::sync::Arc::new("Lcy;".to_string()), std::sync::Arc::new("Л".to_string()), std::sync::Arc::new("LeftAngleBracket;".to_string()), std::sync::Arc::new("\u{27e8}".to_string()), std::sync::Arc::new("LeftArrow;".to_string()), std::sync::Arc::new("\u{2190}".to_string()), std::sync::Arc::new("LeftArrowBar;".to_string()), std::sync::Arc::new("\u{21e4}".to_string()), std::sync::Arc::new("LeftArrowRightArrow;".to_string()), std::sync::Arc::new("\u{21c6}".to_string()), std::sync::Arc::new("LeftCeiling;".to_string()), std::sync::Arc::new("\u{2308}".to_string()), std::sync::Arc::new("LeftDoubleBracket;".to_string()), std::sync::Arc::new("\u{27e6}".to_string()), std::sync::Arc::new("LeftDownTeeVector;".to_string()), std::sync::Arc::new("\u{2961}".to_string()), std::sync::Arc::new("LeftDownVector;".to_string()), std::sync::Arc::new("\u{21c3}".to_string()), std::sync::Arc::new("LeftDownVectorBar;".to_string()), std::sync::Arc::new("\u{2959}".to_string()), std::sync::Arc::new("LeftFloor;".to_string()), std::sync::Arc::new("\u{230a}".to_string()), std::sync::Arc::new("LeftRightArrow;".to_string()), std::sync::Arc::new("\u{2194}".to_string()), std::sync::Arc::new("LeftRightVector;".to_string()), std::sync::Arc::new("\u{294e}".to_string()), std::sync::Arc::new("LeftTee;".to_string()), std::sync::Arc::new("\u{22a3}".to_string()), std::sync::Arc::new("LeftTeeArrow;".to_string()), std::sync::Arc::new("\u{21a4}".to_string()), std::sync::Arc::new("LeftTeeVector;".to_string()), std::sync::Arc::new("\u{295a}".to_string()), std::sync::Arc::new("LeftTriangle;".to_string()), std::sync::Arc::new("\u{22b2}".to_string()), std::sync::Arc::new("LeftTriangleBar;".to_string()), std::sync::Arc::new("\u{29cf}".to_string()), std::sync::Arc::new("LeftTriangleEqual;".to_string()), std::sync::Arc::new("\u{22b4}".to_string()), std::sync::Arc::new("LeftUpDownVector;".to_string()), std::sync::Arc::new("\u{2951}".to_string()), std::sync::Arc::new("LeftUpTeeVector;".to_string()), std::sync::Arc::new("\u{2960}".to_string()), std::sync::Arc::new("LeftUpVector;".to_string()), std::sync::Arc::new("\u{21bf}".to_string()), std::sync::Arc::new("LeftUpVectorBar;".to_string()), std::sync::Arc::new("\u{2958}".to_string()), std::sync::Arc::new("LeftVector;".to_string()), std::sync::Arc::new("\u{21bc}".to_string()), std::sync::Arc::new("LeftVectorBar;".to_string()), std::sync::Arc::new("\u{2952}".to_string()), std::sync::Arc::new("Leftarrow;".to_string()), std::sync::Arc::new("\u{21d0}".to_string()), std::sync::Arc::new("Leftrightarrow;".to_string()), std::sync::Arc::new("\u{21d4}".to_string()), std::sync::Arc::new("LessEqualGreater;".to_string()), std::sync::Arc::new("\u{22da}".to_string()), std::sync::Arc::new("LessFullEqual;".to_string()), std::sync::Arc::new("\u{2266}".to_string()), std::sync::Arc::new("LessGreater;".to_string()), std::sync::Arc::new("\u{2276}".to_string()), std::sync::Arc::new("LessLess;".to_string()), std::sync::Arc::new("\u{2aa1}".to_string()), std::sync::Arc::new("LessSlantEqual;".to_string()), std::sync::Arc::new("\u{2a7d}".to_string()), std::sync::Arc::new("LessTilde;".to_string()), std::sync::Arc::new("\u{2272}".to_string()), std::sync::Arc::new("Lfr;".to_string()), std::sync::Arc::new("𝔏".to_string()), std::sync::Arc::new("Ll;".to_string()), std::sync::Arc::new("\u{22d8}".to_string()), std::sync::Arc::new("Lleftarrow;".to_string()), std::sync::Arc::new("\u{21da}".to_string()), std::sync::Arc::new("Lmidot;".to_string()), std::sync::Arc::new("Ŀ".to_string()), std::sync::Arc::new("LongLeftArrow;".to_string()), std::sync::Arc::new("\u{27f5}".to_string()), std::sync::Arc::new("LongLeftRightArrow;".to_string()), std::sync::Arc::new("\u{27f7}".to_string()), std::sync::Arc::new("LongRightArrow;".to_string()), std::sync::Arc::new("\u{27f6}".to_string()), std::sync::Arc::new("Longleftarrow;".to_string()), std::sync::Arc::new("\u{27f8}".to_string()), std::sync::Arc::new("Longleftrightarrow;".to_string()), std::sync::Arc::new("\u{27fa}".to_string()), std::sync::Arc::new("Longrightarrow;".to_string()), std::sync::Arc::new("\u{27f9}".to_string()), std::sync::Arc::new("Lopf;".to_string()), std::sync::Arc::new("𝕃".to_string()), std::sync::Arc::new("LowerLeftArrow;".to_string()), std::sync::Arc::new("\u{2199}".to_string()), std::sync::Arc::new("LowerRightArrow;".to_string()), std::sync::Arc::new("\u{2198}".to_string()), std::sync::Arc::new("Lscr;".to_string()), std::sync::Arc::new("ℒ".to_string()), std::sync::Arc::new("Lsh;".to_string()), std::sync::Arc::new("\u{21b0}".to_string()), std::sync::Arc::new("Lstrok;".to_string()), std::sync::Arc::new("Ł".to_string()), std::sync::Arc::new("Lt;".to_string()), std::sync::Arc::new("\u{226a}".to_string()), std::sync::Arc::new("Map;".to_string()), std::sync::Arc::new("\u{2905}".to_string()), std::sync::Arc::new("Mcy;".to_string()), std::sync::Arc::new("М".to_string()), std::sync::Arc::new("MediumSpace;".to_string()), std::sync::Arc::new("\u{205f}".to_string()), std::sync::Arc::new("Mellintrf;".to_string()), std::sync::Arc::new("ℳ".to_string()), std::sync::Arc::new("Mfr;".to_string()), std::sync::Arc::new("𝔐".to_string()), std::sync::Arc::new("MinusPlus;".to_string()), std::sync::Arc::new("\u{2213}".to_string()), std::sync::Arc::new("Mopf;".to_string()), std::sync::Arc::new("𝕄".to_string()), std::sync::Arc::new("Mscr;".to_string()), std::sync::Arc::new("ℳ".to_string()), std::sync::Arc::new("Mu;".to_string()), std::sync::Arc::new("Μ".to_string()), std::sync::Arc::new("NJcy;".to_string()), std::sync::Arc::new("Њ".to_string()), std::sync::Arc::new("Nacute;".to_string()), std::sync::Arc::new("Ń".to_string()), std::sync::Arc::new("Ncaron;".to_string()), std::sync::Arc::new("Ň".to_string()), std::sync::Arc::new("Ncedil;".to_string()), std::sync::Arc::new("Ņ".to_string()), std::sync::Arc::new("Ncy;".to_string()), std::sync::Arc::new("Н".to_string()), std::sync::Arc::new("NegativeMediumSpace;".to_string()), std::sync::Arc::new("\u{200b}".to_string()), std::sync::Arc::new("NegativeThickSpace;".to_string()), std::sync::Arc::new("\u{200b}".to_string()), std::sync::Arc::new("NegativeThinSpace;".to_string()), std::sync::Arc::new("\u{200b}".to_string()), std::sync::Arc::new("NegativeVeryThinSpace;".to_string()), std::sync::Arc::new("\u{200b}".to_string()), std::sync::Arc::new("NestedGreaterGreater;".to_string()), std::sync::Arc::new("\u{226b}".to_string()), std::sync::Arc::new("NestedLessLess;".to_string()), std::sync::Arc::new("\u{226a}".to_string()), std::sync::Arc::new("NewLine;".to_string()), std::sync::Arc::new("\x0a".to_string()), std::sync::Arc::new("Nfr;".to_string()), std::sync::Arc::new("𝔑".to_string()), std::sync::Arc::new("NoBreak;".to_string()), std::sync::Arc::new("\u{2060}".to_string()), std::sync::Arc::new("NonBreakingSpace;".to_string()), std::sync::Arc::new("\u{a0}".to_string()), std::sync::Arc::new("Nopf;".to_string()), std::sync::Arc::new("ℕ".to_string()), std::sync::Arc::new("Not;".to_string()), std::sync::Arc::new("\u{2aec}".to_string()), std::sync::Arc::new("NotCongruent;".to_string()), std::sync::Arc::new("\u{2262}".to_string()), std::sync::Arc::new("NotCupCap;".to_string()), std::sync::Arc::new("\u{226d}".to_string()), std::sync::Arc::new("NotDoubleVerticalBar;".to_string()), std::sync::Arc::new("\u{2226}".to_string()), std::sync::Arc::new("NotElement;".to_string()), std::sync::Arc::new("\u{2209}".to_string()), std::sync::Arc::new("NotEqual;".to_string()), std::sync::Arc::new("\u{2260}".to_string()), std::sync::Arc::new("NotEqualTilde;".to_string()), std::sync::Arc::new("\u{2242}\u{338}".to_string()), std::sync::Arc::new("NotExists;".to_string()), std::sync::Arc::new("\u{2204}".to_string()), std::sync::Arc::new("NotGreater;".to_string()), std::sync::Arc::new("\u{226f}".to_string()), std::sync::Arc::new("NotGreaterEqual;".to_string()), std::sync::Arc::new("\u{2271}".to_string()), std::sync::Arc::new("NotGreaterFullEqual;".to_string()), std::sync::Arc::new("\u{2267}\u{338}".to_string()), std::sync::Arc::new("NotGreaterGreater;".to_string()), std::sync::Arc::new("\u{226b}\u{338}".to_string()), std::sync::Arc::new("NotGreaterLess;".to_string()), std::sync::Arc::new("\u{2279}".to_string()), std::sync::Arc::new("NotGreaterSlantEqual;".to_string()), std::sync::Arc::new("\u{2a7e}\u{338}".to_string()), std::sync::Arc::new("NotGreaterTilde;".to_string()), std::sync::Arc::new("\u{2275}".to_string()), std::sync::Arc::new("NotHumpDownHump;".to_string()), std::sync::Arc::new("\u{224e}\u{338}".to_string()), std::sync::Arc::new("NotHumpEqual;".to_string()), std::sync::Arc::new("\u{224f}\u{338}".to_string()), std::sync::Arc::new("NotLeftTriangle;".to_string()), std::sync::Arc::new("\u{22ea}".to_string()), std::sync::Arc::new("NotLeftTriangleBar;".to_string()), std::sync::Arc::new("\u{29cf}\u{338}".to_string()), std::sync::Arc::new("NotLeftTriangleEqual;".to_string()), std::sync::Arc::new("\u{22ec}".to_string()), std::sync::Arc::new("NotLess;".to_string()), std::sync::Arc::new("\u{226e}".to_string()), std::sync::Arc::new("NotLessEqual;".to_string()), std::sync::Arc::new("\u{2270}".to_string()), std::sync::Arc::new("NotLessGreater;".to_string()), std::sync::Arc::new("\u{2278}".to_string()), std::sync::Arc::new("NotLessLess;".to_string()), std::sync::Arc::new("\u{226a}\u{338}".to_string()), std::sync::Arc::new("NotLessSlantEqual;".to_string()), std::sync::Arc::new("\u{2a7d}\u{338}".to_string()), std::sync::Arc::new("NotLessTilde;".to_string()), std::sync::Arc::new("\u{2274}".to_string()), std::sync::Arc::new("NotNestedGreaterGreater;".to_string()), std::sync::Arc::new("\u{2aa2}\u{338}".to_string()), std::sync::Arc::new("NotNestedLessLess;".to_string()), std::sync::Arc::new("\u{2aa1}\u{338}".to_string()), std::sync::Arc::new("NotPrecedes;".to_string()), std::sync::Arc::new("\u{2280}".to_string()), std::sync::Arc::new("NotPrecedesEqual;".to_string()), std::sync::Arc::new("\u{2aaf}\u{338}".to_string()), std::sync::Arc::new("NotPrecedesSlantEqual;".to_string()), std::sync::Arc::new("\u{22e0}".to_string()), std::sync::Arc::new("NotReverseElement;".to_string()), std::sync::Arc::new("\u{220c}".to_string()), std::sync::Arc::new("NotRightTriangle;".to_string()), std::sync::Arc::new("\u{22eb}".to_string()), std::sync::Arc::new("NotRightTriangleBar;".to_string()), std::sync::Arc::new("\u{29d0}\u{338}".to_string()), std::sync::Arc::new("NotRightTriangleEqual;".to_string()), std::sync::Arc::new("\u{22ed}".to_string()), std::sync::Arc::new("NotSquareSubset;".to_string()), std::sync::Arc::new("\u{228f}\u{338}".to_string()), std::sync::Arc::new("NotSquareSubsetEqual;".to_string()), std::sync::Arc::new("\u{22e2}".to_string()), std::sync::Arc::new("NotSquareSuperset;".to_string()), std::sync::Arc::new("\u{2290}\u{338}".to_string()), std::sync::Arc::new("NotSquareSupersetEqual;".to_string()), std::sync::Arc::new("\u{22e3}".to_string()), std::sync::Arc::new("NotSubset;".to_string()), std::sync::Arc::new("\u{2282}\u{20d2}".to_string()), std::sync::Arc::new("NotSubsetEqual;".to_string()), std::sync::Arc::new("\u{2288}".to_string()), std::sync::Arc::new("NotSucceeds;".to_string()), std::sync::Arc::new("\u{2281}".to_string()), std::sync::Arc::new("NotSucceedsEqual;".to_string()), std::sync::Arc::new("\u{2ab0}\u{338}".to_string()), std::sync::Arc::new("NotSucceedsSlantEqual;".to_string()), std::sync::Arc::new("\u{22e1}".to_string()), std::sync::Arc::new("NotSucceedsTilde;".to_string()), std::sync::Arc::new("\u{227f}\u{338}".to_string()), std::sync::Arc::new("NotSuperset;".to_string()), std::sync::Arc::new("\u{2283}\u{20d2}".to_string()), std::sync::Arc::new("NotSupersetEqual;".to_string()), std::sync::Arc::new("\u{2289}".to_string()), std::sync::Arc::new("NotTilde;".to_string()), std::sync::Arc::new("\u{2241}".to_string()), std::sync::Arc::new("NotTildeEqual;".to_string()), std::sync::Arc::new("\u{2244}".to_string()), std::sync::Arc::new("NotTildeFullEqual;".to_string()), std::sync::Arc::new("\u{2247}".to_string()), std::sync::Arc::new("NotTildeTilde;".to_string()), std::sync::Arc::new("\u{2249}".to_string()), std::sync::Arc::new("NotVerticalBar;".to_string()), std::sync::Arc::new("\u{2224}".to_string()), std::sync::Arc::new("Nscr;".to_string()), std::sync::Arc::new("𝒩".to_string()), std::sync::Arc::new("Ntilde".to_string()), std::sync::Arc::new("Ñ".to_string()), std::sync::Arc::new("Ntilde;".to_string()), std::sync::Arc::new("Ñ".to_string()), std::sync::Arc::new("Nu;".to_string()), std::sync::Arc::new("Ν".to_string()), std::sync::Arc::new("OElig;".to_string()), std::sync::Arc::new("Œ".to_string()), std::sync::Arc::new("Oacute".to_string()), std::sync::Arc::new("Ó".to_string()), std::sync::Arc::new("Oacute;".to_string()), std::sync::Arc::new("Ó".to_string()), std::sync::Arc::new("Ocirc".to_string()), std::sync::Arc::new("Ô".to_string()), std::sync::Arc::new("Ocirc;".to_string()), std::sync::Arc::new("Ô".to_string()), std::sync::Arc::new("Ocy;".to_string()), std::sync::Arc::new("О".to_string()), std::sync::Arc::new("Odblac;".to_string()), std::sync::Arc::new("Ő".to_string()), std::sync::Arc::new("Ofr;".to_string()), std::sync::Arc::new("𝔒".to_string()), std::sync::Arc::new("Ograve".to_string()), std::sync::Arc::new("Ò".to_string()), std::sync::Arc::new("Ograve;".to_string()), std::sync::Arc::new("Ò".to_string()), std::sync::Arc::new("Omacr;".to_string()), std::sync::Arc::new("Ō".to_string()), std::sync::Arc::new("Omega;".to_string()), std::sync::Arc::new("Ω".to_string()), std::sync::Arc::new("Omicron;".to_string()), std::sync::Arc::new("Ο".to_string()), std::sync::Arc::new("Oopf;".to_string()), std::sync::Arc::new("𝕆".to_string()), std::sync::Arc::new("OpenCurlyDoubleQuote;".to_string()), std::sync::Arc::new("\u{201c}".to_string()), std::sync::Arc::new("OpenCurlyQuote;".to_string()), std::sync::Arc::new("\u{2018}".to_string()), std::sync::Arc::new("Or;".to_string()), std::sync::Arc::new("\u{2a54}".to_string()), std::sync::Arc::new("Oscr;".to_string()), std::sync::Arc::new("𝒪".to_string()), std::sync::Arc::new("Oslash".to_string()), std::sync::Arc::new("Ø".to_string()), std::sync::Arc::new("Oslash;".to_string()), std::sync::Arc::new("Ø".to_string()), std::sync::Arc::new("Otilde".to_string()), std::sync::Arc::new("Õ".to_string()), std::sync::Arc::new("Otilde;".to_string()), std::sync::Arc::new("Õ".to_string()), std::sync::Arc::new("Otimes;".to_string()), std::sync::Arc::new("\u{2a37}".to_string()), std::sync::Arc::new("Ouml".to_string()), std::sync::Arc::new("Ö".to_string()), std::sync::Arc::new("Ouml;".to_string()), std::sync::Arc::new("Ö".to_string()), std::sync::Arc::new("OverBar;".to_string()), std::sync::Arc::new("\u{203e}".to_string()), std::sync::Arc::new("OverBrace;".to_string()), std::sync::Arc::new("\u{23de}".to_string()), std::sync::Arc::new("OverBracket;".to_string()), std::sync::Arc::new("\u{23b4}".to_string()), std::sync::Arc::new("OverParenthesis;".to_string()), std::sync::Arc::new("\u{23dc}".to_string()), std::sync::Arc::new("PartialD;".to_string()), std::sync::Arc::new("\u{2202}".to_string()), std::sync::Arc::new("Pcy;".to_string()), std::sync::Arc::new("П".to_string()), std::sync::Arc::new("Pfr;".to_string()), std::sync::Arc::new("𝔓".to_string()), std::sync::Arc::new("Phi;".to_string()), std::sync::Arc::new("Φ".to_string()), std::sync::Arc::new("Pi;".to_string()), std::sync::Arc::new("Π".to_string()), std::sync::Arc::new("PlusMinus;".to_string()), std::sync::Arc::new("\u{b1}".to_string()), std::sync::Arc::new("Poincareplane;".to_string()), std::sync::Arc::new("ℌ".to_string()), std::sync::Arc::new("Popf;".to_string()), std::sync::Arc::new("ℙ".to_string()), std::sync::Arc::new("Pr;".to_string()), std::sync::Arc::new("\u{2abb}".to_string()), std::sync::Arc::new("Precedes;".to_string()), std::sync::Arc::new("\u{227a}".to_string()), std::sync::Arc::new("PrecedesEqual;".to_string()), std::sync::Arc::new("\u{2aaf}".to_string()), std::sync::Arc::new("PrecedesSlantEqual;".to_string()), std::sync::Arc::new("\u{227c}".to_string()), std::sync::Arc::new("PrecedesTilde;".to_string()), std::sync::Arc::new("\u{227e}".to_string()), std::sync::Arc::new("Prime;".to_string()), std::sync::Arc::new("\u{2033}".to_string()), std::sync::Arc::new("Product;".to_string()), std::sync::Arc::new("\u{220f}".to_string()), std::sync::Arc::new("Proportion;".to_string()), std::sync::Arc::new("\u{2237}".to_string()), std::sync::Arc::new("Proportional;".to_string()), std::sync::Arc::new("\u{221d}".to_string()), std::sync::Arc::new("Pscr;".to_string()), std::sync::Arc::new("𝒫".to_string()), std::sync::Arc::new("Psi;".to_string()), std::sync::Arc::new("Ψ".to_string()), std::sync::Arc::new("QUOT".to_string()), std::sync::Arc::new("\"".to_string()), std::sync::Arc::new("QUOT;".to_string()), std::sync::Arc::new("\"".to_string()), std::sync::Arc::new("Qfr;".to_string()), std::sync::Arc::new("𝔔".to_string()), std::sync::Arc::new("Qopf;".to_string()), std::sync::Arc::new("ℚ".to_string()), std::sync::Arc::new("Qscr;".to_string()), std::sync::Arc::new("𝒬".to_string()), std::sync::Arc::new("RBarr;".to_string()), std::sync::Arc::new("\u{2910}".to_string()), std::sync::Arc::new("REG".to_string()), std::sync::Arc::new("\u{ae}".to_string()), std::sync::Arc::new("REG;".to_string()), std::sync::Arc::new("\u{ae}".to_string()), std::sync::Arc::new("Racute;".to_string()), std::sync::Arc::new("Ŕ".to_string()), std::sync::Arc::new("Rang;".to_string()), std::sync::Arc::new("\u{27eb}".to_string()), std::sync::Arc::new("Rarr;".to_string()), std::sync::Arc::new("\u{21a0}".to_string()), std::sync::Arc::new("Rarrtl;".to_string()), std::sync::Arc::new("\u{2916}".to_string()), std::sync::Arc::new("Rcaron;".to_string()), std::sync::Arc::new("Ř".to_string()), std::sync::Arc::new("Rcedil;".to_string()), std::sync::Arc::new("Ŗ".to_string()), std::sync::Arc::new("Rcy;".to_string()), std::sync::Arc::new("Р".to_string()), std::sync::Arc::new("Re;".to_string()), std::sync::Arc::new("ℜ".to_string()), std::sync::Arc::new("ReverseElement;".to_string()), std::sync::Arc::new("\u{220b}".to_string()), std::sync::Arc::new("ReverseEquilibrium;".to_string()), std::sync::Arc::new("\u{21cb}".to_string()), std::sync::Arc::new("ReverseUpEquilibrium;".to_string()), std::sync::Arc::new("\u{296f}".to_string()), std::sync::Arc::new("Rfr;".to_string()), std::sync::Arc::new("ℜ".to_string()), std::sync::Arc::new("Rho;".to_string()), std::sync::Arc::new("Ρ".to_string()), std::sync::Arc::new("RightAngleBracket;".to_string()), std::sync::Arc::new("\u{27e9}".to_string()), std::sync::Arc::new("RightArrow;".to_string()), std::sync::Arc::new("\u{2192}".to_string()), std::sync::Arc::new("RightArrowBar;".to_string()), std::sync::Arc::new("\u{21e5}".to_string()), std::sync::Arc::new("RightArrowLeftArrow;".to_string()), std::sync::Arc::new("\u{21c4}".to_string()), std::sync::Arc::new("RightCeiling;".to_string()), std::sync::Arc::new("\u{2309}".to_string()), std::sync::Arc::new("RightDoubleBracket;".to_string()), std::sync::Arc::new("\u{27e7}".to_string()), std::sync::Arc::new("RightDownTeeVector;".to_string()), std::sync::Arc::new("\u{295d}".to_string()), std::sync::Arc::new("RightDownVector;".to_string()), std::sync::Arc::new("\u{21c2}".to_string()), std::sync::Arc::new("RightDownVectorBar;".to_string()), std::sync::Arc::new("\u{2955}".to_string()), std::sync::Arc::new("RightFloor;".to_string()), std::sync::Arc::new("\u{230b}".to_string()), std::sync::Arc::new("RightTee;".to_string()), std::sync::Arc::new("\u{22a2}".to_string()), std::sync::Arc::new("RightTeeArrow;".to_string()), std::sync::Arc::new("\u{21a6}".to_string()), std::sync::Arc::new("RightTeeVector;".to_string()), std::sync::Arc::new("\u{295b}".to_string()), std::sync::Arc::new("RightTriangle;".to_string()), std::sync::Arc::new("\u{22b3}".to_string()), std::sync::Arc::new("RightTriangleBar;".to_string()), std::sync::Arc::new("\u{29d0}".to_string()), std::sync::Arc::new("RightTriangleEqual;".to_string()), std::sync::Arc::new("\u{22b5}".to_string()), std::sync::Arc::new("RightUpDownVector;".to_string()), std::sync::Arc::new("\u{294f}".to_string()), std::sync::Arc::new("RightUpTeeVector;".to_string()), std::sync::Arc::new("\u{295c}".to_string()), std::sync::Arc::new("RightUpVector;".to_string()), std::sync::Arc::new("\u{21be}".to_string()), std::sync::Arc::new("RightUpVectorBar;".to_string()), std::sync::Arc::new("\u{2954}".to_string()), std::sync::Arc::new("RightVector;".to_string()), std::sync::Arc::new("\u{21c0}".to_string()), std::sync::Arc::new("RightVectorBar;".to_string()), std::sync::Arc::new("\u{2953}".to_string()), std::sync::Arc::new("Rightarrow;".to_string()), std::sync::Arc::new("\u{21d2}".to_string()), std::sync::Arc::new("Ropf;".to_string()), std::sync::Arc::new("ℝ".to_string()), std::sync::Arc::new("RoundImplies;".to_string()), std::sync::Arc::new("\u{2970}".to_string()), std::sync::Arc::new("Rrightarrow;".to_string()), std::sync::Arc::new("\u{21db}".to_string()), std::sync::Arc::new("Rscr;".to_string()), std::sync::Arc::new("ℛ".to_string()), std::sync::Arc::new("Rsh;".to_string()), std::sync::Arc::new("\u{21b1}".to_string()), std::sync::Arc::new("RuleDelayed;".to_string()), std::sync::Arc::new("\u{29f4}".to_string()), std::sync::Arc::new("SHCHcy;".to_string()), std::sync::Arc::new("Щ".to_string()), std::sync::Arc::new("SHcy;".to_string()), std::sync::Arc::new("Ш".to_string()), std::sync::Arc::new("SOFTcy;".to_string()), std::sync::Arc::new("Ь".to_string()), std::sync::Arc::new("Sacute;".to_string()), std::sync::Arc::new("Ś".to_string()), std::sync::Arc::new("Sc;".to_string()), std::sync::Arc::new("\u{2abc}".to_string()), std::sync::Arc::new("Scaron;".to_string()), std::sync::Arc::new("Š".to_string()), std::sync::Arc::new("Scedil;".to_string()), std::sync::Arc::new("Ş".to_string()), std::sync::Arc::new("Scirc;".to_string()), std::sync::Arc::new("Ŝ".to_string()), std::sync::Arc::new("Scy;".to_string()), std::sync::Arc::new("С".to_string()), std::sync::Arc::new("Sfr;".to_string()), std::sync::Arc::new("𝔖".to_string()), std::sync::Arc::new("ShortDownArrow;".to_string()), std::sync::Arc::new("\u{2193}".to_string()), std::sync::Arc::new("ShortLeftArrow;".to_string()), std::sync::Arc::new("\u{2190}".to_string()), std::sync::Arc::new("ShortRightArrow;".to_string()), std::sync::Arc::new("\u{2192}".to_string()), std::sync::Arc::new("ShortUpArrow;".to_string()), std::sync::Arc::new("\u{2191}".to_string()), std::sync::Arc::new("Sigma;".to_string()), std::sync::Arc::new("Σ".to_string()), std::sync::Arc::new("SmallCircle;".to_string()), std::sync::Arc::new("\u{2218}".to_string()), std::sync::Arc::new("Sopf;".to_string()), std::sync::Arc::new("𝕊".to_string()), std::sync::Arc::new("Sqrt;".to_string()), std::sync::Arc::new("\u{221a}".to_string()), std::sync::Arc::new("Square;".to_string()), std::sync::Arc::new("\u{25a1}".to_string()), std::sync::Arc::new("SquareIntersection;".to_string()), std::sync::Arc::new("\u{2293}".to_string()), std::sync::Arc::new("SquareSubset;".to_string()), std::sync::Arc::new("\u{228f}".to_string()), std::sync::Arc::new("SquareSubsetEqual;".to_string()), std::sync::Arc::new("\u{2291}".to_string()), std::sync::Arc::new("SquareSuperset;".to_string()), std::sync::Arc::new("\u{2290}".to_string()), std::sync::Arc::new("SquareSupersetEqual;".to_string()), std::sync::Arc::new("\u{2292}".to_string()), std::sync::Arc::new("SquareUnion;".to_string()), std::sync::Arc::new("\u{2294}".to_string()), std::sync::Arc::new("Sscr;".to_string()), std::sync::Arc::new("𝒮".to_string()), std::sync::Arc::new("Star;".to_string()), std::sync::Arc::new("\u{22c6}".to_string()), std::sync::Arc::new("Sub;".to_string()), std::sync::Arc::new("\u{22d0}".to_string()), std::sync::Arc::new("Subset;".to_string()), std::sync::Arc::new("\u{22d0}".to_string()), std::sync::Arc::new("SubsetEqual;".to_string()), std::sync::Arc::new("\u{2286}".to_string()), std::sync::Arc::new("Succeeds;".to_string()), std::sync::Arc::new("\u{227b}".to_string()), std::sync::Arc::new("SucceedsEqual;".to_string()), std::sync::Arc::new("\u{2ab0}".to_string()), std::sync::Arc::new("SucceedsSlantEqual;".to_string()), std::sync::Arc::new("\u{227d}".to_string()), std::sync::Arc::new("SucceedsTilde;".to_string()), std::sync::Arc::new("\u{227f}".to_string()), std::sync::Arc::new("SuchThat;".to_string()), std::sync::Arc::new("\u{220b}".to_string()), std::sync::Arc::new("Sum;".to_string()), std::sync::Arc::new("\u{2211}".to_string()), std::sync::Arc::new("Sup;".to_string()), std::sync::Arc::new("\u{22d1}".to_string()), std::sync::Arc::new("Superset;".to_string()), std::sync::Arc::new("\u{2283}".to_string()), std::sync::Arc::new("SupersetEqual;".to_string()), std::sync::Arc::new("\u{2287}".to_string()), std::sync::Arc::new("Supset;".to_string()), std::sync::Arc::new("\u{22d1}".to_string()), std::sync::Arc::new("THORN".to_string()), std::sync::Arc::new("Þ".to_string()), std::sync::Arc::new("THORN;".to_string()), std::sync::Arc::new("Þ".to_string()), std::sync::Arc::new("TRADE;".to_string()), std::sync::Arc::new("\u{2122}".to_string()), std::sync::Arc::new("TSHcy;".to_string()), std::sync::Arc::new("Ћ".to_string()), std::sync::Arc::new("TScy;".to_string()), std::sync::Arc::new("Ц".to_string()), std::sync::Arc::new("Tab;".to_string()), std::sync::Arc::new("\x09".to_string()), std::sync::Arc::new("Tau;".to_string()), std::sync::Arc::new("Τ".to_string()), std::sync::Arc::new("Tcaron;".to_string()), std::sync::Arc::new("Ť".to_string()), std::sync::Arc::new("Tcedil;".to_string()), std::sync::Arc::new("Ţ".to_string()), std::sync::Arc::new("Tcy;".to_string()), std::sync::Arc::new("Т".to_string()), std::sync::Arc::new("Tfr;".to_string()), std::sync::Arc::new("𝔗".to_string()), std::sync::Arc::new("Therefore;".to_string()), std::sync::Arc::new("\u{2234}".to_string()), std::sync::Arc::new("Theta;".to_string()), std::sync::Arc::new("Θ".to_string()), std::sync::Arc::new("ThickSpace;".to_string()), std::sync::Arc::new("\u{205f}\u{200a}".to_string()), std::sync::Arc::new("ThinSpace;".to_string()), std::sync::Arc::new("\u{2009}".to_string()), std::sync::Arc::new("Tilde;".to_string()), std::sync::Arc::new("\u{223c}".to_string()), std::sync::Arc::new("TildeEqual;".to_string()), std::sync::Arc::new("\u{2243}".to_string()), std::sync::Arc::new("TildeFullEqual;".to_string()), std::sync::Arc::new("\u{2245}".to_string()), std::sync::Arc::new("TildeTilde;".to_string()), std::sync::Arc::new("\u{2248}".to_string()), std::sync::Arc::new("Topf;".to_string()), std::sync::Arc::new("𝕋".to_string()), std::sync::Arc::new("TripleDot;".to_string()), std::sync::Arc::new("\u{20db}".to_string()), std::sync::Arc::new("Tscr;".to_string()), std::sync::Arc::new("𝒯".to_string()), std::sync::Arc::new("Tstrok;".to_string()), std::sync::Arc::new("Ŧ".to_string()), std::sync::Arc::new("Uacute".to_string()), std::sync::Arc::new("Ú".to_string()), std::sync::Arc::new("Uacute;".to_string()), std::sync::Arc::new("Ú".to_string()), std::sync::Arc::new("Uarr;".to_string()), std::sync::Arc::new("\u{219f}".to_string()), std::sync::Arc::new("Uarrocir;".to_string()), std::sync::Arc::new("\u{2949}".to_string()), std::sync::Arc::new("Ubrcy;".to_string()), std::sync::Arc::new("Ў".to_string()), std::sync::Arc::new("Ubreve;".to_string()), std::sync::Arc::new("Ŭ".to_string()), std::sync::Arc::new("Ucirc".to_string()), std::sync::Arc::new("Û".to_string()), std::sync::Arc::new("Ucirc;".to_string()), std::sync::Arc::new("Û".to_string()), std::sync::Arc::new("Ucy;".to_string()), std::sync::Arc::new("У".to_string()), std::sync::Arc::new("Udblac;".to_string()), std::sync::Arc::new("Ű".to_string()), std::sync::Arc::new("Ufr;".to_string()), std::sync::Arc::new("𝔘".to_string()), std::sync::Arc::new("Ugrave".to_string()), std::sync::Arc::new("Ù".to_string()), std::sync::Arc::new("Ugrave;".to_string()), std::sync::Arc::new("Ù".to_string()), std::sync::Arc::new("Umacr;".to_string()), std::sync::Arc::new("Ū".to_string()), std::sync::Arc::new("UnderBar;".to_string()), std::sync::Arc::new("_".to_string()), std::sync::Arc::new("UnderBrace;".to_string()), std::sync::Arc::new("\u{23df}".to_string()), std::sync::Arc::new("UnderBracket;".to_string()), std::sync::Arc::new("\u{23b5}".to_string()), std::sync::Arc::new("UnderParenthesis;".to_string()), std::sync::Arc::new("\u{23dd}".to_string()), std::sync::Arc::new("Union;".to_string()), std::sync::Arc::new("\u{22c3}".to_string()), std::sync::Arc::new("UnionPlus;".to_string()), std::sync::Arc::new("\u{228e}".to_string()), std::sync::Arc::new("Uogon;".to_string()), std::sync::Arc::new("Ų".to_string()), std::sync::Arc::new("Uopf;".to_string()), std::sync::Arc::new("𝕌".to_string()), std::sync::Arc::new("UpArrow;".to_string()), std::sync::Arc::new("\u{2191}".to_string()), std::sync::Arc::new("UpArrowBar;".to_string()), std::sync::Arc::new("\u{2912}".to_string()), std::sync::Arc::new("UpArrowDownArrow;".to_string()), std::sync::Arc::new("\u{21c5}".to_string()), std::sync::Arc::new("UpDownArrow;".to_string()), std::sync::Arc::new("\u{2195}".to_string()), std::sync::Arc::new("UpEquilibrium;".to_string()), std::sync::Arc::new("\u{296e}".to_string()), std::sync::Arc::new("UpTee;".to_string()), std::sync::Arc::new("\u{22a5}".to_string()), std::sync::Arc::new("UpTeeArrow;".to_string()), std::sync::Arc::new("\u{21a5}".to_string()), std::sync::Arc::new("Uparrow;".to_string()), std::sync::Arc::new("\u{21d1}".to_string()), std::sync::Arc::new("Updownarrow;".to_string()), std::sync::Arc::new("\u{21d5}".to_string()), std::sync::Arc::new("UpperLeftArrow;".to_string()), std::sync::Arc::new("\u{2196}".to_string()), std::sync::Arc::new("UpperRightArrow;".to_string()), std::sync::Arc::new("\u{2197}".to_string()), std::sync::Arc::new("Upsi;".to_string()), std::sync::Arc::new("ϒ".to_string()), std::sync::Arc::new("Upsilon;".to_string()), std::sync::Arc::new("Υ".to_string()), std::sync::Arc::new("Uring;".to_string()), std::sync::Arc::new("Ů".to_string()), std::sync::Arc::new("Uscr;".to_string()), std::sync::Arc::new("𝒰".to_string()), std::sync::Arc::new("Utilde;".to_string()), std::sync::Arc::new("Ũ".to_string()), std::sync::Arc::new("Uuml".to_string()), std::sync::Arc::new("Ü".to_string()), std::sync::Arc::new("Uuml;".to_string()), std::sync::Arc::new("Ü".to_string()), std::sync::Arc::new("VDash;".to_string()), std::sync::Arc::new("\u{22ab}".to_string()), std::sync::Arc::new("Vbar;".to_string()), std::sync::Arc::new("\u{2aeb}".to_string()), std::sync::Arc::new("Vcy;".to_string()), std::sync::Arc::new("В".to_string()), std::sync::Arc::new("Vdash;".to_string()), std::sync::Arc::new("\u{22a9}".to_string()), std::sync::Arc::new("Vdashl;".to_string()), std::sync::Arc::new("\u{2ae6}".to_string()), std::sync::Arc::new("Vee;".to_string()), std::sync::Arc::new("\u{22c1}".to_string()), std::sync::Arc::new("Verbar;".to_string()), std::sync::Arc::new("\u{2016}".to_string()), std::sync::Arc::new("Vert;".to_string()), std::sync::Arc::new("\u{2016}".to_string()), std::sync::Arc::new("VerticalBar;".to_string()), std::sync::Arc::new("\u{2223}".to_string()), std::sync::Arc::new("VerticalLine;".to_string()), std::sync::Arc::new("|".to_string()), std::sync::Arc::new("VerticalSeparator;".to_string()), std::sync::Arc::new("\u{2758}".to_string()), std::sync::Arc::new("VerticalTilde;".to_string()), std::sync::Arc::new("\u{2240}".to_string()), std::sync::Arc::new("VeryThinSpace;".to_string()), std::sync::Arc::new("\u{200a}".to_string()), std::sync::Arc::new("Vfr;".to_string()), std::sync::Arc::new("𝔙".to_string()), std::sync::Arc::new("Vopf;".to_string()), std::sync::Arc::new("𝕍".to_string()), std::sync::Arc::new("Vscr;".to_string()), std::sync::Arc::new("𝒱".to_string()), std::sync::Arc::new("Vvdash;".to_string()), std::sync::Arc::new("\u{22aa}".to_string()), std::sync::Arc::new("Wcirc;".to_string()), std::sync::Arc::new("Ŵ".to_string()), std::sync::Arc::new("Wedge;".to_string()), std::sync::Arc::new("\u{22c0}".to_string()), std::sync::Arc::new("Wfr;".to_string()), std::sync::Arc::new("𝔚".to_string()), std::sync::Arc::new("Wopf;".to_string()), std::sync::Arc::new("𝕎".to_string()), std::sync::Arc::new("Wscr;".to_string()), std::sync::Arc::new("𝒲".to_string()), std::sync::Arc::new("Xfr;".to_string()), std::sync::Arc::new("𝔛".to_string()), std::sync::Arc::new("Xi;".to_string()), std::sync::Arc::new("Ξ".to_string()), std::sync::Arc::new("Xopf;".to_string()), std::sync::Arc::new("𝕏".to_string()), std::sync::Arc::new("Xscr;".to_string()), std::sync::Arc::new("𝒳".to_string()), std::sync::Arc::new("YAcy;".to_string()), std::sync::Arc::new("Я".to_string()), std::sync::Arc::new("YIcy;".to_string()), std::sync::Arc::new("Ї".to_string()), std::sync::Arc::new("YUcy;".to_string()), std::sync::Arc::new("Ю".to_string()), std::sync::Arc::new("Yacute".to_string()), std::sync::Arc::new("Ý".to_string()), std::sync::Arc::new("Yacute;".to_string()), std::sync::Arc::new("Ý".to_string()), std::sync::Arc::new("Ycirc;".to_string()), std::sync::Arc::new("Ŷ".to_string()), std::sync::Arc::new("Ycy;".to_string()), std::sync::Arc::new("Ы".to_string()), std::sync::Arc::new("Yfr;".to_string()), std::sync::Arc::new("𝔜".to_string()), std::sync::Arc::new("Yopf;".to_string()), std::sync::Arc::new("𝕐".to_string()), std::sync::Arc::new("Yscr;".to_string()), std::sync::Arc::new("𝒴".to_string()), std::sync::Arc::new("Yuml;".to_string()), std::sync::Arc::new("Ÿ".to_string()), std::sync::Arc::new("ZHcy;".to_string()), std::sync::Arc::new("Ж".to_string()), std::sync::Arc::new("Zacute;".to_string()), std::sync::Arc::new("Ź".to_string()), std::sync::Arc::new("Zcaron;".to_string()), std::sync::Arc::new("Ž".to_string()), std::sync::Arc::new("Zcy;".to_string()), std::sync::Arc::new("З".to_string()), std::sync::Arc::new("Zdot;".to_string()), std::sync::Arc::new("Ż".to_string()), std::sync::Arc::new("ZeroWidthSpace;".to_string()), std::sync::Arc::new("\u{200b}".to_string()), std::sync::Arc::new("Zeta;".to_string()), std::sync::Arc::new("Ζ".to_string()), std::sync::Arc::new("Zfr;".to_string()), std::sync::Arc::new("ℨ".to_string()), std::sync::Arc::new("Zopf;".to_string()), std::sync::Arc::new("ℤ".to_string()), std::sync::Arc::new("Zscr;".to_string()), std::sync::Arc::new("𝒵".to_string()), std::sync::Arc::new("aacute".to_string()), std::sync::Arc::new("á".to_string()), std::sync::Arc::new("aacute;".to_string()), std::sync::Arc::new("á".to_string()), std::sync::Arc::new("abreve;".to_string()), std::sync::Arc::new("ă".to_string()), std::sync::Arc::new("ac;".to_string()), std::sync::Arc::new("\u{223e}".to_string()), std::sync::Arc::new("acE;".to_string()), std::sync::Arc::new("\u{223e}\u{333}".to_string()), std::sync::Arc::new("acd;".to_string()), std::sync::Arc::new("\u{223f}".to_string()), std::sync::Arc::new("acirc".to_string()), std::sync::Arc::new("â".to_string()), std::sync::Arc::new("acirc;".to_string()), std::sync::Arc::new("â".to_string()), std::sync::Arc::new("acute".to_string()), std::sync::Arc::new("\u{b4}".to_string()), std::sync::Arc::new("acute;".to_string()), std::sync::Arc::new("\u{b4}".to_string()), std::sync::Arc::new("acy;".to_string()), std::sync::Arc::new("а".to_string()), std::sync::Arc::new("aelig".to_string()), std::sync::Arc::new("æ".to_string()), std::sync::Arc::new("aelig;".to_string()), std::sync::Arc::new("æ".to_string()), std::sync::Arc::new("af;".to_string()), std::sync::Arc::new("\u{2061}".to_string()), std::sync::Arc::new("afr;".to_string()), std::sync::Arc::new("𝔞".to_string()), std::sync::Arc::new("agrave".to_string()), std::sync::Arc::new("à".to_string()), std::sync::Arc::new("agrave;".to_string()), std::sync::Arc::new("à".to_string()), std::sync::Arc::new("alefsym;".to_string()), std::sync::Arc::new("ℵ".to_string()), std::sync::Arc::new("aleph;".to_string()), std::sync::Arc::new("ℵ".to_string()), std::sync::Arc::new("alpha;".to_string()), std::sync::Arc::new("α".to_string()), std::sync::Arc::new("amacr;".to_string()), std::sync::Arc::new("ā".to_string()), std::sync::Arc::new("amalg;".to_string()), std::sync::Arc::new("\u{2a3f}".to_string()), std::sync::Arc::new("amp".to_string()), std::sync::Arc::new("&".to_string()), std::sync::Arc::new("amp;".to_string()), std::sync::Arc::new("&".to_string()), std::sync::Arc::new("and;".to_string()), std::sync::Arc::new("\u{2227}".to_string()), std::sync::Arc::new("andand;".to_string()), std::sync::Arc::new("\u{2a55}".to_string()), std::sync::Arc::new("andd;".to_string()), std::sync::Arc::new("\u{2a5c}".to_string()), std::sync::Arc::new("andslope;".to_string()), std::sync::Arc::new("\u{2a58}".to_string()), std::sync::Arc::new("andv;".to_string()), std::sync::Arc::new("\u{2a5a}".to_string()), std::sync::Arc::new("ang;".to_string()), std::sync::Arc::new("\u{2220}".to_string()), std::sync::Arc::new("ange;".to_string()), std::sync::Arc::new("\u{29a4}".to_string()), std::sync::Arc::new("angle;".to_string()), std::sync::Arc::new("\u{2220}".to_string()), std::sync::Arc::new("angmsd;".to_string()), std::sync::Arc::new("\u{2221}".to_string()), std::sync::Arc::new("angmsdaa;".to_string()), std::sync::Arc::new("\u{29a8}".to_string()), std::sync::Arc::new("angmsdab;".to_string()), std::sync::Arc::new("\u{29a9}".to_string()), std::sync::Arc::new("angmsdac;".to_string()), std::sync::Arc::new("\u{29aa}".to_string()), std::sync::Arc::new("angmsdad;".to_string()), std::sync::Arc::new("\u{29ab}".to_string()), std::sync::Arc::new("angmsdae;".to_string()), std::sync::Arc::new("\u{29ac}".to_string()), std::sync::Arc::new("angmsdaf;".to_string()), std::sync::Arc::new("\u{29ad}".to_string()), std::sync::Arc::new("angmsdag;".to_string()), std::sync::Arc::new("\u{29ae}".to_string()), std::sync::Arc::new("angmsdah;".to_string()), std::sync::Arc::new("\u{29af}".to_string()), std::sync::Arc::new("angrt;".to_string()), std::sync::Arc::new("\u{221f}".to_string()), std::sync::Arc::new("angrtvb;".to_string()), std::sync::Arc::new("\u{22be}".to_string()), std::sync::Arc::new("angrtvbd;".to_string()), std::sync::Arc::new("\u{299d}".to_string()), std::sync::Arc::new("angsph;".to_string()), std::sync::Arc::new("\u{2222}".to_string()), std::sync::Arc::new("angst;".to_string()), std::sync::Arc::new("Å".to_string()), std::sync::Arc::new("angzarr;".to_string()), std::sync::Arc::new("\u{237c}".to_string()), std::sync::Arc::new("aogon;".to_string()), std::sync::Arc::new("ą".to_string()), std::sync::Arc::new("aopf;".to_string()), std::sync::Arc::new("𝕒".to_string()), std::sync::Arc::new("ap;".to_string()), std::sync::Arc::new("\u{2248}".to_string()), std::sync::Arc::new("apE;".to_string()), std::sync::Arc::new("\u{2a70}".to_string()), std::sync::Arc::new("apacir;".to_string()), std::sync::Arc::new("\u{2a6f}".to_string()), std::sync::Arc::new("ape;".to_string()), std::sync::Arc::new("\u{224a}".to_string()), std::sync::Arc::new("apid;".to_string()), std::sync::Arc::new("\u{224b}".to_string()), std::sync::Arc::new("apos;".to_string()), std::sync::Arc::new("'".to_string()), std::sync::Arc::new("approx;".to_string()), std::sync::Arc::new("\u{2248}".to_string()), std::sync::Arc::new("approxeq;".to_string()), std::sync::Arc::new("\u{224a}".to_string()), std::sync::Arc::new("aring".to_string()), std::sync::Arc::new("å".to_string()), std::sync::Arc::new("aring;".to_string()), std::sync::Arc::new("å".to_string()), std::sync::Arc::new("ascr;".to_string()), std::sync::Arc::new("𝒶".to_string()), std::sync::Arc::new("ast;".to_string()), std::sync::Arc::new("*".to_string()), std::sync::Arc::new("asymp;".to_string()), std::sync::Arc::new("\u{2248}".to_string()), std::sync::Arc::new("asympeq;".to_string()), std::sync::Arc::new("\u{224d}".to_string()), std::sync::Arc::new("atilde".to_string()), std::sync::Arc::new("ã".to_string()), std::sync::Arc::new("atilde;".to_string()), std::sync::Arc::new("ã".to_string()), std::sync::Arc::new("auml".to_string()), std::sync::Arc::new("ä".to_string()), std::sync::Arc::new("auml;".to_string()), std::sync::Arc::new("ä".to_string()), std::sync::Arc::new("awconint;".to_string()), std::sync::Arc::new("\u{2233}".to_string()), std::sync::Arc::new("awint;".to_string()), std::sync::Arc::new("\u{2a11}".to_string()), std::sync::Arc::new("bNot;".to_string()), std::sync::Arc::new("\u{2aed}".to_string()), std::sync::Arc::new("backcong;".to_string()), std::sync::Arc::new("\u{224c}".to_string()), std::sync::Arc::new("backepsilon;".to_string()), std::sync::Arc::new("\u{3f6}".to_string()), std::sync::Arc::new("backprime;".to_string()), std::sync::Arc::new("\u{2035}".to_string()), std::sync::Arc::new("backsim;".to_string()), std::sync::Arc::new("\u{223d}".to_string()), std::sync::Arc::new("backsimeq;".to_string()), std::sync::Arc::new("\u{22cd}".to_string()), std::sync::Arc::new("barvee;".to_string()), std::sync::Arc::new("\u{22bd}".to_string()), std::sync::Arc::new("barwed;".to_string()), std::sync::Arc::new("\u{2305}".to_string()), std::sync::Arc::new("barwedge;".to_string()), std::sync::Arc::new("\u{2305}".to_string()), std::sync::Arc::new("bbrk;".to_string()), std::sync::Arc::new("\u{23b5}".to_string()), std::sync::Arc::new("bbrktbrk;".to_string()), std::sync::Arc::new("\u{23b6}".to_string()), std::sync::Arc::new("bcong;".to_string()), std::sync::Arc::new("\u{224c}".to_string()), std::sync::Arc::new("bcy;".to_string()), std::sync::Arc::new("б".to_string()), std::sync::Arc::new("bdquo;".to_string()), std::sync::Arc::new("\u{201e}".to_string()), std::sync::Arc::new("becaus;".to_string()), std::sync::Arc::new("\u{2235}".to_string()), std::sync::Arc::new("because;".to_string()), std::sync::Arc::new("\u{2235}".to_string()), std::sync::Arc::new("bemptyv;".to_string()), std::sync::Arc::new("\u{29b0}".to_string()), std::sync::Arc::new("bepsi;".to_string()), std::sync::Arc::new("\u{3f6}".to_string()), std::sync::Arc::new("bernou;".to_string()), std::sync::Arc::new("ℬ".to_string()), std::sync::Arc::new("beta;".to_string()), std::sync::Arc::new("β".to_string()), std::sync::Arc::new("beth;".to_string()), std::sync::Arc::new("ℶ".to_string()), std::sync::Arc::new("between;".to_string()), std::sync::Arc::new("\u{226c}".to_string()), std::sync::Arc::new("bfr;".to_string()), std::sync::Arc::new("𝔟".to_string()), std::sync::Arc::new("bigcap;".to_string()), std::sync::Arc::new("\u{22c2}".to_string()), std::sync::Arc::new("bigcirc;".to_string()), std::sync::Arc::new("\u{25ef}".to_string()), std::sync::Arc::new("bigcup;".to_string()), std::sync::Arc::new("\u{22c3}".to_string()), std::sync::Arc::new("bigodot;".to_string()), std::sync::Arc::new("\u{2a00}".to_string()), std::sync::Arc::new("bigoplus;".to_string()), std::sync::Arc::new("\u{2a01}".to_string()), std::sync::Arc::new("bigotimes;".to_string()), std::sync::Arc::new("\u{2a02}".to_string()), std::sync::Arc::new("bigsqcup;".to_string()), std::sync::Arc::new("\u{2a06}".to_string()), std::sync::Arc::new("bigstar;".to_string()), std::sync::Arc::new("\u{2605}".to_string()), std::sync::Arc::new("bigtriangledown;".to_string()), std::sync::Arc::new("\u{25bd}".to_string()), std::sync::Arc::new("bigtriangleup;".to_string()), std::sync::Arc::new("\u{25b3}".to_string()), std::sync::Arc::new("biguplus;".to_string()), std::sync::Arc::new("\u{2a04}".to_string()), std::sync::Arc::new("bigvee;".to_string()), std::sync::Arc::new("\u{22c1}".to_string()), std::sync::Arc::new("bigwedge;".to_string()), std::sync::Arc::new("\u{22c0}".to_string()), std::sync::Arc::new("bkarow;".to_string()), std::sync::Arc::new("\u{290d}".to_string()), std::sync::Arc::new("blacklozenge;".to_string()), std::sync::Arc::new("\u{29eb}".to_string()), std::sync::Arc::new("blacksquare;".to_string()), std::sync::Arc::new("\u{25aa}".to_string()), std::sync::Arc::new("blacktriangle;".to_string()), std::sync::Arc::new("\u{25b4}".to_string()), std::sync::Arc::new("blacktriangledown;".to_string()), std::sync::Arc::new("\u{25be}".to_string()), std::sync::Arc::new("blacktriangleleft;".to_string()), std::sync::Arc::new("\u{25c2}".to_string()), std::sync::Arc::new("blacktriangleright;".to_string()), std::sync::Arc::new("\u{25b8}".to_string()), std::sync::Arc::new("blank;".to_string()), std::sync::Arc::new("\u{2423}".to_string()), std::sync::Arc::new("blk12;".to_string()), std::sync::Arc::new("\u{2592}".to_string()), std::sync::Arc::new("blk14;".to_string()), std::sync::Arc::new("\u{2591}".to_string()), std::sync::Arc::new("blk34;".to_string()), std::sync::Arc::new("\u{2593}".to_string()), std::sync::Arc::new("block;".to_string()), std::sync::Arc::new("\u{2588}".to_string()), std::sync::Arc::new("bne;".to_string()), std::sync::Arc::new("=\u{20e5}".to_string()), std::sync::Arc::new("bnequiv;".to_string()), std::sync::Arc::new("\u{2261}\u{20e5}".to_string()), std::sync::Arc::new("bnot;".to_string()), std::sync::Arc::new("\u{2310}".to_string()), std::sync::Arc::new("bopf;".to_string()), std::sync::Arc::new("𝕓".to_string()), std::sync::Arc::new("bot;".to_string()), std::sync::Arc::new("\u{22a5}".to_string()), std::sync::Arc::new("bottom;".to_string()), std::sync::Arc::new("\u{22a5}".to_string()), std::sync::Arc::new("bowtie;".to_string()), std::sync::Arc::new("\u{22c8}".to_string()), std::sync::Arc::new("boxDL;".to_string()), std::sync::Arc::new("\u{2557}".to_string()), std::sync::Arc::new("boxDR;".to_string()), std::sync::Arc::new("\u{2554}".to_string()), std::sync::Arc::new("boxDl;".to_string()), std::sync::Arc::new("\u{2556}".to_string()), std::sync::Arc::new("boxDr;".to_string()), std::sync::Arc::new("\u{2553}".to_string()), std::sync::Arc::new("boxH;".to_string()), std::sync::Arc::new("\u{2550}".to_string()), std::sync::Arc::new("boxHD;".to_string()), std::sync::Arc::new("\u{2566}".to_string()), std::sync::Arc::new("boxHU;".to_string()), std::sync::Arc::new("\u{2569}".to_string()), std::sync::Arc::new("boxHd;".to_string()), std::sync::Arc::new("\u{2564}".to_string()), std::sync::Arc::new("boxHu;".to_string()), std::sync::Arc::new("\u{2567}".to_string()), std::sync::Arc::new("boxUL;".to_string()), std::sync::Arc::new("\u{255d}".to_string()), std::sync::Arc::new("boxUR;".to_string()), std::sync::Arc::new("\u{255a}".to_string()), std::sync::Arc::new("boxUl;".to_string()), std::sync::Arc::new("\u{255c}".to_string()), std::sync::Arc::new("boxUr;".to_string()), std::sync::Arc::new("\u{2559}".to_string()), std::sync::Arc::new("boxV;".to_string()), std::sync::Arc::new("\u{2551}".to_string()), std::sync::Arc::new("boxVH;".to_string()), std::sync::Arc::new("\u{256c}".to_string()), std::sync::Arc::new("boxVL;".to_string()), std::sync::Arc::new("\u{2563}".to_string()), std::sync::Arc::new("boxVR;".to_string()), std::sync::Arc::new("\u{2560}".to_string()), std::sync::Arc::new("boxVh;".to_string()), std::sync::Arc::new("\u{256b}".to_string()), std::sync::Arc::new("boxVl;".to_string()), std::sync::Arc::new("\u{2562}".to_string()), std::sync::Arc::new("boxVr;".to_string()), std::sync::Arc::new("\u{255f}".to_string()), std::sync::Arc::new("boxbox;".to_string()), std::sync::Arc::new("\u{29c9}".to_string()), std::sync::Arc::new("boxdL;".to_string()), std::sync::Arc::new("\u{2555}".to_string()), std::sync::Arc::new("boxdR;".to_string()), std::sync::Arc::new("\u{2552}".to_string()), std::sync::Arc::new("boxdl;".to_string()), std::sync::Arc::new("\u{2510}".to_string()), std::sync::Arc::new("boxdr;".to_string()), std::sync::Arc::new("\u{250c}".to_string()), std::sync::Arc::new("boxh;".to_string()), std::sync::Arc::new("\u{2500}".to_string()), std::sync::Arc::new("boxhD;".to_string()), std::sync::Arc::new("\u{2565}".to_string()), std::sync::Arc::new("boxhU;".to_string()), std::sync::Arc::new("\u{2568}".to_string()), std::sync::Arc::new("boxhd;".to_string()), std::sync::Arc::new("\u{252c}".to_string()), std::sync::Arc::new("boxhu;".to_string()), std::sync::Arc::new("\u{2534}".to_string()), std::sync::Arc::new("boxminus;".to_string()), std::sync::Arc::new("\u{229f}".to_string()), std::sync::Arc::new("boxplus;".to_string()), std::sync::Arc::new("\u{229e}".to_string()), std::sync::Arc::new("boxtimes;".to_string()), std::sync::Arc::new("\u{22a0}".to_string()), std::sync::Arc::new("boxuL;".to_string()), std::sync::Arc::new("\u{255b}".to_string()), std::sync::Arc::new("boxuR;".to_string()), std::sync::Arc::new("\u{2558}".to_string()), std::sync::Arc::new("boxul;".to_string()), std::sync::Arc::new("\u{2518}".to_string()), std::sync::Arc::new("boxur;".to_string()), std::sync::Arc::new("\u{2514}".to_string()), std::sync::Arc::new("boxv;".to_string()), std::sync::Arc::new("\u{2502}".to_string()), std::sync::Arc::new("boxvH;".to_string()), std::sync::Arc::new("\u{256a}".to_string()), std::sync::Arc::new("boxvL;".to_string()), std::sync::Arc::new("\u{2561}".to_string()), std::sync::Arc::new("boxvR;".to_string()), std::sync::Arc::new("\u{255e}".to_string()), std::sync::Arc::new("boxvh;".to_string()), std::sync::Arc::new("\u{253c}".to_string()), std::sync::Arc::new("boxvl;".to_string()), std::sync::Arc::new("\u{2524}".to_string()), std::sync::Arc::new("boxvr;".to_string()), std::sync::Arc::new("\u{251c}".to_string()), std::sync::Arc::new("bprime;".to_string()), std::sync::Arc::new("\u{2035}".to_string()), std::sync::Arc::new("breve;".to_string()), std::sync::Arc::new("\u{2d8}".to_string()), std::sync::Arc::new("brvbar".to_string()), std::sync::Arc::new("\u{a6}".to_string()), std::sync::Arc::new("brvbar;".to_string()), std::sync::Arc::new("\u{a6}".to_string()), std::sync::Arc::new("bscr;".to_string()), std::sync::Arc::new("𝒷".to_string()), std::sync::Arc::new("bsemi;".to_string()), std::sync::Arc::new("\u{204f}".to_string()), std::sync::Arc::new("bsim;".to_string()), std::sync::Arc::new("\u{223d}".to_string()), std::sync::Arc::new("bsime;".to_string()), std::sync::Arc::new("\u{22cd}".to_string()), std::sync::Arc::new("bsol;".to_string()), std::sync::Arc::new("\\".to_string()), std::sync::Arc::new("bsolb;".to_string()), std::sync::Arc::new("\u{29c5}".to_string()), std::sync::Arc::new("bsolhsub;".to_string()), std::sync::Arc::new("\u{27c8}".to_string()), std::sync::Arc::new("bull;".to_string()), std::sync::Arc::new("\u{2022}".to_string()), std::sync::Arc::new("bullet;".to_string()), std::sync::Arc::new("\u{2022}".to_string()), std::sync::Arc::new("bump;".to_string()), std::sync::Arc::new("\u{224e}".to_string()), std::sync::Arc::new("bumpE;".to_string()), std::sync::Arc::new("\u{2aae}".to_string()), std::sync::Arc::new("bumpe;".to_string()), std::sync::Arc::new("\u{224f}".to_string()), std::sync::Arc::new("bumpeq;".to_string()), std::sync::Arc::new("\u{224f}".to_string()), std::sync::Arc::new("cacute;".to_string()), std::sync::Arc::new("ć".to_string()), std::sync::Arc::new("cap;".to_string()), std::sync::Arc::new("\u{2229}".to_string()), std::sync::Arc::new("capand;".to_string()), std::sync::Arc::new("\u{2a44}".to_string()), std::sync::Arc::new("capbrcup;".to_string()), std::sync::Arc::new("\u{2a49}".to_string()), std::sync::Arc::new("capcap;".to_string()), std::sync::Arc::new("\u{2a4b}".to_string()), std::sync::Arc::new("capcup;".to_string()), std::sync::Arc::new("\u{2a47}".to_string()), std::sync::Arc::new("capdot;".to_string()), std::sync::Arc::new("\u{2a40}".to_string()), std::sync::Arc::new("caps;".to_string()), std::sync::Arc::new("\u{2229}\u{fe00}".to_string()), std::sync::Arc::new("caret;".to_string()), std::sync::Arc::new("\u{2041}".to_string()), std::sync::Arc::new("caron;".to_string()), std::sync::Arc::new("ˇ".to_string()), std::sync::Arc::new("ccaps;".to_string()), std::sync::Arc::new("\u{2a4d}".to_string()), std::sync::Arc::new("ccaron;".to_string()), std::sync::Arc::new("č".to_string()), std::sync::Arc::new("ccedil".to_string()), std::sync::Arc::new("ç".to_string()), std::sync::Arc::new("ccedil;".to_string()), std::sync::Arc::new("ç".to_string()), std::sync::Arc::new("ccirc;".to_string()), std::sync::Arc::new("ĉ".to_string()), std::sync::Arc::new("ccups;".to_string()), std::sync::Arc::new("\u{2a4c}".to_string()), std::sync::Arc::new("ccupssm;".to_string()), std::sync::Arc::new("\u{2a50}".to_string()), std::sync::Arc::new("cdot;".to_string()), std::sync::Arc::new("ċ".to_string()), std::sync::Arc::new("cedil".to_string()), std::sync::Arc::new("\u{b8}".to_string()), std::sync::Arc::new("cedil;".to_string()), std::sync::Arc::new("\u{b8}".to_string()), std::sync::Arc::new("cemptyv;".to_string()), std::sync::Arc::new("\u{29b2}".to_string()), std::sync::Arc::new("cent".to_string()), std::sync::Arc::new("\u{a2}".to_string()), std::sync::Arc::new("cent;".to_string()), std::sync::Arc::new("\u{a2}".to_string()), std::sync::Arc::new("centerdot;".to_string()), std::sync::Arc::new("\u{b7}".to_string()), std::sync::Arc::new("cfr;".to_string()), std::sync::Arc::new("𝔠".to_string()), std::sync::Arc::new("chcy;".to_string()), std::sync::Arc::new("ч".to_string()), std::sync::Arc::new("check;".to_string()), std::sync::Arc::new("\u{2713}".to_string()), std::sync::Arc::new("checkmark;".to_string()), std::sync::Arc::new("\u{2713}".to_string()), std::sync::Arc::new("chi;".to_string()), std::sync::Arc::new("χ".to_string()), std::sync::Arc::new("cir;".to_string()), std::sync::Arc::new("\u{25cb}".to_string()), std::sync::Arc::new("cirE;".to_string()), std::sync::Arc::new("\u{29c3}".to_string()), std::sync::Arc::new("circ;".to_string()), std::sync::Arc::new("ˆ".to_string()), std::sync::Arc::new("circeq;".to_string()), std::sync::Arc::new("\u{2257}".to_string()), std::sync::Arc::new("circlearrowleft;".to_string()), std::sync::Arc::new("\u{21ba}".to_string()), std::sync::Arc::new("circlearrowright;".to_string()), std::sync::Arc::new("\u{21bb}".to_string()), std::sync::Arc::new("circledR;".to_string()), std::sync::Arc::new("\u{ae}".to_string()), std::sync::Arc::new("circledS;".to_string()), std::sync::Arc::new("\u{24c8}".to_string()), std::sync::Arc::new("circledast;".to_string()), std::sync::Arc::new("\u{229b}".to_string()), std::sync::Arc::new("circledcirc;".to_string()), std::sync::Arc::new("\u{229a}".to_string()), std::sync::Arc::new("circleddash;".to_string()), std::sync::Arc::new("\u{229d}".to_string()), std::sync::Arc::new("cire;".to_string()), std::sync::Arc::new("\u{2257}".to_string()), std::sync::Arc::new("cirfnint;".to_string()), std::sync::Arc::new("\u{2a10}".to_string()), std::sync::Arc::new("cirmid;".to_string()), std::sync::Arc::new("\u{2aef}".to_string()), std::sync::Arc::new("cirscir;".to_string()), std::sync::Arc::new("\u{29c2}".to_string()), std::sync::Arc::new("clubs;".to_string()), std::sync::Arc::new("\u{2663}".to_string()), std::sync::Arc::new("clubsuit;".to_string()), std::sync::Arc::new("\u{2663}".to_string()), std::sync::Arc::new("colon;".to_string()), std::sync::Arc::new(":".to_string()), std::sync::Arc::new("colone;".to_string()), std::sync::Arc::new("\u{2254}".to_string()), std::sync::Arc::new("coloneq;".to_string()), std::sync::Arc::new("\u{2254}".to_string()), std::sync::Arc::new("comma;".to_string()), std::sync::Arc::new(",".to_string()), std::sync::Arc::new("commat;".to_string()), std::sync::Arc::new("@".to_string()), std::sync::Arc::new("comp;".to_string()), std::sync::Arc::new("\u{2201}".to_string()), std::sync::Arc::new("compfn;".to_string()), std::sync::Arc::new("\u{2218}".to_string()), std::sync::Arc::new("complement;".to_string()), std::sync::Arc::new("\u{2201}".to_string()), std::sync::Arc::new("complexes;".to_string()), std::sync::Arc::new("ℂ".to_string()), std::sync::Arc::new("cong;".to_string()), std::sync::Arc::new("\u{2245}".to_string()), std::sync::Arc::new("congdot;".to_string()), std::sync::Arc::new("\u{2a6d}".to_string()), std::sync::Arc::new("conint;".to_string()), std::sync::Arc::new("\u{222e}".to_string()), std::sync::Arc::new("copf;".to_string()), std::sync::Arc::new("𝕔".to_string()), std::sync::Arc::new("coprod;".to_string()), std::sync::Arc::new("\u{2210}".to_string()), std::sync::Arc::new("copy".to_string()), std::sync::Arc::new("\u{a9}".to_string()), std::sync::Arc::new("copy;".to_string()), std::sync::Arc::new("\u{a9}".to_string()), std::sync::Arc::new("copysr;".to_string()), std::sync::Arc::new("\u{2117}".to_string()), std::sync::Arc::new("crarr;".to_string()), std::sync::Arc::new("\u{21b5}".to_string()), std::sync::Arc::new("cross;".to_string()), std::sync::Arc::new("\u{2717}".to_string()), std::sync::Arc::new("cscr;".to_string()), std::sync::Arc::new("𝒸".to_string()), std::sync::Arc::new("csub;".to_string()), std::sync::Arc::new("\u{2acf}".to_string()), std::sync::Arc::new("csube;".to_string()), std::sync::Arc::new("\u{2ad1}".to_string()), std::sync::Arc::new("csup;".to_string()), std::sync::Arc::new("\u{2ad0}".to_string()), std::sync::Arc::new("csupe;".to_string()), std::sync::Arc::new("\u{2ad2}".to_string()), std::sync::Arc::new("ctdot;".to_string()), std::sync::Arc::new("\u{22ef}".to_string()), std::sync::Arc::new("cudarrl;".to_string()), std::sync::Arc::new("\u{2938}".to_string()), std::sync::Arc::new("cudarrr;".to_string()), std::sync::Arc::new("\u{2935}".to_string()), std::sync::Arc::new("cuepr;".to_string()), std::sync::Arc::new("\u{22de}".to_string()), std::sync::Arc::new("cuesc;".to_string()), std::sync::Arc::new("\u{22df}".to_string()), std::sync::Arc::new("cularr;".to_string()), std::sync::Arc::new("\u{21b6}".to_string()), std::sync::Arc::new("cularrp;".to_string()), std::sync::Arc::new("\u{293d}".to_string()), std::sync::Arc::new("cup;".to_string()), std::sync::Arc::new("\u{222a}".to_string()), std::sync::Arc::new("cupbrcap;".to_string()), std::sync::Arc::new("\u{2a48}".to_string()), std::sync::Arc::new("cupcap;".to_string()), std::sync::Arc::new("\u{2a46}".to_string()), std::sync::Arc::new("cupcup;".to_string()), std::sync::Arc::new("\u{2a4a}".to_string()), std::sync::Arc::new("cupdot;".to_string()), std::sync::Arc::new("\u{228d}".to_string()), std::sync::Arc::new("cupor;".to_string()), std::sync::Arc::new("\u{2a45}".to_string()), std::sync::Arc::new("cups;".to_string()), std::sync::Arc::new("\u{222a}\u{fe00}".to_string()), std::sync::Arc::new("curarr;".to_string()), std::sync::Arc::new("\u{21b7}".to_string()), std::sync::Arc::new("curarrm;".to_string()), std::sync::Arc::new("\u{293c}".to_string()), std::sync::Arc::new("curlyeqprec;".to_string()), std::sync::Arc::new("\u{22de}".to_string()), std::sync::Arc::new("curlyeqsucc;".to_string()), std::sync::Arc::new("\u{22df}".to_string()), std::sync::Arc::new("curlyvee;".to_string()), std::sync::Arc::new("\u{22ce}".to_string()), std::sync::Arc::new("curlywedge;".to_string()), std::sync::Arc::new("\u{22cf}".to_string()), std::sync::Arc::new("curren".to_string()), std::sync::Arc::new("\u{a4}".to_string()), std::sync::Arc::new("curren;".to_string()), std::sync::Arc::new("\u{a4}".to_string()), std::sync::Arc::new("curvearrowleft;".to_string()), std::sync::Arc::new("\u{21b6}".to_string()), std::sync::Arc::new("curvearrowright;".to_string()), std::sync::Arc::new("\u{21b7}".to_string()), std::sync::Arc::new("cuvee;".to_string()), std::sync::Arc::new("\u{22ce}".to_string()), std::sync::Arc::new("cuwed;".to_string()), std::sync::Arc::new("\u{22cf}".to_string()), std::sync::Arc::new("cwconint;".to_string()), std::sync::Arc::new("\u{2232}".to_string()), std::sync::Arc::new("cwint;".to_string()), std::sync::Arc::new("\u{2231}".to_string()), std::sync::Arc::new("cylcty;".to_string()), std::sync::Arc::new("\u{232d}".to_string()), std::sync::Arc::new("dArr;".to_string()), std::sync::Arc::new("\u{21d3}".to_string()), std::sync::Arc::new("dHar;".to_string()), std::sync::Arc::new("\u{2965}".to_string()), std::sync::Arc::new("dagger;".to_string()), std::sync::Arc::new("\u{2020}".to_string()), std::sync::Arc::new("daleth;".to_string()), std::sync::Arc::new("ℸ".to_string()), std::sync::Arc::new("darr;".to_string()), std::sync::Arc::new("\u{2193}".to_string()), std::sync::Arc::new("dash;".to_string()), std::sync::Arc::new("\u{2010}".to_string()), std::sync::Arc::new("dashv;".to_string()), std::sync::Arc::new("\u{22a3}".to_string()), std::sync::Arc::new("dbkarow;".to_string()), std::sync::Arc::new("\u{290f}".to_string()), std::sync::Arc::new("dblac;".to_string()), std::sync::Arc::new("\u{2dd}".to_string()), std::sync::Arc::new("dcaron;".to_string()), std::sync::Arc::new("ď".to_string()), std::sync::Arc::new("dcy;".to_string()), std::sync::Arc::new("д".to_string()), std::sync::Arc::new("dd;".to_string()), std::sync::Arc::new("ⅆ".to_string()), std::sync::Arc::new("ddagger;".to_string()), std::sync::Arc::new("\u{2021}".to_string()), std::sync::Arc::new("ddarr;".to_string()), std::sync::Arc::new("\u{21ca}".to_string()), std::sync::Arc::new("ddotseq;".to_string()), std::sync::Arc::new("\u{2a77}".to_string()), std::sync::Arc::new("deg".to_string()), std::sync::Arc::new("\u{b0}".to_string()), std::sync::Arc::new("deg;".to_string()), std::sync::Arc::new("\u{b0}".to_string()), std::sync::Arc::new("delta;".to_string()), std::sync::Arc::new("δ".to_string()), std::sync::Arc::new("demptyv;".to_string()), std::sync::Arc::new("\u{29b1}".to_string()), std::sync::Arc::new("dfisht;".to_string()), std::sync::Arc::new("\u{297f}".to_string()), std::sync::Arc::new("dfr;".to_string()), std::sync::Arc::new("𝔡".to_string()), std::sync::Arc::new("dharl;".to_string()), std::sync::Arc::new("\u{21c3}".to_string()), std::sync::Arc::new("dharr;".to_string()), std::sync::Arc::new("\u{21c2}".to_string()), std::sync::Arc::new("diam;".to_string()), std::sync::Arc::new("\u{22c4}".to_string()), std::sync::Arc::new("diamond;".to_string()), std::sync::Arc::new("\u{22c4}".to_string()), std::sync::Arc::new("diamondsuit;".to_string()), std::sync::Arc::new("\u{2666}".to_string()), std::sync::Arc::new("diams;".to_string()), std::sync::Arc::new("\u{2666}".to_string()), std::sync::Arc::new("die;".to_string()), std::sync::Arc::new("\u{a8}".to_string()), std::sync::Arc::new("digamma;".to_string()), std::sync::Arc::new("ϝ".to_string()), std::sync::Arc::new("disin;".to_string()), std::sync::Arc::new("\u{22f2}".to_string()), std::sync::Arc::new("div;".to_string()), std::sync::Arc::new("\u{f7}".to_string()), std::sync::Arc::new("divide".to_string()), std::sync::Arc::new("\u{f7}".to_string()), std::sync::Arc::new("divide;".to_string()), std::sync::Arc::new("\u{f7}".to_string()), std::sync::Arc::new("divideontimes;".to_string()), std::sync::Arc::new("\u{22c7}".to_string()), std::sync::Arc::new("divonx;".to_string()), std::sync::Arc::new("\u{22c7}".to_string()), std::sync::Arc::new("djcy;".to_string()), std::sync::Arc::new("ђ".to_string()), std::sync::Arc::new("dlcorn;".to_string()), std::sync::Arc::new("\u{231e}".to_string()), std::sync::Arc::new("dlcrop;".to_string()), std::sync::Arc::new("\u{230d}".to_string()), std::sync::Arc::new("dollar;".to_string()), std::sync::Arc::new("$".to_string()), std::sync::Arc::new("dopf;".to_string()), std::sync::Arc::new("𝕕".to_string()), std::sync::Arc::new("dot;".to_string()), std::sync::Arc::new("\u{2d9}".to_string()), std::sync::Arc::new("doteq;".to_string()), std::sync::Arc::new("\u{2250}".to_string()), std::sync::Arc::new("doteqdot;".to_string()), std::sync::Arc::new("\u{2251}".to_string()), std::sync::Arc::new("dotminus;".to_string()), std::sync::Arc::new("\u{2238}".to_string()), std::sync::Arc::new("dotplus;".to_string()), std::sync::Arc::new("\u{2214}".to_string()), std::sync::Arc::new("dotsquare;".to_string()), std::sync::Arc::new("\u{22a1}".to_string()), std::sync::Arc::new("doublebarwedge;".to_string()), std::sync::Arc::new("\u{2306}".to_string()), std::sync::Arc::new("downarrow;".to_string()), std::sync::Arc::new("\u{2193}".to_string()), std::sync::Arc::new("downdownarrows;".to_string()), std::sync::Arc::new("\u{21ca}".to_string()), std::sync::Arc::new("downharpoonleft;".to_string()), std::sync::Arc::new("\u{21c3}".to_string()), std::sync::Arc::new("downharpoonright;".to_string()), std::sync::Arc::new("\u{21c2}".to_string()), std::sync::Arc::new("drbkarow;".to_string()), std::sync::Arc::new("\u{2910}".to_string()), std::sync::Arc::new("drcorn;".to_string()), std::sync::Arc::new("\u{231f}".to_string()), std::sync::Arc::new("drcrop;".to_string()), std::sync::Arc::new("\u{230c}".to_string()), std::sync::Arc::new("dscr;".to_string()), std::sync::Arc::new("𝒹".to_string()), std::sync::Arc::new("dscy;".to_string()), std::sync::Arc::new("ѕ".to_string()), std::sync::Arc::new("dsol;".to_string()), std::sync::Arc::new("\u{29f6}".to_string()), std::sync::Arc::new("dstrok;".to_string()), std::sync::Arc::new("đ".to_string()), std::sync::Arc::new("dtdot;".to_string()), std::sync::Arc::new("\u{22f1}".to_string()), std::sync::Arc::new("dtri;".to_string()), std::sync::Arc::new("\u{25bf}".to_string()), std::sync::Arc::new("dtrif;".to_string()), std::sync::Arc::new("\u{25be}".to_string()), std::sync::Arc::new("duarr;".to_string()), std::sync::Arc::new("\u{21f5}".to_string()), std::sync::Arc::new("duhar;".to_string()), std::sync::Arc::new("\u{296f}".to_string()), std::sync::Arc::new("dwangle;".to_string()), std::sync::Arc::new("\u{29a6}".to_string()), std::sync::Arc::new("dzcy;".to_string()), std::sync::Arc::new("џ".to_string()), std::sync::Arc::new("dzigrarr;".to_string()), std::sync::Arc::new("\u{27ff}".to_string()), std::sync::Arc::new("eDDot;".to_string()), std::sync::Arc::new("\u{2a77}".to_string()), std::sync::Arc::new("eDot;".to_string()), std::sync::Arc::new("\u{2251}".to_string()), std::sync::Arc::new("eacute".to_string()), std::sync::Arc::new("é".to_string()), std::sync::Arc::new("eacute;".to_string()), std::sync::Arc::new("é".to_string()), std::sync::Arc::new("easter;".to_string()), std::sync::Arc::new("\u{2a6e}".to_string()), std::sync::Arc::new("ecaron;".to_string()), std::sync::Arc::new("ě".to_string()), std::sync::Arc::new("ecir;".to_string()), std::sync::Arc::new("\u{2256}".to_string()), std::sync::Arc::new("ecirc".to_string()), std::sync::Arc::new("ê".to_string()), std::sync::Arc::new("ecirc;".to_string()), std::sync::Arc::new("ê".to_string()), std::sync::Arc::new("ecolon;".to_string()), std::sync::Arc::new("\u{2255}".to_string()), std::sync::Arc::new("ecy;".to_string()), std::sync::Arc::new("э".to_string()), std::sync::Arc::new("edot;".to_string()), std::sync::Arc::new("ė".to_string()), std::sync::Arc::new("ee;".to_string()), std::sync::Arc::new("ⅇ".to_string()), std::sync::Arc::new("efDot;".to_string()), std::sync::Arc::new("\u{2252}".to_string()), std::sync::Arc::new("efr;".to_string()), std::sync::Arc::new("𝔢".to_string()), std::sync::Arc::new("eg;".to_string()), std::sync::Arc::new("\u{2a9a}".to_string()), std::sync::Arc::new("egrave".to_string()), std::sync::Arc::new("è".to_string()), std::sync::Arc::new("egrave;".to_string()), std::sync::Arc::new("è".to_string()), std::sync::Arc::new("egs;".to_string()), std::sync::Arc::new("\u{2a96}".to_string()), std::sync::Arc::new("egsdot;".to_string()), std::sync::Arc::new("\u{2a98}".to_string()), std::sync::Arc::new("el;".to_string()), std::sync::Arc::new("\u{2a99}".to_string()), std::sync::Arc::new("elinters;".to_string()), std::sync::Arc::new("\u{23e7}".to_string()), std::sync::Arc::new("ell;".to_string()), std::sync::Arc::new("ℓ".to_string()), std::sync::Arc::new("els;".to_string()), std::sync::Arc::new("\u{2a95}".to_string()), std::sync::Arc::new("elsdot;".to_string()), std::sync::Arc::new("\u{2a97}".to_string()), std::sync::Arc::new("emacr;".to_string()), std::sync::Arc::new("ē".to_string()), std::sync::Arc::new("empty;".to_string()), std::sync::Arc::new("\u{2205}".to_string()), std::sync::Arc::new("emptyset;".to_string()), std::sync::Arc::new("\u{2205}".to_string()), std::sync::Arc::new("emptyv;".to_string()), std::sync::Arc::new("\u{2205}".to_string()), std::sync::Arc::new("emsp13;".to_string()), std::sync::Arc::new("\u{2004}".to_string()), std::sync::Arc::new("emsp14;".to_string()), std::sync::Arc::new("\u{2005}".to_string()), std::sync::Arc::new("emsp;".to_string()), std::sync::Arc::new("\u{2003}".to_string()), std::sync::Arc::new("eng;".to_string()), std::sync::Arc::new("ŋ".to_string()), std::sync::Arc::new("ensp;".to_string()), std::sync::Arc::new("\u{2002}".to_string()), std::sync::Arc::new("eogon;".to_string()), std::sync::Arc::new("ę".to_string()), std::sync::Arc::new("eopf;".to_string()), std::sync::Arc::new("𝕖".to_string()), std::sync::Arc::new("epar;".to_string()), std::sync::Arc::new("\u{22d5}".to_string()), std::sync::Arc::new("eparsl;".to_string()), std::sync::Arc::new("\u{29e3}".to_string()), std::sync::Arc::new("eplus;".to_string()), std::sync::Arc::new("\u{2a71}".to_string()), std::sync::Arc::new("epsi;".to_string()), std::sync::Arc::new("ε".to_string()), std::sync::Arc::new("epsilon;".to_string()), std::sync::Arc::new("ε".to_string()), std::sync::Arc::new("epsiv;".to_string()), std::sync::Arc::new("ϵ".to_string()), std::sync::Arc::new("eqcirc;".to_string()), std::sync::Arc::new("\u{2256}".to_string()), std::sync::Arc::new("eqcolon;".to_string()), std::sync::Arc::new("\u{2255}".to_string()), std::sync::Arc::new("eqsim;".to_string()), std::sync::Arc::new("\u{2242}".to_string()), std::sync::Arc::new("eqslantgtr;".to_string()), std::sync::Arc::new("\u{2a96}".to_string()), std::sync::Arc::new("eqslantless;".to_string()), std::sync::Arc::new("\u{2a95}".to_string()), std::sync::Arc::new("equals;".to_string()), std::sync::Arc::new("=".to_string()), std::sync::Arc::new("equest;".to_string()), std::sync::Arc::new("\u{225f}".to_string()), std::sync::Arc::new("equiv;".to_string()), std::sync::Arc::new("\u{2261}".to_string()), std::sync::Arc::new("equivDD;".to_string()), std::sync::Arc::new("\u{2a78}".to_string()), std::sync::Arc::new("eqvparsl;".to_string()), std::sync::Arc::new("\u{29e5}".to_string()), std::sync::Arc::new("erDot;".to_string()), std::sync::Arc::new("\u{2253}".to_string()), std::sync::Arc::new("erarr;".to_string()), std::sync::Arc::new("\u{2971}".to_string()), std::sync::Arc::new("escr;".to_string()), std::sync::Arc::new("ℯ".to_string()), std::sync::Arc::new("esdot;".to_string()), std::sync::Arc::new("\u{2250}".to_string()), std::sync::Arc::new("esim;".to_string()), std::sync::Arc::new("\u{2242}".to_string()), std::sync::Arc::new("eta;".to_string()), std::sync::Arc::new("η".to_string()), std::sync::Arc::new("eth".to_string()), std::sync::Arc::new("ð".to_string()), std::sync::Arc::new("eth;".to_string()), std::sync::Arc::new("ð".to_string()), std::sync::Arc::new("euml".to_string()), std::sync::Arc::new("ë".to_string()), std::sync::Arc::new("euml;".to_string()), std::sync::Arc::new("ë".to_string()), std::sync::Arc::new("euro;".to_string()), std::sync::Arc::new("\u{20ac}".to_string()), std::sync::Arc::new("excl;".to_string()), std::sync::Arc::new("!".to_string()), std::sync::Arc::new("exist;".to_string()), std::sync::Arc::new("\u{2203}".to_string()), std::sync::Arc::new("expectation;".to_string()), std::sync::Arc::new("ℰ".to_string()), std::sync::Arc::new("exponentiale;".to_string()), std::sync::Arc::new("ⅇ".to_string()), std::sync::Arc::new("fallingdotseq;".to_string()), std::sync::Arc::new("\u{2252}".to_string()), std::sync::Arc::new("fcy;".to_string()), std::sync::Arc::new("ф".to_string()), std::sync::Arc::new("female;".to_string()), std::sync::Arc::new("\u{2640}".to_string()), std::sync::Arc::new("ffilig;".to_string()), std::sync::Arc::new("ﬃ".to_string()), std::sync::Arc::new("fflig;".to_string()), std::sync::Arc::new("ﬀ".to_string()), std::sync::Arc::new("ffllig;".to_string()), std::sync::Arc::new("ﬄ".to_string()), std::sync::Arc::new("ffr;".to_string()), std::sync::Arc::new("𝔣".to_string()), std::sync::Arc::new("filig;".to_string()), std::sync::Arc::new("ﬁ".to_string()), std::sync::Arc::new("fjlig;".to_string()), std::sync::Arc::new("fj".to_string()), std::sync::Arc::new("flat;".to_string()), std::sync::Arc::new("\u{266d}".to_string()), std::sync::Arc::new("fllig;".to_string()), std::sync::Arc::new("ﬂ".to_string()), std::sync::Arc::new("fltns;".to_string()), std::sync::Arc::new("\u{25b1}".to_string()), std::sync::Arc::new("fnof;".to_string()), std::sync::Arc::new("ƒ".to_string()), std::sync::Arc::new("fopf;".to_string()), std::sync::Arc::new("𝕗".to_string()), std::sync::Arc::new("forall;".to_string()), std::sync::Arc::new("\u{2200}".to_string()), std::sync::Arc::new("fork;".to_string()), std::sync::Arc::new("\u{22d4}".to_string()), std::sync::Arc::new("forkv;".to_string()), std::sync::Arc::new("\u{2ad9}".to_string()), std::sync::Arc::new("fpartint;".to_string()), std::sync::Arc::new("\u{2a0d}".to_string()), std::sync::Arc::new("frac12".to_string()), std::sync::Arc::new("\u{bd}".to_string()), std::sync::Arc::new("frac12;".to_string()), std::sync::Arc::new("\u{bd}".to_string()), std::sync::Arc::new("frac13;".to_string()), std::sync::Arc::new("\u{2153}".to_string()), std::sync::Arc::new("frac14".to_string()), std::sync::Arc::new("\u{bc}".to_string()), std::sync::Arc::new("frac14;".to_string()), std::sync::Arc::new("\u{bc}".to_string()), std::sync::Arc::new("frac15;".to_string()), std::sync::Arc::new("\u{2155}".to_string()), std::sync::Arc::new("frac16;".to_string()), std::sync::Arc::new("\u{2159}".to_string()), std::sync::Arc::new("frac18;".to_string()), std::sync::Arc::new("\u{215b}".to_string()), std::sync::Arc::new("frac23;".to_string()), std::sync::Arc::new("\u{2154}".to_string()), std::sync::Arc::new("frac25;".to_string()), std::sync::Arc::new("\u{2156}".to_string()), std::sync::Arc::new("frac34".to_string()), std::sync::Arc::new("\u{be}".to_string()), std::sync::Arc::new("frac34;".to_string()), std::sync::Arc::new("\u{be}".to_string()), std::sync::Arc::new("frac35;".to_string()), std::sync::Arc::new("\u{2157}".to_string()), std::sync::Arc::new("frac38;".to_string()), std::sync::Arc::new("\u{215c}".to_string()), std::sync::Arc::new("frac45;".to_string()), std::sync::Arc::new("\u{2158}".to_string()), std::sync::Arc::new("frac56;".to_string()), std::sync::Arc::new("\u{215a}".to_string()), std::sync::Arc::new("frac58;".to_string()), std::sync::Arc::new("\u{215d}".to_string()), std::sync::Arc::new("frac78;".to_string()), std::sync::Arc::new("\u{215e}".to_string()), std::sync::Arc::new("frasl;".to_string()), std::sync::Arc::new("\u{2044}".to_string()), std::sync::Arc::new("frown;".to_string()), std::sync::Arc::new("\u{2322}".to_string()), std::sync::Arc::new("fscr;".to_string()), std::sync::Arc::new("𝒻".to_string()), std::sync::Arc::new("gE;".to_string()), std::sync::Arc::new("\u{2267}".to_string()), std::sync::Arc::new("gEl;".to_string()), std::sync::Arc::new("\u{2a8c}".to_string()), std::sync::Arc::new("gacute;".to_string()), std::sync::Arc::new("ǵ".to_string()), std::sync::Arc::new("gamma;".to_string()), std::sync::Arc::new("γ".to_string()), std::sync::Arc::new("gammad;".to_string()), std::sync::Arc::new("ϝ".to_string()), std::sync::Arc::new("gap;".to_string()), std::sync::Arc::new("\u{2a86}".to_string()), std::sync::Arc::new("gbreve;".to_string()), std::sync::Arc::new("ğ".to_string()), std::sync::Arc::new("gcirc;".to_string()), std::sync::Arc::new("ĝ".to_string()), std::sync::Arc::new("gcy;".to_string()), std::sync::Arc::new("г".to_string()), std::sync::Arc::new("gdot;".to_string()), std::sync::Arc::new("ġ".to_string()), std::sync::Arc::new("ge;".to_string()), std::sync::Arc::new("\u{2265}".to_string()), std::sync::Arc::new("gel;".to_string()), std::sync::Arc::new("\u{22db}".to_string()), std::sync::Arc::new("geq;".to_string()), std::sync::Arc::new("\u{2265}".to_string()), std::sync::Arc::new("geqq;".to_string()), std::sync::Arc::new("\u{2267}".to_string()), std::sync::Arc::new("geqslant;".to_string()), std::sync::Arc::new("\u{2a7e}".to_string()), std::sync::Arc::new("ges;".to_string()), std::sync::Arc::new("\u{2a7e}".to_string()), std::sync::Arc::new("gescc;".to_string()), std::sync::Arc::new("\u{2aa9}".to_string()), std::sync::Arc::new("gesdot;".to_string()), std::sync::Arc::new("\u{2a80}".to_string()), std::sync::Arc::new("gesdoto;".to_string()), std::sync::Arc::new("\u{2a82}".to_string()), std::sync::Arc::new("gesdotol;".to_string()), std::sync::Arc::new("\u{2a84}".to_string()), std::sync::Arc::new("gesl;".to_string()), std::sync::Arc::new("\u{22db}\u{fe00}".to_string()), std::sync::Arc::new("gesles;".to_string()), std::sync::Arc::new("\u{2a94}".to_string()), std::sync::Arc::new("gfr;".to_string()), std::sync::Arc::new("𝔤".to_string()), std::sync::Arc::new("gg;".to_string()), std::sync::Arc::new("\u{226b}".to_string()), std::sync::Arc::new("ggg;".to_string()), std::sync::Arc::new("\u{22d9}".to_string()), std::sync::Arc::new("gimel;".to_string()), std::sync::Arc::new("ℷ".to_string()), std::sync::Arc::new("gjcy;".to_string()), std::sync::Arc::new("ѓ".to_string()), std::sync::Arc::new("gl;".to_string()), std::sync::Arc::new("\u{2277}".to_string()), std::sync::Arc::new("glE;".to_string()), std::sync::Arc::new("\u{2a92}".to_string()), std::sync::Arc::new("gla;".to_string()), std::sync::Arc::new("\u{2aa5}".to_string()), std::sync::Arc::new("glj;".to_string()), std::sync::Arc::new("\u{2aa4}".to_string()), std::sync::Arc::new("gnE;".to_string()), std::sync::Arc::new("\u{2269}".to_string()), std::sync::Arc::new("gnap;".to_string()), std::sync::Arc::new("\u{2a8a}".to_string()), std::sync::Arc::new("gnapprox;".to_string()), std::sync::Arc::new("\u{2a8a}".to_string()), std::sync::Arc::new("gne;".to_string()), std::sync::Arc::new("\u{2a88}".to_string()), std::sync::Arc::new("gneq;".to_string()), std::sync::Arc::new("\u{2a88}".to_string()), std::sync::Arc::new("gneqq;".to_string()), std::sync::Arc::new("\u{2269}".to_string()), std::sync::Arc::new("gnsim;".to_string()), std::sync::Arc::new("\u{22e7}".to_string()), std::sync::Arc::new("gopf;".to_string()), std::sync::Arc::new("𝕘".to_string()), std::sync::Arc::new("grave;".to_string()), std::sync::Arc::new("`".to_string()), std::sync::Arc::new("gscr;".to_string()), std::sync::Arc::new("ℊ".to_string()), std::sync::Arc::new("gsim;".to_string()), std::sync::Arc::new("\u{2273}".to_string()), std::sync::Arc::new("gsime;".to_string()), std::sync::Arc::new("\u{2a8e}".to_string()), std::sync::Arc::new("gsiml;".to_string()), std::sync::Arc::new("\u{2a90}".to_string()), std::sync::Arc::new("gt".to_string()), std::sync::Arc::new(">".to_string()), std::sync::Arc::new("gt;".to_string()), std::sync::Arc::new(">".to_string()), std::sync::Arc::new("gtcc;".to_string()), std::sync::Arc::new("\u{2aa7}".to_string()), std::sync::Arc::new("gtcir;".to_string()), std::sync::Arc::new("\u{2a7a}".to_string()), std::sync::Arc::new("gtdot;".to_string()), std::sync::Arc::new("\u{22d7}".to_string()), std::sync::Arc::new("gtlPar;".to_string()), std::sync::Arc::new("\u{2995}".to_string()), std::sync::Arc::new("gtquest;".to_string()), std::sync::Arc::new("\u{2a7c}".to_string()), std::sync::Arc::new("gtrapprox;".to_string()), std::sync::Arc::new("\u{2a86}".to_string()), std::sync::Arc::new("gtrarr;".to_string()), std::sync::Arc::new("\u{2978}".to_string()), std::sync::Arc::new("gtrdot;".to_string()), std::sync::Arc::new("\u{22d7}".to_string()), std::sync::Arc::new("gtreqless;".to_string()), std::sync::Arc::new("\u{22db}".to_string()), std::sync::Arc::new("gtreqqless;".to_string()), std::sync::Arc::new("\u{2a8c}".to_string()), std::sync::Arc::new("gtrless;".to_string()), std::sync::Arc::new("\u{2277}".to_string()), std::sync::Arc::new("gtrsim;".to_string()), std::sync::Arc::new("\u{2273}".to_string()), std::sync::Arc::new("gvertneqq;".to_string()), std::sync::Arc::new("\u{2269}\u{fe00}".to_string()), std::sync::Arc::new("gvnE;".to_string()), std::sync::Arc::new("\u{2269}\u{fe00}".to_string()), std::sync::Arc::new("hArr;".to_string()), std::sync::Arc::new("\u{21d4}".to_string()), std::sync::Arc::new("hairsp;".to_string()), std::sync::Arc::new("\u{200a}".to_string()), std::sync::Arc::new("half;".to_string()), std::sync::Arc::new("\u{bd}".to_string()), std::sync::Arc::new("hamilt;".to_string()), std::sync::Arc::new("ℋ".to_string()), std::sync::Arc::new("hardcy;".to_string()), std::sync::Arc::new("ъ".to_string()), std::sync::Arc::new("harr;".to_string()), std::sync::Arc::new("\u{2194}".to_string()), std::sync::Arc::new("harrcir;".to_string()), std::sync::Arc::new("\u{2948}".to_string()), std::sync::Arc::new("harrw;".to_string()), std::sync::Arc::new("\u{21ad}".to_string()), std::sync::Arc::new("hbar;".to_string()), std::sync::Arc::new("ℏ".to_string()), std::sync::Arc::new("hcirc;".to_string()), std::sync::Arc::new("ĥ".to_string()), std::sync::Arc::new("hearts;".to_string()), std::sync::Arc::new("\u{2665}".to_string()), std::sync::Arc::new("heartsuit;".to_string()), std::sync::Arc::new("\u{2665}".to_string()), std::sync::Arc::new("hellip;".to_string()), std::sync::Arc::new("\u{2026}".to_string()), std::sync::Arc::new("hercon;".to_string()), std::sync::Arc::new("\u{22b9}".to_string()), std::sync::Arc::new("hfr;".to_string()), std::sync::Arc::new("𝔥".to_string()), std::sync::Arc::new("hksearow;".to_string()), std::sync::Arc::new("\u{2925}".to_string()), std::sync::Arc::new("hkswarow;".to_string()), std::sync::Arc::new("\u{2926}".to_string()), std::sync::Arc::new("hoarr;".to_string()), std::sync::Arc::new("\u{21ff}".to_string()), std::sync::Arc::new("homtht;".to_string()), std::sync::Arc::new("\u{223b}".to_string()), std::sync::Arc::new("hookleftarrow;".to_string()), std::sync::Arc::new("\u{21a9}".to_string()), std::sync::Arc::new("hookrightarrow;".to_string()), std::sync::Arc::new("\u{21aa}".to_string()), std::sync::Arc::new("hopf;".to_string()), std::sync::Arc::new("𝕙".to_string()), std::sync::Arc::new("horbar;".to_string()), std::sync::Arc::new("\u{2015}".to_string()), std::sync::Arc::new("hscr;".to_string()), std::sync::Arc::new("𝒽".to_string()), std::sync::Arc::new("hslash;".to_string()), std::sync::Arc::new("ℏ".to_string()), std::sync::Arc::new("hstrok;".to_string()), std::sync::Arc::new("ħ".to_string()), std::sync::Arc::new("hybull;".to_string()), std::sync::Arc::new("\u{2043}".to_string()), std::sync::Arc::new("hyphen;".to_string()), std::sync::Arc::new("\u{2010}".to_string()), std::sync::Arc::new("iacute".to_string()), std::sync::Arc::new("í".to_string()), std::sync::Arc::new("iacute;".to_string()), std::sync::Arc::new("í".to_string()), std::sync::Arc::new("ic;".to_string()), std::sync::Arc::new("\u{2063}".to_string()), std::sync::Arc::new("icirc".to_string()), std::sync::Arc::new("î".to_string()), std::sync::Arc::new("icirc;".to_string()), std::sync::Arc::new("î".to_string()), std::sync::Arc::new("icy;".to_string()), std::sync::Arc::new("и".to_string()), std::sync::Arc::new("iecy;".to_string()), std::sync::Arc::new("е".to_string()), std::sync::Arc::new("iexcl".to_string()), std::sync::Arc::new("\u{a1}".to_string()), std::sync::Arc::new("iexcl;".to_string()), std::sync::Arc::new("\u{a1}".to_string()), std::sync::Arc::new("iff;".to_string()), std::sync::Arc::new("\u{21d4}".to_string()), std::sync::Arc::new("ifr;".to_string()), std::sync::Arc::new("𝔦".to_string()), std::sync::Arc::new("igrave".to_string()), std::sync::Arc::new("ì".to_string()), std::sync::Arc::new("igrave;".to_string()), std::sync::Arc::new("ì".to_string()), std::sync::Arc::new("ii;".to_string()), std::sync::Arc::new("ⅈ".to_string()), std::sync::Arc::new("iiiint;".to_string()), std::sync::Arc::new("\u{2a0c}".to_string()), std::sync::Arc::new("iiint;".to_string()), std::sync::Arc::new("\u{222d}".to_string()), std::sync::Arc::new("iinfin;".to_string()), std::sync::Arc::new("\u{29dc}".to_string()), std::sync::Arc::new("iiota;".to_string()), std::sync::Arc::new("\u{2129}".to_string()), std::sync::Arc::new("ijlig;".to_string()), std::sync::Arc::new("ĳ".to_string()), std::sync::Arc::new("imacr;".to_string()), std::sync::Arc::new("ī".to_string()), std::sync::Arc::new("image;".to_string()), std::sync::Arc::new("ℑ".to_string()), std::sync::Arc::new("imagline;".to_string()), std::sync::Arc::new("ℐ".to_string()), std::sync::Arc::new("imagpart;".to_string()), std::sync::Arc::new("ℑ".to_string()), std::sync::Arc::new("imath;".to_string()), std::sync::Arc::new("ı".to_string()), std::sync::Arc::new("imof;".to_string()), std::sync::Arc::new("\u{22b7}".to_string()), std::sync::Arc::new("imped;".to_string()), std::sync::Arc::new("Ƶ".to_string()), std::sync::Arc::new("in;".to_string()), std::sync::Arc::new("\u{2208}".to_string()), std::sync::Arc::new("incare;".to_string()), std::sync::Arc::new("\u{2105}".to_string()), std::sync::Arc::new("infin;".to_string()), std::sync::Arc::new("\u{221e}".to_string()), std::sync::Arc::new("infintie;".to_string()), std::sync::Arc::new("\u{29dd}".to_string()), std::sync::Arc::new("inodot;".to_string()), std::sync::Arc::new("ı".to_string()), std::sync::Arc::new("int;".to_string()), std::sync::Arc::new("\u{222b}".to_string()), std::sync::Arc::new("intcal;".to_string()), std::sync::Arc::new("\u{22ba}".to_string()), std::sync::Arc::new("integers;".to_string()), std::sync::Arc::new("ℤ".to_string()), std::sync::Arc::new("intercal;".to_string()), std::sync::Arc::new("\u{22ba}".to_string()), std::sync::Arc::new("intlarhk;".to_string()), std::sync::Arc::new("\u{2a17}".to_string()), std::sync::Arc::new("intprod;".to_string()), std::sync::Arc::new("\u{2a3c}".to_string()), std::sync::Arc::new("iocy;".to_string()), std::sync::Arc::new("ё".to_string()), std::sync::Arc::new("iogon;".to_string()), std::sync::Arc::new("į".to_string()), std::sync::Arc::new("iopf;".to_string()), std::sync::Arc::new("𝕚".to_string()), std::sync::Arc::new("iota;".to_string()), std::sync::Arc::new("ι".to_string()), std::sync::Arc::new("iprod;".to_string()), std::sync::Arc::new("\u{2a3c}".to_string()), std::sync::Arc::new("iquest".to_string()), std::sync::Arc::new("\u{bf}".to_string()), std::sync::Arc::new("iquest;".to_string()), std::sync::Arc::new("\u{bf}".to_string()), std::sync::Arc::new("iscr;".to_string()), std::sync::Arc::new("𝒾".to_string()), std::sync::Arc::new("isin;".to_string()), std::sync::Arc::new("\u{2208}".to_string()), std::sync::Arc::new("isinE;".to_string()), std::sync::Arc::new("\u{22f9}".to_string()), std::sync::Arc::new("isindot;".to_string()), std::sync::Arc::new("\u{22f5}".to_string()), std::sync::Arc::new("isins;".to_string()), std::sync::Arc::new("\u{22f4}".to_string()), std::sync::Arc::new("isinsv;".to_string()), std::sync::Arc::new("\u{22f3}".to_string()), std::sync::Arc::new("isinv;".to_string()), std::sync::Arc::new("\u{2208}".to_string()), std::sync::Arc::new("it;".to_string()), std::sync::Arc::new("\u{2062}".to_string()), std::sync::Arc::new("itilde;".to_string()), std::sync::Arc::new("ĩ".to_string()), std::sync::Arc::new("iukcy;".to_string()), std::sync::Arc::new("і".to_string()), std::sync::Arc::new("iuml".to_string()), std::sync::Arc::new("ï".to_string()), std::sync::Arc::new("iuml;".to_string()), std::sync::Arc::new("ï".to_string()), std::sync::Arc::new("jcirc;".to_string()), std::sync::Arc::new("ĵ".to_string()), std::sync::Arc::new("jcy;".to_string()), std::sync::Arc::new("й".to_string()), std::sync::Arc::new("jfr;".to_string()), std::sync::Arc::new("𝔧".to_string()), std::sync::Arc::new("jmath;".to_string()), std::sync::Arc::new("ȷ".to_string()), std::sync::Arc::new("jopf;".to_string()), std::sync::Arc::new("𝕛".to_string()), std::sync::Arc::new("jscr;".to_string()), std::sync::Arc::new("𝒿".to_string()), std::sync::Arc::new("jsercy;".to_string()), std::sync::Arc::new("ј".to_string()), std::sync::Arc::new("jukcy;".to_string()), std::sync::Arc::new("є".to_string()), std::sync::Arc::new("kappa;".to_string()), std::sync::Arc::new("κ".to_string()), std::sync::Arc::new("kappav;".to_string()), std::sync::Arc::new("ϰ".to_string()), std::sync::Arc::new("kcedil;".to_string()), std::sync::Arc::new("ķ".to_string()), std::sync::Arc::new("kcy;".to_string()), std::sync::Arc::new("к".to_string()), std::sync::Arc::new("kfr;".to_string()), std::sync::Arc::new("𝔨".to_string()), std::sync::Arc::new("kgreen;".to_string()), std::sync::Arc::new("ĸ".to_string()), std::sync::Arc::new("khcy;".to_string()), std::sync::Arc::new("х".to_string()), std::sync::Arc::new("kjcy;".to_string()), std::sync::Arc::new("ќ".to_string()), std::sync::Arc::new("kopf;".to_string()), std::sync::Arc::new("𝕜".to_string()), std::sync::Arc::new("kscr;".to_string()), std::sync::Arc::new("𝓀".to_string()), std::sync::Arc::new("lAarr;".to_string()), std::sync::Arc::new("\u{21da}".to_string()), std::sync::Arc::new("lArr;".to_string()), std::sync::Arc::new("\u{21d0}".to_string()), std::sync::Arc::new("lAtail;".to_string()), std::sync::Arc::new("\u{291b}".to_string()), std::sync::Arc::new("lBarr;".to_string()), std::sync::Arc::new("\u{290e}".to_string()), std::sync::Arc::new("lE;".to_string()), std::sync::Arc::new("\u{2266}".to_string()), std::sync::Arc::new("lEg;".to_string()), std::sync::Arc::new("\u{2a8b}".to_string()), std::sync::Arc::new("lHar;".to_string()), std::sync::Arc::new("\u{2962}".to_string()), std::sync::Arc::new("lacute;".to_string()), std::sync::Arc::new("ĺ".to_string()), std::sync::Arc::new("laemptyv;".to_string()), std::sync::Arc::new("\u{29b4}".to_string()), std::sync::Arc::new("lagran;".to_string()), std::sync::Arc::new("ℒ".to_string()), std::sync::Arc::new("lambda;".to_string()), std::sync::Arc::new("λ".to_string()), std::sync::Arc::new("lang;".to_string()), std::sync::Arc::new("\u{27e8}".to_string()), std::sync::Arc::new("langd;".to_string()), std::sync::Arc::new("\u{2991}".to_string()), std::sync::Arc::new("langle;".to_string()), std::sync::Arc::new("\u{27e8}".to_string()), std::sync::Arc::new("lap;".to_string()), std::sync::Arc::new("\u{2a85}".to_string()), std::sync::Arc::new("laquo".to_string()), std::sync::Arc::new("\u{ab}".to_string()), std::sync::Arc::new("laquo;".to_string()), std::sync::Arc::new("\u{ab}".to_string()), std::sync::Arc::new("larr;".to_string()), std::sync::Arc::new("\u{2190}".to_string()), std::sync::Arc::new("larrb;".to_string()), std::sync::Arc::new("\u{21e4}".to_string()), std::sync::Arc::new("larrbfs;".to_string()), std::sync::Arc::new("\u{291f}".to_string()), std::sync::Arc::new("larrfs;".to_string()), std::sync::Arc::new("\u{291d}".to_string()), std::sync::Arc::new("larrhk;".to_string()), std::sync::Arc::new("\u{21a9}".to_string()), std::sync::Arc::new("larrlp;".to_string()), std::sync::Arc::new("\u{21ab}".to_string()), std::sync::Arc::new("larrpl;".to_string()), std::sync::Arc::new("\u{2939}".to_string()), std::sync::Arc::new("larrsim;".to_string()), std::sync::Arc::new("\u{2973}".to_string()), std::sync::Arc::new("larrtl;".to_string()), std::sync::Arc::new("\u{21a2}".to_string()), std::sync::Arc::new("lat;".to_string()), std::sync::Arc::new("\u{2aab}".to_string()), std::sync::Arc::new("latail;".to_string()), std::sync::Arc::new("\u{2919}".to_string()), std::sync::Arc::new("late;".to_string()), std::sync::Arc::new("\u{2aad}".to_string()), std::sync::Arc::new("lates;".to_string()), std::sync::Arc::new("\u{2aad}\u{fe00}".to_string()), std::sync::Arc::new("lbarr;".to_string()), std::sync::Arc::new("\u{290c}".to_string()), std::sync::Arc::new("lbbrk;".to_string()), std::sync::Arc::new("\u{2772}".to_string()), std::sync::Arc::new("lbrace;".to_string()), std::sync::Arc::new("{".to_string()), std::sync::Arc::new("lbrack;".to_string()), std::sync::Arc::new("[".to_string()), std::sync::Arc::new("lbrke;".to_string()), std::sync::Arc::new("\u{298b}".to_string()), std::sync::Arc::new("lbrksld;".to_string()), std::sync::Arc::new("\u{298f}".to_string()), std::sync::Arc::new("lbrkslu;".to_string()), std::sync::Arc::new("\u{298d}".to_string()), std::sync::Arc::new("lcaron;".to_string()), std::sync::Arc::new("ľ".to_string()), std::sync::Arc::new("lcedil;".to_string()), std::sync::Arc::new("ļ".to_string()), std::sync::Arc::new("lceil;".to_string()), std::sync::Arc::new("\u{2308}".to_string()), std::sync::Arc::new("lcub;".to_string()), std::sync::Arc::new("{".to_string()), std::sync::Arc::new("lcy;".to_string()), std::sync::Arc::new("л".to_string()), std::sync::Arc::new("ldca;".to_string()), std::sync::Arc::new("\u{2936}".to_string()), std::sync::Arc::new("ldquo;".to_string()), std::sync::Arc::new("\u{201c}".to_string()), std::sync::Arc::new("ldquor;".to_string()), std::sync::Arc::new("\u{201e}".to_string()), std::sync::Arc::new("ldrdhar;".to_string()), std::sync::Arc::new("\u{2967}".to_string()), std::sync::Arc::new("ldrushar;".to_string()), std::sync::Arc::new("\u{294b}".to_string()), std::sync::Arc::new("ldsh;".to_string()), std::sync::Arc::new("\u{21b2}".to_string()), std::sync::Arc::new("le;".to_string()), std::sync::Arc::new("\u{2264}".to_string()), std::sync::Arc::new("leftarrow;".to_string()), std::sync::Arc::new("\u{2190}".to_string()), std::sync::Arc::new("leftarrowtail;".to_string()), std::sync::Arc::new("\u{21a2}".to_string()), std::sync::Arc::new("leftharpoondown;".to_string()), std::sync::Arc::new("\u{21bd}".to_string()), std::sync::Arc::new("leftharpoonup;".to_string()), std::sync::Arc::new("\u{21bc}".to_string()), std::sync::Arc::new("leftleftarrows;".to_string()), std::sync::Arc::new("\u{21c7}".to_string()), std::sync::Arc::new("leftrightarrow;".to_string()), std::sync::Arc::new("\u{2194}".to_string()), std::sync::Arc::new("leftrightarrows;".to_string()), std::sync::Arc::new("\u{21c6}".to_string()), std::sync::Arc::new("leftrightharpoons;".to_string()), std::sync::Arc::new("\u{21cb}".to_string()), std::sync::Arc::new("leftrightsquigarrow;".to_string()), std::sync::Arc::new("\u{21ad}".to_string()), std::sync::Arc::new("leftthreetimes;".to_string()), std::sync::Arc::new("\u{22cb}".to_string()), std::sync::Arc::new("leg;".to_string()), std::sync::Arc::new("\u{22da}".to_string()), std::sync::Arc::new("leq;".to_string()), std::sync::Arc::new("\u{2264}".to_string()), std::sync::Arc::new("leqq;".to_string()), std::sync::Arc::new("\u{2266}".to_string()), std::sync::Arc::new("leqslant;".to_string()), std::sync::Arc::new("\u{2a7d}".to_string()), std::sync::Arc::new("les;".to_string()), std::sync::Arc::new("\u{2a7d}".to_string()), std::sync::Arc::new("lescc;".to_string()), std::sync::Arc::new("\u{2aa8}".to_string()), std::sync::Arc::new("lesdot;".to_string()), std::sync::Arc::new("\u{2a7f}".to_string()), std::sync::Arc::new("lesdoto;".to_string()), std::sync::Arc::new("\u{2a81}".to_string()), std::sync::Arc::new("lesdotor;".to_string()), std::sync::Arc::new("\u{2a83}".to_string()), std::sync::Arc::new("lesg;".to_string()), std::sync::Arc::new("\u{22da}\u{fe00}".to_string()), std::sync::Arc::new("lesges;".to_string()), std::sync::Arc::new("\u{2a93}".to_string()), std::sync::Arc::new("lessapprox;".to_string()), std::sync::Arc::new("\u{2a85}".to_string()), std::sync::Arc::new("lessdot;".to_string()), std::sync::Arc::new("\u{22d6}".to_string()), std::sync::Arc::new("lesseqgtr;".to_string()), std::sync::Arc::new("\u{22da}".to_string()), std::sync::Arc::new("lesseqqgtr;".to_string()), std::sync::Arc::new("\u{2a8b}".to_string()), std::sync::Arc::new("lessgtr;".to_string()), std::sync::Arc::new("\u{2276}".to_string()), std::sync::Arc::new("lesssim;".to_string()), std::sync::Arc::new("\u{2272}".to_string()), std::sync::Arc::new("lfisht;".to_string()), std::sync::Arc::new("\u{297c}".to_string()), std::sync::Arc::new("lfloor;".to_string()), std::sync::Arc::new("\u{230a}".to_string()), std::sync::Arc::new("lfr;".to_string()), std::sync::Arc::new("𝔩".to_string()), std::sync::Arc::new("lg;".to_string()), std::sync::Arc::new("\u{2276}".to_string()), std::sync::Arc::new("lgE;".to_string()), std::sync::Arc::new("\u{2a91}".to_string()), std::sync::Arc::new("lhard;".to_string()), std::sync::Arc::new("\u{21bd}".to_string()), std::sync::Arc::new("lharu;".to_string()), std::sync::Arc::new("\u{21bc}".to_string()), std::sync::Arc::new("lharul;".to_string()), std::sync::Arc::new("\u{296a}".to_string()), std::sync::Arc::new("lhblk;".to_string()), std::sync::Arc::new("\u{2584}".to_string()), std::sync::Arc::new("ljcy;".to_string()), std::sync::Arc::new("љ".to_string()), std::sync::Arc::new("ll;".to_string()), std::sync::Arc::new("\u{226a}".to_string()), std::sync::Arc::new("llarr;".to_string()), std::sync::Arc::new("\u{21c7}".to_string()), std::sync::Arc::new("llcorner;".to_string()), std::sync::Arc::new("\u{231e}".to_string()), std::sync::Arc::new("llhard;".to_string()), std::sync::Arc::new("\u{296b}".to_string()), std::sync::Arc::new("lltri;".to_string()), std::sync::Arc::new("\u{25fa}".to_string()), std::sync::Arc::new("lmidot;".to_string()), std::sync::Arc::new("ŀ".to_string()), std::sync::Arc::new("lmoust;".to_string()), std::sync::Arc::new("\u{23b0}".to_string()), std::sync::Arc::new("lmoustache;".to_string()), std::sync::Arc::new("\u{23b0}".to_string()), std::sync::Arc::new("lnE;".to_string()), std::sync::Arc::new("\u{2268}".to_string()), std::sync::Arc::new("lnap;".to_string()), std::sync::Arc::new("\u{2a89}".to_string()), std::sync::Arc::new("lnapprox;".to_string()), std::sync::Arc::new("\u{2a89}".to_string()), std::sync::Arc::new("lne;".to_string()), std::sync::Arc::new("\u{2a87}".to_string()), std::sync::Arc::new("lneq;".to_string()), std::sync::Arc::new("\u{2a87}".to_string()), std::sync::Arc::new("lneqq;".to_string()), std::sync::Arc::new("\u{2268}".to_string()), std::sync::Arc::new("lnsim;".to_string()), std::sync::Arc::new("\u{22e6}".to_string()), std::sync::Arc::new("loang;".to_string()), std::sync::Arc::new("\u{27ec}".to_string()), std::sync::Arc::new("loarr;".to_string()), std::sync::Arc::new("\u{21fd}".to_string()), std::sync::Arc::new("lobrk;".to_string()), std::sync::Arc::new("\u{27e6}".to_string()), std::sync::Arc::new("longleftarrow;".to_string()), std::sync::Arc::new("\u{27f5}".to_string()), std::sync::Arc::new("longleftrightarrow;".to_string()), std::sync::Arc::new("\u{27f7}".to_string()), std::sync::Arc::new("longmapsto;".to_string()), std::sync::Arc::new("\u{27fc}".to_string()), std::sync::Arc::new("longrightarrow;".to_string()), std::sync::Arc::new("\u{27f6}".to_string()), std::sync::Arc::new("looparrowleft;".to_string()), std::sync::Arc::new("\u{21ab}".to_string()), std::sync::Arc::new("looparrowright;".to_string()), std::sync::Arc::new("\u{21ac}".to_string()), std::sync::Arc::new("lopar;".to_string()), std::sync::Arc::new("\u{2985}".to_string()), std::sync::Arc::new("lopf;".to_string()), std::sync::Arc::new("𝕝".to_string()), std::sync::Arc::new("loplus;".to_string()), std::sync::Arc::new("\u{2a2d}".to_string()), std::sync::Arc::new("lotimes;".to_string()), std::sync::Arc::new("\u{2a34}".to_string()), std::sync::Arc::new("lowast;".to_string()), std::sync::Arc::new("\u{2217}".to_string()), std::sync::Arc::new("lowbar;".to_string()), std::sync::Arc::new("_".to_string()), std::sync::Arc::new("loz;".to_string()), std::sync::Arc::new("\u{25ca}".to_string()), std::sync::Arc::new("lozenge;".to_string()), std::sync::Arc::new("\u{25ca}".to_string()), std::sync::Arc::new("lozf;".to_string()), std::sync::Arc::new("\u{29eb}".to_string()), std::sync::Arc::new("lpar;".to_string()), std::sync::Arc::new("(".to_string()), std::sync::Arc::new("lparlt;".to_string()), std::sync::Arc::new("\u{2993}".to_string()), std::sync::Arc::new("lrarr;".to_string()), std::sync::Arc::new("\u{21c6}".to_string()), std::sync::Arc::new("lrcorner;".to_string()), std::sync::Arc::new("\u{231f}".to_string()), std::sync::Arc::new("lrhar;".to_string()), std::sync::Arc::new("\u{21cb}".to_string()), std::sync::Arc::new("lrhard;".to_string()), std::sync::Arc::new("\u{296d}".to_string()), std::sync::Arc::new("lrm;".to_string()), std::sync::Arc::new("\u{200e}".to_string()), std::sync::Arc::new("lrtri;".to_string()), std::sync::Arc::new("\u{22bf}".to_string()), std::sync::Arc::new("lsaquo;".to_string()), std::sync::Arc::new("\u{2039}".to_string()), std::sync::Arc::new("lscr;".to_string()), std::sync::Arc::new("𝓁".to_string()), std::sync::Arc::new("lsh;".to_string()), std::sync::Arc::new("\u{21b0}".to_string()), std::sync::Arc::new("lsim;".to_string()), std::sync::Arc::new("\u{2272}".to_string()), std::sync::Arc::new("lsime;".to_string()), std::sync::Arc::new("\u{2a8d}".to_string()), std::sync::Arc::new("lsimg;".to_string()), std::sync::Arc::new("\u{2a8f}".to_string()), std::sync::Arc::new("lsqb;".to_string()), std::sync::Arc::new("[".to_string()), std::sync::Arc::new("lsquo;".to_string()), std::sync::Arc::new("\u{2018}".to_string()), std::sync::Arc::new("lsquor;".to_string()), std::sync::Arc::new("\u{201a}".to_string()), std::sync::Arc::new("lstrok;".to_string()), std::sync::Arc::new("ł".to_string()), std::sync::Arc::new("lt".to_string()), std::sync::Arc::new("<".to_string()), std::sync::Arc::new("lt;".to_string()), std::sync::Arc::new("<".to_string()), std::sync::Arc::new("ltcc;".to_string()), std::sync::Arc::new("\u{2aa6}".to_string()), std::sync::Arc::new("ltcir;".to_string()), std::sync::Arc::new("\u{2a79}".to_string()), std::sync::Arc::new("ltdot;".to_string()), std::sync::Arc::new("\u{22d6}".to_string()), std::sync::Arc::new("lthree;".to_string()), std::sync::Arc::new("\u{22cb}".to_string()), std::sync::Arc::new("ltimes;".to_string()), std::sync::Arc::new("\u{22c9}".to_string()), std::sync::Arc::new("ltlarr;".to_string()), std::sync::Arc::new("\u{2976}".to_string()), std::sync::Arc::new("ltquest;".to_string()), std::sync::Arc::new("\u{2a7b}".to_string()), std::sync::Arc::new("ltrPar;".to_string()), std::sync::Arc::new("\u{2996}".to_string()), std::sync::Arc::new("ltri;".to_string()), std::sync::Arc::new("\u{25c3}".to_string()), std::sync::Arc::new("ltrie;".to_string()), std::sync::Arc::new("\u{22b4}".to_string()), std::sync::Arc::new("ltrif;".to_string()), std::sync::Arc::new("\u{25c2}".to_string()), std::sync::Arc::new("lurdshar;".to_string()), std::sync::Arc::new("\u{294a}".to_string()), std::sync::Arc::new("luruhar;".to_string()), std::sync::Arc::new("\u{2966}".to_string()), std::sync::Arc::new("lvertneqq;".to_string()), std::sync::Arc::new("\u{2268}\u{fe00}".to_string()), std::sync::Arc::new("lvnE;".to_string()), std::sync::Arc::new("\u{2268}\u{fe00}".to_string()), std::sync::Arc::new("mDDot;".to_string()), std::sync::Arc::new("\u{223a}".to_string()), std::sync::Arc::new("macr".to_string()), std::sync::Arc::new("\u{af}".to_string()), std::sync::Arc::new("macr;".to_string()), std::sync::Arc::new("\u{af}".to_string()), std::sync::Arc::new("male;".to_string()), std::sync::Arc::new("\u{2642}".to_string()), std::sync::Arc::new("malt;".to_string()), std::sync::Arc::new("\u{2720}".to_string()), std::sync::Arc::new("maltese;".to_string()), std::sync::Arc::new("\u{2720}".to_string()), std::sync::Arc::new("map;".to_string()), std::sync::Arc::new("\u{21a6}".to_string()), std::sync::Arc::new("mapsto;".to_string()), std::sync::Arc::new("\u{21a6}".to_string()), std::sync::Arc::new("mapstodown;".to_string()), std::sync::Arc::new("\u{21a7}".to_string()), std::sync::Arc::new("mapstoleft;".to_string()), std::sync::Arc::new("\u{21a4}".to_string()), std::sync::Arc::new("mapstoup;".to_string()), std::sync::Arc::new("\u{21a5}".to_string()), std::sync::Arc::new("marker;".to_string()), std::sync::Arc::new("\u{25ae}".to_string()), std::sync::Arc::new("mcomma;".to_string()), std::sync::Arc::new("\u{2a29}".to_string()), std::sync::Arc::new("mcy;".to_string()), std::sync::Arc::new("м".to_string()), std::sync::Arc::new("mdash;".to_string()), std::sync::Arc::new("\u{2014}".to_string()), std::sync::Arc::new("measuredangle;".to_string()), std::sync::Arc::new("\u{2221}".to_string()), std::sync::Arc::new("mfr;".to_string()), std::sync::Arc::new("𝔪".to_string()), std::sync::Arc::new("mho;".to_string()), std::sync::Arc::new("\u{2127}".to_string()), std::sync::Arc::new("micro".to_string()), std::sync::Arc::new("µ".to_string()), std::sync::Arc::new("micro;".to_string()), std::sync::Arc::new("µ".to_string()), std::sync::Arc::new("mid;".to_string()), std::sync::Arc::new("\u{2223}".to_string()), std::sync::Arc::new("midast;".to_string()), std::sync::Arc::new("*".to_string()), std::sync::Arc::new("midcir;".to_string()), std::sync::Arc::new("\u{2af0}".to_string()), std::sync::Arc::new("middot".to_string()), std::sync::Arc::new("\u{b7}".to_string()), std::sync::Arc::new("middot;".to_string()), std::sync::Arc::new("\u{b7}".to_string()), std::sync::Arc::new("minus;".to_string()), std::sync::Arc::new("\u{2212}".to_string()), std::sync::Arc::new("minusb;".to_string()), std::sync::Arc::new("\u{229f}".to_string()), std::sync::Arc::new("minusd;".to_string()), std::sync::Arc::new("\u{2238}".to_string()), std::sync::Arc::new("minusdu;".to_string()), std::sync::Arc::new("\u{2a2a}".to_string()), std::sync::Arc::new("mlcp;".to_string()), std::sync::Arc::new("\u{2adb}".to_string()), std::sync::Arc::new("mldr;".to_string()), std::sync::Arc::new("\u{2026}".to_string()), std::sync::Arc::new("mnplus;".to_string()), std::sync::Arc::new("\u{2213}".to_string()), std::sync::Arc::new("models;".to_string()), std::sync::Arc::new("\u{22a7}".to_string()), std::sync::Arc::new("mopf;".to_string()), std::sync::Arc::new("𝕞".to_string()), std::sync::Arc::new("mp;".to_string()), std::sync::Arc::new("\u{2213}".to_string()), std::sync::Arc::new("mscr;".to_string()), std::sync::Arc::new("𝓂".to_string()), std::sync::Arc::new("mstpos;".to_string()), std::sync::Arc::new("\u{223e}".to_string()), std::sync::Arc::new("mu;".to_string()), std::sync::Arc::new("μ".to_string()), std::sync::Arc::new("multimap;".to_string()), std::sync::Arc::new("\u{22b8}".to_string()), std::sync::Arc::new("mumap;".to_string()), std::sync::Arc::new("\u{22b8}".to_string()), std::sync::Arc::new("nGg;".to_string()), std::sync::Arc::new("\u{22d9}\u{338}".to_string()), std::sync::Arc::new("nGt;".to_string()), std::sync::Arc::new("\u{226b}\u{20d2}".to_string()), std::sync::Arc::new("nGtv;".to_string()), std::sync::Arc::new("\u{226b}\u{338}".to_string()), std::sync::Arc::new("nLeftarrow;".to_string()), std::sync::Arc::new("\u{21cd}".to_string()), std::sync::Arc::new("nLeftrightarrow;".to_string()), std::sync::Arc::new("\u{21ce}".to_string()), std::sync::Arc::new("nLl;".to_string()), std::sync::Arc::new("\u{22d8}\u{338}".to_string()), std::sync::Arc::new("nLt;".to_string()), std::sync::Arc::new("\u{226a}\u{20d2}".to_string()), std::sync::Arc::new("nLtv;".to_string()), std::sync::Arc::new("\u{226a}\u{338}".to_string()), std::sync::Arc::new("nRightarrow;".to_string()), std::sync::Arc::new("\u{21cf}".to_string()), std::sync::Arc::new("nVDash;".to_string()), std::sync::Arc::new("\u{22af}".to_string()), std::sync::Arc::new("nVdash;".to_string()), std::sync::Arc::new("\u{22ae}".to_string()), std::sync::Arc::new("nabla;".to_string()), std::sync::Arc::new("\u{2207}".to_string()), std::sync::Arc::new("nacute;".to_string()), std::sync::Arc::new("ń".to_string()), std::sync::Arc::new("nang;".to_string()), std::sync::Arc::new("\u{2220}\u{20d2}".to_string()), std::sync::Arc::new("nap;".to_string()), std::sync::Arc::new("\u{2249}".to_string()), std::sync::Arc::new("napE;".to_string()), std::sync::Arc::new("\u{2a70}\u{338}".to_string()), std::sync::Arc::new("napid;".to_string()), std::sync::Arc::new("\u{224b}\u{338}".to_string()), std::sync::Arc::new("napos;".to_string()), std::sync::Arc::new("ŉ".to_string()), std::sync::Arc::new("napprox;".to_string()), std::sync::Arc::new("\u{2249}".to_string()), std::sync::Arc::new("natur;".to_string()), std::sync::Arc::new("\u{266e}".to_string()), std::sync::Arc::new("natural;".to_string()), std::sync::Arc::new("\u{266e}".to_string()), std::sync::Arc::new("naturals;".to_string()), std::sync::Arc::new("ℕ".to_string()), std::sync::Arc::new("nbsp".to_string()), std::sync::Arc::new("\u{a0}".to_string()), std::sync::Arc::new("nbsp;".to_string()), std::sync::Arc::new("\u{a0}".to_string()), std::sync::Arc::new("nbump;".to_string()), std::sync::Arc::new("\u{224e}\u{338}".to_string()), std::sync::Arc::new("nbumpe;".to_string()), std::sync::Arc::new("\u{224f}\u{338}".to_string()), std::sync::Arc::new("ncap;".to_string()), std::sync::Arc::new("\u{2a43}".to_string()), std::sync::Arc::new("ncaron;".to_string()), std::sync::Arc::new("ň".to_string()), std::sync::Arc::new("ncedil;".to_string()), std::sync::Arc::new("ņ".to_string()), std::sync::Arc::new("ncong;".to_string()), std::sync::Arc::new("\u{2247}".to_string()), std::sync::Arc::new("ncongdot;".to_string()), std::sync::Arc::new("\u{2a6d}\u{338}".to_string()), std::sync::Arc::new("ncup;".to_string()), std::sync::Arc::new("\u{2a42}".to_string()), std::sync::Arc::new("ncy;".to_string()), std::sync::Arc::new("н".to_string()), std::sync::Arc::new("ndash;".to_string()), std::sync::Arc::new("\u{2013}".to_string()), std::sync::Arc::new("ne;".to_string()), std::sync::Arc::new("\u{2260}".to_string()), std::sync::Arc::new("neArr;".to_string()), std::sync::Arc::new("\u{21d7}".to_string()), std::sync::Arc::new("nearhk;".to_string()), std::sync::Arc::new("\u{2924}".to_string()), std::sync::Arc::new("nearr;".to_string()), std::sync::Arc::new("\u{2197}".to_string()), std::sync::Arc::new("nearrow;".to_string()), std::sync::Arc::new("\u{2197}".to_string()), std::sync::Arc::new("nedot;".to_string()), std::sync::Arc::new("\u{2250}\u{338}".to_string()), std::sync::Arc::new("nequiv;".to_string()), std::sync::Arc::new("\u{2262}".to_string()), std::sync::Arc::new("nesear;".to_string()), std::sync::Arc::new("\u{2928}".to_string()), std::sync::Arc::new("nesim;".to_string()), std::sync::Arc::new("\u{2242}\u{338}".to_string()), std::sync::Arc::new("nexist;".to_string()), std::sync::Arc::new("\u{2204}".to_string()), std::sync::Arc::new("nexists;".to_string()), std::sync::Arc::new("\u{2204}".to_string()), std::sync::Arc::new("nfr;".to_string()), std::sync::Arc::new("𝔫".to_string()), std::sync::Arc::new("ngE;".to_string()), std::sync::Arc::new("\u{2267}\u{338}".to_string()), std::sync::Arc::new("nge;".to_string()), std::sync::Arc::new("\u{2271}".to_string()), std::sync::Arc::new("ngeq;".to_string()), std::sync::Arc::new("\u{2271}".to_string()), std::sync::Arc::new("ngeqq;".to_string()), std::sync::Arc::new("\u{2267}\u{338}".to_string()), std::sync::Arc::new("ngeqslant;".to_string()), std::sync::Arc::new("\u{2a7e}\u{338}".to_string()), std::sync::Arc::new("nges;".to_string()), std::sync::Arc::new("\u{2a7e}\u{338}".to_string()), std::sync::Arc::new("ngsim;".to_string()), std::sync::Arc::new("\u{2275}".to_string()), std::sync::Arc::new("ngt;".to_string()), std::sync::Arc::new("\u{226f}".to_string()), std::sync::Arc::new("ngtr;".to_string()), std::sync::Arc::new("\u{226f}".to_string()), std::sync::Arc::new("nhArr;".to_string()), std::sync::Arc::new("\u{21ce}".to_string()), std::sync::Arc::new("nharr;".to_string()), std::sync::Arc::new("\u{21ae}".to_string()), std::sync::Arc::new("nhpar;".to_string()), std::sync::Arc::new("\u{2af2}".to_string()), std::sync::Arc::new("ni;".to_string()), std::sync::Arc::new("\u{220b}".to_string()), std::sync::Arc::new("nis;".to_string()), std::sync::Arc::new("\u{22fc}".to_string()), std::sync::Arc::new("nisd;".to_string()), std::sync::Arc::new("\u{22fa}".to_string()), std::sync::Arc::new("niv;".to_string()), std::sync::Arc::new("\u{220b}".to_string()), std::sync::Arc::new("njcy;".to_string()), std::sync::Arc::new("њ".to_string()), std::sync::Arc::new("nlArr;".to_string()), std::sync::Arc::new("\u{21cd}".to_string()), std::sync::Arc::new("nlE;".to_string()), std::sync::Arc::new("\u{2266}\u{338}".to_string()), std::sync::Arc::new("nlarr;".to_string()), std::sync::Arc::new("\u{219a}".to_string()), std::sync::Arc::new("nldr;".to_string()), std::sync::Arc::new("\u{2025}".to_string()), std::sync::Arc::new("nle;".to_string()), std::sync::Arc::new("\u{2270}".to_string()), std::sync::Arc::new("nleftarrow;".to_string()), std::sync::Arc::new("\u{219a}".to_string()), std::sync::Arc::new("nleftrightarrow;".to_string()), std::sync::Arc::new("\u{21ae}".to_string()), std::sync::Arc::new("nleq;".to_string()), std::sync::Arc::new("\u{2270}".to_string()), std::sync::Arc::new("nleqq;".to_string()), std::sync::Arc::new("\u{2266}\u{338}".to_string()), std::sync::Arc::new("nleqslant;".to_string()), std::sync::Arc::new("\u{2a7d}\u{338}".to_string()), std::sync::Arc::new("nles;".to_string()), std::sync::Arc::new("\u{2a7d}\u{338}".to_string()), std::sync::Arc::new("nless;".to_string()), std::sync::Arc::new("\u{226e}".to_string()), std::sync::Arc::new("nlsim;".to_string()), std::sync::Arc::new("\u{2274}".to_string()), std::sync::Arc::new("nlt;".to_string()), std::sync::Arc::new("\u{226e}".to_string()), std::sync::Arc::new("nltri;".to_string()), std::sync::Arc::new("\u{22ea}".to_string()), std::sync::Arc::new("nltrie;".to_string()), std::sync::Arc::new("\u{22ec}".to_string()), std::sync::Arc::new("nmid;".to_string()), std::sync::Arc::new("\u{2224}".to_string()), std::sync::Arc::new("nopf;".to_string()), std::sync::Arc::new("𝕟".to_string()), std::sync::Arc::new("not".to_string()), std::sync::Arc::new("\u{ac}".to_string()), std::sync::Arc::new("not;".to_string()), std::sync::Arc::new("\u{ac}".to_string()), std::sync::Arc::new("notin;".to_string()), std::sync::Arc::new("\u{2209}".to_string()), std::sync::Arc::new("notinE;".to_string()), std::sync::Arc::new("\u{22f9}\u{338}".to_string()), std::sync::Arc::new("notindot;".to_string()), std::sync::Arc::new("\u{22f5}\u{338}".to_string()), std::sync::Arc::new("notinva;".to_string()), std::sync::Arc::new("\u{2209}".to_string()), std::sync::Arc::new("notinvb;".to_string()), std::sync::Arc::new("\u{22f7}".to_string()), std::sync::Arc::new("notinvc;".to_string()), std::sync::Arc::new("\u{22f6}".to_string()), std::sync::Arc::new("notni;".to_string()), std::sync::Arc::new("\u{220c}".to_string()), std::sync::Arc::new("notniva;".to_string()), std::sync::Arc::new("\u{220c}".to_string()), std::sync::Arc::new("notnivb;".to_string()), std::sync::Arc::new("\u{22fe}".to_string()), std::sync::Arc::new("notnivc;".to_string()), std::sync::Arc::new("\u{22fd}".to_string()), std::sync::Arc::new("npar;".to_string()), std::sync::Arc::new("\u{2226}".to_string()), std::sync::Arc::new("nparallel;".to_string()), std::sync::Arc::new("\u{2226}".to_string()), std::sync::Arc::new("nparsl;".to_string()), std::sync::Arc::new("\u{2afd}\u{20e5}".to_string()), std::sync::Arc::new("npart;".to_string()), std::sync::Arc::new("\u{2202}\u{338}".to_string()), std::sync::Arc::new("npolint;".to_string()), std::sync::Arc::new("\u{2a14}".to_string()), std::sync::Arc::new("npr;".to_string()), std::sync::Arc::new("\u{2280}".to_string()), std::sync::Arc::new("nprcue;".to_string()), std::sync::Arc::new("\u{22e0}".to_string()), std::sync::Arc::new("npre;".to_string()), std::sync::Arc::new("\u{2aaf}\u{338}".to_string()), std::sync::Arc::new("nprec;".to_string()), std::sync::Arc::new("\u{2280}".to_string()), std::sync::Arc::new("npreceq;".to_string()), std::sync::Arc::new("\u{2aaf}\u{338}".to_string()), std::sync::Arc::new("nrArr;".to_string()), std::sync::Arc::new("\u{21cf}".to_string()), std::sync::Arc::new("nrarr;".to_string()), std::sync::Arc::new("\u{219b}".to_string()), std::sync::Arc::new("nrarrc;".to_string()), std::sync::Arc::new("\u{2933}\u{338}".to_string()), std::sync::Arc::new("nrarrw;".to_string()), std::sync::Arc::new("\u{219d}\u{338}".to_string()), std::sync::Arc::new("nrightarrow;".to_string()), std::sync::Arc::new("\u{219b}".to_string()), std::sync::Arc::new("nrtri;".to_string()), std::sync::Arc::new("\u{22eb}".to_string()), std::sync::Arc::new("nrtrie;".to_string()), std::sync::Arc::new("\u{22ed}".to_string()), std::sync::Arc::new("nsc;".to_string()), std::sync::Arc::new("\u{2281}".to_string()), std::sync::Arc::new("nsccue;".to_string()), std::sync::Arc::new("\u{22e1}".to_string()), std::sync::Arc::new("nsce;".to_string()), std::sync::Arc::new("\u{2ab0}\u{338}".to_string()), std::sync::Arc::new("nscr;".to_string()), std::sync::Arc::new("𝓃".to_string()), std::sync::Arc::new("nshortmid;".to_string()), std::sync::Arc::new("\u{2224}".to_string()), std::sync::Arc::new("nshortparallel;".to_string()), std::sync::Arc::new("\u{2226}".to_string()), std::sync::Arc::new("nsim;".to_string()), std::sync::Arc::new("\u{2241}".to_string()), std::sync::Arc::new("nsime;".to_string()), std::sync::Arc::new("\u{2244}".to_string()), std::sync::Arc::new("nsimeq;".to_string()), std::sync::Arc::new("\u{2244}".to_string()), std::sync::Arc::new("nsmid;".to_string()), std::sync::Arc::new("\u{2224}".to_string()), std::sync::Arc::new("nspar;".to_string()), std::sync::Arc::new("\u{2226}".to_string()), std::sync::Arc::new("nsqsube;".to_string()), std::sync::Arc::new("\u{22e2}".to_string()), std::sync::Arc::new("nsqsupe;".to_string()), std::sync::Arc::new("\u{22e3}".to_string()), std::sync::Arc::new("nsub;".to_string()), std::sync::Arc::new("\u{2284}".to_string()), std::sync::Arc::new("nsubE;".to_string()), std::sync::Arc::new("\u{2ac5}\u{338}".to_string()), std::sync::Arc::new("nsube;".to_string()), std::sync::Arc::new("\u{2288}".to_string()), std::sync::Arc::new("nsubset;".to_string()), std::sync::Arc::new("\u{2282}\u{20d2}".to_string()), std::sync::Arc::new("nsubseteq;".to_string()), std::sync::Arc::new("\u{2288}".to_string()), std::sync::Arc::new("nsubseteqq;".to_string()), std::sync::Arc::new("\u{2ac5}\u{338}".to_string()), std::sync::Arc::new("nsucc;".to_string()), std::sync::Arc::new("\u{2281}".to_string()), std::sync::Arc::new("nsucceq;".to_string()), std::sync::Arc::new("\u{2ab0}\u{338}".to_string()), std::sync::Arc::new("nsup;".to_string()), std::sync::Arc::new("\u{2285}".to_string()), std::sync::Arc::new("nsupE;".to_string()), std::sync::Arc::new("\u{2ac6}\u{338}".to_string()), std::sync::Arc::new("nsupe;".to_string()), std::sync::Arc::new("\u{2289}".to_string()), std::sync::Arc::new("nsupset;".to_string()), std::sync::Arc::new("\u{2283}\u{20d2}".to_string()), std::sync::Arc::new("nsupseteq;".to_string()), std::sync::Arc::new("\u{2289}".to_string()), std::sync::Arc::new("nsupseteqq;".to_string()), std::sync::Arc::new("\u{2ac6}\u{338}".to_string()), std::sync::Arc::new("ntgl;".to_string()), std::sync::Arc::new("\u{2279}".to_string()), std::sync::Arc::new("ntilde".to_string()), std::sync::Arc::new("ñ".to_string()), std::sync::Arc::new("ntilde;".to_string()), std::sync::Arc::new("ñ".to_string()), std::sync::Arc::new("ntlg;".to_string()), std::sync::Arc::new("\u{2278}".to_string()), std::sync::Arc::new("ntriangleleft;".to_string()), std::sync::Arc::new("\u{22ea}".to_string()), std::sync::Arc::new("ntrianglelefteq;".to_string()), std::sync::Arc::new("\u{22ec}".to_string()), std::sync::Arc::new("ntriangleright;".to_string()), std::sync::Arc::new("\u{22eb}".to_string()), std::sync::Arc::new("ntrianglerighteq;".to_string()), std::sync::Arc::new("\u{22ed}".to_string()), std::sync::Arc::new("nu;".to_string()), std::sync::Arc::new("ν".to_string()), std::sync::Arc::new("num;".to_string()), std::sync::Arc::new("#".to_string()), std::sync::Arc::new("numero;".to_string()), std::sync::Arc::new("\u{2116}".to_string()), std::sync::Arc::new("numsp;".to_string()), std::sync::Arc::new("\u{2007}".to_string()), std::sync::Arc::new("nvDash;".to_string()), std::sync::Arc::new("\u{22ad}".to_string()), std::sync::Arc::new("nvHarr;".to_string()), std::sync::Arc::new("\u{2904}".to_string()), std::sync::Arc::new("nvap;".to_string()), std::sync::Arc::new("\u{224d}\u{20d2}".to_string()), std::sync::Arc::new("nvdash;".to_string()), std::sync::Arc::new("\u{22ac}".to_string()), std::sync::Arc::new("nvge;".to_string()), std::sync::Arc::new("\u{2265}\u{20d2}".to_string()), std::sync::Arc::new("nvgt;".to_string()), std::sync::Arc::new(">\u{20d2}".to_string()), std::sync::Arc::new("nvinfin;".to_string()), std::sync::Arc::new("\u{29de}".to_string()), std::sync::Arc::new("nvlArr;".to_string()), std::sync::Arc::new("\u{2902}".to_string()), std::sync::Arc::new("nvle;".to_string()), std::sync::Arc::new("\u{2264}\u{20d2}".to_string()), std::sync::Arc::new("nvlt;".to_string()), std::sync::Arc::new("<\u{20d2}".to_string()), std::sync::Arc::new("nvltrie;".to_string()), std::sync::Arc::new("\u{22b4}\u{20d2}".to_string()), std::sync::Arc::new("nvrArr;".to_string()), std::sync::Arc::new("\u{2903}".to_string()), std::sync::Arc::new("nvrtrie;".to_string()), std::sync::Arc::new("\u{22b5}\u{20d2}".to_string()), std::sync::Arc::new("nvsim;".to_string()), std::sync::Arc::new("\u{223c}\u{20d2}".to_string()), std::sync::Arc::new("nwArr;".to_string()), std::sync::Arc::new("\u{21d6}".to_string()), std::sync::Arc::new("nwarhk;".to_string()), std::sync::Arc::new("\u{2923}".to_string()), std::sync::Arc::new("nwarr;".to_string()), std::sync::Arc::new("\u{2196}".to_string()), std::sync::Arc::new("nwarrow;".to_string()), std::sync::Arc::new("\u{2196}".to_string()), std::sync::Arc::new("nwnear;".to_string()), std::sync::Arc::new("\u{2927}".to_string()), std::sync::Arc::new("oS;".to_string()), std::sync::Arc::new("\u{24c8}".to_string()), std::sync::Arc::new("oacute".to_string()), std::sync::Arc::new("ó".to_string()), std::sync::Arc::new("oacute;".to_string()), std::sync::Arc::new("ó".to_string()), std::sync::Arc::new("oast;".to_string()), std::sync::Arc::new("\u{229b}".to_string()), std::sync::Arc::new("ocir;".to_string()), std::sync::Arc::new("\u{229a}".to_string()), std::sync::Arc::new("ocirc".to_string()), std::sync::Arc::new("ô".to_string()), std::sync::Arc::new("ocirc;".to_string()), std::sync::Arc::new("ô".to_string()), std::sync::Arc::new("ocy;".to_string()), std::sync::Arc::new("о".to_string()), std::sync::Arc::new("odash;".to_string()), std::sync::Arc::new("\u{229d}".to_string()), std::sync::Arc::new("odblac;".to_string()), std::sync::Arc::new("ő".to_string()), std::sync::Arc::new("odiv;".to_string()), std::sync::Arc::new("\u{2a38}".to_string()), std::sync::Arc::new("odot;".to_string()), std::sync::Arc::new("\u{2299}".to_string()), std::sync::Arc::new("odsold;".to_string()), std::sync::Arc::new("\u{29bc}".to_string()), std::sync::Arc::new("oelig;".to_string()), std::sync::Arc::new("œ".to_string()), std::sync::Arc::new("ofcir;".to_string()), std::sync::Arc::new("\u{29bf}".to_string()), std::sync::Arc::new("ofr;".to_string()), std::sync::Arc::new("𝔬".to_string()), std::sync::Arc::new("ogon;".to_string()), std::sync::Arc::new("\u{2db}".to_string()), std::sync::Arc::new("ograve".to_string()), std::sync::Arc::new("ò".to_string()), std::sync::Arc::new("ograve;".to_string()), std::sync::Arc::new("ò".to_string()), std::sync::Arc::new("ogt;".to_string()), std::sync::Arc::new("\u{29c1}".to_string()), std::sync::Arc::new("ohbar;".to_string()), std::sync::Arc::new("\u{29b5}".to_string()), std::sync::Arc::new("ohm;".to_string()), std::sync::Arc::new("Ω".to_string()), std::sync::Arc::new("oint;".to_string()), std::sync::Arc::new("\u{222e}".to_string()), std::sync::Arc::new("olarr;".to_string()), std::sync::Arc::new("\u{21ba}".to_string()), std::sync::Arc::new("olcir;".to_string()), std::sync::Arc::new("\u{29be}".to_string()), std::sync::Arc::new("olcross;".to_string()), std::sync::Arc::new("\u{29bb}".to_string()), std::sync::Arc::new("oline;".to_string()), std::sync::Arc::new("\u{203e}".to_string()), std::sync::Arc::new("olt;".to_string()), std::sync::Arc::new("\u{29c0}".to_string()), std::sync::Arc::new("omacr;".to_string()), std::sync::Arc::new("ō".to_string()), std::sync::Arc::new("omega;".to_string()), std::sync::Arc::new("ω".to_string()), std::sync::Arc::new("omicron;".to_string()), std::sync::Arc::new("ο".to_string()), std::sync::Arc::new("omid;".to_string()), std::sync::Arc::new("\u{29b6}".to_string()), std::sync::Arc::new("ominus;".to_string()), std::sync::Arc::new("\u{2296}".to_string()), std::sync::Arc::new("oopf;".to_string()), std::sync::Arc::new("𝕠".to_string()), std::sync::Arc::new("opar;".to_string()), std::sync::Arc::new("\u{29b7}".to_string()), std::sync::Arc::new("operp;".to_string()), std::sync::Arc::new("\u{29b9}".to_string()), std::sync::Arc::new("oplus;".to_string()), std::sync::Arc::new("\u{2295}".to_string()), std::sync::Arc::new("or;".to_string()), std::sync::Arc::new("\u{2228}".to_string()), std::sync::Arc::new("orarr;".to_string()), std::sync::Arc::new("\u{21bb}".to_string()), std::sync::Arc::new("ord;".to_string()), std::sync::Arc::new("\u{2a5d}".to_string()), std::sync::Arc::new("order;".to_string()), std::sync::Arc::new("ℴ".to_string()), std::sync::Arc::new("orderof;".to_string()), std::sync::Arc::new("ℴ".to_string()), std::sync::Arc::new("ordf".to_string()), std::sync::Arc::new("ª".to_string()), std::sync::Arc::new("ordf;".to_string()), std::sync::Arc::new("ª".to_string()), std::sync::Arc::new("ordm".to_string()), std::sync::Arc::new("º".to_string()), std::sync::Arc::new("ordm;".to_string()), std::sync::Arc::new("º".to_string()), std::sync::Arc::new("origof;".to_string()), std::sync::Arc::new("\u{22b6}".to_string()), std::sync::Arc::new("oror;".to_string()), std::sync::Arc::new("\u{2a56}".to_string()), std::sync::Arc::new("orslope;".to_string()), std::sync::Arc::new("\u{2a57}".to_string()), std::sync::Arc::new("orv;".to_string()), std::sync::Arc::new("\u{2a5b}".to_string()), std::sync::Arc::new("oscr;".to_string()), std::sync::Arc::new("ℴ".to_string()), std::sync::Arc::new("oslash".to_string()), std::sync::Arc::new("ø".to_string()), std::sync::Arc::new("oslash;".to_string()), std::sync::Arc::new("ø".to_string()), std::sync::Arc::new("osol;".to_string()), std::sync::Arc::new("\u{2298}".to_string()), std::sync::Arc::new("otilde".to_string()), std::sync::Arc::new("õ".to_string()), std::sync::Arc::new("otilde;".to_string()), std::sync::Arc::new("õ".to_string()), std::sync::Arc::new("otimes;".to_string()), std::sync::Arc::new("\u{2297}".to_string()), std::sync::Arc::new("otimesas;".to_string()), std::sync::Arc::new("\u{2a36}".to_string()), std::sync::Arc::new("ouml".to_string()), std::sync::Arc::new("ö".to_string()), std::sync::Arc::new("ouml;".to_string()), std::sync::Arc::new("ö".to_string()), std::sync::Arc::new("ovbar;".to_string()), std::sync::Arc::new("\u{233d}".to_string()), std::sync::Arc::new("par;".to_string()), std::sync::Arc::new("\u{2225}".to_string()), std::sync::Arc::new("para".to_string()), std::sync::Arc::new("\u{b6}".to_string()), std::sync::Arc::new("para;".to_string()), std::sync::Arc::new("\u{b6}".to_string()), std::sync::Arc::new("parallel;".to_string()), std::sync::Arc::new("\u{2225}".to_string()), std::sync::Arc::new("parsim;".to_string()), std::sync::Arc::new("\u{2af3}".to_string()), std::sync::Arc::new("parsl;".to_string()), std::sync::Arc::new("\u{2afd}".to_string()), std::sync::Arc::new("part;".to_string()), std::sync::Arc::new("\u{2202}".to_string()), std::sync::Arc::new("pcy;".to_string()), std::sync::Arc::new("п".to_string()), std::sync::Arc::new("percnt;".to_string()), std::sync::Arc::new("%".to_string()), std::sync::Arc::new("period;".to_string()), std::sync::Arc::new(".".to_string()), std::sync::Arc::new("permil;".to_string()), std::sync::Arc::new("\u{2030}".to_string()), std::sync::Arc::new("perp;".to_string()), std::sync::Arc::new("\u{22a5}".to_string()), std::sync::Arc::new("pertenk;".to_string()), std::sync::Arc::new("\u{2031}".to_string()), std::sync::Arc::new("pfr;".to_string()), std::sync::Arc::new("𝔭".to_string()), std::sync::Arc::new("phi;".to_string()), std::sync::Arc::new("φ".to_string()), std::sync::Arc::new("phiv;".to_string()), std::sync::Arc::new("ϕ".to_string()), std::sync::Arc::new("phmmat;".to_string()), std::sync::Arc::new("ℳ".to_string()), std::sync::Arc::new("phone;".to_string()), std::sync::Arc::new("\u{260e}".to_string()), std::sync::Arc::new("pi;".to_string()), std::sync::Arc::new("π".to_string()), std::sync::Arc::new("pitchfork;".to_string()), std::sync::Arc::new("\u{22d4}".to_string()), std::sync::Arc::new("piv;".to_string()), std::sync::Arc::new("ϖ".to_string()), std::sync::Arc::new("planck;".to_string()), std::sync::Arc::new("ℏ".to_string()), std::sync::Arc::new("planckh;".to_string()), std::sync::Arc::new("ℎ".to_string()), std::sync::Arc::new("plankv;".to_string()), std::sync::Arc::new("ℏ".to_string()), std::sync::Arc::new("plus;".to_string()), std::sync::Arc::new("+".to_string()), std::sync::Arc::new("plusacir;".to_string()), std::sync::Arc::new("\u{2a23}".to_string()), std::sync::Arc::new("plusb;".to_string()), std::sync::Arc::new("\u{229e}".to_string()), std::sync::Arc::new("pluscir;".to_string()), std::sync::Arc::new("\u{2a22}".to_string()), std::sync::Arc::new("plusdo;".to_string()), std::sync::Arc::new("\u{2214}".to_string()), std::sync::Arc::new("plusdu;".to_string()), std::sync::Arc::new("\u{2a25}".to_string()), std::sync::Arc::new("pluse;".to_string()), std::sync::Arc::new("\u{2a72}".to_string()), std::sync::Arc::new("plusmn".to_string()), std::sync::Arc::new("\u{b1}".to_string()), std::sync::Arc::new("plusmn;".to_string()), std::sync::Arc::new("\u{b1}".to_string()), std::sync::Arc::new("plussim;".to_string()), std::sync::Arc::new("\u{2a26}".to_string()), std::sync::Arc::new("plustwo;".to_string()), std::sync::Arc::new("\u{2a27}".to_string()), std::sync::Arc::new("pm;".to_string()), std::sync::Arc::new("\u{b1}".to_string()), std::sync::Arc::new("pointint;".to_string()), std::sync::Arc::new("\u{2a15}".to_string()), std::sync::Arc::new("popf;".to_string()), std::sync::Arc::new("𝕡".to_string()), std::sync::Arc::new("pound".to_string()), std::sync::Arc::new("\u{a3}".to_string()), std::sync::Arc::new("pound;".to_string()), std::sync::Arc::new("\u{a3}".to_string()), std::sync::Arc::new("pr;".to_string()), std::sync::Arc::new("\u{227a}".to_string()), std::sync::Arc::new("prE;".to_string()), std::sync::Arc::new("\u{2ab3}".to_string()), std::sync::Arc::new("prap;".to_string()), std::sync::Arc::new("\u{2ab7}".to_string()), std::sync::Arc::new("prcue;".to_string()), std::sync::Arc::new("\u{227c}".to_string()), std::sync::Arc::new("pre;".to_string()), std::sync::Arc::new("\u{2aaf}".to_string()), std::sync::Arc::new("prec;".to_string()), std::sync::Arc::new("\u{227a}".to_string()), std::sync::Arc::new("precapprox;".to_string()), std::sync::Arc::new("\u{2ab7}".to_string()), std::sync::Arc::new("preccurlyeq;".to_string()), std::sync::Arc::new("\u{227c}".to_string()), std::sync::Arc::new("preceq;".to_string()), std::sync::Arc::new("\u{2aaf}".to_string()), std::sync::Arc::new("precnapprox;".to_string()), std::sync::Arc::new("\u{2ab9}".to_string()), std::sync::Arc::new("precneqq;".to_string()), std::sync::Arc::new("\u{2ab5}".to_string()), std::sync::Arc::new("precnsim;".to_string()), std::sync::Arc::new("\u{22e8}".to_string()), std::sync::Arc::new("precsim;".to_string()), std::sync::Arc::new("\u{227e}".to_string()), std::sync::Arc::new("prime;".to_string()), std::sync::Arc::new("\u{2032}".to_string()), std::sync::Arc::new("primes;".to_string()), std::sync::Arc::new("ℙ".to_string()), std::sync::Arc::new("prnE;".to_string()), std::sync::Arc::new("\u{2ab5}".to_string()), std::sync::Arc::new("prnap;".to_string()), std::sync::Arc::new("\u{2ab9}".to_string()), std::sync::Arc::new("prnsim;".to_string()), std::sync::Arc::new("\u{22e8}".to_string()), std::sync::Arc::new("prod;".to_string()), std::sync::Arc::new("\u{220f}".to_string()), std::sync::Arc::new("profalar;".to_string()), std::sync::Arc::new("\u{232e}".to_string()), std::sync::Arc::new("profline;".to_string()), std::sync::Arc::new("\u{2312}".to_string()), std::sync::Arc::new("profsurf;".to_string()), std::sync::Arc::new("\u{2313}".to_string()), std::sync::Arc::new("prop;".to_string()), std::sync::Arc::new("\u{221d}".to_string()), std::sync::Arc::new("propto;".to_string()), std::sync::Arc::new("\u{221d}".to_string()), std::sync::Arc::new("prsim;".to_string()), std::sync::Arc::new("\u{227e}".to_string()), std::sync::Arc::new("prurel;".to_string()), std::sync::Arc::new("\u{22b0}".to_string()), std::sync::Arc::new("pscr;".to_string()), std::sync::Arc::new("𝓅".to_string()), std::sync::Arc::new("psi;".to_string()), std::sync::Arc::new("ψ".to_string()), std::sync::Arc::new("puncsp;".to_string()), std::sync::Arc::new("\u{2008}".to_string()), std::sync::Arc::new("qfr;".to_string()), std::sync::Arc::new("𝔮".to_string()), std::sync::Arc::new("qint;".to_string()), std::sync::Arc::new("\u{2a0c}".to_string()), std::sync::Arc::new("qopf;".to_string()), std::sync::Arc::new("𝕢".to_string()), std::sync::Arc::new("qprime;".to_string()), std::sync::Arc::new("\u{2057}".to_string()), std::sync::Arc::new("qscr;".to_string()), std::sync::Arc::new("𝓆".to_string()), std::sync::Arc::new("quaternions;".to_string()), std::sync::Arc::new("ℍ".to_string()), std::sync::Arc::new("quatint;".to_string()), std::sync::Arc::new("\u{2a16}".to_string()), std::sync::Arc::new("quest;".to_string()), std::sync::Arc::new("?".to_string()), std::sync::Arc::new("questeq;".to_string()), std::sync::Arc::new("\u{225f}".to_string()), std::sync::Arc::new("quot".to_string()), std::sync::Arc::new("\"".to_string()), std::sync::Arc::new("quot;".to_string()), std::sync::Arc::new("\"".to_string()), std::sync::Arc::new("rAarr;".to_string()), std::sync::Arc::new("\u{21db}".to_string()), std::sync::Arc::new("rArr;".to_string()), std::sync::Arc::new("\u{21d2}".to_string()), std::sync::Arc::new("rAtail;".to_string()), std::sync::Arc::new("\u{291c}".to_string()), std::sync::Arc::new("rBarr;".to_string()), std::sync::Arc::new("\u{290f}".to_string()), std::sync::Arc::new("rHar;".to_string()), std::sync::Arc::new("\u{2964}".to_string()), std::sync::Arc::new("race;".to_string()), std::sync::Arc::new("\u{223d}\u{331}".to_string()), std::sync::Arc::new("racute;".to_string()), std::sync::Arc::new("ŕ".to_string()), std::sync::Arc::new("radic;".to_string()), std::sync::Arc::new("\u{221a}".to_string()), std::sync::Arc::new("raemptyv;".to_string()), std::sync::Arc::new("\u{29b3}".to_string()), std::sync::Arc::new("rang;".to_string()), std::sync::Arc::new("\u{27e9}".to_string()), std::sync::Arc::new("rangd;".to_string()), std::sync::Arc::new("\u{2992}".to_string()), std::sync::Arc::new("range;".to_string()), std::sync::Arc::new("\u{29a5}".to_string()), std::sync::Arc::new("rangle;".to_string()), std::sync::Arc::new("\u{27e9}".to_string()), std::sync::Arc::new("raquo".to_string()), std::sync::Arc::new("\u{bb}".to_string()), std::sync::Arc::new("raquo;".to_string()), std::sync::Arc::new("\u{bb}".to_string()), std::sync::Arc::new("rarr;".to_string()), std::sync::Arc::new("\u{2192}".to_string()), std::sync::Arc::new("rarrap;".to_string()), std::sync::Arc::new("\u{2975}".to_string()), std::sync::Arc::new("rarrb;".to_string()), std::sync::Arc::new("\u{21e5}".to_string()), std::sync::Arc::new("rarrbfs;".to_string()), std::sync::Arc::new("\u{2920}".to_string()), std::sync::Arc::new("rarrc;".to_string()), std::sync::Arc::new("\u{2933}".to_string()), std::sync::Arc::new("rarrfs;".to_string()), std::sync::Arc::new("\u{291e}".to_string()), std::sync::Arc::new("rarrhk;".to_string()), std::sync::Arc::new("\u{21aa}".to_string()), std::sync::Arc::new("rarrlp;".to_string()), std::sync::Arc::new("\u{21ac}".to_string()), std::sync::Arc::new("rarrpl;".to_string()), std::sync::Arc::new("\u{2945}".to_string()), std::sync::Arc::new("rarrsim;".to_string()), std::sync::Arc::new("\u{2974}".to_string()), std::sync::Arc::new("rarrtl;".to_string()), std::sync::Arc::new("\u{21a3}".to_string()), std::sync::Arc::new("rarrw;".to_string()), std::sync::Arc::new("\u{219d}".to_string()), std::sync::Arc::new("ratail;".to_string()), std::sync::Arc::new("\u{291a}".to_string()), std::sync::Arc::new("ratio;".to_string()), std::sync::Arc::new("\u{2236}".to_string()), std::sync::Arc::new("rationals;".to_string()), std::sync::Arc::new("ℚ".to_string()), std::sync::Arc::new("rbarr;".to_string()), std::sync::Arc::new("\u{290d}".to_string()), std::sync::Arc::new("rbbrk;".to_string()), std::sync::Arc::new("\u{2773}".to_string()), std::sync::Arc::new("rbrace;".to_string()), std::sync::Arc::new("}".to_string()), std::sync::Arc::new("rbrack;".to_string()), std::sync::Arc::new("]".to_string()), std::sync::Arc::new("rbrke;".to_string()), std::sync::Arc::new("\u{298c}".to_string()), std::sync::Arc::new("rbrksld;".to_string()), std::sync::Arc::new("\u{298e}".to_string()), std::sync::Arc::new("rbrkslu;".to_string()), std::sync::Arc::new("\u{2990}".to_string()), std::sync::Arc::new("rcaron;".to_string()), std::sync::Arc::new("ř".to_string()), std::sync::Arc::new("rcedil;".to_string()), std::sync::Arc::new("ŗ".to_string()), std::sync::Arc::new("rceil;".to_string()), std::sync::Arc::new("\u{2309}".to_string()), std::sync::Arc::new("rcub;".to_string()), std::sync::Arc::new("}".to_string()), std::sync::Arc::new("rcy;".to_string()), std::sync::Arc::new("р".to_string()), std::sync::Arc::new("rdca;".to_string()), std::sync::Arc::new("\u{2937}".to_string()), std::sync::Arc::new("rdldhar;".to_string()), std::sync::Arc::new("\u{2969}".to_string()), std::sync::Arc::new("rdquo;".to_string()), std::sync::Arc::new("\u{201d}".to_string()), std::sync::Arc::new("rdquor;".to_string()), std::sync::Arc::new("\u{201d}".to_string()), std::sync::Arc::new("rdsh;".to_string()), std::sync::Arc::new("\u{21b3}".to_string()), std::sync::Arc::new("real;".to_string()), std::sync::Arc::new("ℜ".to_string()), std::sync::Arc::new("realine;".to_string()), std::sync::Arc::new("ℛ".to_string()), std::sync::Arc::new("realpart;".to_string()), std::sync::Arc::new("ℜ".to_string()), std::sync::Arc::new("reals;".to_string()), std::sync::Arc::new("ℝ".to_string()), std::sync::Arc::new("rect;".to_string()), std::sync::Arc::new("\u{25ad}".to_string()), std::sync::Arc::new("reg".to_string()), std::sync::Arc::new("\u{ae}".to_string()), std::sync::Arc::new("reg;".to_string()), std::sync::Arc::new("\u{ae}".to_string()), std::sync::Arc::new("rfisht;".to_string()), std::sync::Arc::new("\u{297d}".to_string()), std::sync::Arc::new("rfloor;".to_string()), std::sync::Arc::new("\u{230b}".to_string()), std::sync::Arc::new("rfr;".to_string()), std::sync::Arc::new("𝔯".to_string()), std::sync::Arc::new("rhard;".to_string()), std::sync::Arc::new("\u{21c1}".to_string()), std::sync::Arc::new("rharu;".to_string()), std::sync::Arc::new("\u{21c0}".to_string()), std::sync::Arc::new("rharul;".to_string()), std::sync::Arc::new("\u{296c}".to_string()), std::sync::Arc::new("rho;".to_string()), std::sync::Arc::new("ρ".to_string()), std::sync::Arc::new("rhov;".to_string()), std::sync::Arc::new("ϱ".to_string()), std::sync::Arc::new("rightarrow;".to_string()), std::sync::Arc::new("\u{2192}".to_string()), std::sync::Arc::new("rightarrowtail;".to_string()), std::sync::Arc::new("\u{21a3}".to_string()), std::sync::Arc::new("rightharpoondown;".to_string()), std::sync::Arc::new("\u{21c1}".to_string()), std::sync::Arc::new("rightharpoonup;".to_string()), std::sync::Arc::new("\u{21c0}".to_string()), std::sync::Arc::new("rightleftarrows;".to_string()), std::sync::Arc::new("\u{21c4}".to_string()), std::sync::Arc::new("rightleftharpoons;".to_string()), std::sync::Arc::new("\u{21cc}".to_string()), std::sync::Arc::new("rightrightarrows;".to_string()), std::sync::Arc::new("\u{21c9}".to_string()), std::sync::Arc::new("rightsquigarrow;".to_string()), std::sync::Arc::new("\u{219d}".to_string()), std::sync::Arc::new("rightthreetimes;".to_string()), std::sync::Arc::new("\u{22cc}".to_string()), std::sync::Arc::new("ring;".to_string()), std::sync::Arc::new("\u{2da}".to_string()), std::sync::Arc::new("risingdotseq;".to_string()), std::sync::Arc::new("\u{2253}".to_string()), std::sync::Arc::new("rlarr;".to_string()), std::sync::Arc::new("\u{21c4}".to_string()), std::sync::Arc::new("rlhar;".to_string()), std::sync::Arc::new("\u{21cc}".to_string()), std::sync::Arc::new("rlm;".to_string()), std::sync::Arc::new("\u{200f}".to_string()), std::sync::Arc::new("rmoust;".to_string()), std::sync::Arc::new("\u{23b1}".to_string()), std::sync::Arc::new("rmoustache;".to_string()), std::sync::Arc::new("\u{23b1}".to_string()), std::sync::Arc::new("rnmid;".to_string()), std::sync::Arc::new("\u{2aee}".to_string()), std::sync::Arc::new("roang;".to_string()), std::sync::Arc::new("\u{27ed}".to_string()), std::sync::Arc::new("roarr;".to_string()), std::sync::Arc::new("\u{21fe}".to_string()), std::sync::Arc::new("robrk;".to_string()), std::sync::Arc::new("\u{27e7}".to_string()), std::sync::Arc::new("ropar;".to_string()), std::sync::Arc::new("\u{2986}".to_string()), std::sync::Arc::new("ropf;".to_string()), std::sync::Arc::new("𝕣".to_string()), std::sync::Arc::new("roplus;".to_string()), std::sync::Arc::new("\u{2a2e}".to_string()), std::sync::Arc::new("rotimes;".to_string()), std::sync::Arc::new("\u{2a35}".to_string()), std::sync::Arc::new("rpar;".to_string()), std::sync::Arc::new(")".to_string()), std::sync::Arc::new("rpargt;".to_string()), std::sync::Arc::new("\u{2994}".to_string()), std::sync::Arc::new("rppolint;".to_string()), std::sync::Arc::new("\u{2a12}".to_string()), std::sync::Arc::new("rrarr;".to_string()), std::sync::Arc::new("\u{21c9}".to_string()), std::sync::Arc::new("rsaquo;".to_string()), std::sync::Arc::new("\u{203a}".to_string()), std::sync::Arc::new("rscr;".to_string()), std::sync::Arc::new("𝓇".to_string()), std::sync::Arc::new("rsh;".to_string()), std::sync::Arc::new("\u{21b1}".to_string()), std::sync::Arc::new("rsqb;".to_string()), std::sync::Arc::new("]".to_string()), std::sync::Arc::new("rsquo;".to_string()), std::sync::Arc::new("\u{2019}".to_string()), std::sync::Arc::new("rsquor;".to_string()), std::sync::Arc::new("\u{2019}".to_string()), std::sync::Arc::new("rthree;".to_string()), std::sync::Arc::new("\u{22cc}".to_string()), std::sync::Arc::new("rtimes;".to_string()), std::sync::Arc::new("\u{22ca}".to_string()), std::sync::Arc::new("rtri;".to_string()), std::sync::Arc::new("\u{25b9}".to_string()), std::sync::Arc::new("rtrie;".to_string()), std::sync::Arc::new("\u{22b5}".to_string()), std::sync::Arc::new("rtrif;".to_string()), std::sync::Arc::new("\u{25b8}".to_string()), std::sync::Arc::new("rtriltri;".to_string()), std::sync::Arc::new("\u{29ce}".to_string()), std::sync::Arc::new("ruluhar;".to_string()), std::sync::Arc::new("\u{2968}".to_string()), std::sync::Arc::new("rx;".to_string()), std::sync::Arc::new("\u{211e}".to_string()), std::sync::Arc::new("sacute;".to_string()), std::sync::Arc::new("ś".to_string()), std::sync::Arc::new("sbquo;".to_string()), std::sync::Arc::new("\u{201a}".to_string()), std::sync::Arc::new("sc;".to_string()), std::sync::Arc::new("\u{227b}".to_string()), std::sync::Arc::new("scE;".to_string()), std::sync::Arc::new("\u{2ab4}".to_string()), std::sync::Arc::new("scap;".to_string()), std::sync::Arc::new("\u{2ab8}".to_string()), std::sync::Arc::new("scaron;".to_string()), std::sync::Arc::new("š".to_string()), std::sync::Arc::new("sccue;".to_string()), std::sync::Arc::new("\u{227d}".to_string()), std::sync::Arc::new("sce;".to_string()), std::sync::Arc::new("\u{2ab0}".to_string()), std::sync::Arc::new("scedil;".to_string()), std::sync::Arc::new("ş".to_string()), std::sync::Arc::new("scirc;".to_string()), std::sync::Arc::new("ŝ".to_string()), std::sync::Arc::new("scnE;".to_string()), std::sync::Arc::new("\u{2ab6}".to_string()), std::sync::Arc::new("scnap;".to_string()), std::sync::Arc::new("\u{2aba}".to_string()), std::sync::Arc::new("scnsim;".to_string()), std::sync::Arc::new("\u{22e9}".to_string()), std::sync::Arc::new("scpolint;".to_string()), std::sync::Arc::new("\u{2a13}".to_string()), std::sync::Arc::new("scsim;".to_string()), std::sync::Arc::new("\u{227f}".to_string()), std::sync::Arc::new("scy;".to_string()), std::sync::Arc::new("с".to_string()), std::sync::Arc::new("sdot;".to_string()), std::sync::Arc::new("\u{22c5}".to_string()), std::sync::Arc::new("sdotb;".to_string()), std::sync::Arc::new("\u{22a1}".to_string()), std::sync::Arc::new("sdote;".to_string()), std::sync::Arc::new("\u{2a66}".to_string()), std::sync::Arc::new("seArr;".to_string()), std::sync::Arc::new("\u{21d8}".to_string()), std::sync::Arc::new("searhk;".to_string()), std::sync::Arc::new("\u{2925}".to_string()), std::sync::Arc::new("searr;".to_string()), std::sync::Arc::new("\u{2198}".to_string()), std::sync::Arc::new("searrow;".to_string()), std::sync::Arc::new("\u{2198}".to_string()), std::sync::Arc::new("sect".to_string()), std::sync::Arc::new("\u{a7}".to_string()), std::sync::Arc::new("sect;".to_string()), std::sync::Arc::new("\u{a7}".to_string()), std::sync::Arc::new("semi;".to_string()), std::sync::Arc::new(";".to_string()), std::sync::Arc::new("seswar;".to_string()), std::sync::Arc::new("\u{2929}".to_string()), std::sync::Arc::new("setminus;".to_string()), std::sync::Arc::new("\u{2216}".to_string()), std::sync::Arc::new("setmn;".to_string()), std::sync::Arc::new("\u{2216}".to_string()), std::sync::Arc::new("sext;".to_string()), std::sync::Arc::new("\u{2736}".to_string()), std::sync::Arc::new("sfr;".to_string()), std::sync::Arc::new("𝔰".to_string()), std::sync::Arc::new("sfrown;".to_string()), std::sync::Arc::new("\u{2322}".to_string()), std::sync::Arc::new("sharp;".to_string()), std::sync::Arc::new("\u{266f}".to_string()), std::sync::Arc::new("shchcy;".to_string()), std::sync::Arc::new("щ".to_string()), std::sync::Arc::new("shcy;".to_string()), std::sync::Arc::new("ш".to_string()), std::sync::Arc::new("shortmid;".to_string()), std::sync::Arc::new("\u{2223}".to_string()), std::sync::Arc::new("shortparallel;".to_string()), std::sync::Arc::new("\u{2225}".to_string()), std::sync::Arc::new("shy".to_string()), std::sync::Arc::new("\u{ad}".to_string()), std::sync::Arc::new("shy;".to_string()), std::sync::Arc::new("\u{ad}".to_string()), std::sync::Arc::new("sigma;".to_string()), std::sync::Arc::new("σ".to_string()), std::sync::Arc::new("sigmaf;".to_string()), std::sync::Arc::new("ς".to_string()), std::sync::Arc::new("sigmav;".to_string()), std::sync::Arc::new("ς".to_string()), std::sync::Arc::new("sim;".to_string()), std::sync::Arc::new("\u{223c}".to_string()), std::sync::Arc::new("simdot;".to_string()), std::sync::Arc::new("\u{2a6a}".to_string()), std::sync::Arc::new("sime;".to_string()), std::sync::Arc::new("\u{2243}".to_string()), std::sync::Arc::new("simeq;".to_string()), std::sync::Arc::new("\u{2243}".to_string()), std::sync::Arc::new("simg;".to_string()), std::sync::Arc::new("\u{2a9e}".to_string()), std::sync::Arc::new("simgE;".to_string()), std::sync::Arc::new("\u{2aa0}".to_string()), std::sync::Arc::new("siml;".to_string()), std::sync::Arc::new("\u{2a9d}".to_string()), std::sync::Arc::new("simlE;".to_string()), std::sync::Arc::new("\u{2a9f}".to_string()), std::sync::Arc::new("simne;".to_string()), std::sync::Arc::new("\u{2246}".to_string()), std::sync::Arc::new("simplus;".to_string()), std::sync::Arc::new("\u{2a24}".to_string()), std::sync::Arc::new("simrarr;".to_string()), std::sync::Arc::new("\u{2972}".to_string()), std::sync::Arc::new("slarr;".to_string()), std::sync::Arc::new("\u{2190}".to_string()), std::sync::Arc::new("smallsetminus;".to_string()), std::sync::Arc::new("\u{2216}".to_string()), std::sync::Arc::new("smashp;".to_string()), std::sync::Arc::new("\u{2a33}".to_string()), std::sync::Arc::new("smeparsl;".to_string()), std::sync::Arc::new("\u{29e4}".to_string()), std::sync::Arc::new("smid;".to_string()), std::sync::Arc::new("\u{2223}".to_string()), std::sync::Arc::new("smile;".to_string()), std::sync::Arc::new("\u{2323}".to_string()), std::sync::Arc::new("smt;".to_string()), std::sync::Arc::new("\u{2aaa}".to_string()), std::sync::Arc::new("smte;".to_string()), std::sync::Arc::new("\u{2aac}".to_string()), std::sync::Arc::new("smtes;".to_string()), std::sync::Arc::new("\u{2aac}\u{fe00}".to_string()), std::sync::Arc::new("softcy;".to_string()), std::sync::Arc::new("ь".to_string()), std::sync::Arc::new("sol;".to_string()), std::sync::Arc::new("/".to_string()), std::sync::Arc::new("solb;".to_string()), std::sync::Arc::new("\u{29c4}".to_string()), std::sync::Arc::new("solbar;".to_string()), std::sync::Arc::new("\u{233f}".to_string()), std::sync::Arc::new("sopf;".to_string()), std::sync::Arc::new("𝕤".to_string()), std::sync::Arc::new("spades;".to_string()), std::sync::Arc::new("\u{2660}".to_string()), std::sync::Arc::new("spadesuit;".to_string()), std::sync::Arc::new("\u{2660}".to_string()), std::sync::Arc::new("spar;".to_string()), std::sync::Arc::new("\u{2225}".to_string()), std::sync::Arc::new("sqcap;".to_string()), std::sync::Arc::new("\u{2293}".to_string()), std::sync::Arc::new("sqcaps;".to_string()), std::sync::Arc::new("\u{2293}\u{fe00}".to_string()), std::sync::Arc::new("sqcup;".to_string()), std::sync::Arc::new("\u{2294}".to_string()), std::sync::Arc::new("sqcups;".to_string()), std::sync::Arc::new("\u{2294}\u{fe00}".to_string()), std::sync::Arc::new("sqsub;".to_string()), std::sync::Arc::new("\u{228f}".to_string()), std::sync::Arc::new("sqsube;".to_string()), std::sync::Arc::new("\u{2291}".to_string()), std::sync::Arc::new("sqsubset;".to_string()), std::sync::Arc::new("\u{228f}".to_string()), std::sync::Arc::new("sqsubseteq;".to_string()), std::sync::Arc::new("\u{2291}".to_string()), std::sync::Arc::new("sqsup;".to_string()), std::sync::Arc::new("\u{2290}".to_string()), std::sync::Arc::new("sqsupe;".to_string()), std::sync::Arc::new("\u{2292}".to_string()), std::sync::Arc::new("sqsupset;".to_string()), std::sync::Arc::new("\u{2290}".to_string()), std::sync::Arc::new("sqsupseteq;".to_string()), std::sync::Arc::new("\u{2292}".to_string()), std::sync::Arc::new("squ;".to_string()), std::sync::Arc::new("\u{25a1}".to_string()), std::sync::Arc::new("square;".to_string()), std::sync::Arc::new("\u{25a1}".to_string()), std::sync::Arc::new("squarf;".to_string()), std::sync::Arc::new("\u{25aa}".to_string()), std::sync::Arc::new("squf;".to_string()), std::sync::Arc::new("\u{25aa}".to_string()), std::sync::Arc::new("srarr;".to_string()), std::sync::Arc::new("\u{2192}".to_string()), std::sync::Arc::new("sscr;".to_string()), std::sync::Arc::new("𝓈".to_string()), std::sync::Arc::new("ssetmn;".to_string()), std::sync::Arc::new("\u{2216}".to_string()), std::sync::Arc::new("ssmile;".to_string()), std::sync::Arc::new("\u{2323}".to_string()), std::sync::Arc::new("sstarf;".to_string()), std::sync::Arc::new("\u{22c6}".to_string()), std::sync::Arc::new("star;".to_string()), std::sync::Arc::new("\u{2606}".to_string()), std::sync::Arc::new("starf;".to_string()), std::sync::Arc::new("\u{2605}".to_string()), std::sync::Arc::new("straightepsilon;".to_string()), std::sync::Arc::new("ϵ".to_string()), std::sync::Arc::new("straightphi;".to_string()), std::sync::Arc::new("ϕ".to_string()), std::sync::Arc::new("strns;".to_string()), std::sync::Arc::new("\u{af}".to_string()), std::sync::Arc::new("sub;".to_string()), std::sync::Arc::new("\u{2282}".to_string()), std::sync::Arc::new("subE;".to_string()), std::sync::Arc::new("\u{2ac5}".to_string()), std::sync::Arc::new("subdot;".to_string()), std::sync::Arc::new("\u{2abd}".to_string()), std::sync::Arc::new("sube;".to_string()), std::sync::Arc::new("\u{2286}".to_string()), std::sync::Arc::new("subedot;".to_string()), std::sync::Arc::new("\u{2ac3}".to_string()), std::sync::Arc::new("submult;".to_string()), std::sync::Arc::new("\u{2ac1}".to_string()), std::sync::Arc::new("subnE;".to_string()), std::sync::Arc::new("\u{2acb}".to_string()), std::sync::Arc::new("subne;".to_string()), std::sync::Arc::new("\u{228a}".to_string()), std::sync::Arc::new("subplus;".to_string()), std::sync::Arc::new("\u{2abf}".to_string()), std::sync::Arc::new("subrarr;".to_string()), std::sync::Arc::new("\u{2979}".to_string()), std::sync::Arc::new("subset;".to_string()), std::sync::Arc::new("\u{2282}".to_string()), std::sync::Arc::new("subseteq;".to_string()), std::sync::Arc::new("\u{2286}".to_string()), std::sync::Arc::new("subseteqq;".to_string()), std::sync::Arc::new("\u{2ac5}".to_string()), std::sync::Arc::new("subsetneq;".to_string()), std::sync::Arc::new("\u{228a}".to_string()), std::sync::Arc::new("subsetneqq;".to_string()), std::sync::Arc::new("\u{2acb}".to_string()), std::sync::Arc::new("subsim;".to_string()), std::sync::Arc::new("\u{2ac7}".to_string()), std::sync::Arc::new("subsub;".to_string()), std::sync::Arc::new("\u{2ad5}".to_string()), std::sync::Arc::new("subsup;".to_string()), std::sync::Arc::new("\u{2ad3}".to_string()), std::sync::Arc::new("succ;".to_string()), std::sync::Arc::new("\u{227b}".to_string()), std::sync::Arc::new("succapprox;".to_string()), std::sync::Arc::new("\u{2ab8}".to_string()), std::sync::Arc::new("succcurlyeq;".to_string()), std::sync::Arc::new("\u{227d}".to_string()), std::sync::Arc::new("succeq;".to_string()), std::sync::Arc::new("\u{2ab0}".to_string()), std::sync::Arc::new("succnapprox;".to_string()), std::sync::Arc::new("\u{2aba}".to_string()), std::sync::Arc::new("succneqq;".to_string()), std::sync::Arc::new("\u{2ab6}".to_string()), std::sync::Arc::new("succnsim;".to_string()), std::sync::Arc::new("\u{22e9}".to_string()), std::sync::Arc::new("succsim;".to_string()), std::sync::Arc::new("\u{227f}".to_string()), std::sync::Arc::new("sum;".to_string()), std::sync::Arc::new("\u{2211}".to_string()), std::sync::Arc::new("sung;".to_string()), std::sync::Arc::new("\u{266a}".to_string()), std::sync::Arc::new("sup1".to_string()), std::sync::Arc::new("\u{b9}".to_string()), std::sync::Arc::new("sup1;".to_string()), std::sync::Arc::new("\u{b9}".to_string()), std::sync::Arc::new("sup2".to_string()), std::sync::Arc::new("\u{b2}".to_string()), std::sync::Arc::new("sup2;".to_string()), std::sync::Arc::new("\u{b2}".to_string()), std::sync::Arc::new("sup3".to_string()), std::sync::Arc::new("\u{b3}".to_string()), std::sync::Arc::new("sup3;".to_string()), std::sync::Arc::new("\u{b3}".to_string()), std::sync::Arc::new("sup;".to_string()), std::sync::Arc::new("\u{2283}".to_string()), std::sync::Arc::new("supE;".to_string()), std::sync::Arc::new("\u{2ac6}".to_string()), std::sync::Arc::new("supdot;".to_string()), std::sync::Arc::new("\u{2abe}".to_string()), std::sync::Arc::new("supdsub;".to_string()), std::sync::Arc::new("\u{2ad8}".to_string()), std::sync::Arc::new("supe;".to_string()), std::sync::Arc::new("\u{2287}".to_string()), std::sync::Arc::new("supedot;".to_string()), std::sync::Arc::new("\u{2ac4}".to_string()), std::sync::Arc::new("suphsol;".to_string()), std::sync::Arc::new("\u{27c9}".to_string()), std::sync::Arc::new("suphsub;".to_string()), std::sync::Arc::new("\u{2ad7}".to_string()), std::sync::Arc::new("suplarr;".to_string()), std::sync::Arc::new("\u{297b}".to_string()), std::sync::Arc::new("supmult;".to_string()), std::sync::Arc::new("\u{2ac2}".to_string()), std::sync::Arc::new("supnE;".to_string()), std::sync::Arc::new("\u{2acc}".to_string()), std::sync::Arc::new("supne;".to_string()), std::sync::Arc::new("\u{228b}".to_string()), std::sync::Arc::new("supplus;".to_string()), std::sync::Arc::new("\u{2ac0}".to_string()), std::sync::Arc::new("supset;".to_string()), std::sync::Arc::new("\u{2283}".to_string()), std::sync::Arc::new("supseteq;".to_string()), std::sync::Arc::new("\u{2287}".to_string()), std::sync::Arc::new("supseteqq;".to_string()), std::sync::Arc::new("\u{2ac6}".to_string()), std::sync::Arc::new("supsetneq;".to_string()), std::sync::Arc::new("\u{228b}".to_string()), std::sync::Arc::new("supsetneqq;".to_string()), std::sync::Arc::new("\u{2acc}".to_string()), std::sync::Arc::new("supsim;".to_string()), std::sync::Arc::new("\u{2ac8}".to_string()), std::sync::Arc::new("supsub;".to_string()), std::sync::Arc::new("\u{2ad4}".to_string()), std::sync::Arc::new("supsup;".to_string()), std::sync::Arc::new("\u{2ad6}".to_string()), std::sync::Arc::new("swArr;".to_string()), std::sync::Arc::new("\u{21d9}".to_string()), std::sync::Arc::new("swarhk;".to_string()), std::sync::Arc::new("\u{2926}".to_string()), std::sync::Arc::new("swarr;".to_string()), std::sync::Arc::new("\u{2199}".to_string()), std::sync::Arc::new("swarrow;".to_string()), std::sync::Arc::new("\u{2199}".to_string()), std::sync::Arc::new("swnwar;".to_string()), std::sync::Arc::new("\u{292a}".to_string()), std::sync::Arc::new("szlig".to_string()), std::sync::Arc::new("ß".to_string()), std::sync::Arc::new("szlig;".to_string()), std::sync::Arc::new("ß".to_string()), std::sync::Arc::new("target;".to_string()), std::sync::Arc::new("\u{2316}".to_string()), std::sync::Arc::new("tau;".to_string()), std::sync::Arc::new("τ".to_string()), std::sync::Arc::new("tbrk;".to_string()), std::sync::Arc::new("\u{23b4}".to_string()), std::sync::Arc::new("tcaron;".to_string()), std::sync::Arc::new("ť".to_string()), std::sync::Arc::new("tcedil;".to_string()), std::sync::Arc::new("ţ".to_string()), std::sync::Arc::new("tcy;".to_string()), std::sync::Arc::new("т".to_string()), std::sync::Arc::new("tdot;".to_string()), std::sync::Arc::new("\u{20db}".to_string()), std::sync::Arc::new("telrec;".to_string()), std::sync::Arc::new("\u{2315}".to_string()), std::sync::Arc::new("tfr;".to_string()), std::sync::Arc::new("𝔱".to_string()), std::sync::Arc::new("there4;".to_string()), std::sync::Arc::new("\u{2234}".to_string()), std::sync::Arc::new("therefore;".to_string()), std::sync::Arc::new("\u{2234}".to_string()), std::sync::Arc::new("theta;".to_string()), std::sync::Arc::new("θ".to_string()), std::sync::Arc::new("thetasym;".to_string()), std::sync::Arc::new("ϑ".to_string()), std::sync::Arc::new("thetav;".to_string()), std::sync::Arc::new("ϑ".to_string()), std::sync::Arc::new("thickapprox;".to_string()), std::sync::Arc::new("\u{2248}".to_string()), std::sync::Arc::new("thicksim;".to_string()), std::sync::Arc::new("\u{223c}".to_string()), std::sync::Arc::new("thinsp;".to_string()), std::sync::Arc::new("\u{2009}".to_string()), std::sync::Arc::new("thkap;".to_string()), std::sync::Arc::new("\u{2248}".to_string()), std::sync::Arc::new("thksim;".to_string()), std::sync::Arc::new("\u{223c}".to_string()), std::sync::Arc::new("thorn".to_string()), std::sync::Arc::new("þ".to_string()), std::sync::Arc::new("thorn;".to_string()), std::sync::Arc::new("þ".to_string()), std::sync::Arc::new("tilde;".to_string()), std::sync::Arc::new("\u{2dc}".to_string()), std::sync::Arc::new("times".to_string()), std::sync::Arc::new("\u{d7}".to_string()), std::sync::Arc::new("times;".to_string()), std::sync::Arc::new("\u{d7}".to_string()), std::sync::Arc::new("timesb;".to_string()), std::sync::Arc::new("\u{22a0}".to_string()), std::sync::Arc::new("timesbar;".to_string()), std::sync::Arc::new("\u{2a31}".to_string()), std::sync::Arc::new("timesd;".to_string()), std::sync::Arc::new("\u{2a30}".to_string()), std::sync::Arc::new("tint;".to_string()), std::sync::Arc::new("\u{222d}".to_string()), std::sync::Arc::new("toea;".to_string()), std::sync::Arc::new("\u{2928}".to_string()), std::sync::Arc::new("top;".to_string()), std::sync::Arc::new("\u{22a4}".to_string()), std::sync::Arc::new("topbot;".to_string()), std::sync::Arc::new("\u{2336}".to_string()), std::sync::Arc::new("topcir;".to_string()), std::sync::Arc::new("\u{2af1}".to_string()), std::sync::Arc::new("topf;".to_string()), std::sync::Arc::new("𝕥".to_string()), std::sync::Arc::new("topfork;".to_string()), std::sync::Arc::new("\u{2ada}".to_string()), std::sync::Arc::new("tosa;".to_string()), std::sync::Arc::new("\u{2929}".to_string()), std::sync::Arc::new("tprime;".to_string()), std::sync::Arc::new("\u{2034}".to_string()), std::sync::Arc::new("trade;".to_string()), std::sync::Arc::new("\u{2122}".to_string()), std::sync::Arc::new("triangle;".to_string()), std::sync::Arc::new("\u{25b5}".to_string()), std::sync::Arc::new("triangledown;".to_string()), std::sync::Arc::new("\u{25bf}".to_string()), std::sync::Arc::new("triangleleft;".to_string()), std::sync::Arc::new("\u{25c3}".to_string()), std::sync::Arc::new("trianglelefteq;".to_string()), std::sync::Arc::new("\u{22b4}".to_string()), std::sync::Arc::new("triangleq;".to_string()), std::sync::Arc::new("\u{225c}".to_string()), std::sync::Arc::new("triangleright;".to_string()), std::sync::Arc::new("\u{25b9}".to_string()), std::sync::Arc::new("trianglerighteq;".to_string()), std::sync::Arc::new("\u{22b5}".to_string()), std::sync::Arc::new("tridot;".to_string()), std::sync::Arc::new("\u{25ec}".to_string()), std::sync::Arc::new("trie;".to_string()), std::sync::Arc::new("\u{225c}".to_string()), std::sync::Arc::new("triminus;".to_string()), std::sync::Arc::new("\u{2a3a}".to_string()), std::sync::Arc::new("triplus;".to_string()), std::sync::Arc::new("\u{2a39}".to_string()), std::sync::Arc::new("trisb;".to_string()), std::sync::Arc::new("\u{29cd}".to_string()), std::sync::Arc::new("tritime;".to_string()), std::sync::Arc::new("\u{2a3b}".to_string()), std::sync::Arc::new("trpezium;".to_string()), std::sync::Arc::new("\u{23e2}".to_string()), std::sync::Arc::new("tscr;".to_string()), std::sync::Arc::new("𝓉".to_string()), std::sync::Arc::new("tscy;".to_string()), std::sync::Arc::new("ц".to_string()), std::sync::Arc::new("tshcy;".to_string()), std::sync::Arc::new("ћ".to_string()), std::sync::Arc::new("tstrok;".to_string()), std::sync::Arc::new("ŧ".to_string()), std::sync::Arc::new("twixt;".to_string()), std::sync::Arc::new("\u{226c}".to_string()), std::sync::Arc::new("twoheadleftarrow;".to_string()), std::sync::Arc::new("\u{219e}".to_string()), std::sync::Arc::new("twoheadrightarrow;".to_string()), std::sync::Arc::new("\u{21a0}".to_string()), std::sync::Arc::new("uArr;".to_string()), std::sync::Arc::new("\u{21d1}".to_string()), std::sync::Arc::new("uHar;".to_string()), std::sync::Arc::new("\u{2963}".to_string()), std::sync::Arc::new("uacute".to_string()), std::sync::Arc::new("ú".to_string()), std::sync::Arc::new("uacute;".to_string()), std::sync::Arc::new("ú".to_string()), std::sync::Arc::new("uarr;".to_string()), std::sync::Arc::new("\u{2191}".to_string()), std::sync::Arc::new("ubrcy;".to_string()), std::sync::Arc::new("ў".to_string()), std::sync::Arc::new("ubreve;".to_string()), std::sync::Arc::new("ŭ".to_string()), std::sync::Arc::new("ucirc".to_string()), std::sync::Arc::new("û".to_string()), std::sync::Arc::new("ucirc;".to_string()), std::sync::Arc::new("û".to_string()), std::sync::Arc::new("ucy;".to_string()), std::sync::Arc::new("у".to_string()), std::sync::Arc::new("udarr;".to_string()), std::sync::Arc::new("\u{21c5}".to_string()), std::sync::Arc::new("udblac;".to_string()), std::sync::Arc::new("ű".to_string()), std::sync::Arc::new("udhar;".to_string()), std::sync::Arc::new("\u{296e}".to_string()), std::sync::Arc::new("ufisht;".to_string()), std::sync::Arc::new("\u{297e}".to_string()), std::sync::Arc::new("ufr;".to_string()), std::sync::Arc::new("𝔲".to_string()), std::sync::Arc::new("ugrave".to_string()), std::sync::Arc::new("ù".to_string()), std::sync::Arc::new("ugrave;".to_string()), std::sync::Arc::new("ù".to_string()), std::sync::Arc::new("uharl;".to_string()), std::sync::Arc::new("\u{21bf}".to_string()), std::sync::Arc::new("uharr;".to_string()), std::sync::Arc::new("\u{21be}".to_string()), std::sync::Arc::new("uhblk;".to_string()), std::sync::Arc::new("\u{2580}".to_string()), std::sync::Arc::new("ulcorn;".to_string()), std::sync::Arc::new("\u{231c}".to_string()), std::sync::Arc::new("ulcorner;".to_string()), std::sync::Arc::new("\u{231c}".to_string()), std::sync::Arc::new("ulcrop;".to_string()), std::sync::Arc::new("\u{230f}".to_string()), std::sync::Arc::new("ultri;".to_string()), std::sync::Arc::new("\u{25f8}".to_string()), std::sync::Arc::new("umacr;".to_string()), std::sync::Arc::new("ū".to_string()), std::sync::Arc::new("uml".to_string()), std::sync::Arc::new("\u{a8}".to_string()), std::sync::Arc::new("uml;".to_string()), std::sync::Arc::new("\u{a8}".to_string()), std::sync::Arc::new("uogon;".to_string()), std::sync::Arc::new("ų".to_string()), std::sync::Arc::new("uopf;".to_string()), std::sync::Arc::new("𝕦".to_string()), std::sync::Arc::new("uparrow;".to_string()), std::sync::Arc::new("\u{2191}".to_string()), std::sync::Arc::new("updownarrow;".to_string()), std::sync::Arc::new("\u{2195}".to_string()), std::sync::Arc::new("upharpoonleft;".to_string()), std::sync::Arc::new("\u{21bf}".to_string()), std::sync::Arc::new("upharpoonright;".to_string()), std::sync::Arc::new("\u{21be}".to_string()), std::sync::Arc::new("uplus;".to_string()), std::sync::Arc::new("\u{228e}".to_string()), std::sync::Arc::new("upsi;".to_string()), std::sync::Arc::new("υ".to_string()), std::sync::Arc::new("upsih;".to_string()), std::sync::Arc::new("ϒ".to_string()), std::sync::Arc::new("upsilon;".to_string()), std::sync::Arc::new("υ".to_string()), std::sync::Arc::new("upuparrows;".to_string()), std::sync::Arc::new("\u{21c8}".to_string()), std::sync::Arc::new("urcorn;".to_string()), std::sync::Arc::new("\u{231d}".to_string()), std::sync::Arc::new("urcorner;".to_string()), std::sync::Arc::new("\u{231d}".to_string()), std::sync::Arc::new("urcrop;".to_string()), std::sync::Arc::new("\u{230e}".to_string()), std::sync::Arc::new("uring;".to_string()), std::sync::Arc::new("ů".to_string()), std::sync::Arc::new("urtri;".to_string()), std::sync::Arc::new("\u{25f9}".to_string()), std::sync::Arc::new("uscr;".to_string()), std::sync::Arc::new("𝓊".to_string()), std::sync::Arc::new("utdot;".to_string()), std::sync::Arc::new("\u{22f0}".to_string()), std::sync::Arc::new("utilde;".to_string()), std::sync::Arc::new("ũ".to_string()), std::sync::Arc::new("utri;".to_string()), std::sync::Arc::new("\u{25b5}".to_string()), std::sync::Arc::new("utrif;".to_string()), std::sync::Arc::new("\u{25b4}".to_string()), std::sync::Arc::new("uuarr;".to_string()), std::sync::Arc::new("\u{21c8}".to_string()), std::sync::Arc::new("uuml".to_string()), std::sync::Arc::new("ü".to_string()), std::sync::Arc::new("uuml;".to_string()), std::sync::Arc::new("ü".to_string()), std::sync::Arc::new("uwangle;".to_string()), std::sync::Arc::new("\u{29a7}".to_string()), std::sync::Arc::new("vArr;".to_string()), std::sync::Arc::new("\u{21d5}".to_string()), std::sync::Arc::new("vBar;".to_string()), std::sync::Arc::new("\u{2ae8}".to_string()), std::sync::Arc::new("vBarv;".to_string()), std::sync::Arc::new("\u{2ae9}".to_string()), std::sync::Arc::new("vDash;".to_string()), std::sync::Arc::new("\u{22a8}".to_string()), std::sync::Arc::new("vangrt;".to_string()), std::sync::Arc::new("\u{299c}".to_string()), std::sync::Arc::new("varepsilon;".to_string()), std::sync::Arc::new("ϵ".to_string()), std::sync::Arc::new("varkappa;".to_string()), std::sync::Arc::new("ϰ".to_string()), std::sync::Arc::new("varnothing;".to_string()), std::sync::Arc::new("\u{2205}".to_string()), std::sync::Arc::new("varphi;".to_string()), std::sync::Arc::new("ϕ".to_string()), std::sync::Arc::new("varpi;".to_string()), std::sync::Arc::new("ϖ".to_string()), std::sync::Arc::new("varpropto;".to_string()), std::sync::Arc::new("\u{221d}".to_string()), std::sync::Arc::new("varr;".to_string()), std::sync::Arc::new("\u{2195}".to_string()), std::sync::Arc::new("varrho;".to_string()), std::sync::Arc::new("ϱ".to_string()), std::sync::Arc::new("varsigma;".to_string()), std::sync::Arc::new("ς".to_string()), std::sync::Arc::new("varsubsetneq;".to_string()), std::sync::Arc::new("\u{228a}\u{fe00}".to_string()), std::sync::Arc::new("varsubsetneqq;".to_string()), std::sync::Arc::new("\u{2acb}\u{fe00}".to_string()), std::sync::Arc::new("varsupsetneq;".to_string()), std::sync::Arc::new("\u{228b}\u{fe00}".to_string()), std::sync::Arc::new("varsupsetneqq;".to_string()), std::sync::Arc::new("\u{2acc}\u{fe00}".to_string()), std::sync::Arc::new("vartheta;".to_string()), std::sync::Arc::new("ϑ".to_string()), std::sync::Arc::new("vartriangleleft;".to_string()), std::sync::Arc::new("\u{22b2}".to_string()), std::sync::Arc::new("vartriangleright;".to_string()), std::sync::Arc::new("\u{22b3}".to_string()), std::sync::Arc::new("vcy;".to_string()), std::sync::Arc::new("в".to_string()), std::sync::Arc::new("vdash;".to_string()), std::sync::Arc::new("\u{22a2}".to_string()), std::sync::Arc::new("vee;".to_string()), std::sync::Arc::new("\u{2228}".to_string()), std::sync::Arc::new("veebar;".to_string()), std::sync::Arc::new("\u{22bb}".to_string()), std::sync::Arc::new("veeeq;".to_string()), std::sync::Arc::new("\u{225a}".to_string()), std::sync::Arc::new("vellip;".to_string()), std::sync::Arc::new("\u{22ee}".to_string()), std::sync::Arc::new("verbar;".to_string()), std::sync::Arc::new("|".to_string()), std::sync::Arc::new("vert;".to_string()), std::sync::Arc::new("|".to_string()), std::sync::Arc::new("vfr;".to_string()), std::sync::Arc::new("𝔳".to_string()), std::sync::Arc::new("vltri;".to_string()), std::sync::Arc::new("\u{22b2}".to_string()), std::sync::Arc::new("vnsub;".to_string()), std::sync::Arc::new("\u{2282}\u{20d2}".to_string()), std::sync::Arc::new("vnsup;".to_string()), std::sync::Arc::new("\u{2283}\u{20d2}".to_string()), std::sync::Arc::new("vopf;".to_string()), std::sync::Arc::new("𝕧".to_string()), std::sync::Arc::new("vprop;".to_string()), std::sync::Arc::new("\u{221d}".to_string()), std::sync::Arc::new("vrtri;".to_string()), std::sync::Arc::new("\u{22b3}".to_string()), std::sync::Arc::new("vscr;".to_string()), std::sync::Arc::new("𝓋".to_string()), std::sync::Arc::new("vsubnE;".to_string()), std::sync::Arc::new("\u{2acb}\u{fe00}".to_string()), std::sync::Arc::new("vsubne;".to_string()), std::sync::Arc::new("\u{228a}\u{fe00}".to_string()), std::sync::Arc::new("vsupnE;".to_string()), std::sync::Arc::new("\u{2acc}\u{fe00}".to_string()), std::sync::Arc::new("vsupne;".to_string()), std::sync::Arc::new("\u{228b}\u{fe00}".to_string()), std::sync::Arc::new("vzigzag;".to_string()), std::sync::Arc::new("\u{299a}".to_string()), std::sync::Arc::new("wcirc;".to_string()), std::sync::Arc::new("ŵ".to_string()), std::sync::Arc::new("wedbar;".to_string()), std::sync::Arc::new("\u{2a5f}".to_string()), std::sync::Arc::new("wedge;".to_string()), std::sync::Arc::new("\u{2227}".to_string()), std::sync::Arc::new("wedgeq;".to_string()), std::sync::Arc::new("\u{2259}".to_string()), std::sync::Arc::new("weierp;".to_string()), std::sync::Arc::new("\u{2118}".to_string()), std::sync::Arc::new("wfr;".to_string()), std::sync::Arc::new("𝔴".to_string()), std::sync::Arc::new("wopf;".to_string()), std::sync::Arc::new("𝕨".to_string()), std::sync::Arc::new("wp;".to_string()), std::sync::Arc::new("\u{2118}".to_string()), std::sync::Arc::new("wr;".to_string()), std::sync::Arc::new("\u{2240}".to_string()), std::sync::Arc::new("wreath;".to_string()), std::sync::Arc::new("\u{2240}".to_string()), std::sync::Arc::new("wscr;".to_string()), std::sync::Arc::new("𝓌".to_string()), std::sync::Arc::new("xcap;".to_string()), std::sync::Arc::new("\u{22c2}".to_string()), std::sync::Arc::new("xcirc;".to_string()), std::sync::Arc::new("\u{25ef}".to_string()), std::sync::Arc::new("xcup;".to_string()), std::sync::Arc::new("\u{22c3}".to_string()), std::sync::Arc::new("xdtri;".to_string()), std::sync::Arc::new("\u{25bd}".to_string()), std::sync::Arc::new("xfr;".to_string()), std::sync::Arc::new("𝔵".to_string()), std::sync::Arc::new("xhArr;".to_string()), std::sync::Arc::new("\u{27fa}".to_string()), std::sync::Arc::new("xharr;".to_string()), std::sync::Arc::new("\u{27f7}".to_string()), std::sync::Arc::new("xi;".to_string()), std::sync::Arc::new("ξ".to_string()), std::sync::Arc::new("xlArr;".to_string()), std::sync::Arc::new("\u{27f8}".to_string()), std::sync::Arc::new("xlarr;".to_string()), std::sync::Arc::new("\u{27f5}".to_string()), std::sync::Arc::new("xmap;".to_string()), std::sync::Arc::new("\u{27fc}".to_string()), std::sync::Arc::new("xnis;".to_string()), std::sync::Arc::new("\u{22fb}".to_string()), std::sync::Arc::new("xodot;".to_string()), std::sync::Arc::new("\u{2a00}".to_string()), std::sync::Arc::new("xopf;".to_string()), std::sync::Arc::new("𝕩".to_string()), std::sync::Arc::new("xoplus;".to_string()), std::sync::Arc::new("\u{2a01}".to_string()), std::sync::Arc::new("xotime;".to_string()), std::sync::Arc::new("\u{2a02}".to_string()), std::sync::Arc::new("xrArr;".to_string()), std::sync::Arc::new("\u{27f9}".to_string()), std::sync::Arc::new("xrarr;".to_string()), std::sync::Arc::new("\u{27f6}".to_string()), std::sync::Arc::new("xscr;".to_string()), std::sync::Arc::new("𝓍".to_string()), std::sync::Arc::new("xsqcup;".to_string()), std::sync::Arc::new("\u{2a06}".to_string()), std::sync::Arc::new("xuplus;".to_string()), std::sync::Arc::new("\u{2a04}".to_string()), std::sync::Arc::new("xutri;".to_string()), std::sync::Arc::new("\u{25b3}".to_string()), std::sync::Arc::new("xvee;".to_string()), std::sync::Arc::new("\u{22c1}".to_string()), std::sync::Arc::new("xwedge;".to_string()), std::sync::Arc::new("\u{22c0}".to_string()), std::sync::Arc::new("yacute".to_string()), std::sync::Arc::new("ý".to_string()), std::sync::Arc::new("yacute;".to_string()), std::sync::Arc::new("ý".to_string()), std::sync::Arc::new("yacy;".to_string()), std::sync::Arc::new("я".to_string()), std::sync::Arc::new("ycirc;".to_string()), std::sync::Arc::new("ŷ".to_string()), std::sync::Arc::new("ycy;".to_string()), std::sync::Arc::new("ы".to_string()), std::sync::Arc::new("yen".to_string()), std::sync::Arc::new("\u{a5}".to_string()), std::sync::Arc::new("yen;".to_string()), std::sync::Arc::new("\u{a5}".to_string()), std::sync::Arc::new("yfr;".to_string()), std::sync::Arc::new("𝔶".to_string()), std::sync::Arc::new("yicy;".to_string()), std::sync::Arc::new("ї".to_string()), std::sync::Arc::new("yopf;".to_string()), std::sync::Arc::new("𝕪".to_string()), std::sync::Arc::new("yscr;".to_string()), std::sync::Arc::new("𝓎".to_string()), std::sync::Arc::new("yucy;".to_string()), std::sync::Arc::new("ю".to_string()), std::sync::Arc::new("yuml".to_string()), std::sync::Arc::new("ÿ".to_string()), std::sync::Arc::new("yuml;".to_string()), std::sync::Arc::new("ÿ".to_string()), std::sync::Arc::new("zacute;".to_string()), std::sync::Arc::new("ź".to_string()), std::sync::Arc::new("zcaron;".to_string()), std::sync::Arc::new("ž".to_string()), std::sync::Arc::new("zcy;".to_string()), std::sync::Arc::new("з".to_string()), std::sync::Arc::new("zdot;".to_string()), std::sync::Arc::new("ż".to_string()), std::sync::Arc::new("zeetrf;".to_string()), std::sync::Arc::new("ℨ".to_string()), std::sync::Arc::new("zeta;".to_string()), std::sync::Arc::new("ζ".to_string()), std::sync::Arc::new("zfr;".to_string()), std::sync::Arc::new("𝔷".to_string()), std::sync::Arc::new("zhcy;".to_string()), std::sync::Arc::new("ж".to_string()), std::sync::Arc::new("zigrarr;".to_string()), std::sync::Arc::new("\u{21dd}".to_string()), std::sync::Arc::new("zopf;".to_string()), std::sync::Arc::new("𝕫".to_string()), std::sync::Arc::new("zscr;".to_string()), std::sync::Arc::new("𝓏".to_string()), std::sync::Arc::new("zwj;".to_string()), std::sync::Arc::new("\u{200d}".to_string()), std::sync::Arc::new("zwnj;".to_string()), std::sync::Arc::new("\u{200c}".to_string())]);
            let mb__376: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
            let mut i__377: i32 = 0;
            let mut n__378: i32 = temper_core::ListedTrait::len( & strs__375);
            'loop___5923: while Some(i__377) < Some(n__378) {
                temper_core::MapBuilder::set( & mb__376, temper_core::ListedTrait::get( & strs__375, i__377), temper_core::ListedTrait::get( & strs__375, i__377.wrapping_add(1)));
                i__377 = i__377.wrapping_add(2);
            }
            let return__374: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MappedTrait::to_map( & mb__376);
            HTML_NAMED_CHARACTERS.set(return__374.clone()).unwrap_or_else(| _ | panic!());
            let return__373: HtmlCodec = HtmlCodec::new();
            HTML_CODEC.set(return__373.clone()).unwrap_or_else(| _ | panic!());
            let htmlStatePcdata__269: i32 = 0;
            let htmlStateOName__270: i32 = 1;
            let htmlStateCName__271: i32 = 2;
            let htmlStateBeforeAttr__272: i32 = 3;
            let htmlStateBeforeEq__273: i32 = 4;
            let htmlStateBeforeValue__274: i32 = 5;
            let htmlStateAttr__275: i32 = 6;
            let htmlStateAfterAttr__276: i32 = 7;
            let htmlStateSpecialBody__277: i32 = 8;
            let attribStateGeneric__278: i32 = 0;
            let attribStateCss__279: i32 = 1;
            let attribStateJs__280: i32 = 2;
            let attribStateUrl__281: i32 = 3;
            let attribStateUrls__282: i32 = 4;
            let delimStateUq__283: i32 = 0;
            let delimStateSq__284: i32 = 1;
            let delimStateDq__285: i32 = 2;
            let urlStateStart__286: i32 = 0;
            let urlStateBeforeQuery__287: i32 = 1;
            let urlStateQuery__288: i32 = 2;
            let urlStateFragment__289: i32 = 3;
            URL_CONTEXT_PROPAGATOR.set(UrlContextPropagator::new()).unwrap_or_else(| _ | panic!());
            PROTOCOL_ALLOW_LIST.set(temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(temper_std::regex::Or::new([temper_std::regex::RegexNode::new(temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("H")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("h"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("T")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("t"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("T")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("t"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("P")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("p"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("S")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("s"))], Some(false))), 0, Some(1), Some(false)))])), temper_std::regex::RegexNode::new(temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("M")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("m"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("A")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("a"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("I")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("i"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("L")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("l"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("T")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("t"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("O")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("o"))], Some(false)))]))])), temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::end().clone()).unwrap()]).compiled()).unwrap_or_else(| _ | panic!());
            FALLBACK_SAFE_URL.set(SafeUrl::new("about:zz_Temper_zz#")).unwrap_or_else(| _ | panic!());
            let lb__732: temper_core::ListBuilder<bool> = temper_core::listed::new_builder();
            let mut i__733: i32 = 0;
            'loop___5938: while Some(i__733) < Some(128) {
                if Some(i__733) == Some(47) {
                    t___3455 = true;
                } else {
                    if Some(i__733) == Some(46) {
                        t___3454 = true;
                    } else {
                        if Some(i__733) == Some(45) {
                            t___3453 = true;
                        } else {
                            if Some(i__733) == Some(95) {
                                t___3452 = true;
                            } else {
                                if Some(48) <= Some(i__733) {
                                    t___3449 = Some(i__733) <= Some(57);
                                } else {
                                    t___3449 = false;
                                }
                                if t___3449 {
                                    t___3451 = true;
                                } else {
                                    if Some(97) <= Some(i__733 | 32) {
                                        t___3450 = Some(i__733 | 32) <= Some(122);
                                    } else {
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
                temper_core::listed::add( & lb__732, t___3455, None);
                i__733 = i__733.wrapping_add(1);
            }
            URL_QUERY_SAFE.set(temper_core::ListedTrait::to_list( & lb__732)).unwrap_or_else(| _ | panic!());
            let lb__734: temper_core::ListBuilder<bool> = temper_core::listed::new_builder();
            let mut i__735: i32 = 0;
            'loop___5939: while Some(i__735) < Some(128) {
                if temper_core::ListedTrait::get( & url_query_safe(), i__735) {
                    t___3464 = true;
                } else {
                    if Some(i__735) == Some(58) {
                        t___3463 = true;
                    } else {
                        if Some(i__735) == Some(63) {
                            t___3462 = true;
                        } else {
                            if Some(i__735) == Some(35) {
                                t___3461 = true;
                            } else {
                                t___3461 = Some(i__735) == Some(38);
                            }
                            t___3462 = t___3461;
                        }
                        t___3463 = t___3462;
                    }
                    t___3464 = t___3463;
                }
                temper_core::listed::add( & lb__734, t___3464, None);
                i__735 = i__735.wrapping_add(1);
            }
            URL_SAFE.set(temper_core::ListedTrait::to_list( & lb__734)).unwrap_or_else(| _ | panic!());
            let return__749: HtmlUrlPartUrlEscaper = HtmlUrlPartUrlEscaper::new();
            HTML_URL_PART_URL_ESCAPER.set(return__749.clone()).unwrap_or_else(| _ | panic!());
            let return__731: HtmlProtocolFilteringUrlEscaper = HtmlProtocolFilteringUrlEscaper::new();
            HTML_PROTOCOL_FILTERING_URL_ESCAPER.set(return__731.clone()).unwrap_or_else(| _ | panic!());
            let return__763: HtmlAsIfQueryUrlEscaper = HtmlAsIfQueryUrlEscaper::new();
            HTML_AS_IF_QUERY_URL_ESCAPER.set(return__763.clone()).unwrap_or_else(| _ | panic!());
            let return__593: HtmlPcdataEscaper = HtmlPcdataEscaper::new();
            HTML_PCDATA_ESCAPER.set(return__593.clone()).unwrap_or_else(| _ | panic!());
            let return__572: OutputHtmlSpaceEscaper = OutputHtmlSpaceEscaper::new();
            OUTPUT_HTML_SPACE_ESCAPER.set(return__572.clone()).unwrap_or_else(| _ | panic!());
            let return__614: HtmlAttributeEscaper = HtmlAttributeEscaper::new();
            HTML_ATTRIBUTE_ESCAPER.set(return__614.clone()).unwrap_or_else(| _ | panic!());
            Ok(())
    }).clone()
}
static HTML_NAMED_CHARACTERS: std::sync::OnceLock<temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>> = std::sync::OnceLock::new();
fn html_named_characters() -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
    ( * HTML_NAMED_CHARACTERS.get().unwrap()).clone()
}
static HTML_CODEC: std::sync::OnceLock<crate::core::Codec> = std::sync::OnceLock::new();
pub fn html_codec() -> crate::core::Codec {
    ( * HTML_CODEC.get().unwrap()).clone()
}
static URL_CONTEXT_PROPAGATOR: std::sync::OnceLock<UrlContextPropagator> = std::sync::OnceLock::new();
fn url_context_propagator() -> UrlContextPropagator {
    ( * URL_CONTEXT_PROPAGATOR.get().unwrap()).clone()
}
static PROTOCOL_ALLOW_LIST: std::sync::OnceLock<temper_std::regex::Regex> = std::sync::OnceLock::new();
fn protocol_allow_list() -> temper_std::regex::Regex {
    ( * PROTOCOL_ALLOW_LIST.get().unwrap()).clone()
}
static FALLBACK_SAFE_URL: std::sync::OnceLock<SafeUrl> = std::sync::OnceLock::new();
fn fallback_safe_url() -> SafeUrl {
    ( * FALLBACK_SAFE_URL.get().unwrap()).clone()
}
static URL_QUERY_SAFE: std::sync::OnceLock<temper_core::List<bool>> = std::sync::OnceLock::new();
fn url_query_safe() -> temper_core::List<bool> {
    ( * URL_QUERY_SAFE.get().unwrap()).clone()
}
static URL_SAFE: std::sync::OnceLock<temper_core::List<bool>> = std::sync::OnceLock::new();
fn url_safe() -> temper_core::List<bool> {
    ( * URL_SAFE.get().unwrap()).clone()
}
static HTML_URL_PART_URL_ESCAPER: std::sync::OnceLock<HtmlUrlPartUrlEscaper> = std::sync::OnceLock::new();
fn html_url_part_url_escaper() -> HtmlUrlPartUrlEscaper {
    ( * HTML_URL_PART_URL_ESCAPER.get().unwrap()).clone()
}
static HTML_PROTOCOL_FILTERING_URL_ESCAPER: std::sync::OnceLock<HtmlProtocolFilteringUrlEscaper> = std::sync::OnceLock::new();
fn html_protocol_filtering_url_escaper() -> HtmlProtocolFilteringUrlEscaper {
    ( * HTML_PROTOCOL_FILTERING_URL_ESCAPER.get().unwrap()).clone()
}
static HTML_AS_IF_QUERY_URL_ESCAPER: std::sync::OnceLock<HtmlAsIfQueryUrlEscaper> = std::sync::OnceLock::new();
fn html_as_if_query_url_escaper() -> HtmlAsIfQueryUrlEscaper {
    ( * HTML_AS_IF_QUERY_URL_ESCAPER.get().unwrap()).clone()
}
static HTML_PCDATA_ESCAPER: std::sync::OnceLock<HtmlPcdataEscaper> = std::sync::OnceLock::new();
fn html_pcdata_escaper() -> HtmlPcdataEscaper {
    ( * HTML_PCDATA_ESCAPER.get().unwrap()).clone()
}
static OUTPUT_HTML_SPACE_ESCAPER: std::sync::OnceLock<OutputHtmlSpaceEscaper> = std::sync::OnceLock::new();
fn output_html_space_escaper() -> OutputHtmlSpaceEscaper {
    ( * OUTPUT_HTML_SPACE_ESCAPER.get().unwrap()).clone()
}
static HTML_ATTRIBUTE_ESCAPER: std::sync::OnceLock<HtmlAttributeEscaper> = std::sync::OnceLock::new();
fn html_attribute_escaper() -> HtmlAttributeEscaper {
    ( * HTML_ATTRIBUTE_ESCAPER.get().unwrap()).clone()
}
struct HtmlCodecStruct {}
#[derive(Clone)]
pub struct HtmlCodec(std::sync::Arc<HtmlCodecStruct>);
impl HtmlCodec {
    pub fn encode(& self, s__345: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let s__345 = s__345.to_arc_string();
        let return__127: std::sync::Arc<String>;
        let mut t___5670: i32;
        let mut t___5673: usize;
        let mut t___5674: usize;
        let mut t___3392: std::sync::Arc<String>;
        let sb__347: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let end__348: usize = s__345.len();
        let mut encodedTo__349: usize = 0usize;
        let mut i__350: usize = 0usize;
        'loop___5914: while Some(i__350) < Some(end__348) {
            'continue___5752: {
                t___5670 = temper_core::string::get( & s__345, i__350);
                if Some(t___5670) == Some(38) {
                    t___3392 = std::sync::Arc::new("&amp;".to_string());
                } else {
                    if Some(t___5670) == Some(60) {
                        t___3392 = std::sync::Arc::new("&lt;".to_string());
                    } else {
                        if Some(t___5670) == Some(62) {
                            t___3392 = std::sync::Arc::new("&gt;".to_string());
                        } else {
                            if Some(t___5670) == Some(39) {
                                t___3392 = std::sync::Arc::new("&#39;".to_string());
                            } else {
                                if Some(t___5670) == Some(34) {
                                    t___3392 = std::sync::Arc::new("&#34;".to_string());
                                } else {
                                    if Some(t___5670) == Some(0) {
                                        t___3392 = std::sync::Arc::new("&#0;".to_string());
                                    } else {
                                        break 'continue___5752;
                                    }
                                }
                            }
                        }
                    }
                }
                let replacement__351: std::sync::Arc<String> = t___3392.clone();
                temper_core::string::builder::append_between( & sb__347, s__345.clone(), encodedTo__349, i__350);
                temper_core::string::builder::append( & sb__347, replacement__351.clone());
                t___5673 = temper_core::string::next( & s__345, i__350);
                encodedTo__349 = t___5673;
            }
            t___5674 = temper_core::string::next( & s__345, i__350);
            i__350 = t___5674;
        }
        if Some(encodedTo__349) > Some(0usize) {
            temper_core::string::builder::append_between( & sb__347, s__345.clone(), encodedTo__349, end__348);
            return__127 = temper_core::string::builder::to_string( & sb__347);
        } else {
            return__127 = s__345.clone();
        }
        return return__127.clone();
    }
    pub fn decode(& self, s__353: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let s__353 = s__353.to_arc_string();
        let return__128: std::sync::Arc<String>;
        let mut t___5647: i32;
        let mut t___5648: usize;
        let mut t___5650: usize;
        let mut t___5652: usize;
        let mut t___5653: i32;
        let mut t___5654: usize;
        let mut t___5659: usize;
        let mut t___5665: usize;
        let mut t___3360: bool;
        let mut t___3366: bool;
        let mut t___3367: bool;
        let mut t___3370: bool;
        let mut t___3373: i32;
        let mut t___3375: bool;
        let mut t___3376: bool;
        let mut t___3380: bool;
        let sb__355: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let end__356: usize = s__353.len();
        let mut decodedTo__357: usize = 0usize;
        let mut i__358: usize = 0usize;
        'loop___5915: while Some(i__358) < Some(end__356) {
            'continue___5753: {
                if Some(temper_core::string::get( & s__353, i__358)) == Some(38) {
                    let startOfEntity__359: usize = temper_core::string::next( & s__353, i__358);
                    let mut endOfEntity__360: usize = startOfEntity__359;
                    if Some(startOfEntity__359) < Some(end__356) {
                        t___5647 = temper_core::string::get( & s__353, startOfEntity__359);
                        t___3360 = t___5647 == "#";
                    } else {
                        t___3360 = false;
                    }
                    if t___3360 {
                        t___5648 = temper_core::string::next( & s__353, startOfEntity__359);
                        endOfEntity__360 = t___5648;
                        if Some(endOfEntity__360) >= Some(end__356) {
                            break 'continue___5753;
                        }
                        let mut base__361: i32 = 10;
                        if Some(temper_core::string::get( & s__353, endOfEntity__360) | 32) == Some(120) {
                            t___5650 = temper_core::string::next( & s__353, endOfEntity__360);
                            endOfEntity__360 = t___5650;
                            base__361 = 16;
                        }
                        let mut digitQuota__362: i32 = 7;
                        let startOfDigits__363: usize = endOfEntity__360;
                        'loop___5916: loop {
                            if ! (Some(endOfEntity__360) < Some(end__356)) {
                                break;
                            }
                            let cp__364: i32 = temper_core::string::get( & s__353, endOfEntity__360);
                            if Some(48) <= Some(cp__364) {
                                t___3366 = Some(cp__364) <= Some(57);
                            } else {
                                t___3366 = false;
                            }
                            if ! t___3366 {
                                if Some(base__361) == Some(16) {
                                    let lcp__365: i32 = cp__364 | 32;
                                    if Some(97) <= Some(lcp__365) {
                                        t___3367 = Some(lcp__365) <= Some(102);
                                    } else {
                                        t___3367 = false;
                                    }
                                    if ! t___3367 {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            t___5652 = temper_core::string::next( & s__353, endOfEntity__360);
                            endOfEntity__360 = t___5652;
                        }
                        let endOfDigits__366: usize = endOfEntity__360;
                        if endOfDigits__366 == startOfDigits__363 {
                            break 'continue___5753;
                        }
                        if Some(endOfEntity__360) < Some(end__356) {
                            t___5653 = temper_core::string::get( & s__353, endOfEntity__360);
                            t___3370 = Some(t___5653) == Some(59);
                        } else {
                            t___3370 = false;
                        }
                        if t___3370 {
                            t___5654 = temper_core::string::next( & s__353, endOfEntity__360);
                            endOfEntity__360 = t___5654;
                        }
                        'ok___5797: {
                            'orelse___939: {
                                t___3373 = match temper_core::string::to_int( & temper_core::string::slice( & s__353, startOfDigits__363, endOfDigits__366), Some(base__361)) {
                                    Ok(x) => x,
                                    _ => break 'orelse___939
                                };
                                break 'ok___5797;
                            }
                            break 'continue___5753;
                        }
                        let decodedCp__367: i32 = t___3373;
                        if Some(0) <= Some(decodedCp__367) {
                            t___3375 = Some(decodedCp__367) <= Some(1114111);
                        } else {
                            t___3375 = false;
                        }
                        if t___3375 {
                            temper_core::string::builder::append_between( & sb__355, s__353.clone(), decodedTo__357, i__358);
                            if Some(55296) <= Some(decodedCp__367) {
                                t___3376 = Some(decodedCp__367) <= Some(57343);
                            } else {
                                t___3376 = false;
                            }
                            if t___3376 {
                                temper_core::string::builder::append( & sb__355, "\u{fffd}");
                            } else {
                                'ok___5798: {
                                    'orelse___940: {
                                        match temper_core::string::builder::append_code_point( & sb__355, decodedCp__367) {
                                            Ok(x) => x,
                                            _ => break 'orelse___940
                                        };
                                        break 'ok___5798;
                                    }
                                    break 'continue___5753;
                                }
                            }
                            decodedTo__357 = endOfEntity__360;
                        }
                    } else {
                        'loop___5917: while Some(endOfEntity__360) < Some(end__356) {
                            let cp__368: i32 = temper_core::string::get( & s__353, endOfEntity__360);
                            t___5659 = temper_core::string::next( & s__353, endOfEntity__360);
                            endOfEntity__360 = t___5659;
                            if Some(cp__368) == Some(59) {
                                break;
                            }
                            let lcp__369: i32 = cp__368 | 32;
                            if Some(97) <= Some(lcp__369) {
                                t___3380 = Some(lcp__369) <= Some(122);
                            } else {
                                t___3380 = false;
                            }
                            if ! t___3380 {
                                break;
                            }
                        }
                        if Some(startOfEntity__359) < Some(endOfEntity__360) {
                            let entityName__370: std::sync::Arc<String> = temper_core::string::slice( & s__353, startOfEntity__359, endOfEntity__360);
                            let entityValue__371: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & html_named_characters(), entityName__370.clone(), std::sync::Arc::new("".to_string()));
                            if ! entityValue__371.is_empty() {
                                temper_core::string::builder::append_between( & sb__355, s__353.clone(), decodedTo__357, i__358);
                                temper_core::string::builder::append( & sb__355, entityValue__371.clone());
                                decodedTo__357 = endOfEntity__360;
                            }
                        }
                    }
                }
            }
            t___5665 = temper_core::string::next( & s__353, i__358);
            i__358 = t___5665;
        }
        if Some(decodedTo__357) > Some(0usize) {
            temper_core::string::builder::append_between( & sb__355, s__353.clone(), decodedTo__357, end__356);
            return__128 = temper_core::string::builder::to_string( & sb__355);
        } else {
            return__128 = s__353.clone();
        }
        return return__128.clone();
    }
    pub fn new() -> HtmlCodec {
        let selfish = HtmlCodec(std::sync::Arc::new(HtmlCodecStruct {}));
        return selfish;
    }
}
impl crate::core::CodecTrait for HtmlCodec {
    fn clone_boxed(& self) -> crate::core::Codec {
        crate::core::Codec::new(self.clone())
    }
    fn encode(& self, s__345: std::sync::Arc<String>) -> std::sync::Arc<String> {
        self.encode(s__345)
    }
    fn decode(& self, s__353: std::sync::Arc<String>) -> std::sync::Arc<String> {
        self.decode(s__353)
    }
}
temper_core::impl_any_value_trait!(HtmlCodec, [crate::core::Codec]);
struct HtmlContextPropagatorStruct {}
#[derive(Clone)]
pub struct HtmlContextPropagator(std::sync::Arc<HtmlContextPropagatorStruct>);
impl HtmlContextPropagator {
    pub fn after(& self, before__452: crate::core::AutoescState<HtmlEscaperContext>, literalPart__453: Option<impl temper_core::ToArcString>) -> crate::core::AfterPropagate<HtmlEscaperContext> {
        let literalPart__453 = literalPart__453.map(| x | x.to_arc_string());
        return htmlPropagateContext__264(before__452.clone(), literalPart__453.clone());
    }
    pub fn new() -> HtmlContextPropagator {
        let selfish = HtmlContextPropagator(std::sync::Arc::new(HtmlContextPropagatorStruct {}));
        return selfish;
    }
}
impl crate::core::ContextPropagatorTrait<HtmlEscaperContext> for HtmlContextPropagator {
    fn clone_boxed(& self) -> crate::core::ContextPropagator<HtmlEscaperContext> {
        crate::core::ContextPropagator::new(self.clone())
    }
    fn after(& self, before__452: crate::core::AutoescState<HtmlEscaperContext>, literalPart__453: Option<std::sync::Arc<String>>) -> crate::core::AfterPropagate<HtmlEscaperContext> {
        self.after(before__452, literalPart__453)
    }
}
temper_core::impl_any_value_trait!(HtmlContextPropagator, [crate::core::ContextPropagator<HtmlEscaperContext>]);
struct UrlContextPropagatorStruct {}
#[derive(Clone)]
pub struct UrlContextPropagator(std::sync::Arc<UrlContextPropagatorStruct>);
impl UrlContextPropagator {
    pub fn after(& self, before__476: crate::core::AutoescState<UrlEscaperContext>, literalPart__477: Option<impl temper_core::ToArcString>) -> crate::core::AfterPropagate<UrlEscaperContext> {
        let literalPart__477 = literalPart__477.map(| x | x.to_arc_string());
        return urlPropagateContext__266(before__476.clone(), literalPart__477.clone());
    }
    pub fn new() -> UrlContextPropagator {
        let selfish = UrlContextPropagator(std::sync::Arc::new(UrlContextPropagatorStruct {}));
        return selfish;
    }
}
impl crate::core::ContextPropagatorTrait<UrlEscaperContext> for UrlContextPropagator {
    fn clone_boxed(& self) -> crate::core::ContextPropagator<UrlEscaperContext> {
        crate::core::ContextPropagator::new(self.clone())
    }
    fn after(& self, before__476: crate::core::AutoescState<UrlEscaperContext>, literalPart__477: Option<std::sync::Arc<String>>) -> crate::core::AfterPropagate<UrlEscaperContext> {
        self.after(before__476, literalPart__477)
    }
}
temper_core::impl_any_value_trait!(UrlContextPropagator, [crate::core::ContextPropagator<UrlEscaperContext>]);
struct HtmlEscaperContextStruct {
    html_state: i32, tag_state: i32, attrib_state: i32, delim_state: i32
}
#[derive(Clone)]
pub struct HtmlEscaperContext(std::sync::Arc<HtmlEscaperContextStruct>);
#[derive(Clone)]
pub struct HtmlEscaperContextBuilder {
    pub html_state: i32, pub tag_state: i32, pub attrib_state: i32, pub delim_state: i32
}
impl HtmlEscaperContextBuilder {
    pub fn build(self) -> HtmlEscaperContext {
        HtmlEscaperContext::new(self.html_state, self.tag_state, self.attrib_state, self.delim_state)
    }
}
impl HtmlEscaperContext {
    pub fn to_string(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!("HtmlEscaperContext({}, {}, {}, {})", htmlStateStr__260(self.0.html_state), tagStateStr__261(self.0.tag_state), attribStateStr__262(self.0.attrib_state), delimStateStr__263(self.0.delim_state)));
    }
    pub fn new(htmlState__503: i32, tagState__504: i32, attribState__505: i32, delimState__506: i32) -> HtmlEscaperContext {
        let html_state;
        let tag_state;
        let attrib_state;
        let delim_state;
        html_state = htmlState__503;
        tag_state = tagState__504;
        attrib_state = attribState__505;
        delim_state = delimState__506;
        let selfish = HtmlEscaperContext(std::sync::Arc::new(HtmlEscaperContextStruct {
                    html_state, tag_state, attrib_state, delim_state
        }));
        return selfish;
    }
    pub fn html_state(& self) -> i32 {
        return self.0.html_state;
    }
    pub fn tag_state(& self) -> i32 {
        return self.0.tag_state;
    }
    pub fn attrib_state(& self) -> i32 {
        return self.0.attrib_state;
    }
    pub fn delim_state(& self) -> i32 {
        return self.0.delim_state;
    }
}
impl crate::core::ContextTrait for HtmlEscaperContext {
    fn clone_boxed(& self) -> crate::core::Context {
        crate::core::Context::new(self.clone())
    }
    fn to_string(& self) -> std::sync::Arc<String> {
        self.to_string()
    }
}
temper_core::impl_any_value_trait!(HtmlEscaperContext, [crate::core::Context]);
struct JsEscaperContextStruct {
    js_state: i32, js_stack: temper_core::List<bool>
}
#[derive(Clone)]
pub (crate) struct JsEscaperContext(std::sync::Arc<JsEscaperContextStruct>);
impl JsEscaperContext {
    pub fn to_string(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!("JsEscaperContext({})", self.0.js_state));
    }
    pub fn new(jsState__512: i32, jsStack__513: impl temper_core::ToList<bool>) -> JsEscaperContext {
        let jsStack__513 = jsStack__513.to_list();
        let js_state;
        let js_stack;
        js_state = jsState__512;
        js_stack = jsStack__513.clone();
        let selfish = JsEscaperContext(std::sync::Arc::new(JsEscaperContextStruct {
                    js_state, js_stack
        }));
        return selfish;
    }
    pub fn js_state(& self) -> i32 {
        return self.0.js_state;
    }
    pub fn js_stack(& self) -> temper_core::List<bool> {
        return self.0.js_stack.clone();
    }
}
impl crate::core::ContextTrait for JsEscaperContext {
    fn clone_boxed(& self) -> crate::core::Context {
        crate::core::Context::new(self.clone())
    }
    fn to_string(& self) -> std::sync::Arc<String> {
        self.to_string()
    }
}
temper_core::impl_any_value_trait!(JsEscaperContext, [crate::core::Context]);
struct CssEscaperContextStruct {
    css_state: i32
}
#[derive(Clone)]
pub (crate) struct CssEscaperContext(std::sync::Arc<CssEscaperContextStruct>);
impl CssEscaperContext {
    pub fn to_string(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!("CssEscaperContext({})", self.0.css_state));
    }
    pub fn new(cssState__518: i32) -> CssEscaperContext {
        let css_state;
        css_state = cssState__518;
        let selfish = CssEscaperContext(std::sync::Arc::new(CssEscaperContextStruct {
                    css_state
        }));
        return selfish;
    }
    pub fn css_state(& self) -> i32 {
        return self.0.css_state;
    }
}
impl crate::core::ContextTrait for CssEscaperContext {
    fn clone_boxed(& self) -> crate::core::Context {
        crate::core::Context::new(self.clone())
    }
    fn to_string(& self) -> std::sync::Arc<String> {
        self.to_string()
    }
}
temper_core::impl_any_value_trait!(CssEscaperContext, [crate::core::Context]);
struct UrlEscaperContextStruct {
    url_state: i32
}
#[derive(Clone)]
pub struct UrlEscaperContext(std::sync::Arc<UrlEscaperContextStruct>);
impl UrlEscaperContext {
    pub fn to_string(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!("UrlEscaperContext({})", urlStateStr__265(self.0.url_state)));
    }
    pub fn new(urlState__523: i32) -> UrlEscaperContext {
        let url_state;
        url_state = urlState__523;
        let selfish = UrlEscaperContext(std::sync::Arc::new(UrlEscaperContextStruct {
                    url_state
        }));
        return selfish;
    }
    pub fn url_state(& self) -> i32 {
        return self.0.url_state;
    }
}
impl crate::core::ContextTrait for UrlEscaperContext {
    fn clone_boxed(& self) -> crate::core::Context {
        crate::core::Context::new(self.clone())
    }
    fn to_string(& self) -> std::sync::Arc<String> {
        self.to_string()
    }
}
temper_core::impl_any_value_trait!(UrlEscaperContext, [crate::core::Context]);
struct SafeHtmlStruct {
    text: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SafeHtml(std::sync::Arc<SafeHtmlStruct>);
impl SafeHtml {
    pub fn to_string(& self) -> std::sync::Arc<String> {
        return self.0.text.clone();
    }
    pub fn new(text__528: impl temper_core::ToArcString) -> SafeHtml {
        let text__528 = text__528.to_arc_string();
        let text;
        text = text__528.clone();
        let selfish = SafeHtml(std::sync::Arc::new(SafeHtmlStruct {
                    text
        }));
        return selfish;
    }
    pub fn text(& self) -> std::sync::Arc<String> {
        return self.0.text.clone();
    }
}
temper_core::impl_any_value_trait!(SafeHtml, []);
struct SafeUrlStruct {
    text: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SafeUrl(std::sync::Arc<SafeUrlStruct>);
impl SafeUrl {
    pub fn to_string(& self) -> std::sync::Arc<String> {
        return self.0.text.clone();
    }
    pub fn new(text__533: impl temper_core::ToArcString) -> SafeUrl {
        let text__533 = text__533.to_arc_string();
        let text;
        text = text__533.clone();
        let selfish = SafeUrl(std::sync::Arc::new(SafeUrlStruct {
                    text
        }));
        return selfish;
    }
    pub fn text(& self) -> std::sync::Arc<String> {
        return self.0.text.clone();
    }
}
temper_core::impl_any_value_trait!(SafeUrl, []);
pub enum HtmlEscaperEnum {
    HtmlUrlEscaperAdapter(HtmlUrlEscaperAdapter), HtmlPcdataEscaper(HtmlPcdataEscaper), OutputHtmlSpaceEscaper(OutputHtmlSpaceEscaper), HtmlAttributeEscaper(HtmlAttributeEscaper)
}
pub trait HtmlEscaperTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + crate::core::EscaperTrait {
    fn as_enum(& self) -> HtmlEscaperEnum;
    fn clone_boxed(& self) -> HtmlEscaper;
    fn apply_safe_html(& self, x__535: SafeHtml) -> std::sync::Arc<String>;
    fn apply_safe_url(& self, x__538: SafeUrl) -> std::sync::Arc<String>;
    fn apply_int32(& self, x__541: i32) -> std::sync::Arc<String>;
    fn apply_int64(& self, x__544: i64) -> std::sync::Arc<String>;
    fn apply_float64(& self, x__547: f64) -> std::sync::Arc<String>;
    fn apply_string(& self, x__550: std::sync::Arc<String>) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct HtmlEscaper(std::sync::Arc<dyn HtmlEscaperTrait>);
impl HtmlEscaper {
    pub fn new(selfish: impl HtmlEscaperTrait + 'static) -> HtmlEscaper {
        HtmlEscaper(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(HtmlEscaper);
impl std::ops::Deref for HtmlEscaper {
    type Target = dyn HtmlEscaperTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct OutputHtmlSpaceEscaperStruct {}
#[derive(Clone)]
pub struct OutputHtmlSpaceEscaper(std::sync::Arc<OutputHtmlSpaceEscaperStruct>);
static OUTPUT_HTML_SPACE_ESCAPER__INSTANCE: std::sync::OnceLock<OutputHtmlSpaceEscaper> = std::sync::OnceLock::new();
impl OutputHtmlSpaceEscaper {
    pub fn instance() -> OutputHtmlSpaceEscaper {
        ( * OUTPUT_HTML_SPACE_ESCAPER__INSTANCE.get().unwrap()).clone()
    }
    pub fn apply_safe_html(& self, x__554: SafeHtml) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" ".to_string());
    }
    pub fn apply_safe_url(& self, x__557: SafeUrl) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" ".to_string());
    }
    pub fn apply_int32(& self, x__560: i32) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" ".to_string());
    }
    pub fn apply_int64(& self, x__563: i64) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" ".to_string());
    }
    pub fn apply_float64(& self, x__566: f64) -> std::sync::Arc<String> {
        return std::sync::Arc::new(" ".to_string());
    }
    pub fn apply_string(& self, x__569: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let x__569 = x__569.to_arc_string();
        return std::sync::Arc::new(" ".to_string());
    }
    pub fn new() -> OutputHtmlSpaceEscaper {
        let selfish = OutputHtmlSpaceEscaper(std::sync::Arc::new(OutputHtmlSpaceEscaperStruct {}));
        return selfish;
    }
}
impl HtmlEscaperTrait for OutputHtmlSpaceEscaper {
    fn as_enum(& self) -> HtmlEscaperEnum {
        HtmlEscaperEnum::OutputHtmlSpaceEscaper(self.clone())
    }
    fn clone_boxed(& self) -> HtmlEscaper {
        HtmlEscaper::new(self.clone())
    }
    fn apply_safe_html(& self, x__554: SafeHtml) -> std::sync::Arc<String> {
        self.apply_safe_html(x__554)
    }
    fn apply_safe_url(& self, x__557: SafeUrl) -> std::sync::Arc<String> {
        self.apply_safe_url(x__557)
    }
    fn apply_int32(& self, x__560: i32) -> std::sync::Arc<String> {
        self.apply_int32(x__560)
    }
    fn apply_int64(& self, x__563: i64) -> std::sync::Arc<String> {
        self.apply_int64(x__563)
    }
    fn apply_float64(& self, x__566: f64) -> std::sync::Arc<String> {
        self.apply_float64(x__566)
    }
    fn apply_string(& self, x__569: std::sync::Arc<String>) -> std::sync::Arc<String> {
        self.apply_string(x__569)
    }
}
impl crate::core::EscaperTrait for OutputHtmlSpaceEscaper {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(OutputHtmlSpaceEscaper, [HtmlEscaper, crate::core::Escaper]);
struct HtmlPcdataEscaperStruct {}
#[derive(Clone)]
pub struct HtmlPcdataEscaper(std::sync::Arc<HtmlPcdataEscaperStruct>);
static HTML_PCDATA_ESCAPER__INSTANCE: std::sync::OnceLock<HtmlPcdataEscaper> = std::sync::OnceLock::new();
impl HtmlPcdataEscaper {
    pub fn instance() -> HtmlPcdataEscaper {
        ( * HTML_PCDATA_ESCAPER__INSTANCE.get().unwrap()).clone()
    }
    pub fn apply_safe_html(& self, x__575: SafeHtml) -> std::sync::Arc<String> {
        return x__575.to_string();
    }
    pub fn apply_safe_url(& self, x__578: SafeUrl) -> std::sync::Arc<String> {
        let mut t___4740: std::sync::Arc<String> = x__578.text();
        return self.apply_string(t___4740.clone());
    }
    pub fn apply_int32(& self, x__581: i32) -> std::sync::Arc<String> {
        let mut t___4738: std::sync::Arc<String> = temper_core::int_to_string(x__581, None);
        return self.apply_string(t___4738.clone());
    }
    pub fn apply_int64(& self, x__584: i64) -> std::sync::Arc<String> {
        let mut t___4736: std::sync::Arc<String> = temper_core::int64_to_string(x__584, None);
        return self.apply_string(t___4736.clone());
    }
    pub fn apply_float64(& self, x__587: f64) -> std::sync::Arc<String> {
        let mut t___4734: std::sync::Arc<String> = temper_core::float64::to_string(x__587);
        return self.apply_string(t___4734.clone());
    }
    pub fn apply_string(& self, x__590: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let x__590 = x__590.to_arc_string();
        return html_codec().encode(x__590.clone());
    }
    pub fn new() -> HtmlPcdataEscaper {
        let selfish = HtmlPcdataEscaper(std::sync::Arc::new(HtmlPcdataEscaperStruct {}));
        return selfish;
    }
}
impl HtmlEscaperTrait for HtmlPcdataEscaper {
    fn as_enum(& self) -> HtmlEscaperEnum {
        HtmlEscaperEnum::HtmlPcdataEscaper(self.clone())
    }
    fn clone_boxed(& self) -> HtmlEscaper {
        HtmlEscaper::new(self.clone())
    }
    fn apply_safe_html(& self, x__575: SafeHtml) -> std::sync::Arc<String> {
        self.apply_safe_html(x__575)
    }
    fn apply_safe_url(& self, x__578: SafeUrl) -> std::sync::Arc<String> {
        self.apply_safe_url(x__578)
    }
    fn apply_int32(& self, x__581: i32) -> std::sync::Arc<String> {
        self.apply_int32(x__581)
    }
    fn apply_int64(& self, x__584: i64) -> std::sync::Arc<String> {
        self.apply_int64(x__584)
    }
    fn apply_float64(& self, x__587: f64) -> std::sync::Arc<String> {
        self.apply_float64(x__587)
    }
    fn apply_string(& self, x__590: std::sync::Arc<String>) -> std::sync::Arc<String> {
        self.apply_string(x__590)
    }
}
impl crate::core::EscaperTrait for HtmlPcdataEscaper {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(HtmlPcdataEscaper, [HtmlEscaper, crate::core::Escaper]);
struct HtmlAttributeEscaperStruct {}
#[derive(Clone)]
pub struct HtmlAttributeEscaper(std::sync::Arc<HtmlAttributeEscaperStruct>);
static HTML_ATTRIBUTE_ESCAPER__INSTANCE: std::sync::OnceLock<HtmlAttributeEscaper> = std::sync::OnceLock::new();
impl HtmlAttributeEscaper {
    pub fn instance() -> HtmlAttributeEscaper {
        ( * HTML_ATTRIBUTE_ESCAPER__INSTANCE.get().unwrap()).clone()
    }
    pub fn apply_safe_html(& self, x__596: SafeHtml) -> std::sync::Arc<String> {
        let mut t___4731: std::sync::Arc<String> = html_codec().decode(x__596.text());
        return self.apply_string(t___4731.clone());
    }
    pub fn apply_safe_url(& self, x__599: SafeUrl) -> std::sync::Arc<String> {
        let mut t___4728: std::sync::Arc<String> = x__599.text();
        return self.apply_string(t___4728.clone());
    }
    pub fn apply_int32(& self, x__602: i32) -> std::sync::Arc<String> {
        let mut t___4726: std::sync::Arc<String> = temper_core::int_to_string(x__602, None);
        return self.apply_string(t___4726.clone());
    }
    pub fn apply_int64(& self, x__605: i64) -> std::sync::Arc<String> {
        let mut t___4724: std::sync::Arc<String> = temper_core::int64_to_string(x__605, None);
        return self.apply_string(t___4724.clone());
    }
    pub fn apply_float64(& self, x__608: f64) -> std::sync::Arc<String> {
        let mut t___4722: std::sync::Arc<String> = temper_core::float64::to_string(x__608);
        return self.apply_string(t___4722.clone());
    }
    pub fn apply_string(& self, x__611: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let x__611 = x__611.to_arc_string();
        return html_codec().encode(x__611.clone());
    }
    pub fn new() -> HtmlAttributeEscaper {
        let selfish = HtmlAttributeEscaper(std::sync::Arc::new(HtmlAttributeEscaperStruct {}));
        return selfish;
    }
}
impl HtmlEscaperTrait for HtmlAttributeEscaper {
    fn as_enum(& self) -> HtmlEscaperEnum {
        HtmlEscaperEnum::HtmlAttributeEscaper(self.clone())
    }
    fn clone_boxed(& self) -> HtmlEscaper {
        HtmlEscaper::new(self.clone())
    }
    fn apply_safe_html(& self, x__596: SafeHtml) -> std::sync::Arc<String> {
        self.apply_safe_html(x__596)
    }
    fn apply_safe_url(& self, x__599: SafeUrl) -> std::sync::Arc<String> {
        self.apply_safe_url(x__599)
    }
    fn apply_int32(& self, x__602: i32) -> std::sync::Arc<String> {
        self.apply_int32(x__602)
    }
    fn apply_int64(& self, x__605: i64) -> std::sync::Arc<String> {
        self.apply_int64(x__605)
    }
    fn apply_float64(& self, x__608: f64) -> std::sync::Arc<String> {
        self.apply_float64(x__608)
    }
    fn apply_string(& self, x__611: std::sync::Arc<String>) -> std::sync::Arc<String> {
        self.apply_string(x__611)
    }
}
impl crate::core::EscaperTrait for HtmlAttributeEscaper {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(HtmlAttributeEscaper, [HtmlEscaper, crate::core::Escaper]);
struct HtmlEscaperPickerStruct {}
#[derive(Clone)]
pub struct HtmlEscaperPicker(std::sync::Arc<HtmlEscaperPickerStruct>);
impl HtmlEscaperPicker {
    pub fn escaper_for(& self, stateBefore__621: crate::core::AutoescState<HtmlEscaperContext>) -> HtmlEscaper {
        return pickHtmlEscaper(stateBefore__621.clone());
    }
    pub fn new() -> HtmlEscaperPicker {
        let selfish = HtmlEscaperPicker(std::sync::Arc::new(HtmlEscaperPickerStruct {}));
        return selfish;
    }
}
impl crate::core::EscaperPickerTrait<HtmlEscaperContext, HtmlEscaper> for HtmlEscaperPicker {
    fn clone_boxed(& self) -> crate::core::EscaperPicker<HtmlEscaperContext, HtmlEscaper> {
        crate::core::EscaperPicker::new(self.clone())
    }
    fn escaper_for(& self, stateBefore__621: crate::core::AutoescState<HtmlEscaperContext>) -> HtmlEscaper {
        self.escaper_for(stateBefore__621)
    }
}
temper_core::impl_any_value_trait!(HtmlEscaperPicker, [crate::core::EscaperPicker<HtmlEscaperContext, HtmlEscaper>]);
struct SafeHtmlBuilderStruct {
    state: crate::core::AutoescState<HtmlEscaperContext>, collector: std::sync::Arc<std::sync::RwLock<String>>
}
#[derive(Clone)]
pub struct SafeHtmlBuilder(std::sync::Arc<std::sync::RwLock<SafeHtmlBuilderStruct>>);
impl SafeHtmlBuilder {
    pub fn new_collector() -> std::sync::Arc<std::sync::RwLock<String>> {
        return std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    }
    pub fn initial_state() -> crate::core::AutoescState<HtmlEscaperContext> {
        return crate::core::AutoescState::new(HtmlEscaperContext::new(0, 0, 0, 0), None);
    }
    pub fn propagator() -> HtmlContextPropagator {
        return HtmlContextPropagator::new();
    }
    pub fn picker() -> crate::core::EscaperPicker<HtmlEscaperContext, HtmlEscaper> {
        return crate::core::EscaperPicker::new(HtmlEscaperPicker::new());
    }
    pub fn from_collector(collector__633: std::sync::Arc<std::sync::RwLock<String>>) -> SafeHtml {
        return SafeHtml::new(temper_core::string::builder::to_string( & collector__633));
    }
    pub fn merge_states(a__636: crate::core::AutoescState<HtmlEscaperContext>, b__637: crate::core::AutoescState<HtmlEscaperContext>) -> crate::core::AutoescState<HtmlEscaperContext> {
        return a__636.clone();
    }
    fn state(& self) -> crate::core::AutoescState<HtmlEscaperContext> {
        return self.0.read().unwrap().state.clone();
    }
    fn set_state(& self, x__644: crate::core::AutoescState<HtmlEscaperContext>) {
        self.0.write().unwrap().state = x__644.clone();
    }
    pub fn escaper_picker(& self) -> crate::core::EscaperPicker<HtmlEscaperContext, HtmlEscaper> {
        return SafeHtmlBuilder::picker();
    }
    pub fn context_propagator(& self) -> crate::core::ContextPropagator<HtmlEscaperContext> {
        return crate::core::ContextPropagator::new(SafeHtmlBuilder::propagator());
    }
    pub fn accumulated(& self) -> SafeHtml {
        return SafeHtmlBuilder::from_collector(self.0.read().unwrap().collector.clone());
    }
    fn collect_fixed(& self, fixed__653: impl temper_core::ToArcString) {
        let fixed__653 = fixed__653.to_arc_string();
        temper_core::string::builder::append( & self.0.read().unwrap().collector, fixed__653.clone());
    }
    pub fn append_safe_html(& self, x__656: SafeHtml) {
        let mut t___4702: std::sync::Arc<String> = self.prepare_for_append().apply_safe_html(x__656.clone());
        temper_core::string::builder::append( & self.0.read().unwrap().collector, t___4702.clone());
    }
    pub fn append_safe_url(& self, x__659: SafeUrl) {
        let mut t___4699: std::sync::Arc<String> = self.prepare_for_append().apply_safe_url(x__659.clone());
        temper_core::string::builder::append( & self.0.read().unwrap().collector, t___4699.clone());
    }
    pub fn append_int32(& self, x__662: i32) {
        let mut t___4696: std::sync::Arc<String> = self.prepare_for_append().apply_int32(x__662);
        temper_core::string::builder::append( & self.0.read().unwrap().collector, t___4696.clone());
    }
    pub fn append_int64(& self, x__665: i64) {
        let mut t___4693: std::sync::Arc<String> = self.prepare_for_append().apply_int64(x__665);
        temper_core::string::builder::append( & self.0.read().unwrap().collector, t___4693.clone());
    }
    pub fn append_float64(& self, x__668: f64) {
        let mut t___4690: std::sync::Arc<String> = self.prepare_for_append().apply_float64(x__668);
        temper_core::string::builder::append( & self.0.read().unwrap().collector, t___4690.clone());
    }
    pub fn append_string(& self, x__671: impl temper_core::ToArcString) {
        let x__671 = x__671.to_arc_string();
        let mut t___4687: std::sync::Arc<String> = self.prepare_for_append().apply_string(x__671.clone());
        temper_core::string::builder::append( & self.0.read().unwrap().collector, t___4687.clone());
    }
    pub fn new() -> SafeHtmlBuilder {
        let state;
        let collector;
        let mut t___4684: crate::core::AutoescState<HtmlEscaperContext> = SafeHtmlBuilder::initial_state();
        state = t___4684.clone();
        let mut t___4685: std::sync::Arc<std::sync::RwLock<String>> = SafeHtmlBuilder::new_collector();
        collector = t___4685.clone();
        let selfish = SafeHtmlBuilder(std::sync::Arc::new(std::sync::RwLock::new(SafeHtmlBuilderStruct {
                        state, collector
        })));
        return selfish;
    }
}
impl crate::core::ContextualAutoescapingAccumulatorTrait<HtmlEscaperContext, HtmlEscaper> for SafeHtmlBuilder {
    fn clone_boxed(& self) -> crate::core::ContextualAutoescapingAccumulator<HtmlEscaperContext, HtmlEscaper> {
        crate::core::ContextualAutoescapingAccumulator::new(self.clone())
    }
    fn state(& self) -> crate::core::AutoescState<HtmlEscaperContext> {
        self.state()
    }
    fn set_state(& self, x__644: crate::core::AutoescState<HtmlEscaperContext>) {
        self.set_state(x__644)
    }
    fn escaper_picker(& self) -> crate::core::EscaperPicker<HtmlEscaperContext, HtmlEscaper> {
        self.escaper_picker()
    }
    fn context_propagator(& self) -> crate::core::ContextPropagator<HtmlEscaperContext> {
        self.context_propagator()
    }
    fn collect_fixed(& self, fixed__653: std::sync::Arc<String>) {
        self.collect_fixed(fixed__653)
    }
}
temper_core::impl_any_value_trait!(SafeHtmlBuilder, [crate::core::ContextualAutoescapingAccumulator<HtmlEscaperContext, HtmlEscaper>]);
pub enum HtmlDelegateEnum {
    HtmlUrlDelegate(HtmlUrlDelegate), HtmlCssDelegate(HtmlCssDelegate), HtmlJsDelegate(HtmlJsDelegate)
}
pub trait HtmlDelegateTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + crate::core::DelegateTrait {
    fn as_enum(& self) -> HtmlDelegateEnum;
    fn clone_boxed(& self) -> HtmlDelegate;
    fn escaper(& self, outer__675: HtmlEscaper) -> HtmlEscaper;
}
#[derive(Clone)]
pub struct HtmlDelegate(std::sync::Arc<dyn HtmlDelegateTrait>);
impl HtmlDelegate {
    pub fn new(selfish: impl HtmlDelegateTrait + 'static) -> HtmlDelegate {
        HtmlDelegate(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(HtmlDelegate);
impl std::ops::Deref for HtmlDelegate {
    type Target = dyn HtmlDelegateTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct HtmlUrlDelegateStruct {
    state: crate::core::AutoescState<UrlEscaperContext>, subsidiary: Option<crate::core::Subsidiary>
}
#[derive(Clone)]
pub struct HtmlUrlDelegate(std::sync::Arc<std::sync::RwLock<HtmlUrlDelegateStruct>>);
impl HtmlUrlDelegate {
    fn state(& self) -> crate::core::AutoescState<UrlEscaperContext> {
        return self.0.read().unwrap().state.clone();
    }
    fn set_state(& self, x__682: crate::core::AutoescState<UrlEscaperContext>) {
        self.0.write().unwrap().state = x__682.clone();
    }
    fn context_propagator(& self) -> crate::core::ContextPropagator<UrlEscaperContext> {
        return crate::core::ContextPropagator::new(url_context_propagator().clone());
    }
    pub fn escaper(& self, outer__687: HtmlEscaper) -> HtmlEscaper {
        let return__229: HtmlEscaper;
        let mut t___3102: bool;
        let mut t___5402: i32 = self.state().context().url_state();
        if Some(t___5402) == Some(0) {
            return__229 = HtmlEscaper::new(HtmlUrlEscaperAdapter::new(UrlEscaper::new(html_protocol_filtering_url_escaper().clone()), outer__687.clone()));
        } else {
            if Some(t___5402) == Some(1) {
                return__229 = HtmlEscaper::new(HtmlUrlEscaperAdapter::new(UrlEscaper::new(html_url_part_url_escaper().clone()), outer__687.clone()));
            } else {
                if Some(t___5402) == Some(2) {
                    t___3102 = true;
                } else {
                    t___3102 = Some(t___5402) == Some(3);
                }
                if t___3102 {
                    return__229 = HtmlEscaper::new(HtmlUrlEscaperAdapter::new(UrlEscaper::new(html_as_if_query_url_escaper().clone()), outer__687.clone()));
                } else {
                    return__229 = temper_core::cast::<HtmlEscaper>(panic!()).unwrap();
                }
            }
        }
        return return__229.clone();
    }
    pub fn new() -> HtmlUrlDelegate {
        let state;
        let subsidiary;
        let mut t___5399: crate::core::AutoescState<UrlEscaperContext> = crate::core::AutoescState::new(UrlEscaperContext::new(0), None);
        state = t___5399.clone();
        subsidiary = None;
        let selfish = HtmlUrlDelegate(std::sync::Arc::new(std::sync::RwLock::new(HtmlUrlDelegateStruct {
                        state, subsidiary
        })));
        return selfish;
    }
}
impl crate::core::ContextDelegateTrait<UrlEscaperContext> for HtmlUrlDelegate {
    fn clone_boxed(& self) -> crate::core::ContextDelegate<UrlEscaperContext> {
        crate::core::ContextDelegate::new(self.clone())
    }
    fn state(& self) -> crate::core::AutoescState<UrlEscaperContext> {
        self.state()
    }
    fn set_state(& self, x__682: crate::core::AutoescState<UrlEscaperContext>) {
        self.set_state(x__682)
    }
    fn context_propagator(& self) -> crate::core::ContextPropagator<UrlEscaperContext> {
        self.context_propagator()
    }
}
impl crate::core::DelegateTrait for HtmlUrlDelegate {
    fn clone_boxed(& self) -> crate::core::Delegate {
        crate::core::Delegate::new(self.clone())
    }
}
impl HtmlDelegateTrait for HtmlUrlDelegate {
    fn as_enum(& self) -> HtmlDelegateEnum {
        HtmlDelegateEnum::HtmlUrlDelegate(self.clone())
    }
    fn clone_boxed(& self) -> HtmlDelegate {
        HtmlDelegate::new(self.clone())
    }
    fn escaper(& self, outer__687: HtmlEscaper) -> HtmlEscaper {
        self.escaper(outer__687)
    }
}
temper_core::impl_any_value_trait!(HtmlUrlDelegate, [crate::core::ContextDelegate<UrlEscaperContext>, crate::core::Delegate, HtmlDelegate]);
struct HtmlUrlEscaperAdapterStruct {
    first: UrlEscaper, second: HtmlEscaper
}
#[derive(Clone)]
pub struct HtmlUrlEscaperAdapter(std::sync::Arc<HtmlUrlEscaperAdapterStruct>);
#[derive(Clone)]
pub struct HtmlUrlEscaperAdapterBuilder {
    pub first: UrlEscaper, pub second: HtmlEscaper
}
impl HtmlUrlEscaperAdapterBuilder {
    pub fn build(self) -> HtmlUrlEscaperAdapter {
        HtmlUrlEscaperAdapter::new(self.first, self.second)
    }
}
impl HtmlUrlEscaperAdapter {
    pub fn apply_safe_html(& self, x__693: SafeHtml) -> std::sync::Arc<String> {
        let mut t___5449: std::sync::Arc<String> = x__693.text();
        let mut t___5450: SafeUrl = self.0.first.apply_string(t___5449.clone());
        return self.0.second.apply_safe_url(t___5450.clone());
    }
    pub fn apply_safe_url(& self, x__696: SafeUrl) -> std::sync::Arc<String> {
        let mut t___5447: SafeUrl = self.0.first.apply_safe_url(x__696.clone());
        return self.0.second.apply_safe_url(t___5447.clone());
    }
    pub fn apply_int32(& self, x__699: i32) -> std::sync::Arc<String> {
        let mut t___5444: std::sync::Arc<String> = temper_core::int_to_string(x__699, None);
        let mut t___5445: SafeUrl = self.0.first.apply_string(t___5444.clone());
        return self.0.second.apply_safe_url(t___5445.clone());
    }
    pub fn apply_int64(& self, x__702: i64) -> std::sync::Arc<String> {
        let mut t___5441: std::sync::Arc<String> = temper_core::int64_to_string(x__702, None);
        let mut t___5442: SafeUrl = self.0.first.apply_string(t___5441.clone());
        return self.0.second.apply_safe_url(t___5442.clone());
    }
    pub fn apply_float64(& self, x__705: f64) -> std::sync::Arc<String> {
        let mut t___5438: std::sync::Arc<String> = temper_core::float64::to_string(x__705);
        let mut t___5439: SafeUrl = self.0.first.apply_string(t___5438.clone());
        return self.0.second.apply_safe_url(t___5439.clone());
    }
    pub fn apply_string(& self, x__708: impl temper_core::ToArcString) -> std::sync::Arc<String> {
        let x__708 = x__708.to_arc_string();
        let mut t___5436: SafeUrl = self.0.first.apply_string(x__708.clone());
        return self.0.second.apply_safe_url(t___5436.clone());
    }
    pub fn new(first__711: UrlEscaper, second__712: HtmlEscaper) -> HtmlUrlEscaperAdapter {
        let first;
        let second;
        first = first__711.clone();
        second = second__712.clone();
        let selfish = HtmlUrlEscaperAdapter(std::sync::Arc::new(HtmlUrlEscaperAdapterStruct {
                    first, second
        }));
        return selfish;
    }
    pub fn first(& self) -> UrlEscaper {
        return self.0.first.clone();
    }
    pub fn second(& self) -> HtmlEscaper {
        return self.0.second.clone();
    }
}
impl HtmlEscaperTrait for HtmlUrlEscaperAdapter {
    fn as_enum(& self) -> HtmlEscaperEnum {
        HtmlEscaperEnum::HtmlUrlEscaperAdapter(self.clone())
    }
    fn clone_boxed(& self) -> HtmlEscaper {
        HtmlEscaper::new(self.clone())
    }
    fn apply_safe_html(& self, x__693: SafeHtml) -> std::sync::Arc<String> {
        self.apply_safe_html(x__693)
    }
    fn apply_safe_url(& self, x__696: SafeUrl) -> std::sync::Arc<String> {
        self.apply_safe_url(x__696)
    }
    fn apply_int32(& self, x__699: i32) -> std::sync::Arc<String> {
        self.apply_int32(x__699)
    }
    fn apply_int64(& self, x__702: i64) -> std::sync::Arc<String> {
        self.apply_int64(x__702)
    }
    fn apply_float64(& self, x__705: f64) -> std::sync::Arc<String> {
        self.apply_float64(x__705)
    }
    fn apply_string(& self, x__708: std::sync::Arc<String>) -> std::sync::Arc<String> {
        self.apply_string(x__708)
    }
}
impl crate::core::EscaperTrait for HtmlUrlEscaperAdapter {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(HtmlUrlEscaperAdapter, [HtmlEscaper, crate::core::Escaper]);
pub enum UrlEscaperEnum {
    HtmlUrlPartUrlEscaper(HtmlUrlPartUrlEscaper), HtmlProtocolFilteringUrlEscaper(HtmlProtocolFilteringUrlEscaper), HtmlAsIfQueryUrlEscaper(HtmlAsIfQueryUrlEscaper)
}
pub trait UrlEscaperTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + crate::core::EscaperTrait {
    fn as_enum(& self) -> UrlEscaperEnum;
    fn clone_boxed(& self) -> UrlEscaper;
    fn apply_safe_url(& self, x__714: SafeUrl) -> SafeUrl;
    fn apply_string(& self, x__717: std::sync::Arc<String>) -> SafeUrl;
}
#[derive(Clone)]
pub struct UrlEscaper(std::sync::Arc<dyn UrlEscaperTrait>);
impl UrlEscaper {
    pub fn new(selfish: impl UrlEscaperTrait + 'static) -> UrlEscaper {
        UrlEscaper(std::sync::Arc::new(selfish))
    }
}
temper_core::impl_any_value_trait_for_interface!(UrlEscaper);
impl std::ops::Deref for UrlEscaper {
    type Target = dyn UrlEscaperTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct HtmlProtocolFilteringUrlEscaperStruct {}
#[derive(Clone)]
pub struct HtmlProtocolFilteringUrlEscaper(std::sync::Arc<HtmlProtocolFilteringUrlEscaperStruct>);
static HTML_PROTOCOL_FILTERING_URL_ESCAPER__INSTANCE: std::sync::OnceLock<HtmlProtocolFilteringUrlEscaper> = std::sync::OnceLock::new();
impl HtmlProtocolFilteringUrlEscaper {
    pub fn instance() -> HtmlProtocolFilteringUrlEscaper {
        ( * HTML_PROTOCOL_FILTERING_URL_ESCAPER__INSTANCE.get().unwrap()).clone()
    }
    pub fn apply_safe_url(& self, x__721: SafeUrl) -> SafeUrl {
        return x__721.clone();
    }
    pub fn apply_string(& self, x__724: impl temper_core::ToArcString) -> SafeUrl {
        let x__724 = x__724.to_arc_string();
        let return__243: SafeUrl;
        let mut t___5422: usize;
        let mut t___3125: temper_std::regex::Match;
        let mut t___3126: Option<temper_std::regex::Match>;
        'fn__725: {
            let mut protocolEnd__726: usize = 0usize;
            let end__727: usize = x__724.len();
            'loop___5920: while Some(protocolEnd__726) < Some(end__727) {
                let cp__728: i32 = temper_core::string::get( & x__724, protocolEnd__726);
                if Some(cp__728) == Some(58) {
                    let protocol__729: std::sync::Arc<String> = temper_core::string::slice( & x__724, 0usize, protocolEnd__726);
                    'ok___5806: {
                        'orelse___949: {
                            t___3125 = match protocol_allow_list().find(protocol__729.clone(), None) {
                                Ok(x) => x,
                                _ => break 'orelse___949
                            };
                            t___3126 = Some(t___3125.clone());
                            break 'ok___5806;
                        }
                        t___3126 = None;
                    }
                    if ! t___3126.is_none() {
                        return__243 = SafeUrl::new(x__724.clone());
                    } else {
                        return__243 = fallback_safe_url().clone();
                    }
                    break 'fn__725;
                }
                t___5422 = temper_core::string::next( & x__724, protocolEnd__726);
                protocolEnd__726 = t___5422;
            }
            return__243 = html_url_part_url_escaper().apply_string(x__724.clone());
        }
        return return__243.clone();
    }
    pub fn new() -> HtmlProtocolFilteringUrlEscaper {
        let selfish = HtmlProtocolFilteringUrlEscaper(std::sync::Arc::new(HtmlProtocolFilteringUrlEscaperStruct {}));
        return selfish;
    }
}
impl UrlEscaperTrait for HtmlProtocolFilteringUrlEscaper {
    fn as_enum(& self) -> UrlEscaperEnum {
        UrlEscaperEnum::HtmlProtocolFilteringUrlEscaper(self.clone())
    }
    fn clone_boxed(& self) -> UrlEscaper {
        UrlEscaper::new(self.clone())
    }
    fn apply_safe_url(& self, x__721: SafeUrl) -> SafeUrl {
        self.apply_safe_url(x__721)
    }
    fn apply_string(& self, x__724: std::sync::Arc<String>) -> SafeUrl {
        self.apply_string(x__724)
    }
}
impl crate::core::EscaperTrait for HtmlProtocolFilteringUrlEscaper {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(HtmlProtocolFilteringUrlEscaper, [UrlEscaper, crate::core::Escaper]);
struct HtmlUrlPartUrlEscaperStruct {}
#[derive(Clone)]
pub struct HtmlUrlPartUrlEscaper(std::sync::Arc<HtmlUrlPartUrlEscaperStruct>);
static HTML_URL_PART_URL_ESCAPER__INSTANCE: std::sync::OnceLock<HtmlUrlPartUrlEscaper> = std::sync::OnceLock::new();
impl HtmlUrlPartUrlEscaper {
    pub fn instance() -> HtmlUrlPartUrlEscaper {
        ( * HTML_URL_PART_URL_ESCAPER__INSTANCE.get().unwrap()).clone()
    }
    pub fn apply_safe_url(& self, x__738: SafeUrl) -> SafeUrl {
        return x__738.clone();
    }
    pub fn apply_string(& self, x__741: impl temper_core::ToArcString) -> SafeUrl {
        let x__741 = x__741.to_arc_string();
        let mut t___5428: bool;
        let mut t___5431: usize;
        let mut t___5432: usize;
        let mut t___5434: std::sync::Arc<String>;
        let mut t___3137: bool;
        let mut t___3143: std::sync::Arc<String>;
        let mut i__743: usize = 0usize;
        let end__744: usize = x__741.len();
        let mut emitted__745: usize = 0usize;
        let sb__746: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        'loop___5921: while Some(i__743) < Some(end__744) {
            let cp__747: i32 = temper_core::string::get( & x__741, i__743);
            if Some(cp__747) < Some(temper_core::ListedTrait::len( & url_safe())) {
                t___5428 = temper_core::ListedTrait::get( & url_safe(), cp__747);
                t___3137 = ! t___5428;
            } else {
                t___3137 = false;
            }
            if t___3137 {
                temper_core::string::builder::append_between( & sb__746, x__741.clone(), emitted__745, i__743);
                crate::url::percent_escape_octet_to(cp__747, sb__746.clone());
                t___5431 = temper_core::string::next( & x__741, i__743);
                emitted__745 = t___5431;
            }
            t___5432 = temper_core::string::next( & x__741, i__743);
            i__743 = t___5432;
        }
        if Some(emitted__745) > Some(0usize) {
            temper_core::string::builder::append_between( & sb__746, x__741.clone(), emitted__745, end__744);
            t___5434 = temper_core::string::builder::to_string( & sb__746);
            t___3143 = t___5434.clone();
        } else {
            t___3143 = x__741.clone();
        }
        return SafeUrl::new(t___3143.clone());
    }
    pub fn new() -> HtmlUrlPartUrlEscaper {
        let selfish = HtmlUrlPartUrlEscaper(std::sync::Arc::new(HtmlUrlPartUrlEscaperStruct {}));
        return selfish;
    }
}
impl UrlEscaperTrait for HtmlUrlPartUrlEscaper {
    fn as_enum(& self) -> UrlEscaperEnum {
        UrlEscaperEnum::HtmlUrlPartUrlEscaper(self.clone())
    }
    fn clone_boxed(& self) -> UrlEscaper {
        UrlEscaper::new(self.clone())
    }
    fn apply_safe_url(& self, x__738: SafeUrl) -> SafeUrl {
        self.apply_safe_url(x__738)
    }
    fn apply_string(& self, x__741: std::sync::Arc<String>) -> SafeUrl {
        self.apply_string(x__741)
    }
}
impl crate::core::EscaperTrait for HtmlUrlPartUrlEscaper {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(HtmlUrlPartUrlEscaper, [UrlEscaper, crate::core::Escaper]);
struct HtmlAsIfQueryUrlEscaperStruct {}
#[derive(Clone)]
pub struct HtmlAsIfQueryUrlEscaper(std::sync::Arc<HtmlAsIfQueryUrlEscaperStruct>);
static HTML_AS_IF_QUERY_URL_ESCAPER__INSTANCE: std::sync::OnceLock<HtmlAsIfQueryUrlEscaper> = std::sync::OnceLock::new();
impl HtmlAsIfQueryUrlEscaper {
    pub fn instance() -> HtmlAsIfQueryUrlEscaper {
        ( * HTML_AS_IF_QUERY_URL_ESCAPER__INSTANCE.get().unwrap()).clone()
    }
    pub fn apply_safe_url(& self, x__752: SafeUrl) -> SafeUrl {
        return x__752.clone();
    }
    pub fn apply_string(& self, x__755: impl temper_core::ToArcString) -> SafeUrl {
        let x__755 = x__755.to_arc_string();
        let mut t___5410: bool;
        let mut t___5413: usize;
        let mut t___5414: usize;
        let mut t___5416: std::sync::Arc<String>;
        let mut t___3113: bool;
        let mut t___3119: std::sync::Arc<String>;
        let mut i__757: usize = 0usize;
        let end__758: usize = x__755.len();
        let mut emitted__759: usize = 0usize;
        let sb__760: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        'loop___5922: while Some(i__757) < Some(end__758) {
            let cp__761: i32 = temper_core::string::get( & x__755, i__757);
            if Some(cp__761) < Some(temper_core::ListedTrait::len( & url_query_safe())) {
                t___5410 = temper_core::ListedTrait::get( & url_query_safe(), cp__761);
                t___3113 = ! t___5410;
            } else {
                t___3113 = false;
            }
            if t___3113 {
                temper_core::string::builder::append_between( & sb__760, x__755.clone(), emitted__759, i__757);
                crate::url::percent_escape_octet_to(cp__761, sb__760.clone());
                t___5413 = temper_core::string::next( & x__755, i__757);
                emitted__759 = t___5413;
            }
            t___5414 = temper_core::string::next( & x__755, i__757);
            i__757 = t___5414;
        }
        if Some(emitted__759) > Some(0usize) {
            temper_core::string::builder::append_between( & sb__760, x__755.clone(), emitted__759, end__758);
            t___5416 = temper_core::string::builder::to_string( & sb__760);
            t___3119 = t___5416.clone();
        } else {
            t___3119 = x__755.clone();
        }
        return SafeUrl::new(t___3119.clone());
    }
    pub fn new() -> HtmlAsIfQueryUrlEscaper {
        let selfish = HtmlAsIfQueryUrlEscaper(std::sync::Arc::new(HtmlAsIfQueryUrlEscaperStruct {}));
        return selfish;
    }
}
impl UrlEscaperTrait for HtmlAsIfQueryUrlEscaper {
    fn as_enum(& self) -> UrlEscaperEnum {
        UrlEscaperEnum::HtmlAsIfQueryUrlEscaper(self.clone())
    }
    fn clone_boxed(& self) -> UrlEscaper {
        UrlEscaper::new(self.clone())
    }
    fn apply_safe_url(& self, x__752: SafeUrl) -> SafeUrl {
        self.apply_safe_url(x__752)
    }
    fn apply_string(& self, x__755: std::sync::Arc<String>) -> SafeUrl {
        self.apply_string(x__755)
    }
}
impl crate::core::EscaperTrait for HtmlAsIfQueryUrlEscaper {
    fn clone_boxed(& self) -> crate::core::Escaper {
        crate::core::Escaper::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(HtmlAsIfQueryUrlEscaper, [UrlEscaper, crate::core::Escaper]);
struct HtmlCssDelegateStruct {}
#[derive(Clone)]
pub struct HtmlCssDelegate(std::sync::Arc<HtmlCssDelegateStruct>);
impl HtmlCssDelegate {
    pub fn process(& self, s__765: Option<impl temper_core::ToArcString>) -> std::sync::Arc<String> {
        let s__765 = s__765.map(| x | x.to_arc_string());
        let return__254: std::sync::Arc<String>;
        if ! s__765.is_none() {
            return__254 = s__765.clone().unwrap();
        } else {
            return__254 = std::sync::Arc::new("".to_string());
        }
        return return__254.clone();
    }
    pub fn escaper(& self, outer__768: HtmlEscaper) -> Option<HtmlEscaper> {
        return Some(outer__768.clone());
    }
    pub fn new() -> HtmlCssDelegate {
        let selfish = HtmlCssDelegate(std::sync::Arc::new(HtmlCssDelegateStruct {}));
        return selfish;
    }
}
impl HtmlDelegateTrait for HtmlCssDelegate {
    fn as_enum(& self) -> HtmlDelegateEnum {
        HtmlDelegateEnum::HtmlCssDelegate(self.clone())
    }
    fn clone_boxed(& self) -> HtmlDelegate {
        HtmlDelegate::new(self.clone())
    }
    fn escaper(& self, outer__768: HtmlEscaper) -> Option<HtmlEscaper> {
        self.escaper(outer__768)
    }
}
impl crate::core::DelegateTrait for HtmlCssDelegate {
    fn clone_boxed(& self) -> crate::core::Delegate {
        crate::core::Delegate::new(self.clone())
    }
    fn process(& self, s__765: Option<std::sync::Arc<String>>) -> std::sync::Arc<String> {
        self.process(s__765)
    }
}
temper_core::impl_any_value_trait!(HtmlCssDelegate, [HtmlDelegate, crate::core::Delegate]);
struct HtmlJsDelegateStruct {}
#[derive(Clone)]
pub struct HtmlJsDelegate(std::sync::Arc<HtmlJsDelegateStruct>);
impl HtmlJsDelegate {
    pub fn process(& self, s__772: Option<impl temper_core::ToArcString>) -> std::sync::Arc<String> {
        let s__772 = s__772.map(| x | x.to_arc_string());
        let return__258: std::sync::Arc<String>;
        if ! s__772.is_none() {
            return__258 = s__772.clone().unwrap();
        } else {
            return__258 = std::sync::Arc::new("".to_string());
        }
        return return__258.clone();
    }
    pub fn escaper(& self, outer__775: HtmlEscaper) -> Option<HtmlEscaper> {
        return Some(outer__775.clone());
    }
    pub fn new() -> HtmlJsDelegate {
        let selfish = HtmlJsDelegate(std::sync::Arc::new(HtmlJsDelegateStruct {}));
        return selfish;
    }
}
impl HtmlDelegateTrait for HtmlJsDelegate {
    fn as_enum(& self) -> HtmlDelegateEnum {
        HtmlDelegateEnum::HtmlJsDelegate(self.clone())
    }
    fn clone_boxed(& self) -> HtmlDelegate {
        HtmlDelegate::new(self.clone())
    }
    fn escaper(& self, outer__775: HtmlEscaper) -> Option<HtmlEscaper> {
        self.escaper(outer__775)
    }
}
impl crate::core::DelegateTrait for HtmlJsDelegate {
    fn clone_boxed(& self) -> crate::core::Delegate {
        crate::core::Delegate::new(self.clone())
    }
    fn process(& self, s__772: Option<std::sync::Arc<String>>) -> std::sync::Arc<String> {
        self.process(s__772)
    }
}
temper_core::impl_any_value_trait!(HtmlJsDelegate, [HtmlDelegate, crate::core::Delegate]);
fn htmlStateStr__260(x__379: i32) -> std::sync::Arc<String> {
    let return__129: std::sync::Arc<String>;
    if Some(x__379) == Some(0) {
        return__129 = std::sync::Arc::new("Pcdata".to_string());
    } else {
        if Some(x__379) == Some(1) {
            return__129 = std::sync::Arc::new("OName".to_string());
        } else {
            if Some(x__379) == Some(2) {
                return__129 = std::sync::Arc::new("CName".to_string());
            } else {
                if Some(x__379) == Some(3) {
                    return__129 = std::sync::Arc::new("BeforeAttr".to_string());
                } else {
                    if Some(x__379) == Some(4) {
                        return__129 = std::sync::Arc::new("BeforeEq".to_string());
                    } else {
                        if Some(x__379) == Some(5) {
                            return__129 = std::sync::Arc::new("BeforeValue".to_string());
                        } else {
                            if Some(x__379) == Some(6) {
                                return__129 = std::sync::Arc::new("Attr".to_string());
                            } else {
                                if Some(x__379) == Some(7) {
                                    return__129 = std::sync::Arc::new("AfterAttr".to_string());
                                } else {
                                    if Some(x__379) == Some(8) {
                                        return__129 = std::sync::Arc::new("SpecialBody".to_string());
                                    } else {
                                        return__129 = temper_core::int_to_string(x__379, None);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return return__129.clone();
}
fn tagStateStr__261(x__381: i32) -> std::sync::Arc<String> {
    return temper_core::int_to_string(x__381, None);
}
fn attribStateStr__262(x__383: i32) -> std::sync::Arc<String> {
    let return__131: std::sync::Arc<String>;
    if Some(x__383) == Some(0) {
        return__131 = std::sync::Arc::new("Generic".to_string());
    } else {
        if Some(x__383) == Some(1) {
            return__131 = std::sync::Arc::new("Css".to_string());
        } else {
            if Some(x__383) == Some(2) {
                return__131 = std::sync::Arc::new("Js".to_string());
            } else {
                if Some(x__383) == Some(3) {
                    return__131 = std::sync::Arc::new("Url".to_string());
                } else {
                    if Some(x__383) == Some(4) {
                        return__131 = std::sync::Arc::new("Urls".to_string());
                    } else {
                        return__131 = temper_core::int_to_string(x__383, None);
                    }
                }
            }
        }
    }
    return return__131.clone();
}
fn delimStateStr__263(x__385: i32) -> std::sync::Arc<String> {
    let return__132: std::sync::Arc<String>;
    if Some(x__385) == Some(0) {
        return__132 = std::sync::Arc::new("Uq".to_string());
    } else {
        if Some(x__385) == Some(1) {
            return__132 = std::sync::Arc::new("Sq".to_string());
        } else {
            if Some(x__385) == Some(2) {
                return__132 = std::sync::Arc::new("Dq".to_string());
            } else {
                return__132 = temper_core::int_to_string(x__385, None);
            }
        }
    }
    return return__132.clone();
}
fn urlStateStr__265(x__456: i32) -> std::sync::Arc<String> {
    let return__137: std::sync::Arc<String>;
    if Some(x__456) == Some(0) {
        return__137 = std::sync::Arc::new("Start".to_string());
    } else {
        if Some(x__456) == Some(1) {
            return__137 = std::sync::Arc::new("BeforeQuery".to_string());
        } else {
            if Some(x__456) == Some(2) {
                return__137 = std::sync::Arc::new("Query".to_string());
            } else {
                if Some(x__456) == Some(3) {
                    return__137 = std::sync::Arc::new("Fragment".to_string());
                } else {
                    return__137 = temper_core::int_to_string(x__456, None);
                }
            }
        }
    }
    return return__137.clone();
}
fn urlPropagateContext__266(before__458: crate::core::AutoescState<UrlEscaperContext>, literalPart__459: Option<impl temper_core::ToArcString>) -> crate::core::AfterPropagate<UrlEscaperContext> {
    let literalPart__459 = literalPart__459.map(| x | x.to_arc_string());
    let return__138: crate::core::AfterPropagate<UrlEscaperContext>;
    let mut t___5455: temper_std::regex::CodeSet;
    let mut t___5459: temper_std::regex::CodeSet;
    let mut t___5464: temper_std::regex::Repeat;
    let mut t___5482: temper_std::regex::Or;
    let mut t___5488: temper_std::regex::Repeat;
    let mut t___5493: std::sync::Arc<String>;
    let mut t___5495: usize;
    let mut t___5498: crate::core::AutoescState<UrlEscaperContext>;
    let mut t___5502: std::sync::Arc<String>;
    let mut t___5504: usize;
    let mut t___5507: crate::core::AutoescState<UrlEscaperContext>;
    let mut t___5511: std::sync::Arc<String>;
    let mut t___5513: usize;
    let mut t___5516: crate::core::AutoescState<UrlEscaperContext>;
    let mut t___5520: std::sync::Arc<String>;
    let mut t___5522: usize;
    let mut t___5526: std::sync::Arc<String>;
    let mut t___5528: usize;
    let mut t___5531: crate::core::AutoescState<UrlEscaperContext>;
    let mut t___5535: std::sync::Arc<String>;
    let mut t___5537: usize;
    let mut t___5540: crate::core::AutoescState<UrlEscaperContext>;
    let mut t___5544: std::sync::Arc<String>;
    let mut t___5546: usize;
    let mut t___5550: std::sync::Arc<String>;
    let mut t___5552: usize;
    let mut t___5555: crate::core::AutoescState<UrlEscaperContext>;
    let mut t___3204: Option<temper_std::regex::Match>;
    let mut t___3215: Option<temper_std::regex::Match>;
    let mut t___3226: Option<temper_std::regex::Match>;
    let mut t___3237: Option<temper_std::regex::Match>;
    let mut t___3245: Option<temper_std::regex::Match>;
    let mut t___3256: Option<temper_std::regex::Match>;
    let mut t___3267: Option<temper_std::regex::Match>;
    let mut t___3275: Option<temper_std::regex::Match>;
    'fn__460: {
        let contextBefore__461: UrlEscaperContext = before__458.context();
        t___5455 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("#"))], Some(false));
        let pattern0__462: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5455.clone())]).compiled();
        t___5459 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("?"))], Some(false));
        let pattern1__463: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5459.clone())]).compiled();
        t___5464 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("#"))], Some(true))), 1, None, Some(false));
        let pattern2__464: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5464.clone())]).compiled();
        t___5482 = temper_std::regex::Or::new([temper_std::regex::RegexNode::new(temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(":")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("#")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("?"))], Some(true))), 0, None, Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new(":"))])), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" ")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(":")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("#")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("?"))], Some(true)))]);
        let pattern3__465: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5482.clone())]).compiled();
        t___5488 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("?")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("#"))], Some(true))), 1, None, Some(false));
        let pattern4__466: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5488.clone())]).compiled();
        if ! literalPart__459.is_none() {
            let literalPart___992: std::sync::Arc<String> = literalPart__459.clone().unwrap();
            if Some(contextBefore__461.url_state()) == Some(0) {
                let match__467: Option<temper_std::regex::Match>;
                'ok___5812: {
                    'orelse___941: {
                        t___3204 = match pattern3__465.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___941
                        };
                        match__467 = t___3204.clone();
                        break 'ok___5812;
                    }
                    match__467 = None;
                }
                if ! match__467.is_none() {
                    let match___993: temper_std::regex::Match = match__467.clone().unwrap();
                    t___5493 = match___993.full().value();
                    t___5495 = match___993.full().end();
                    t___5498 = crate::core::AutoescState::new(UrlEscaperContext::new(1), before__458.subsidiary());
                    return__138 = crate::core::AfterPropagate::new(t___5493.clone(), t___5495, t___5498.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(0) {
                let match__468: Option<temper_std::regex::Match>;
                'ok___5813: {
                    'orelse___942: {
                        t___3215 = match pattern1__463.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___942
                        };
                        match__468 = t___3215.clone();
                        break 'ok___5813;
                    }
                    match__468 = None;
                }
                if ! match__468.is_none() {
                    let match___994: temper_std::regex::Match = match__468.clone().unwrap();
                    t___5502 = match___994.full().value();
                    t___5504 = match___994.full().end();
                    t___5507 = crate::core::AutoescState::new(UrlEscaperContext::new(2), before__458.subsidiary());
                    return__138 = crate::core::AfterPropagate::new(t___5502.clone(), t___5504, t___5507.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(0) {
                let match__469: Option<temper_std::regex::Match>;
                'ok___5814: {
                    'orelse___943: {
                        t___3226 = match pattern0__462.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___943
                        };
                        match__469 = t___3226.clone();
                        break 'ok___5814;
                    }
                    match__469 = None;
                }
                if ! match__469.is_none() {
                    let match___995: temper_std::regex::Match = match__469.clone().unwrap();
                    t___5511 = match___995.full().value();
                    t___5513 = match___995.full().end();
                    t___5516 = crate::core::AutoescState::new(UrlEscaperContext::new(3), before__458.subsidiary());
                    return__138 = crate::core::AfterPropagate::new(t___5511.clone(), t___5513, t___5516.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(1) {
                let match__470: Option<temper_std::regex::Match>;
                'ok___5815: {
                    'orelse___944: {
                        t___3237 = match pattern4__466.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___944
                        };
                        match__470 = t___3237.clone();
                        break 'ok___5815;
                    }
                    match__470 = None;
                }
                if ! match__470.is_none() {
                    let match___996: temper_std::regex::Match = match__470.clone().unwrap();
                    t___5520 = match___996.full().value();
                    t___5522 = match___996.full().end();
                    return__138 = crate::core::AfterPropagate::new(t___5520.clone(), t___5522, before__458.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(1) {
                let match__471: Option<temper_std::regex::Match>;
                'ok___5816: {
                    'orelse___945: {
                        t___3245 = match pattern1__463.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___945
                        };
                        match__471 = t___3245.clone();
                        break 'ok___5816;
                    }
                    match__471 = None;
                }
                if ! match__471.is_none() {
                    let match___997: temper_std::regex::Match = match__471.clone().unwrap();
                    t___5526 = match___997.full().value();
                    t___5528 = match___997.full().end();
                    t___5531 = crate::core::AutoescState::new(UrlEscaperContext::new(2), before__458.subsidiary());
                    return__138 = crate::core::AfterPropagate::new(t___5526.clone(), t___5528, t___5531.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(1) {
                let match__472: Option<temper_std::regex::Match>;
                'ok___5817: {
                    'orelse___946: {
                        t___3256 = match pattern0__462.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___946
                        };
                        match__472 = t___3256.clone();
                        break 'ok___5817;
                    }
                    match__472 = None;
                }
                if ! match__472.is_none() {
                    let match___998: temper_std::regex::Match = match__472.clone().unwrap();
                    t___5535 = match___998.full().value();
                    t___5537 = match___998.full().end();
                    t___5540 = crate::core::AutoescState::new(UrlEscaperContext::new(3), before__458.subsidiary());
                    return__138 = crate::core::AfterPropagate::new(t___5535.clone(), t___5537, t___5540.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(2) {
                let match__473: Option<temper_std::regex::Match>;
                'ok___5818: {
                    'orelse___947: {
                        t___3267 = match pattern2__464.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___947
                        };
                        match__473 = t___3267.clone();
                        break 'ok___5818;
                    }
                    match__473 = None;
                }
                if ! match__473.is_none() {
                    let match___999: temper_std::regex::Match = match__473.clone().unwrap();
                    t___5544 = match___999.full().value();
                    t___5546 = match___999.full().end();
                    return__138 = crate::core::AfterPropagate::new(t___5544.clone(), t___5546, before__458.clone());
                    break 'fn__460;
                }
            }
            if Some(contextBefore__461.url_state()) == Some(2) {
                let match__474: Option<temper_std::regex::Match>;
                'ok___5819: {
                    'orelse___948: {
                        t___3275 = match pattern0__462.find(literalPart___992.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___948
                        };
                        match__474 = t___3275.clone();
                        break 'ok___5819;
                    }
                    match__474 = None;
                }
                if ! match__474.is_none() {
                    let match___1000: temper_std::regex::Match = match__474.clone().unwrap();
                    t___5550 = match___1000.full().value();
                    t___5552 = match___1000.full().end();
                    t___5555 = crate::core::AutoescState::new(UrlEscaperContext::new(3), before__458.subsidiary());
                    return__138 = crate::core::AfterPropagate::new(t___5550.clone(), t___5552, t___5555.clone());
                    break 'fn__460;
                }
            }
        }
        if literalPart__459.is_none() {
            return__138 = crate::core::AfterPropagate::new("", 0usize, before__458.clone());
            break 'fn__460;
        }
        return__138 = panic!();
    }
    return return__138.clone();
}
fn htmlPropagateContext__264(before__387: crate::core::AutoescState<HtmlEscaperContext>, literalPart__388: Option<impl temper_core::ToArcString>) -> crate::core::AfterPropagate<HtmlEscaperContext> {
    let literalPart__388 = literalPart__388.map(| x | x.to_arc_string());
    let return__133: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___4745: temper_std::regex::CodePoints;
    let mut t___4754: temper_std::regex::Sequence;
    let mut t___4757: temper_std::regex::CodePoints;
    let mut t___4766: temper_std::regex::Sequence;
    let mut t___4769: temper_std::regex::CodePoints;
    let mut t___4777: temper_std::regex::CodeSet;
    let mut t___4785: temper_std::regex::CodeSet;
    let mut t___4806: temper_std::regex::Sequence;
    let mut t___4832: temper_std::regex::Or;
    let mut t___4844: temper_std::regex::Sequence;
    let mut t___4852: temper_std::regex::CodeSet;
    let mut t___4857: temper_std::regex::CodeSet;
    let mut t___4860: temper_std::regex::CodePoints;
    let mut t___4863: temper_std::regex::CodePoints;
    let mut t___4866: temper_std::regex::CodePoints;
    let mut t___4869: temper_std::regex::CodePoints;
    let mut t___4872: temper_std::regex::CodePoints;
    let mut t___4915: temper_std::regex::Sequence;
    let mut t___4932: temper_std::regex::Sequence;
    let mut t___4950: temper_std::regex::Sequence;
    let mut t___4958: temper_std::regex::Repeat;
    let mut t___4963: temper_std::regex::Repeat;
    let mut t___4968: temper_std::regex::Repeat;
    let mut t___4974: temper_std::regex::Repeat;
    let mut t___4984: temper_std::regex::Repeat;
    let mut t___4994: temper_std::regex::Repeat;
    let mut t___4998: temper_std::regex::CodeSet;
    let mut t___5008: temper_std::regex::Sequence;
    let mut t___5021: temper_std::regex::Sequence;
    let mut t___5030: std::sync::Arc<String>;
    let mut t___5032: usize;
    let mut t___5038: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5046: std::sync::Arc<String>;
    let mut t___5048: usize;
    let mut t___5054: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5058: usize;
    let mut t___5062: usize;
    let mut t___5066: std::sync::Arc<String>;
    let mut t___5068: usize;
    let mut t___5072: std::sync::Arc<String>;
    let mut t___5074: usize;
    let mut t___5078: std::sync::Arc<String>;
    let mut t___5080: usize;
    let mut t___5084: std::sync::Arc<String>;
    let mut t___5086: usize;
    let mut t___5090: std::sync::Arc<String>;
    let mut t___5092: usize;
    let mut t___5098: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5102: std::sync::Arc<String>;
    let mut t___5104: usize;
    let mut t___5112: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5116: std::sync::Arc<String>;
    let mut t___5118: usize;
    let mut t___5124: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5128: std::sync::Arc<String>;
    let mut t___5130: usize;
    let mut t___5134: std::sync::Arc<String>;
    let mut t___5136: usize;
    let mut t___5152: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5153: HtmlUrlDelegate;
    let mut t___5169: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5170: HtmlUrlDelegate;
    let mut t___5182: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5183: HtmlUrlDelegate;
    let mut t___5199: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5200: HtmlCssDelegate;
    let mut t___5212: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5213: HtmlJsDelegate;
    let mut t___5217: std::sync::Arc<String>;
    let mut t___5219: usize;
    let mut t___5225: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5229: std::sync::Arc<String>;
    let mut t___5231: usize;
    let mut t___5235: std::sync::Arc<String>;
    let mut t___5237: usize;
    let mut t___5243: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5251: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5255: std::sync::Arc<String>;
    let mut t___5257: usize;
    let mut t___5262: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5266: std::sync::Arc<String>;
    let mut t___5268: usize;
    let mut t___5273: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5280: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5282: i32;
    let mut t___5287: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5290: i32;
    let mut t___5295: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5298: i32;
    let mut t___5300: std::sync::Arc<String>;
    let mut t___5302: usize;
    let mut t___5307: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5310: i32;
    let mut t___5312: std::sync::Arc<String>;
    let mut t___5314: usize;
    let mut t___5319: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5322: i32;
    let mut t___5328: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5329: HtmlUrlDelegate;
    let mut t___5332: i32;
    let mut t___5337: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5340: i32;
    let mut t___5343: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5346: i32;
    let mut t___5351: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5354: i32;
    let mut t___5359: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5362: i32;
    let mut t___5364: i32;
    let mut t___5365: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5368: i32;
    let mut t___5374: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___5383: crate::core::AfterPropagate<HtmlEscaperContext>;
    let mut t___5387: std::sync::Arc<String>;
    let mut t___5389: usize;
    let mut t___5395: crate::core::AutoescState<HtmlEscaperContext>;
    let mut t___2619: Option<temper_std::regex::Match>;
    let mut t___2625: temper_std::regex::Match;
    let mut t___2626: Option<temper_std::regex::Match>;
    let mut t___2639: Option<temper_std::regex::Match>;
    let mut t___2645: temper_std::regex::Match;
    let mut t___2646: Option<temper_std::regex::Match>;
    let mut t___2659: Option<temper_std::regex::Match>;
    let mut t___2665: Option<temper_std::regex::Match>;
    let mut t___2671: Option<temper_std::regex::Match>;
    let mut t___2679: Option<temper_std::regex::Match>;
    let mut t___2687: Option<temper_std::regex::Match>;
    let mut t___2695: Option<temper_std::regex::Match>;
    let mut t___2703: Option<temper_std::regex::Match>;
    let mut t___2717: Option<temper_std::regex::Match>;
    let mut t___2725: temper_std::regex::Match;
    let mut t___2726: Option<temper_std::regex::Match>;
    let mut t___2736: Option<temper_std::regex::Match>;
    let mut t___2750: Option<temper_std::regex::Match>;
    let mut t___2758: Option<temper_std::regex::Match>;
    let mut t___2766: Option<temper_std::regex::Match>;
    let mut t___2772: temper_std::regex::Match;
    let mut t___2773: Option<temper_std::regex::Match>;
    let mut t___2787: Option<temper_std::regex::Match>;
    let mut t___2793: temper_std::regex::Match;
    let mut t___2794: Option<temper_std::regex::Match>;
    let mut t___2808: Option<temper_std::regex::Match>;
    let mut t___2823: Option<temper_std::regex::Match>;
    let mut t___2829: temper_std::regex::Match;
    let mut t___2830: Option<temper_std::regex::Match>;
    let mut t___2844: Option<temper_std::regex::Match>;
    let mut t___2859: Option<temper_std::regex::Match>;
    let mut t___2873: Option<temper_std::regex::Match>;
    let mut t___2881: Option<temper_std::regex::Match>;
    let mut t___2895: temper_std::regex::Match;
    let mut t___2896: Option<temper_std::regex::Match>;
    let mut t___2906: Option<temper_std::regex::Match>;
    let mut t___2919: Option<temper_std::regex::Match>;
    let mut t___2932: temper_std::regex::Match;
    let mut t___2933: Option<temper_std::regex::Match>;
    let mut t___2942: bool;
    let mut t___2952: bool;
    let mut t___2953: temper_std::regex::Match;
    let mut t___2954: Option<temper_std::regex::Match>;
    let mut t___2964: bool;
    let mut t___2965: Option<temper_std::regex::Match>;
    let mut t___2979: bool;
    let mut t___2980: Option<temper_std::regex::Match>;
    let mut t___2994: bool;
    let mut t___2995: Option<temper_std::regex::Match>;
    let mut t___3007: bool;
    let mut t___3008: Option<temper_std::regex::Match>;
    let mut t___3018: bool;
    let mut t___3019: Option<temper_std::regex::Match>;
    let mut t___3027: bool;
    let mut t___3028: Option<temper_std::regex::Match>;
    let mut t___3038: bool;
    let mut t___3039: Option<temper_std::regex::Match>;
    let mut t___3049: bool;
    let mut t___3050: bool;
    let mut t___3053: bool;
    let mut t___3058: bool;
    let mut t___3077: Option<temper_std::regex::Match>;
    'fn__389: {
        let contextBefore__390: HtmlEscaperContext = before__387.context();
        t___4745 = temper_std::regex::CodePoints::new("\"");
        let pattern0__391: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4745.clone())]).compiled();
        t___4754 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new("\"")), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\""))], Some(true))), 0, None, Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new("\"")), 0, Some(1), Some(false)))]);
        let pattern1__392: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4754.clone())]).compiled();
        t___4757 = temper_std::regex::CodePoints::new("'");
        let pattern2__393: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4757.clone())]).compiled();
        t___4766 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new("'")), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("'"))], Some(true))), 0, None, Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new("'")), 0, Some(1), Some(false)))]);
        let pattern3__394: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4766.clone())]).compiled();
        t___4769 = temper_std::regex::CodePoints::new(">");
        let pattern4__395: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4769.clone())]).compiled();
        t___4777 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" "))], Some(false));
        let pattern5__396: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4777.clone())]).compiled();
        t___4785 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(65, 90)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(97, 122)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(48, 57)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(58, 58)), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("-"))], Some(false));
        let pattern6__397: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4785.clone())]).compiled();
        t___4806 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("S")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("s"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("R")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("r"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("C")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("c"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("S")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("s"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("E")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("e"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("T")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("t"))], Some(false)))]);
        let pattern7__398: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4806.clone())]).compiled();
        t___4832 = temper_std::regex::Or::new([temper_std::regex::RegexNode::new(temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("S")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("s"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("R")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("r"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("C")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("c"))], Some(false)))])), temper_std::regex::RegexNode::new(temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("H")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("h"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("R")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("r"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("E")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("e"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("F")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("f"))], Some(false)))]))]);
        let pattern8__399: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4832.clone())]).compiled();
        t___4844 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" "))], Some(false))), 0, None, Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new("/")), 0, Some(1), Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new(">"))]);
        let pattern9__400: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4844.clone())]).compiled();
        t___4852 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" "))], Some(true));
        let pattern10__401: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4852.clone())]).compiled();
        t___4857 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(97, 122)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(65, 90))], Some(false));
        let pattern11__402: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4857.clone())]).compiled();
        t___4860 = temper_std::regex::CodePoints::new(",");
        let pattern12__403: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4860.clone())]).compiled();
        t___4863 = temper_std::regex::CodePoints::new("<");
        let pattern13__404: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4863.clone())]).compiled();
        t___4866 = temper_std::regex::CodePoints::new("</");
        let pattern14__405: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4866.clone())]).compiled();
        t___4869 = temper_std::regex::CodePoints::new("=");
        let pattern15__406: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4869.clone())]).compiled();
        t___4872 = temper_std::regex::CodePoints::new(">");
        let pattern16__407: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4872.clone())]).compiled();
        t___4915 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("D")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("d"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("A")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("a"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("T")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("t"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("A")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("a"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new("-")), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("=")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" ")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">"))], Some(true))), 0, None, Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("U")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("u"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("R")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("r"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("L")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("l")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("I")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("i"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("=")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" ")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">"))], Some(true))), 0, None, Some(false)))]);
        let pattern17__408: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4915.clone())]).compiled();
        t___4932 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("O")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("o"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("N")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("n"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("=")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" ")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">"))], Some(true))), 0, None, Some(false)))]);
        let pattern18__409: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4932.clone())]).compiled();
        t___4950 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("S")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("s"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("T")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("t"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("Y")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("y"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("L")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("l"))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("E")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("e"))], Some(false)))]);
        let pattern19__410: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4950.clone())]).compiled();
        t___4958 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" "))], Some(false))), 1, None, Some(false));
        let pattern20__411: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4958.clone())]).compiled();
        t___4963 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\""))], Some(true))), 1, None, Some(false));
        let pattern21__412: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4963.clone())]).compiled();
        t___4968 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("'"))], Some(true))), 1, None, Some(false));
        let pattern22__413: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4968.clone())]).compiled();
        t___4974 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("<")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">"))], Some(true))), 1, None, Some(false));
        let pattern23__414: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4974.clone())]).compiled();
        t___4984 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("=")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" "))], Some(true))), 1, None, Some(false));
        let pattern24__415: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4984.clone())]).compiled();
        t___4994 = temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x09")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0d")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\x0a")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(" ")), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("\""))], Some(true))), 1, None, Some(false));
        let pattern25__416: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4994.clone())]).compiled();
        t___4998 = temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new(">"))], Some(true));
        let pattern26__417: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___4998.clone())]).compiled();
        t___5008 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(97, 122)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(65, 90)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(48, 57)), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("-"))], Some(false))), 1, None, Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::CodePoints::new(":"))]);
        let pattern27__418: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5008.clone())]).compiled();
        t___5021 = temper_std::regex::Sequence::new([temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(97, 122)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(65, 90))], Some(false))), temper_std::regex::RegexNode::new(temper_std::regex::Repeat::new(temper_std::regex::RegexNode::new(temper_std::regex::CodeSet::new([temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(97, 122)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(65, 90)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(48, 57)), temper_std::regex::CodePart::new(temper_std::regex::CodeRange::new(58, 58)), temper_std::regex::CodePart::new(temper_std::regex::CodePoints::new("-"))], Some(false))), 0, None, Some(false)))]);
        let pattern28__419: temper_std::regex::Regex = temper_std::regex::Sequence::new([temper_core::cast::<temper_std::regex::RegexNode>(temper_std::regex::begin().clone()).unwrap(), temper_std::regex::RegexNode::new(t___5021.clone())]).compiled();
        if ! literalPart__388.is_none() {
            let literalPart___1003: std::sync::Arc<String> = literalPart__388.clone().unwrap();
            if Some(contextBefore__390.html_state()) == Some(0) {
                let match__420: Option<temper_std::regex::Match>;
                'ok___5823: {
                    'orelse___950: {
                        t___2619 = match pattern14__405.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___950
                        };
                        match__420 = t___2619.clone();
                        break 'ok___5823;
                    }
                    match__420 = None;
                }
                if ! match__420.is_none() {
                    let match___1004: temper_std::regex::Match = match__420.clone().unwrap();
                    'ok___5824: {
                        'orelse___951: {
                            t___2625 = match pattern11__402.find(temper_core::string::slice( & literalPart___1003, match___1004.full().end(), literalPart___1003.len()), None) {
                                Ok(x) => x,
                                _ => break 'orelse___951
                            };
                            t___2626 = Some(t___2625.clone());
                            break 'ok___5824;
                        }
                        t___2626 = None;
                    }
                    if ! t___2626.is_none() {
                        t___5030 = match___1004.full().value();
                        t___5032 = match___1004.full().end();
                        t___5038 = crate::core::AutoescState::new(HtmlEscaperContext::new(2, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                        return__133 = crate::core::AfterPropagate::new(t___5030.clone(), t___5032, t___5038.clone());
                        break 'fn__389;
                    }
                }
            }
            if Some(contextBefore__390.html_state()) == Some(0) {
                let match__421: Option<temper_std::regex::Match>;
                'ok___5825: {
                    'orelse___952: {
                        t___2639 = match pattern13__404.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___952
                        };
                        match__421 = t___2639.clone();
                        break 'ok___5825;
                    }
                    match__421 = None;
                }
                if ! match__421.is_none() {
                    let match___1005: temper_std::regex::Match = match__421.clone().unwrap();
                    'ok___5826: {
                        'orelse___953: {
                            t___2645 = match pattern11__402.find(temper_core::string::slice( & literalPart___1003, match___1005.full().end(), literalPart___1003.len()), None) {
                                Ok(x) => x,
                                _ => break 'orelse___953
                            };
                            t___2646 = Some(t___2645.clone());
                            break 'ok___5826;
                        }
                        t___2646 = None;
                    }
                    if ! t___2646.is_none() {
                        t___5046 = match___1005.full().value();
                        t___5048 = match___1005.full().end();
                        t___5054 = crate::core::AutoescState::new(HtmlEscaperContext::new(1, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                        return__133 = crate::core::AfterPropagate::new(t___5046.clone(), t___5048, t___5054.clone());
                        break 'fn__389;
                    }
                }
            }
            if Some(contextBefore__390.html_state()) == Some(0) {
                let match__422: Option<temper_std::regex::Match>;
                'ok___5827: {
                    'orelse___954: {
                        t___2659 = match pattern13__404.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___954
                        };
                        match__422 = t___2659.clone();
                        break 'ok___5827;
                    }
                    match__422 = None;
                }
                if ! match__422.is_none() {
                    t___5058 = match__422.clone().unwrap().full().end();
                    return__133 = crate::core::AfterPropagate::new("&lt;", t___5058, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(0) {
                let match__423: Option<temper_std::regex::Match>;
                'ok___5828: {
                    'orelse___955: {
                        t___2665 = match pattern16__407.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___955
                        };
                        match__423 = t___2665.clone();
                        break 'ok___5828;
                    }
                    match__423 = None;
                }
                if ! match__423.is_none() {
                    t___5062 = match__423.clone().unwrap().full().end();
                    return__133 = crate::core::AfterPropagate::new("&gt;", t___5062, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(0) {
                let match__424: Option<temper_std::regex::Match>;
                'ok___5829: {
                    'orelse___956: {
                        t___2671 = match pattern23__414.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___956
                        };
                        match__424 = t___2671.clone();
                        break 'ok___5829;
                    }
                    match__424 = None;
                }
                if ! match__424.is_none() {
                    let match___1008: temper_std::regex::Match = match__424.clone().unwrap();
                    t___5066 = match___1008.full().value();
                    t___5068 = match___1008.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5066.clone(), t___5068, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(2) {
                let match__425: Option<temper_std::regex::Match>;
                'ok___5830: {
                    'orelse___957: {
                        t___2679 = match pattern1__392.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___957
                        };
                        match__425 = t___2679.clone();
                        break 'ok___5830;
                    }
                    match__425 = None;
                }
                if ! match__425.is_none() {
                    let match___1009: temper_std::regex::Match = match__425.clone().unwrap();
                    t___5072 = match___1009.full().value();
                    t___5074 = match___1009.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5072.clone(), t___5074, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(2) {
                let match__426: Option<temper_std::regex::Match>;
                'ok___5831: {
                    'orelse___958: {
                        t___2687 = match pattern3__394.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___958
                        };
                        match__426 = t___2687.clone();
                        break 'ok___5831;
                    }
                    match__426 = None;
                }
                if ! match__426.is_none() {
                    let match___1010: temper_std::regex::Match = match__426.clone().unwrap();
                    t___5078 = match___1010.full().value();
                    t___5080 = match___1010.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5078.clone(), t___5080, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(2) {
                let match__427: Option<temper_std::regex::Match>;
                'ok___5832: {
                    'orelse___959: {
                        t___2695 = match pattern26__417.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___959
                        };
                        match__427 = t___2695.clone();
                        break 'ok___5832;
                    }
                    match__427 = None;
                }
                if ! match__427.is_none() {
                    let match___1011: temper_std::regex::Match = match__427.clone().unwrap();
                    t___5084 = match___1011.full().value();
                    t___5086 = match___1011.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5084.clone(), t___5086, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(2) {
                let match__428: Option<temper_std::regex::Match>;
                'ok___5833: {
                    'orelse___960: {
                        t___2703 = match pattern16__407.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___960
                        };
                        match__428 = t___2703.clone();
                        break 'ok___5833;
                    }
                    match__428 = None;
                }
                if ! match__428.is_none() {
                    let match___1012: temper_std::regex::Match = match__428.clone().unwrap();
                    t___5090 = match___1012.full().value();
                    t___5092 = match___1012.full().end();
                    t___5098 = crate::core::AutoescState::new(HtmlEscaperContext::new(0, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5090.clone(), t___5092, t___5098.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(1) {
                let match__429: Option<temper_std::regex::Match>;
                'ok___5834: {
                    'orelse___961: {
                        t___2717 = match pattern28__419.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___961
                        };
                        match__429 = t___2717.clone();
                        break 'ok___5834;
                    }
                    match__429 = None;
                }
                if ! match__429.is_none() {
                    let match___1013: temper_std::regex::Match = match__429.clone().unwrap();
                    t___5102 = match___1013.full().value();
                    t___5104 = match___1013.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5102.clone(), t___5104, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(1) {
                'ok___5835: {
                    'orelse___962: {
                        t___2725 = match pattern4__395.find(literalPart___1003.clone(), None) {
                            Ok(x) => x,
                            _ => break 'orelse___962
                        };
                        t___2726 = Some(t___2725.clone());
                        break 'ok___5835;
                    }
                    t___2726 = None;
                }
                if ! t___2726.is_none() {
                    t___5112 = crate::core::AutoescState::new(HtmlEscaperContext::new(3, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new("", 0usize, t___5112.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(1) {
                let match__430: Option<temper_std::regex::Match>;
                'ok___5836: {
                    'orelse___963: {
                        t___2736 = match pattern20__411.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___963
                        };
                        match__430 = t___2736.clone();
                        break 'ok___5836;
                    }
                    match__430 = None;
                }
                if ! match__430.is_none() {
                    let match___1014: temper_std::regex::Match = match__430.clone().unwrap();
                    t___5116 = match___1014.full().value();
                    t___5118 = match___1014.full().end();
                    t___5124 = crate::core::AutoescState::new(HtmlEscaperContext::new(3, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5116.clone(), t___5118, t___5124.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__431: Option<temper_std::regex::Match>;
                'ok___5837: {
                    'orelse___964: {
                        t___2750 = match pattern20__411.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___964
                        };
                        match__431 = t___2750.clone();
                        break 'ok___5837;
                    }
                    match__431 = None;
                }
                if ! match__431.is_none() {
                    let match___1015: temper_std::regex::Match = match__431.clone().unwrap();
                    t___5128 = match___1015.full().value();
                    t___5130 = match___1015.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5128.clone(), t___5130, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__432: Option<temper_std::regex::Match>;
                'ok___5838: {
                    'orelse___965: {
                        t___2758 = match pattern27__418.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___965
                        };
                        match__432 = t___2758.clone();
                        break 'ok___5838;
                    }
                    match__432 = None;
                }
                if ! match__432.is_none() {
                    let match___1016: temper_std::regex::Match = match__432.clone().unwrap();
                    t___5134 = match___1016.full().value();
                    t___5136 = match___1016.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5134.clone(), t___5136, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__433: Option<temper_std::regex::Match>;
                'ok___5839: {
                    'orelse___966: {
                        t___2766 = match pattern7__398.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___966
                        };
                        match__433 = t___2766.clone();
                        break 'ok___5839;
                    }
                    match__433 = None;
                }
                if ! match__433.is_none() {
                    let match___1017: temper_std::regex::Match = match__433.clone().unwrap();
                    'ok___5840: {
                        'orelse___967: {
                            t___2772 = match pattern6__397.find(temper_core::string::slice( & literalPart___1003, match___1017.full().end(), literalPart___1003.len()), None) {
                                Ok(x) => x,
                                _ => break 'orelse___967
                            };
                            t___2773 = Some(t___2772.clone());
                            break 'ok___5840;
                        }
                        t___2773 = None;
                    }
                    if t___2773.is_none() {
                        t___5152 = crate::core::AfterPropagate::new(match___1017.full().value(), match___1017.full().end(), crate::core::AutoescState::new(HtmlEscaperContext::new(4, contextBefore__390.tag_state(), 4, contextBefore__390.delim_state()), before__387.subsidiary()));
                        t___5153 = HtmlUrlDelegate::new();
                        return__133 = t___5152.push(crate::core::Delegate::new(t___5153.clone()), html_codec().clone());
                        break 'fn__389;
                    }
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__434: Option<temper_std::regex::Match>;
                'ok___5841: {
                    'orelse___968: {
                        t___2787 = match pattern8__399.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___968
                        };
                        match__434 = t___2787.clone();
                        break 'ok___5841;
                    }
                    match__434 = None;
                }
                if ! match__434.is_none() {
                    let match___1018: temper_std::regex::Match = match__434.clone().unwrap();
                    'ok___5842: {
                        'orelse___969: {
                            t___2793 = match pattern6__397.find(temper_core::string::slice( & literalPart___1003, match___1018.full().end(), literalPart___1003.len()), None) {
                                Ok(x) => x,
                                _ => break 'orelse___969
                            };
                            t___2794 = Some(t___2793.clone());
                            break 'ok___5842;
                        }
                        t___2794 = None;
                    }
                    if t___2794.is_none() {
                        t___5169 = crate::core::AfterPropagate::new(match___1018.full().value(), match___1018.full().end(), crate::core::AutoescState::new(HtmlEscaperContext::new(4, contextBefore__390.tag_state(), 3, contextBefore__390.delim_state()), before__387.subsidiary()));
                        t___5170 = HtmlUrlDelegate::new();
                        return__133 = t___5169.push(crate::core::Delegate::new(t___5170.clone()), html_codec().clone());
                        break 'fn__389;
                    }
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__435: Option<temper_std::regex::Match>;
                'ok___5843: {
                    'orelse___970: {
                        t___2808 = match pattern17__408.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___970
                        };
                        match__435 = t___2808.clone();
                        break 'ok___5843;
                    }
                    match__435 = None;
                }
                if ! match__435.is_none() {
                    let match___1019: temper_std::regex::Match = match__435.clone().unwrap();
                    t___5182 = crate::core::AfterPropagate::new(match___1019.full().value(), match___1019.full().end(), crate::core::AutoescState::new(HtmlEscaperContext::new(4, contextBefore__390.tag_state(), 3, contextBefore__390.delim_state()), before__387.subsidiary()));
                    t___5183 = HtmlUrlDelegate::new();
                    return__133 = t___5182.push(crate::core::Delegate::new(t___5183.clone()), html_codec().clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__436: Option<temper_std::regex::Match>;
                'ok___5844: {
                    'orelse___971: {
                        t___2823 = match pattern19__410.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___971
                        };
                        match__436 = t___2823.clone();
                        break 'ok___5844;
                    }
                    match__436 = None;
                }
                if ! match__436.is_none() {
                    let match___1020: temper_std::regex::Match = match__436.clone().unwrap();
                    'ok___5845: {
                        'orelse___972: {
                            t___2829 = match pattern6__397.find(temper_core::string::slice( & literalPart___1003, match___1020.full().end(), literalPart___1003.len()), None) {
                                Ok(x) => x,
                                _ => break 'orelse___972
                            };
                            t___2830 = Some(t___2829.clone());
                            break 'ok___5845;
                        }
                        t___2830 = None;
                    }
                    if t___2830.is_none() {
                        t___5199 = crate::core::AfterPropagate::new(match___1020.full().value(), match___1020.full().end(), crate::core::AutoescState::new(HtmlEscaperContext::new(4, contextBefore__390.tag_state(), 1, contextBefore__390.delim_state()), before__387.subsidiary()));
                        t___5200 = HtmlCssDelegate::new();
                        return__133 = t___5199.push(crate::core::Delegate::new(t___5200.clone()), html_codec().clone());
                        break 'fn__389;
                    }
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__437: Option<temper_std::regex::Match>;
                'ok___5846: {
                    'orelse___973: {
                        t___2844 = match pattern18__409.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___973
                        };
                        match__437 = t___2844.clone();
                        break 'ok___5846;
                    }
                    match__437 = None;
                }
                if ! match__437.is_none() {
                    let match___1021: temper_std::regex::Match = match__437.clone().unwrap();
                    t___5212 = crate::core::AfterPropagate::new(match___1021.full().value(), match___1021.full().end(), crate::core::AutoescState::new(HtmlEscaperContext::new(4, contextBefore__390.tag_state(), 2, contextBefore__390.delim_state()), before__387.subsidiary()));
                    t___5213 = HtmlJsDelegate::new();
                    return__133 = t___5212.push(crate::core::Delegate::new(t___5213.clone()), html_codec().clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__438: Option<temper_std::regex::Match>;
                'ok___5847: {
                    'orelse___974: {
                        t___2859 = match pattern24__415.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___974
                        };
                        match__438 = t___2859.clone();
                        break 'ok___5847;
                    }
                    match__438 = None;
                }
                if ! match__438.is_none() {
                    let match___1022: temper_std::regex::Match = match__438.clone().unwrap();
                    t___5217 = match___1022.full().value();
                    t___5219 = match___1022.full().end();
                    t___5225 = crate::core::AutoescState::new(HtmlEscaperContext::new(4, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5217.clone(), t___5219, t___5225.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(4) {
                let match__439: Option<temper_std::regex::Match>;
                'ok___5848: {
                    'orelse___975: {
                        t___2873 = match pattern20__411.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___975
                        };
                        match__439 = t___2873.clone();
                        break 'ok___5848;
                    }
                    match__439 = None;
                }
                if ! match__439.is_none() {
                    let match___1023: temper_std::regex::Match = match__439.clone().unwrap();
                    t___5229 = match___1023.full().value();
                    t___5231 = match___1023.full().end();
                    return__133 = crate::core::AfterPropagate::new(t___5229.clone(), t___5231, before__387.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(4) {
                let match__440: Option<temper_std::regex::Match>;
                'ok___5849: {
                    'orelse___976: {
                        t___2881 = match pattern15__406.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___976
                        };
                        match__440 = t___2881.clone();
                        break 'ok___5849;
                    }
                    match__440 = None;
                }
                if ! match__440.is_none() {
                    let match___1024: temper_std::regex::Match = match__440.clone().unwrap();
                    t___5235 = match___1024.full().value();
                    t___5237 = match___1024.full().end();
                    t___5243 = crate::core::AutoescState::new(HtmlEscaperContext::new(5, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5235.clone(), t___5237, t___5243.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(4) {
                'ok___5850: {
                    'orelse___977: {
                        t___2895 = match pattern9__400.find(literalPart___1003.clone(), None) {
                            Ok(x) => x,
                            _ => break 'orelse___977
                        };
                        t___2896 = Some(t___2895.clone());
                        break 'ok___5850;
                    }
                    t___2896 = None;
                }
                if ! t___2896.is_none() {
                    t___5251 = crate::core::AutoescState::new(HtmlEscaperContext::new(7, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new("", 0usize, t___5251.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(5) {
                let match__441: Option<temper_std::regex::Match>;
                'ok___5851: {
                    'orelse___978: {
                        t___2906 = match pattern0__391.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___978
                        };
                        match__441 = t___2906.clone();
                        break 'ok___5851;
                    }
                    match__441 = None;
                }
                if ! match__441.is_none() {
                    let match___1025: temper_std::regex::Match = match__441.clone().unwrap();
                    t___5255 = match___1025.full().value();
                    t___5257 = match___1025.full().end();
                    t___5262 = crate::core::AutoescState::new(HtmlEscaperContext::new(6, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 2), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5255.clone(), t___5257, t___5262.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(5) {
                let match__442: Option<temper_std::regex::Match>;
                'ok___5852: {
                    'orelse___979: {
                        t___2919 = match pattern2__393.find(literalPart___1003.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___979
                        };
                        match__442 = t___2919.clone();
                        break 'ok___5852;
                    }
                    match__442 = None;
                }
                if ! match__442.is_none() {
                    let match___1026: temper_std::regex::Match = match__442.clone().unwrap();
                    t___5266 = match___1026.full().value();
                    t___5268 = match___1026.full().end();
                    t___5273 = crate::core::AutoescState::new(HtmlEscaperContext::new(6, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 1), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5266.clone(), t___5268, t___5273.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(5) {
                'ok___5853: {
                    'orelse___980: {
                        t___2932 = match pattern10__401.find(literalPart___1003.clone(), None) {
                            Ok(x) => x,
                            _ => break 'orelse___980
                        };
                        t___2933 = Some(t___2932.clone());
                        break 'ok___5853;
                    }
                    t___2933 = None;
                }
                if ! t___2933.is_none() {
                    t___5280 = crate::core::AutoescState::new(HtmlEscaperContext::new(6, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 0), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new("\"", 0usize, t___5280.clone());
                    break 'fn__389;
                }
            }
        }
        if literalPart__388.is_none() {
            t___5282 = contextBefore__390.html_state();
            t___2942 = Some(t___5282) == Some(5);
        } else {
            t___2942 = false;
        }
        if t___2942 {
            t___5287 = crate::core::AutoescState::new(HtmlEscaperContext::new(6, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 0), before__387.subsidiary());
            return__133 = crate::core::AfterPropagate::new("\"", 0usize, t___5287.clone());
            break 'fn__389;
        }
        if ! literalPart__388.is_none() {
            let literalPart___1028: std::sync::Arc<String> = literalPart__388.clone().unwrap();
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5290 = contextBefore__390.delim_state();
                t___2952 = Some(t___5290) == Some(0);
            } else {
                t___2952 = false;
            }
            if t___2952 {
                'ok___5854: {
                    'orelse___981: {
                        t___2953 = match pattern5__396.find(literalPart___1028.clone(), None) {
                            Ok(x) => x,
                            _ => break 'orelse___981
                        };
                        t___2954 = Some(t___2953.clone());
                        break 'ok___5854;
                    }
                    t___2954 = None;
                }
                if ! t___2954.is_none() {
                    t___5295 = crate::core::AutoescState::new(HtmlEscaperContext::new(7, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 0), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new("\"", 0usize, t___5295.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5298 = contextBefore__390.delim_state();
                t___2964 = Some(t___5298) == Some(2);
            } else {
                t___2964 = false;
            }
            if t___2964 {
                let match__443: Option<temper_std::regex::Match>;
                'ok___5855: {
                    'orelse___982: {
                        t___2965 = match pattern0__391.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___982
                        };
                        match__443 = t___2965.clone();
                        break 'ok___5855;
                    }
                    match__443 = None;
                }
                if ! match__443.is_none() {
                    let match___1029: temper_std::regex::Match = match__443.clone().unwrap();
                    t___5300 = match___1029.full().value();
                    t___5302 = match___1029.full().end();
                    t___5307 = crate::core::AutoescState::new(HtmlEscaperContext::new(7, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 0), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5300.clone(), t___5302, t___5307.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5310 = contextBefore__390.delim_state();
                t___2979 = Some(t___5310) == Some(1);
            } else {
                t___2979 = false;
            }
            if t___2979 {
                let match__444: Option<temper_std::regex::Match>;
                'ok___5856: {
                    'orelse___983: {
                        t___2980 = match pattern2__393.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___983
                        };
                        match__444 = t___2980.clone();
                        break 'ok___5856;
                    }
                    match__444 = None;
                }
                if ! match__444.is_none() {
                    let match___1030: temper_std::regex::Match = match__444.clone().unwrap();
                    t___5312 = match___1030.full().value();
                    t___5314 = match___1030.full().end();
                    t___5319 = crate::core::AutoescState::new(HtmlEscaperContext::new(7, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), 0), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5312.clone(), t___5314, t___5319.clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5322 = contextBefore__390.attrib_state();
                t___2994 = Some(t___5322) == Some(4);
            } else {
                t___2994 = false;
            }
            if t___2994 {
                let match__445: Option<temper_std::regex::Match>;
                'ok___5857: {
                    'orelse___984: {
                        t___2995 = match pattern12__403.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___984
                        };
                        match__445 = t___2995.clone();
                        break 'ok___5857;
                    }
                    match__445 = None;
                }
                if ! match__445.is_none() {
                    let match___1031: temper_std::regex::Match = match__445.clone().unwrap();
                    t___5328 = crate::core::AfterPropagate::new(match___1031.full().value(), match___1031.full().end(), before__387.clone()).pop();
                    t___5329 = HtmlUrlDelegate::new();
                    return__133 = t___5328.push(crate::core::Delegate::new(t___5329.clone()), html_codec().clone());
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5332 = contextBefore__390.delim_state();
                t___3007 = Some(t___5332) == Some(0);
            } else {
                t___3007 = false;
            }
            if t___3007 {
                let match__446: Option<temper_std::regex::Match>;
                'ok___5858: {
                    'orelse___985: {
                        t___3008 = match pattern25__416.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___985
                        };
                        match__446 = t___3008.clone();
                        break 'ok___5858;
                    }
                    match__446 = None;
                }
                if ! match__446.is_none() {
                    let match___1032: temper_std::regex::Match = match__446.clone().unwrap();
                    t___5337 = crate::core::AfterPropagate::new(match___1032.full().value(), match___1032.full().end(), before__387.clone());
                    return__133 = t___5337.feed(false);
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5340 = contextBefore__390.delim_state();
                t___3018 = Some(t___5340) == Some(0);
            } else {
                t___3018 = false;
            }
            if t___3018 {
                let match__447: Option<temper_std::regex::Match>;
                'ok___5859: {
                    'orelse___986: {
                        t___3019 = match pattern0__391.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___986
                        };
                        match__447 = t___3019.clone();
                        break 'ok___5859;
                    }
                    match__447 = None;
                }
                if ! match__447.is_none() {
                    t___5343 = crate::core::AfterPropagate::new("&#34;", match__447.clone().unwrap().full().end(), before__387.clone());
                    return__133 = t___5343.feed(false);
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5346 = contextBefore__390.delim_state();
                t___3027 = Some(t___5346) == Some(2);
            } else {
                t___3027 = false;
            }
            if t___3027 {
                let match__448: Option<temper_std::regex::Match>;
                'ok___5860: {
                    'orelse___987: {
                        t___3028 = match pattern21__412.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___987
                        };
                        match__448 = t___3028.clone();
                        break 'ok___5860;
                    }
                    match__448 = None;
                }
                if ! match__448.is_none() {
                    let match___1034: temper_std::regex::Match = match__448.clone().unwrap();
                    t___5351 = crate::core::AfterPropagate::new(match___1034.full().value(), match___1034.full().end(), before__387.clone());
                    return__133 = t___5351.feed(false);
                    break 'fn__389;
                }
            }
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5354 = contextBefore__390.delim_state();
                t___3038 = Some(t___5354) == Some(1);
            } else {
                t___3038 = false;
            }
            if t___3038 {
                let match__449: Option<temper_std::regex::Match>;
                'ok___5861: {
                    'orelse___988: {
                        t___3039 = match pattern22__413.find(literalPart___1028.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___988
                        };
                        match__449 = t___3039.clone();
                        break 'ok___5861;
                    }
                    match__449 = None;
                }
                if ! match__449.is_none() {
                    let match___1035: temper_std::regex::Match = match__449.clone().unwrap();
                    t___5359 = crate::core::AfterPropagate::new(match___1035.full().value(), match___1035.full().end(), before__387.clone());
                    return__133 = t___5359.feed(false);
                    break 'fn__389;
                }
            }
        }
        if literalPart__388.is_none() {
            if Some(contextBefore__390.html_state()) == Some(6) {
                t___5362 = contextBefore__390.attrib_state();
                t___3049 = Some(t___5362) == Some(0);
            } else {
                t___3049 = false;
            }
            t___3050 = t___3049;
        } else {
            t___3050 = false;
        }
        if t___3050 {
            return__133 = crate::core::AfterPropagate::new("", 0usize, before__387.clone());
            break 'fn__389;
        }
        if literalPart__388.is_none() {
            t___5364 = contextBefore__390.html_state();
            t___3053 = Some(t___5364) == Some(6);
        } else {
            t___3053 = false;
        }
        if t___3053 {
            t___5365 = crate::core::AfterPropagate::new("", 0usize, before__387.clone());
            return__133 = t___5365.feed(true);
            break 'fn__389;
        }
        if Some(contextBefore__390.html_state()) == Some(7) {
            t___5368 = contextBefore__390.attrib_state();
            t___3058 = Some(t___5368) == Some(0);
        } else {
            t___3058 = false;
        }
        if t___3058 {
            t___5374 = crate::core::AutoescState::new(HtmlEscaperContext::new(3, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
            return__133 = crate::core::AfterPropagate::new("", 0usize, t___5374.clone());
            break 'fn__389;
        }
        if Some(contextBefore__390.html_state()) == Some(7) {
            t___5383 = crate::core::AfterPropagate::new("", 0usize, crate::core::AutoescState::new(HtmlEscaperContext::new(3, contextBefore__390.tag_state(), 0, contextBefore__390.delim_state()), before__387.subsidiary()));
            return__133 = t___5383.pop();
            break 'fn__389;
        }
        if ! literalPart__388.is_none() {
            let literalPart___1038: std::sync::Arc<String> = literalPart__388.clone().unwrap();
            if Some(contextBefore__390.html_state()) == Some(3) {
                let match__450: Option<temper_std::regex::Match>;
                'ok___5862: {
                    'orelse___989: {
                        t___3077 = match pattern16__407.find(literalPart___1038.clone(), None) {
                            Ok(x) => Some(x),
                            _ => break 'orelse___989
                        };
                        match__450 = t___3077.clone();
                        break 'ok___5862;
                    }
                    match__450 = None;
                }
                if ! match__450.is_none() {
                    let match___1039: temper_std::regex::Match = match__450.clone().unwrap();
                    t___5387 = match___1039.full().value();
                    t___5389 = match___1039.full().end();
                    t___5395 = crate::core::AutoescState::new(HtmlEscaperContext::new(0, contextBefore__390.tag_state(), contextBefore__390.attrib_state(), contextBefore__390.delim_state()), before__387.subsidiary());
                    return__133 = crate::core::AfterPropagate::new(t___5387.clone(), t___5389, t___5395.clone());
                    break 'fn__389;
                }
            }
        }
        if literalPart__388.is_none() {
            return__133 = crate::core::AfterPropagate::new("", 0usize, before__387.clone());
            break 'fn__389;
        }
        return__133 = panic!();
    }
    return return__133.clone();
}
pub fn pick_html_escaper(stateBefore__615: crate::core::AutoescState<HtmlEscaperContext>) -> HtmlEscaper {
    let return__193: HtmlEscaper;
    let mut t___2297: bool;
    let mut t___2298: bool;
    let mut t___2299: bool;
    let mut t___2300: bool;
    let mut t___2305: HtmlDelegate;
    let mut escaper__617: HtmlEscaper;
    let mut t___4717: i32 = stateBefore__615.context().html_state();
    if Some(t___4717) == Some(0) {
        escaper__617 = HtmlEscaper::new(html_pcdata_escaper().clone());
    } else {
        if Some(t___4717) == Some(1) {
            t___2300 = true;
        } else {
            if Some(t___4717) == Some(2) {
                t___2299 = true;
            } else {
                if Some(t___4717) == Some(3) {
                    t___2298 = true;
                } else {
                    if Some(t___4717) == Some(4) {
                        t___2297 = true;
                    } else {
                        t___2297 = Some(t___4717) == Some(7);
                    }
                    t___2298 = t___2297;
                }
                t___2299 = t___2298;
            }
            t___2300 = t___2299;
        }
        if t___2300 {
            escaper__617 = HtmlEscaper::new(output_html_space_escaper().clone());
        } else {
            if Some(t___4717) == Some(5) {
                escaper__617 = temper_core::cast::<HtmlEscaper>(panic!()).unwrap();
            } else {
                if Some(t___4717) == Some(6) {
                    escaper__617 = HtmlEscaper::new(html_attribute_escaper().clone());
                } else {
                    if Some(t___4717) == Some(8) {
                        escaper__617 = HtmlEscaper::new(output_html_space_escaper().clone());
                    } else {
                        escaper__617 = temper_core::cast::<HtmlEscaper>(panic!()).unwrap();
                    }
                }
            }
        }
    }
    let subsidiary__618: Option<crate::core::Subsidiary> = stateBefore__615.subsidiary();
    if ! subsidiary__618.is_none() {
        let subsidiary___1040: crate::core::Subsidiary = subsidiary__618.clone().unwrap();
        let delegate__619: HtmlDelegate;
        'ok___5863: {
            'orelse___990: {
                t___2305 = match temper_core::cast::<HtmlDelegate>(subsidiary___1040.delegate()).ok_or_else(| | temper_core::Error::new()) {
                    Ok(x) => x,
                    _ => break 'orelse___990
                };
                delegate__619 = t___2305.clone();
                break 'ok___5863;
            }
            delegate__619 = temper_core::cast::<HtmlDelegate>(panic!()).unwrap();
        }
        return__193 = delegate__619.escaper(escaper__617.clone());
    } else {
        return__193 = escaper__617.clone();
    }
    return return__193.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn htmlDecoding__804() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___9 = temper_std::testing::Test::new();
        let actual___805: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("".to_string()));
        let mut t___5597: bool = Some(actual___805.as_str()) == Some("");
        #[derive(Clone)]
        struct ClosureGroup___1 {
            actual___805: std::sync::Arc<String>
        }
        impl ClosureGroup___1 {
            fn fn__5594(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"\") == () not ({})", self.actual___805.clone()));
            }
        }
        let closure_group = ClosureGroup___1 {
            actual___805: actual___805.clone()
        };
        let fn__5594 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5594())
        };
        test___9.assert(t___5597, fn__5594.clone());
        let actual___807: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&l".to_string()));
        let mut t___5601: bool = Some(actual___807.as_str()) == Some("&l");
        #[derive(Clone)]
        struct ClosureGroup___2 {
            actual___807: std::sync::Arc<String>
        }
        impl ClosureGroup___2 {
            fn fn__5593(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&l\") == (&l) not ({})", self.actual___807.clone()));
            }
        }
        let closure_group = ClosureGroup___2 {
            actual___807: actual___807.clone()
        };
        let fn__5593 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5593())
        };
        test___9.assert(t___5601, fn__5593.clone());
        let actual___809: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&lt".to_string()));
        let mut t___5605: bool = Some(actual___809.as_str()) == Some("<");
        #[derive(Clone)]
        struct ClosureGroup___3 {
            actual___809: std::sync::Arc<String>
        }
        impl ClosureGroup___3 {
            fn fn__5592(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&lt\") == (<) not ({})", self.actual___809.clone()));
            }
        }
        let closure_group = ClosureGroup___3 {
            actual___809: actual___809.clone()
        };
        let fn__5592 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5592())
        };
        test___9.assert(t___5605, fn__5592.clone());
        let actual___811: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&lt;".to_string()));
        let mut t___5609: bool = Some(actual___811.as_str()) == Some("<");
        #[derive(Clone)]
        struct ClosureGroup___4 {
            actual___811: std::sync::Arc<String>
        }
        impl ClosureGroup___4 {
            fn fn__5591(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&lt;\") == (<) not ({})", self.actual___811.clone()));
            }
        }
        let closure_group = ClosureGroup___4 {
            actual___811: actual___811.clone()
        };
        let fn__5591 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5591())
        };
        test___9.assert(t___5609, fn__5591.clone());
        let actual___813: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&Bcy;".to_string()));
        let mut t___5613: bool = Some(actual___813.as_str()) == Some("Б");
        #[derive(Clone)]
        struct ClosureGroup___5 {
            actual___813: std::sync::Arc<String>
        }
        impl ClosureGroup___5 {
            fn fn__5590(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&Bcy;\") == (Б) not ({})", self.actual___813.clone()));
            }
        }
        let closure_group = ClosureGroup___5 {
            actual___813: actual___813.clone()
        };
        let fn__5590 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5590())
        };
        test___9.assert(t___5613, fn__5590.clone());
        let actual___815: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&Bcy".to_string()));
        let mut t___5617: bool = Some(actual___815.as_str()) == Some("&Bcy");
        #[derive(Clone)]
        struct ClosureGroup___6 {
            actual___815: std::sync::Arc<String>
        }
        impl ClosureGroup___6 {
            fn fn__5589(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&Bcy\") == (&Bcy) not ({})", self.actual___815.clone()));
            }
        }
        let closure_group = ClosureGroup___6 {
            actual___815: actual___815.clone()
        };
        let fn__5589 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5589())
        };
        test___9.assert(t___5617, fn__5589.clone());
        let actual___817: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&LT;".to_string()));
        let mut t___5621: bool = Some(actual___817.as_str()) == Some("<");
        #[derive(Clone)]
        struct ClosureGroup___7 {
            actual___817: std::sync::Arc<String>
        }
        impl ClosureGroup___7 {
            fn fn__5588(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&LT;\") == (<) not ({})", self.actual___817.clone()));
            }
        }
        let closure_group = ClosureGroup___7 {
            actual___817: actual___817.clone()
        };
        let fn__5588 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5588())
        };
        test___9.assert(t___5621, fn__5588.clone());
        let actual___819: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&Aacute;".to_string()));
        let mut t___5625: bool = Some(actual___819.as_str()) == Some("Á");
        #[derive(Clone)]
        struct ClosureGroup___8 {
            actual___819: std::sync::Arc<String>
        }
        impl ClosureGroup___8 {
            fn fn__5587(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&Aacute;\") == (Á) not ({})", self.actual___819.clone()));
            }
        }
        let closure_group = ClosureGroup___8 {
            actual___819: actual___819.clone()
        };
        let fn__5587 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5587())
        };
        test___9.assert(t___5625, fn__5587.clone());
        let actual___821: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&aacute;".to_string()));
        let mut t___5629: bool = Some(actual___821.as_str()) == Some("á");
        #[derive(Clone)]
        struct ClosureGroup___9 {
            actual___821: std::sync::Arc<String>
        }
        impl ClosureGroup___9 {
            fn fn__5586(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&aacute;\") == (á) not ({})", self.actual___821.clone()));
            }
        }
        let closure_group = ClosureGroup___9 {
            actual___821: actual___821.clone()
        };
        let fn__5586 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5586())
        };
        test___9.assert(t___5629, fn__5586.clone());
        let actual___823: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&AaCuTe;".to_string()));
        let mut t___5633: bool = Some(actual___823.as_str()) == Some("&AaCuTe;");
        #[derive(Clone)]
        struct ClosureGroup___10 {
            actual___823: std::sync::Arc<String>
        }
        impl ClosureGroup___10 {
            fn fn__5585(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&AaCuTe;\") == (&AaCuTe;) not ({})", self.actual___823.clone()));
            }
        }
        let closure_group = ClosureGroup___10 {
            actual___823: actual___823.clone()
        };
        let fn__5585 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5585())
        };
        test___9.assert(t___5633, fn__5585.clone());
        let actual___825: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&gt;;".to_string()));
        let mut t___5637: bool = Some(actual___825.as_str()) == Some(">;");
        #[derive(Clone)]
        struct ClosureGroup___11 {
            actual___825: std::sync::Arc<String>
        }
        impl ClosureGroup___11 {
            fn fn__5584(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&gt;;\") == (>;) not ({})", self.actual___825.clone()));
            }
        }
        let closure_group = ClosureGroup___11 {
            actual___825: actual___825.clone()
        };
        let fn__5584 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5584())
        };
        test___9.assert(t___5637, fn__5584.clone());
        let actual___827: std::sync::Arc<String> = html_codec().decode(std::sync::Arc::new("&amp;lt;".to_string()));
        let mut t___5641: bool = Some(actual___827.as_str()) == Some("&lt;");
        #[derive(Clone)]
        struct ClosureGroup___12 {
            actual___827: std::sync::Arc<String>
        }
        impl ClosureGroup___12 {
            fn fn__5583(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.decode(\"&amp;lt;\") == (&lt;) not ({})", self.actual___827.clone()));
            }
        }
        let closure_group = ClosureGroup___12 {
            actual___827: actual___827.clone()
        };
        let fn__5583 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5583())
        };
        test___9.assert(t___5641, fn__5583.clone());
        test___9.soft_fail_to_hard()
    }
    #[test]
    fn htmlEncoding__829() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___10 = temper_std::testing::Test::new();
        let actual___830: std::sync::Arc<String> = html_codec().encode(std::sync::Arc::new("".to_string()));
        let mut t___5573: bool = Some(actual___830.as_str()) == Some("");
        #[derive(Clone)]
        struct ClosureGroup___13 {
            actual___830: std::sync::Arc<String>
        }
        impl ClosureGroup___13 {
            fn fn__5570(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.encode(\"\") == () not ({})", self.actual___830.clone()));
            }
        }
        let closure_group = ClosureGroup___13 {
            actual___830: actual___830.clone()
        };
        let fn__5570 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5570())
        };
        test___10.assert(t___5573, fn__5570.clone());
        let actual___832: std::sync::Arc<String> = html_codec().encode(std::sync::Arc::new("Hello, World!".to_string()));
        let mut t___5577: bool = Some(actual___832.as_str()) == Some("Hello, World!");
        #[derive(Clone)]
        struct ClosureGroup___14 {
            actual___832: std::sync::Arc<String>
        }
        impl ClosureGroup___14 {
            fn fn__5569(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.encode(\"Hello, World!\") == (Hello, World!) not ({})", self.actual___832.clone()));
            }
        }
        let closure_group = ClosureGroup___14 {
            actual___832: actual___832.clone()
        };
        let fn__5569 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5569())
        };
        test___10.assert(t___5577, fn__5569.clone());
        let actual___834: std::sync::Arc<String> = html_codec().encode(std::sync::Arc::new("<foo> & <bar baz='b\"oo'> far".to_string()));
        let mut t___5581: bool = Some(actual___834.as_str()) == Some("&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far");
        #[derive(Clone)]
        struct ClosureGroup___15 {
            actual___834: std::sync::Arc<String>
        }
        impl ClosureGroup___15 {
            fn fn__5568(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected `-work/src//html/`.htmlCodec.encode(\"<foo> & <bar baz='b\\\"oo'> far\") == (&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far) not ({})", self.actual___834.clone()));
            }
        }
        let closure_group = ClosureGroup___15 {
            actual___834: actual___834.clone()
        };
        let fn__5568 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__5568())
        };
        test___10.assert(t___5581, fn__5568.clone());
        test___10.soft_fail_to_hard()
    }
    #[test]
    fn helloWorldHtmlStyle__899() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let mut t___4675: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4675.append_safe(std::sync::Arc::new("Hello, <b>".to_string()));
        t___4675.append_string("World");
        t___4675.append_safe(std::sync::Arc::new("</b>!".to_string()));
        let actual___900: std::sync::Arc<String> = t___4675.accumulated().to_string();
        let mut t___4682: bool = Some(actual___900.as_str()) == Some("Hello, <b>World</b>!");
        #[derive(Clone)]
        struct ClosureGroup___16 {
            actual___900: std::sync::Arc<String>
        }
        impl ClosureGroup___16 {
            fn fn__4674(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"Hello, <b>\", \\interpolate, \"World\", \"</b>!\").toString() == (Hello, <b>World</b>!) not ({})", self.actual___900.clone()));
            }
        }
        let closure_group = ClosureGroup___16 {
            actual___900: actual___900.clone()
        };
        let fn__4674 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4674())
        };
        test___32.assert(t___4682, fn__4674.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn autoescaped__903() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let mut t___4665: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4665.append_safe(std::sync::Arc::new("1 + 1 ".to_string()));
        t___4665.append_string("<");
        t___4665.append_safe(std::sync::Arc::new(" 3.".to_string()));
        let actual___904: std::sync::Arc<String> = t___4665.accumulated().to_string();
        let mut t___4672: bool = Some(actual___904.as_str()) == Some("1 + 1 &lt; 3.");
        #[derive(Clone)]
        struct ClosureGroup___17 {
            actual___904: std::sync::Arc<String>
        }
        impl ClosureGroup___17 {
            fn fn__4664(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"1 + 1 \", \\interpolate, \"<\", \" 3.\").toString() == (1 + 1 &lt; 3.) not ({})", self.actual___904.clone()));
            }
        }
        let closure_group = ClosureGroup___17 {
            actual___904: actual___904.clone()
        };
        let fn__4664 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4664())
        };
        test___33.assert(t___4672, fn__4664.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn contextMattersUrlsEmbed__907() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let mut t___4642: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4642.append_safe(std::sync::Arc::new("<a href='".to_string()));
        t___4642.append_string("https://example.com/isn't-a-problem");
        t___4642.append_safe(std::sync::Arc::new("'>".to_string()));
        t___4642.append_string("https://example.com/isn't-a-problem");
        t___4642.append_safe(std::sync::Arc::new("</a>".to_string()));
        let actual___908: std::sync::Arc<String> = t___4642.accumulated().to_string();
        let mut t___4651: bool = Some(actual___908.as_str()) == Some("<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            actual___908: std::sync::Arc<String>
        }
        impl ClosureGroup___18 {
            fn okUrl__483(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("https://example.com/isn't-a-problem".to_string());
            }
            fn evilUrl__484(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("javascript:alert('evil done')".to_string());
            }
            fn fn__4641(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"<a href='\", \\interpolate, okUrl(), \"'>\", \\interpolate, okUrl(), \"</a>\").toString() == (<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>) not ({})", self.actual___908.clone()));
            }
        }
        let closure_group = ClosureGroup___18 {
            actual___908: actual___908.clone()
        };
        let okUrl__483 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.okUrl__483())
        };
        let evilUrl__484 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.evilUrl__484())
        };
        let fn__4641 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4641())
        };
        test___34.assert(t___4651, fn__4641.clone());
        let mut t___4653: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4653.append_safe(std::sync::Arc::new("<a href='".to_string()));
        t___4653.append_string("javascript:alert('evil done')");
        t___4653.append_safe(std::sync::Arc::new("'>".to_string()));
        t___4653.append_string("javascript:alert('evil done')");
        t___4653.append_safe(std::sync::Arc::new("</a>".to_string()));
        let actual___911: std::sync::Arc<String> = t___4653.accumulated().to_string();
        let mut t___4662: bool = Some(actual___911.as_str()) == Some("<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>");
        #[derive(Clone)]
        struct ClosureGroup___19 {
            actual___911: std::sync::Arc<String>
        }
        impl ClosureGroup___19 {
            fn fn__4640(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"<a href='\", \\interpolate, evilUrl(), \"'>\", \\interpolate, evilUrl(), \"</a>\").toString() == (<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>) not ({})", self.actual___911.clone()));
            }
        }
        let closure_group = ClosureGroup___19 {
            actual___911: actual___911.clone()
        };
        let fn__4640 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4640())
        };
        test___34.assert(t___4662, fn__4640.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn quoteAdjustments__914() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let className__488: std::sync::Arc<String> = std::sync::Arc::new("some-class".to_string());
        let mut t___4629: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4629.append_safe(std::sync::Arc::new("<hr class=".to_string()));
        t___4629.append_string("some-class");
        t___4629.append_safe(std::sync::Arc::new("><hr class='".to_string()));
        t___4629.append_string("some-class");
        t___4629.append_safe(std::sync::Arc::new("'><hr class=other-class>".to_string()));
        let actual___915: std::sync::Arc<String> = t___4629.accumulated().to_string();
        let mut t___4638: bool = Some(actual___915.as_str()) == Some("<hr class=\"some-class\"><hr class='some-class'><hr class=\"other-class\">");
        #[derive(Clone)]
        struct ClosureGroup___20 {
            actual___915: std::sync::Arc<String>
        }
        impl ClosureGroup___20 {
            fn fn__4628(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"<hr class=\", \\interpolate, className, \"><hr class='\", \\interpolate, className, \"'><hr class=other-class>\").toString() == (<hr class=\"some-class\"><hr class='some-class'><hr class=\"other-class\">) not ({})", self.actual___915.clone()));
            }
        }
        let closure_group = ClosureGroup___20 {
            actual___915: actual___915.clone()
        };
        let fn__4628 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4628())
        };
        test___35.assert(t___4638, fn__4628.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn safehtmlInjectedInTagAndAttributeContext__918() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let mut t___4614: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4614.append_safe(std::sync::Arc::new("I <3 <b>Ponies</b>!".to_string()));
        let love__490: SafeHtml = t___4614.accumulated();
        let mut t___4617: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4617.append_safe(std::sync::Arc::new("<b>".to_string()));
        t___4617.append_safe_html(love__490.clone());
        t___4617.append_safe(std::sync::Arc::new("</b><img alt='".to_string()));
        t___4617.append_safe_html(love__490.clone());
        t___4617.append_safe(std::sync::Arc::new("' src='ponies'>".to_string()));
        let actual___920: std::sync::Arc<String> = t___4617.accumulated().to_string();
        let mut t___4626: bool = Some(actual___920.as_str()) == Some("<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>");
        #[derive(Clone)]
        struct ClosureGroup___21 {
            actual___920: std::sync::Arc<String>
        }
        impl ClosureGroup___21 {
            fn fn__4613(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"<b>\", \\interpolate, love, \"</b><img alt='\", \\interpolate, love, \"' src='ponies'>\").toString() == (<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>) not ({})", self.actual___920.clone()));
            }
        }
        let closure_group = ClosureGroup___21 {
            actual___920: actual___920.clone()
        };
        let fn__4613 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4613())
        };
        test___36.assert(t___4626, fn__4613.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn loopingInsideAnHtmlExpression__923() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        let items__492: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("One".to_string()), std::sync::Arc::new("<Two>".to_string()), std::sync::Arc::new("Three".to_string())]);
        let accumulator___38: SafeHtmlBuilder = SafeHtmlBuilder::new();
        accumulator___38.append_safe(std::sync::Arc::new("<ul>\x0a".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {
            accumulator___38: SafeHtmlBuilder
        }
        impl ClosureGroup___22 {
            fn fn__4602(& self, item__495: impl temper_core::ToArcString) {
                let item__495 = item__495.to_arc_string();
                self.accumulator___38.append_safe(std::sync::Arc::new("  <li>".to_string()));
                self.accumulator___38.append_string(item__495.clone());
                self.accumulator___38.append_safe(std::sync::Arc::new("</li>\x0a".to_string()));
            }
        }
        let closure_group = ClosureGroup___22 {
            accumulator___38: accumulator___38.clone()
        };
        let fn__4602 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | item__495: std::sync::Arc<String> | closure_group.fn__4602(item__495))
        };
        temper_core::listed::list_for_each( & items__492, & ( * fn__4602.clone()));
        accumulator___38.append_safe(std::sync::Arc::new("</ul>".to_string()));
        let got__493: SafeHtml = accumulator___38.accumulated();
        let actual___924: std::sync::Arc<String> = got__493.text();
        let mut t___4611: bool = Some(actual___924.as_str()) == Some("<ul>\x0a  <li>One</li>\x0a  <li>&lt;Two&gt;</li>\x0a  <li>Three</li>\x0a</ul>");
        #[derive(Clone)]
        struct ClosureGroup___23 {
            actual___924: std::sync::Arc<String>
        }
        impl ClosureGroup___23 {
            fn fn__4601(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected got.text == (<ul>\x0a  <li>One</li>\x0a  <li>&lt;Two&gt;</li>\x0a  <li>Three</li>\x0a</ul>) not ({})", self.actual___924.clone()));
            }
        }
        let closure_group = ClosureGroup___23 {
            actual___924: actual___924.clone()
        };
        let fn__4601 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4601())
        };
        test___37.assert(t___4611, fn__4601.clone());
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn doubleQuotesInAttributeValueWithInsertedQuotes__935() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___4589: SafeHtmlBuilder = SafeHtmlBuilder::new();
        t___4589.append_safe(std::sync::Arc::new("<div id=a\"b>".to_string()));
        let actual___936: std::sync::Arc<String> = t___4589.accumulated().text();
        let mut t___4594: bool = Some(actual___936.as_str()) == Some("<div id=\"a&#34;b\">");
        #[derive(Clone)]
        struct ClosureGroup___24 {
            actual___936: std::sync::Arc<String>
        }
        impl ClosureGroup___24 {
            fn fn__4588(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work/src//html/`.html, true, \"<div id=a\\\"b>\").text == (<div id=\"a&#34;b\">) not ({})", self.actual___936.clone()));
            }
        }
        let closure_group = ClosureGroup___24 {
            actual___936: actual___936.clone()
        };
        let fn__4588 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4588())
        };
        test___52.assert(t___4594, fn__4588.clone());
        test___52.soft_fail_to_hard()
    }
    use super::*;
}
