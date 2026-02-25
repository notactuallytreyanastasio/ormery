using S = System;
using C = TemperLang.Core;
namespace Ormery.Html
{
    public class HtmlUrlEscaperAdapter: IHtmlEscaper
    {
        readonly IUrlEscaper first__690;
        readonly IHtmlEscaper second__691;
        public string ApplySafeHtml(SafeHtml x__693)
        {
            string t___5449 = x__693.Text;
            SafeUrl t___5450 = this.first__690.ApplyString(t___5449);
            return this.second__691.ApplySafeUrl(t___5450);
        }
        public string ApplySafeUrl(SafeUrl x__696)
        {
            SafeUrl t___5447 = this.first__690.ApplySafeUrl(x__696);
            return this.second__691.ApplySafeUrl(t___5447);
        }
        public string ApplyInt32(int x__699)
        {
            string t___5444 = S::Convert.ToString(x__699);
            SafeUrl t___5445 = this.first__690.ApplyString(t___5444);
            return this.second__691.ApplySafeUrl(t___5445);
        }
        public string ApplyInt64(long x__702)
        {
            string t___5441 = S::Convert.ToString(x__702);
            SafeUrl t___5442 = this.first__690.ApplyString(t___5441);
            return this.second__691.ApplySafeUrl(t___5442);
        }
        public string ApplyFloat64(double x__705)
        {
            string t___5438 = C::Float64.Format(x__705);
            SafeUrl t___5439 = this.first__690.ApplyString(t___5438);
            return this.second__691.ApplySafeUrl(t___5439);
        }
        public string ApplyString(string x__708)
        {
            SafeUrl t___5436 = this.first__690.ApplyString(x__708);
            return this.second__691.ApplySafeUrl(t___5436);
        }
        public HtmlUrlEscaperAdapter(IUrlEscaper first__711, IHtmlEscaper second__712)
        {
            this.first__690 = first__711;
            this.second__691 = second__712;
        }
        public IUrlEscaper First
        {
            get
            {
                return this.first__690;
            }
        }
        public IHtmlEscaper Second
        {
            get
            {
                return this.second__691;
            }
        }
    }
}
