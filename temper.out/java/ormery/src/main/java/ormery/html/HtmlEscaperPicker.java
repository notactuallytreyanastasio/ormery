package ormery.html;
import ormery.core.AutoescState;
import ormery.core.EscaperPicker;
public final class HtmlEscaperPicker implements EscaperPicker<HtmlEscaperContext, HtmlEscaper> {
    public HtmlEscaper escaperFor(AutoescState<HtmlEscaperContext> stateBefore__621) {
        return HtmlGlobal.pickHtmlEscaper(stateBefore__621);
    }
    public HtmlEscaperPicker() {
    }
}
