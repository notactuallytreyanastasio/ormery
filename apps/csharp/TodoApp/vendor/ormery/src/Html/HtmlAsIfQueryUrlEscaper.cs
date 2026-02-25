using H = Ormery.Html;
using U = Ormery.Url;
using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Html
{
    public class HtmlAsIfQueryUrlEscaper: IUrlEscaper
    {
        public static readonly HtmlAsIfQueryUrlEscaper Instance = new HtmlAsIfQueryUrlEscaper();
        public SafeUrl ApplySafeUrl(SafeUrl x__752)
        {
            return x__752;
        }
        public SafeUrl ApplyString(string x__755)
        {
            bool t___5410;
            int t___5413;
            int t___5414;
            string t___5416;
            bool t___3113;
            string t___3119;
            int i__757 = 0;
            int end__758 = x__755.Length;
            int emitted__759 = 0;
            T::StringBuilder sb__760 = new T::StringBuilder();
            while (i__757 < end__758)
            {
                int cp__761 = C::StringUtil.Get(x__755, i__757);
                if (cp__761 < H::HtmlGlobal.urlQuerySafe__296.Count)
                {
                    t___5410 = H::HtmlGlobal.urlQuerySafe__296[cp__761];
                    t___3113 = !t___5410;
                }
                else
                {
                    t___3113 = false;
                }
                if (t___3113)
                {
                    C::StringUtil.AppendBetween(sb__760, x__755, emitted__759, i__757);
                    U::UrlGlobal.PercentEscapeOctetTo(cp__761, sb__760);
                    t___5413 = C::StringUtil.Next(x__755, i__757);
                    emitted__759 = t___5413;
                }
                t___5414 = C::StringUtil.Next(x__755, i__757);
                i__757 = t___5414;
            }
            if (emitted__759 > 0)
            {
                C::StringUtil.AppendBetween(sb__760, x__755, emitted__759, end__758);
                t___5416 = sb__760.ToString();
                t___3119 = t___5416;
            }
            else
            {
                t___3119 = x__755;
            }
            return new SafeUrl(t___3119);
        }
        public HtmlAsIfQueryUrlEscaper()
        {
        }
    }
}
