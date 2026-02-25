using H = Ormery.Html;
using U = Ormery.Url;
using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Html
{
    public class HtmlUrlPartUrlEscaper: IUrlEscaper
    {
        public static readonly HtmlUrlPartUrlEscaper Instance = new HtmlUrlPartUrlEscaper();
        public SafeUrl ApplySafeUrl(SafeUrl x__738)
        {
            return x__738;
        }
        public SafeUrl ApplyString(string x__741)
        {
            bool t___5428;
            int t___5431;
            int t___5432;
            string t___5434;
            bool t___3137;
            string t___3143;
            int i__743 = 0;
            int end__744 = x__741.Length;
            int emitted__745 = 0;
            T::StringBuilder sb__746 = new T::StringBuilder();
            while (i__743 < end__744)
            {
                int cp__747 = C::StringUtil.Get(x__741, i__743);
                if (cp__747 < H::HtmlGlobal.urlSafe__297.Count)
                {
                    t___5428 = H::HtmlGlobal.urlSafe__297[cp__747];
                    t___3137 = !t___5428;
                }
                else
                {
                    t___3137 = false;
                }
                if (t___3137)
                {
                    C::StringUtil.AppendBetween(sb__746, x__741, emitted__745, i__743);
                    U::UrlGlobal.PercentEscapeOctetTo(cp__747, sb__746);
                    t___5431 = C::StringUtil.Next(x__741, i__743);
                    emitted__745 = t___5431;
                }
                t___5432 = C::StringUtil.Next(x__741, i__743);
                i__743 = t___5432;
            }
            if (emitted__745 > 0)
            {
                C::StringUtil.AppendBetween(sb__746, x__741, emitted__745, end__744);
                t___5434 = sb__746.ToString();
                t___3143 = t___5434;
            }
            else
            {
                t___3143 = x__741;
            }
            return new SafeUrl(t___3143);
        }
        public HtmlUrlPartUrlEscaper()
        {
        }
    }
}
