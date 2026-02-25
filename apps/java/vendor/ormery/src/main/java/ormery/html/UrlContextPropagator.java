package ormery.html;
import temper.core.Nullable;
import ormery.core.AutoescState;
import ormery.core.AfterPropagate;
import ormery.core.ContextPropagator;
public final class UrlContextPropagator implements ContextPropagator<UrlEscaperContext> {
    public AfterPropagate<UrlEscaperContext> after(AutoescState<UrlEscaperContext> before__476, @Nullable String literalPart__477) {
        return HtmlGlobal.urlPropagateContext__266(before__476, literalPart__477);
    }
    public UrlContextPropagator() {
    }
}
