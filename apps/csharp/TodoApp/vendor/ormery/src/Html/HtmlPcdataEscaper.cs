using H = Ormery.Html;
using S = System;
using C = TemperLang.Core;
namespace Ormery.Html
{
    public class HtmlPcdataEscaper: IHtmlEscaper
    {
        public static readonly HtmlPcdataEscaper Instance = new HtmlPcdataEscaper();
        public string ApplySafeHtml(SafeHtml x__575)
        {
            return x__575.ToString();
        }
        public string ApplySafeUrl(SafeUrl x__578)
        {
            string t___4740 = x__578.Text;
            return this.ApplyString(t___4740);
        }
        public string ApplyInt32(int x__581)
        {
            string t___4738 = S::Convert.ToString(x__581);
            return this.ApplyString(t___4738);
        }
        public string ApplyInt64(long x__584)
        {
            string t___4736 = S::Convert.ToString(x__584);
            return this.ApplyString(t___4736);
        }
        public string ApplyFloat64(double x__587)
        {
            string t___4734 = C::Float64.Format(x__587);
            return this.ApplyString(t___4734);
        }
        public string ApplyString(string x__590)
        {
            return H::HtmlGlobal.HtmlCodec.Encode(x__590);
        }
        public HtmlPcdataEscaper()
        {
        }
    }
}
