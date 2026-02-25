package ormery.html;
import ormery.core.Escaper;
public interface HtmlEscaper extends Escaper {
    String applySafeHtml(SafeHtml x__535);
    String applySafeUrl(SafeUrl x__538);
    String applyInt32(int x__541);
    String applyInt64(long x__544);
    String applyFloat64(double x__547);
    String applyString(String x__550);
}
