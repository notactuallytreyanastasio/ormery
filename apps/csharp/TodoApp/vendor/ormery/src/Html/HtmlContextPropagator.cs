using C = Ormery.Core;
using H = Ormery.Html;
namespace Ormery.Html
{
    public class HtmlContextPropagator: C::IContextPropagator<HtmlEscaperContext>
    {
        public C::AfterPropagate<HtmlEscaperContext> After(C::AutoescState<HtmlEscaperContext> before__452, string ? literalPart__453)
        {
            return H::HtmlGlobal.htmlPropagateContext__264(before__452, literalPart__453);
        }
        public HtmlContextPropagator()
        {
        }
    }
}
