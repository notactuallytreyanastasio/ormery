package ormery.html;
import temper.core.Nullable;
public final class HtmlJsDelegate implements HtmlDelegate {
    public String process(@Nullable String s__772) {
        String return__258;
        if (s__772 != null) {
            return__258 = s__772;
        } else {
            return__258 = "";
        }
        return return__258;
    }
    public @Nullable HtmlEscaper escaper(HtmlEscaper outer__775) {
        return outer__775;
    }
    public HtmlJsDelegate() {
    }
}
