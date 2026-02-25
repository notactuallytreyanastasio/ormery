package ormery.html;
/**
 * Defangs interpolations in weird locations.  TODO: These should be reported at compile time.
 */
public final class OutputHtmlSpaceEscaper implements HtmlEscaper {
    public static final OutputHtmlSpaceEscaper instance = new ormery.html.OutputHtmlSpaceEscaper();
    public String applySafeHtml(SafeHtml x__554) {
        return " ";
    }
    public String applySafeUrl(SafeUrl x__557) {
        return " ";
    }
    public String applyInt32(int x__560) {
        return " ";
    }
    public String applyInt64(long x__563) {
        return " ";
    }
    public String applyFloat64(double x__566) {
        return " ";
    }
    public String applyString(String x__569) {
        return " ";
    }
    public OutputHtmlSpaceEscaper() {
    }
}
