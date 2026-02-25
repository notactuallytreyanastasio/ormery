package ormery;
import ormery.html.SafeHtmlBuilder;
import ormery.html.SafeHtml;
public final class Token {
    public final TokenType tokenType;
    public final String value;
    public String cssClass() {
        String return__185;
        String name__678 = this.tokenType.getName();
        if (name__678.equals("keyword")) {
            return__185 = "kw";
        } else if (name__678.equals("type")) {
            return__185 = "typ";
        } else if (name__678.equals("string")) {
            return__185 = "str";
        } else if (name__678.equals("number")) {
            return__185 = "num";
        } else if (name__678.equals("comment")) {
            return__185 = "cmt";
        } else if (name__678.equals("operator")) {
            return__185 = "op";
        } else {
            return__185 = "id";
        }
        return return__185;
    }
    public SafeHtml toHtml() {
        String cls__681 = this.cssClass();
        SafeHtmlBuilder t_3288 = new SafeHtmlBuilder();
        t_3288.appendSafe("<span class='");
        t_3288.appendString(cls__681);
        t_3288.appendSafe("'>");
        t_3288.appendString(this.value);
        t_3288.appendSafe("</span>");
        return t_3288.getAccumulated();
    }
    public static final class Builder {
        TokenType tokenType;
        public Builder tokenType(TokenType tokenType) {
            this.tokenType = tokenType;
            return this;
        }
        String value;
        public Builder value(String value) {
            this.value = value;
            return this;
        }
        public Token build() {
            return new Token(tokenType, value);
        }
    }
    public Token(TokenType tokenType__683, String value__684) {
        this.tokenType = tokenType__683;
        this.value = value__684;
    }
    public TokenType getTokenType() {
        return this.tokenType;
    }
    public String getValue() {
        return this.value;
    }
}
