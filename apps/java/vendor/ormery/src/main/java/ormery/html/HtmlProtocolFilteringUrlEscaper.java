package ormery.html;
import temper.std.regex.Match;
import temper.core.Core;
import temper.core.Nullable;
import static ormery.html.HtmlGlobal.fallbackSafeUrl__294;
import static ormery.html.HtmlGlobal.protocolAllowList__293;
import static ormery.html.HtmlGlobal.htmlUrlPartUrlEscaper__298;
public final class HtmlProtocolFilteringUrlEscaper implements UrlEscaper {
    public static final HtmlProtocolFilteringUrlEscaper instance = new ormery.html.HtmlProtocolFilteringUrlEscaper();
    public SafeUrl applySafeUrl(SafeUrl x__721) {
        return x__721;
    }
    public SafeUrl applyString(String x__724) {
        SafeUrl return__243;
        int t_5422;
        Match t_3125;
        @Nullable Match t_3126;
        fn__725: {
            int protocolEnd__726 = 0;
            int end__727 = x__724.length();
            while (protocolEnd__726 < end__727) {
                int cp__728 = x__724.codePointAt(protocolEnd__726);
                if (cp__728 == 58) {
                    String protocol__729 = Core.stringSlice(x__724, 0, protocolEnd__726);
                    try {
                        t_3125 = protocolAllowList__293.find(protocol__729);
                        t_3126 = t_3125;
                    } catch (RuntimeException ignored$3) {
                        t_3126 = null;
                    }
                    if (t_3126 != null) {
                        return__243 = new SafeUrl(x__724);
                    } else {
                        return__243 = fallbackSafeUrl__294;
                    }
                    break fn__725;
                }
                t_5422 = Core.stringNext(x__724, protocolEnd__726);
                protocolEnd__726 = t_5422;
            }
            return__243 = htmlUrlPartUrlEscaper__298.applyString(x__724);
        }
        return return__243;
    }
    public HtmlProtocolFilteringUrlEscaper() {
    }
}
