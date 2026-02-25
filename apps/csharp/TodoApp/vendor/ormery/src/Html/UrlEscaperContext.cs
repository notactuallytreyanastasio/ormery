using C = Ormery.Core;
using H = Ormery.Html;
namespace Ormery.Html
{
    public class UrlEscaperContext: C::IContext
    {
        readonly int urlState__519;
        public string ToString()
        {
            return "UrlEscaperContext(" + H::HtmlGlobal.urlStateStr__265(this.urlState__519) + ")";
        }
        public UrlEscaperContext(int urlState__523)
        {
            this.urlState__519 = urlState__523;
        }
        public int UrlState
        {
            get
            {
                return this.urlState__519;
            }
        }
    }
}
