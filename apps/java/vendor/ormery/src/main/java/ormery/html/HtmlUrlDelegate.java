package ormery.html;
import ormery.core.AutoescState;
import temper.core.Core;
import temper.core.Nullable;
import ormery.core.Subsidiary;
import ormery.core.ContextDelegate;
import ormery.core.ContextPropagator;
import static ormery.html.HtmlGlobal.urlContextPropagator__268;
import static ormery.html.HtmlGlobal.htmlUrlPartUrlEscaper__298;
import static ormery.html.HtmlGlobal.htmlAsIfQueryUrlEscaper__299;
import static ormery.html.HtmlGlobal.htmlProtocolFilteringUrlEscaper__295;
public final class HtmlUrlDelegate implements ContextDelegate<UrlEscaperContext>, HtmlDelegate {
    AutoescState<UrlEscaperContext> _state;
    @Nullable Subsidiary _subsidiary;
    public AutoescState<UrlEscaperContext> getState() {
        return this._state;
    }
    public void setState(AutoescState<UrlEscaperContext> x__682) {
        this._state = x__682;
    }
    public ContextPropagator<UrlEscaperContext> getContextPropagator() {
        return urlContextPropagator__268;
    }
    public HtmlEscaper escaper(HtmlEscaper outer__687) {
        HtmlEscaper return__229;
        boolean t_3102;
        int t_5402 = this.getState().getContext().getUrlState();
        if (t_5402 == 0) {
            return__229 = new HtmlUrlEscaperAdapter(htmlProtocolFilteringUrlEscaper__295, outer__687);
        } else if (t_5402 == 1) {
            return__229 = new HtmlUrlEscaperAdapter(htmlUrlPartUrlEscaper__298, outer__687);
        } else {
            if (t_5402 == 2) {
                t_3102 = true;
            } else {
                t_3102 = t_5402 == 3;
            }
            if (t_3102) {
                return__229 = new HtmlUrlEscaperAdapter(htmlAsIfQueryUrlEscaper__299, outer__687);
            } else {
                throw Core.bubble();
            }
        }
        return return__229;
    }
    public HtmlUrlDelegate() {
        AutoescState<UrlEscaperContext> t_5399 = new AutoescState<>(new UrlEscaperContext(0), null);
        this._state = t_5399;
        this._subsidiary = null;
    }
}
