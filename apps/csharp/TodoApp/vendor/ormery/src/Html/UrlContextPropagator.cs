using C = Ormery.Core;
using H = Ormery.Html;
namespace Ormery.Html
{
    public class UrlContextPropagator: C::IContextPropagator<UrlEscaperContext>
    {
        public C::AfterPropagate<UrlEscaperContext> After(C::AutoescState<UrlEscaperContext> before__476, string ? literalPart__477)
        {
            return H::HtmlGlobal.urlPropagateContext__266(before__476, literalPart__477);
        }
        public UrlContextPropagator()
        {
        }
    }
}
