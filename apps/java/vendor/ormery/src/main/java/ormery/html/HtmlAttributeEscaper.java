package ormery.html;
import static ormery.html.HtmlGlobal.htmlCodec;
import temper.core.Core;
/**
 * Encodes HTML meta-characters using HTML entities in a way that preserves attribute boundaries.
 */
public final class HtmlAttributeEscaper implements HtmlEscaper {
    public static final HtmlAttributeEscaper instance = new ormery.html.HtmlAttributeEscaper();
    public String applySafeHtml(SafeHtml x__596) {
        String t_4731 = htmlCodec.decode(x__596.getText());
        return this.applyString(t_4731);
    }
    public String applySafeUrl(SafeUrl x__599) {
        String t_4728 = x__599.getText();
        return this.applyString(t_4728);
    }
    public String applyInt32(int x__602) {
        String t_4726 = Integer.toString(x__602);
        return this.applyString(t_4726);
    }
    public String applyInt64(long x__605) {
        String t_4724 = Long.toString(x__605);
        return this.applyString(t_4724);
    }
    public String applyFloat64(double x__608) {
        String t_4722 = Core.float64ToString(x__608);
        return this.applyString(t_4722);
    }
    public String applyString(String x__611) {
        return htmlCodec.encode(x__611);
    }
    public HtmlAttributeEscaper() {
    }
}
