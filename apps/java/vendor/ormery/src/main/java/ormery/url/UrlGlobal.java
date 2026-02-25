package ormery.url;
import temper.core.Core;
public final class UrlGlobal {
    private UrlGlobal() {
    }
    static void appendHex__3(int n__7, StringBuilder sb__8) {
        int t_26;
        int i__10 = n__7 & 15;
        if (i__10 < 10) {
            t_26 = 48;
        } else {
            t_26 = 87;
        }
        Core.stringBuilderAppendCodePoint(sb__8, i__10 + t_26);
    }
    public static void percentEscapeOctetTo(int octet__4, StringBuilder sb__5) {
        sb__5.append("%");
        int t_21 = (octet__4 & 255) / 16;
        UrlGlobal.appendHex__3(t_21, sb__5);
        int t_23 = octet__4 & 15;
        UrlGlobal.appendHex__3(t_23, sb__5);
    }
}
