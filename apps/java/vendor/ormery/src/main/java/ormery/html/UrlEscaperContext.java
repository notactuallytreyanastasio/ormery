package ormery.html;
import ormery.core.Context;
public final class UrlEscaperContext implements Context {
    public final int urlState;
    public String toString() {
        return "UrlEscaperContext(" + HtmlGlobal.urlStateStr__265(this.urlState) + ")";
    }
    public UrlEscaperContext(int urlState__523) {
        this.urlState = urlState__523;
    }
    public int getUrlState() {
        return this.urlState;
    }
}
