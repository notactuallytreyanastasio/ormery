package ormery.html;
import temper.core.Nullable;
public final class HtmlCssDelegate implements HtmlDelegate {
    public String process(@Nullable String s__765) {
        String return__254;
        if (s__765 != null) {
            return__254 = s__765;
        } else {
            return__254 = "";
        }
        return return__254;
    }
    public @Nullable HtmlEscaper escaper(HtmlEscaper outer__768) {
        return outer__768;
    }
    public HtmlCssDelegate() {
    }
}
