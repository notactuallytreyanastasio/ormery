using C = Ormery.Core;
using H = Ormery.Html;
namespace Ormery.Html
{
    public class HtmlEscaperPicker: C::IEscaperPicker<HtmlEscaperContext, IHtmlEscaper>
    {
        public IHtmlEscaper EscaperFor(C::AutoescState<HtmlEscaperContext> stateBefore__621)
        {
            return H::HtmlGlobal.PickHtmlEscaper(stateBefore__621);
        }
        public HtmlEscaperPicker()
        {
        }
    }
}
