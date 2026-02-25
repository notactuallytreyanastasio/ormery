using H = Ormery.Html;
using S = System;
using C = TemperLang.Core;
namespace Ormery.Html
{
    public class HtmlAttributeEscaper: IHtmlEscaper
    {
        public static readonly HtmlAttributeEscaper Instance = new HtmlAttributeEscaper();
        public string ApplySafeHtml(SafeHtml x__596)
        {
            string t___4731 = H::HtmlGlobal.HtmlCodec.Decode(x__596.Text);
            return this.ApplyString(t___4731);
        }
        public string ApplySafeUrl(SafeUrl x__599)
        {
            string t___4728 = x__599.Text;
            return this.ApplyString(t___4728);
        }
        public string ApplyInt32(int x__602)
        {
            string t___4726 = S::Convert.ToString(x__602);
            return this.ApplyString(t___4726);
        }
        public string ApplyInt64(long x__605)
        {
            string t___4724 = S::Convert.ToString(x__605);
            return this.ApplyString(t___4724);
        }
        public string ApplyFloat64(double x__608)
        {
            string t___4722 = C::Float64.Format(x__608);
            return this.ApplyString(t___4722);
        }
        public string ApplyString(string x__611)
        {
            return H::HtmlGlobal.HtmlCodec.Encode(x__611);
        }
        public HtmlAttributeEscaper()
        {
        }
    }
}
