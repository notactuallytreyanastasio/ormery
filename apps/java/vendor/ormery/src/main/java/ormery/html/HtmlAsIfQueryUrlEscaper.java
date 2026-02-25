package ormery.html;
import temper.core.Core;
import static ormery.html.HtmlGlobal.urlQuerySafe__296;
import ormery.url.UrlGlobal;
public final class HtmlAsIfQueryUrlEscaper implements UrlEscaper {
    public static final HtmlAsIfQueryUrlEscaper instance = new ormery.html.HtmlAsIfQueryUrlEscaper();
    public SafeUrl applySafeUrl(SafeUrl x__752) {
        return x__752;
    }
    public SafeUrl applyString(String x__755) {
        boolean t_5410;
        int t_5413;
        int t_5414;
        String t_5416;
        boolean t_3113;
        String t_3119;
        int i__757 = 0;
        int end__758 = x__755.length();
        int emitted__759 = 0;
        StringBuilder sb__760 = new StringBuilder();
        while (i__757 < end__758) {
            int cp__761 = x__755.codePointAt(i__757);
            if (cp__761 < urlQuerySafe__296.size()) {
                t_5410 = Core.listGet(urlQuerySafe__296, cp__761);
                t_3113 = !t_5410;
            } else {
                t_3113 = false;
            }
            if (t_3113) {
                Core.stringBuilderAppendBetween(sb__760, x__755, emitted__759, i__757);
                UrlGlobal.percentEscapeOctetTo(cp__761, sb__760);
                t_5413 = Core.stringNext(x__755, i__757);
                emitted__759 = t_5413;
            }
            t_5414 = Core.stringNext(x__755, i__757);
            i__757 = t_5414;
        }
        if (emitted__759 > 0) {
            Core.stringBuilderAppendBetween(sb__760, x__755, emitted__759, end__758);
            t_5416 = sb__760.toString();
            t_3119 = t_5416;
        } else {
            t_3119 = x__755;
        }
        return new SafeUrl(t_3119);
    }
    public HtmlAsIfQueryUrlEscaper() {
    }
}
