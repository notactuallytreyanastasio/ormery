package ormery.html;
import ormery.core.AutoescState;
import ormery.core.EscaperPicker;
import ormery.core.ContextPropagator;
import ormery.core.ContextualAutoescapingAccumulator;
public final class SafeHtmlBuilder implements ContextualAutoescapingAccumulator<HtmlEscaperContext, HtmlEscaper> {
    public static StringBuilder newCollector() {
        return new StringBuilder();
    }
    public static AutoescState<HtmlEscaperContext> initialState() {
        return new AutoescState<>(new HtmlEscaperContext(0, 0, 0, 0), null);
    }
    public static HtmlContextPropagator propagator() {
        return new HtmlContextPropagator();
    }
    public static EscaperPicker<HtmlEscaperContext, HtmlEscaper> picker() {
        return new HtmlEscaperPicker();
    }
    public static SafeHtml fromCollector(StringBuilder collector__633) {
        return new SafeHtml(collector__633.toString());
    }
    public static AutoescState<HtmlEscaperContext> mergeStates(AutoescState<HtmlEscaperContext> a__636, AutoescState<HtmlEscaperContext> b__637) {
        return a__636;
    }
    AutoescState<HtmlEscaperContext> _state;
    final StringBuilder collector;
    public AutoescState<HtmlEscaperContext> getState() {
        return this._state;
    }
    public void setState(AutoescState<HtmlEscaperContext> x__644) {
        this._state = x__644;
    }
    public EscaperPicker<HtmlEscaperContext, HtmlEscaper> getEscaperPicker() {
        return SafeHtmlBuilder.picker();
    }
    public ContextPropagator<HtmlEscaperContext> getContextPropagator() {
        return SafeHtmlBuilder.propagator();
    }
    public SafeHtml getAccumulated() {
        return SafeHtmlBuilder.fromCollector(this.collector);
    }
    public void collectFixed(String fixed__653) {
        this.collector.append(fixed__653);
    }
    public void appendSafeHtml(SafeHtml x__656) {
        String t_4702 = this.prepareForAppend().applySafeHtml(x__656);
        this.collector.append(t_4702);
    }
    public void appendSafeUrl(SafeUrl x__659) {
        String t_4699 = this.prepareForAppend().applySafeUrl(x__659);
        this.collector.append(t_4699);
    }
    public void appendInt32(int x__662) {
        String t_4696 = this.prepareForAppend().applyInt32(x__662);
        this.collector.append(t_4696);
    }
    public void appendInt64(long x__665) {
        String t_4693 = this.prepareForAppend().applyInt64(x__665);
        this.collector.append(t_4693);
    }
    public void appendFloat64(double x__668) {
        String t_4690 = this.prepareForAppend().applyFloat64(x__668);
        this.collector.append(t_4690);
    }
    public void appendString(String x__671) {
        String t_4687 = this.prepareForAppend().applyString(x__671);
        this.collector.append(t_4687);
    }
    public SafeHtmlBuilder() {
        AutoescState<HtmlEscaperContext> t_4684 = SafeHtmlBuilder.initialState();
        this._state = t_4684;
        StringBuilder t_4685 = SafeHtmlBuilder.newCollector();
        this.collector = t_4685;
    }
}
