package ormery.html;
import temper.core.Core;
import static ormery.html.HtmlGlobal.htmlCodec;
/**
 * Encodes HTML meta-characters using HTML entities in a way that preserves tag boundaries.
 */
public final class HtmlPcdataEscaper implements HtmlEscaper {
    public static final HtmlPcdataEscaper instance = new ormery.html.HtmlPcdataEscaper();
    public String applySafeHtml(SafeHtml x__575) {
        return x__575.toString();
    }
    public String applySafeUrl(SafeUrl x__578) {
        String t_4740 = x__578.getText();
        return this.applyString(t_4740);
    }
    public String applyInt32(int x__581) {
        String t_4738 = Integer.toString(x__581);
        return this.applyString(t_4738);
    }
    public String applyInt64(long x__584) {
        String t_4736 = Long.toString(x__584);
        return this.applyString(t_4736);
    }
    public String applyFloat64(double x__587) {
        String t_4734 = Core.float64ToString(x__587);
        return this.applyString(t_4734);
    }
    public String applyString(String x__590) {
        return htmlCodec.encode(x__590);
    }
    public HtmlPcdataEscaper() {
    }
}
