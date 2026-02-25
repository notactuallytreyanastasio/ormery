using H = Ormery.Html;
using C = TemperLang.Core;
using R = TemperLang.Std.Regex;
namespace Ormery.Html
{
    public class HtmlProtocolFilteringUrlEscaper: IUrlEscaper
    {
        public static readonly HtmlProtocolFilteringUrlEscaper Instance = new HtmlProtocolFilteringUrlEscaper();
        public SafeUrl ApplySafeUrl(SafeUrl x__721)
        {
            return x__721;
        }
        public SafeUrl ApplyString(string x__724)
        {
            SafeUrl return__243;
            int t___5422;
            R::Match t___3125;
            R::Match ? t___3126;
            {
                {
                    int protocolEnd__726 = 0;
                    int end__727 = x__724.Length;
                    while (protocolEnd__726 < end__727)
                    {
                        int cp__728 = C::StringUtil.Get(x__724, protocolEnd__726);
                        if (cp__728 == 58)
                        {
                            string protocol__729 = C::StringUtil.Slice(x__724, 0, protocolEnd__726);
                            try
                            {
                                t___3125 = H::HtmlGlobal.protocolAllowList__293.Find(protocol__729);
                                t___3126 = t___3125;
                            }
                            catch
                            {
                                t___3126 = null;
                            }
                            if (!(t___3126 == null))
                            {
                                return__243 = new SafeUrl(x__724);
                            }
                            else
                            {
                                return__243 = H::HtmlGlobal.fallbackSafeUrl__294;
                            }
                            goto fn__725;
                        }
                        t___5422 = C::StringUtil.Next(x__724, protocolEnd__726);
                        protocolEnd__726 = t___5422;
                    }
                    return__243 = H::HtmlGlobal.htmlUrlPartUrlEscaper__298.ApplyString(x__724);
                }
                fn__725:
                {
                }
            }
            return return__243;
        }
        public HtmlProtocolFilteringUrlEscaper()
        {
        }
    }
}
