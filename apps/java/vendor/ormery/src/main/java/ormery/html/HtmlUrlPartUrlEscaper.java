package ormery.html;
import temper.core.Core;
import static ormery.html.HtmlGlobal.urlSafe__297;
import ormery.url.UrlGlobal;
public final class HtmlUrlPartUrlEscaper implements UrlEscaper {
    public static final HtmlUrlPartUrlEscaper instance = new ormery.html.HtmlUrlPartUrlEscaper();
    public SafeUrl applySafeUrl(SafeUrl x__738) {
        return x__738;
    }
    public SafeUrl applyString(String x__741) {
        boolean t_5428;
        int t_5431;
        int t_5432;
        String t_5434;
        boolean t_3137;
        String t_3143;
        int i__743 = 0;
        int end__744 = x__741.length();
        int emitted__745 = 0;
        StringBuilder sb__746 = new StringBuilder();
        while (i__743 < end__744) {
            int cp__747 = x__741.codePointAt(i__743);
            if (cp__747 < urlSafe__297.size()) {
                t_5428 = Core.listGet(urlSafe__297, cp__747);
                t_3137 = !t_5428;
            } else {
                t_3137 = false;
            }
            if (t_3137) {
                Core.stringBuilderAppendBetween(sb__746, x__741, emitted__745, i__743);
                UrlGlobal.percentEscapeOctetTo(cp__747, sb__746);
                t_5431 = Core.stringNext(x__741, i__743);
                emitted__745 = t_5431;
            }
            t_5432 = Core.stringNext(x__741, i__743);
            i__743 = t_5432;
        }
        if (emitted__745 > 0) {
            Core.stringBuilderAppendBetween(sb__746, x__741, emitted__745, end__744);
            t_5434 = sb__746.toString();
            t_3143 = t_5434;
        } else {
            t_3143 = x__741;
        }
        return new SafeUrl(t_3143);
    }
    public HtmlUrlPartUrlEscaper() {
    }
}
