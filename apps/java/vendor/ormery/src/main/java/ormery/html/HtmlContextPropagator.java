package ormery.html;
import temper.core.Nullable;
import ormery.core.AutoescState;
import ormery.core.AfterPropagate;
import ormery.core.ContextPropagator;
public final class HtmlContextPropagator implements ContextPropagator<HtmlEscaperContext> {
    public AfterPropagate<HtmlEscaperContext> after(AutoescState<HtmlEscaperContext> before__452, @Nullable String literalPart__453) {
        return HtmlGlobal.htmlPropagateContext__264(before__452, literalPart__453);
    }
    public HtmlContextPropagator() {
    }
}
