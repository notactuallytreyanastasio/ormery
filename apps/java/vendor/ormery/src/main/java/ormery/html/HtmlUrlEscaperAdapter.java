package ormery.html;
import temper.core.Core;
public final class HtmlUrlEscaperAdapter implements HtmlEscaper {
    public final UrlEscaper first;
    public final HtmlEscaper second;
    public String applySafeHtml(SafeHtml x__693) {
        String t_5449 = x__693.getText();
        SafeUrl t_5450 = this.first.applyString(t_5449);
        return this.second.applySafeUrl(t_5450);
    }
    public String applySafeUrl(SafeUrl x__696) {
        SafeUrl t_5447 = this.first.applySafeUrl(x__696);
        return this.second.applySafeUrl(t_5447);
    }
    public String applyInt32(int x__699) {
        String t_5444 = Integer.toString(x__699);
        SafeUrl t_5445 = this.first.applyString(t_5444);
        return this.second.applySafeUrl(t_5445);
    }
    public String applyInt64(long x__702) {
        String t_5441 = Long.toString(x__702);
        SafeUrl t_5442 = this.first.applyString(t_5441);
        return this.second.applySafeUrl(t_5442);
    }
    public String applyFloat64(double x__705) {
        String t_5438 = Core.float64ToString(x__705);
        SafeUrl t_5439 = this.first.applyString(t_5438);
        return this.second.applySafeUrl(t_5439);
    }
    public String applyString(String x__708) {
        SafeUrl t_5436 = this.first.applyString(x__708);
        return this.second.applySafeUrl(t_5436);
    }
    public static final class Builder {
        UrlEscaper first;
        public Builder first(UrlEscaper first) {
            this.first = first;
            return this;
        }
        HtmlEscaper second;
        public Builder second(HtmlEscaper second) {
            this.second = second;
            return this;
        }
        public HtmlUrlEscaperAdapter build() {
            return new HtmlUrlEscaperAdapter(first, second);
        }
    }
    public HtmlUrlEscaperAdapter(UrlEscaper first__711, HtmlEscaper second__712) {
        this.first = first__711;
        this.second = second__712;
    }
    public UrlEscaper getFirst() {
        return this.first;
    }
    public HtmlEscaper getSecond() {
        return this.second;
    }
}
