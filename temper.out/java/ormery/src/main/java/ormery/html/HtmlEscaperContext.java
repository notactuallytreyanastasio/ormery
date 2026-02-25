package ormery.html;
import ormery.core.Context;
/**
 * HtmlEscaperContext represents a path into an HTML grammar that allows pausable
 * parsing of HTML with holes.
 */
public final class HtmlEscaperContext implements Context {
    /**
     * htmlState describes where the parser is in the HTML parsing algorithm.
     */
    public final int htmlState;
    /**
     * tagState describes any special tag the HTML parser is in.
     * For example, inside a tag, it describes that tag.
     * In the body of a special tag, e.g. `&lt;script&gt;` or `&lt;style&gt;` it captures
     * the state exit conditions, for example, a `&lt;/script&gt;` exits the special element.
     */
    public final int tagState;
    /**
     * attribState describes the kind of attribute being parsed.
     * For example, for `src="..."` we might need to know that the `...` is a URL embedded
     * in HTML, or in `onclick="..."` that the value is JavaScript embedded in HTML.
     */
    public final int attribState;
    /**
     * delimState identifies the kind of quotation mark for the current attribute value.
     */
    public final int delimState;
    public String toString() {
        return "HtmlEscaperContext(" + HtmlGlobal.htmlStateStr__260(this.htmlState) + ", " + HtmlGlobal.tagStateStr__261(this.tagState) + ", " + HtmlGlobal.attribStateStr__262(this.attribState) + ", " + HtmlGlobal.delimStateStr__263(this.delimState) + ")";
    }
    public static final class Builder {
        int htmlState;
        boolean htmlState__set;
        public Builder htmlState(int htmlState) {
            htmlState__set = true;
            this.htmlState = htmlState;
            return this;
        }
        int tagState;
        boolean tagState__set;
        public Builder tagState(int tagState) {
            tagState__set = true;
            this.tagState = tagState;
            return this;
        }
        int attribState;
        boolean attribState__set;
        public Builder attribState(int attribState) {
            attribState__set = true;
            this.attribState = attribState;
            return this;
        }
        int delimState;
        boolean delimState__set;
        public Builder delimState(int delimState) {
            delimState__set = true;
            this.delimState = delimState;
            return this;
        }
        public HtmlEscaperContext build() {
            if (!htmlState__set || !tagState__set || !attribState__set || !delimState__set) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!htmlState__set) {
                    _message.append(" htmlState");
                }
                if (!tagState__set) {
                    _message.append(" tagState");
                }
                if (!attribState__set) {
                    _message.append(" attribState");
                }
                if (!delimState__set) {
                    _message.append(" delimState");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new HtmlEscaperContext(htmlState, tagState, attribState, delimState);
        }
    }
    public HtmlEscaperContext(int htmlState__503, int tagState__504, int attribState__505, int delimState__506) {
        this.htmlState = htmlState__503;
        this.tagState = tagState__504;
        this.attribState = attribState__505;
        this.delimState = delimState__506;
    }
    /**
     * htmlState describes where the parser is in the HTML parsing algorithm.
     */
    public int getHtmlState() {
        return this.htmlState;
    }
    /**
     * tagState describes any special tag the HTML parser is in.
     * For example, inside a tag, it describes that tag.
     * In the body of a special tag, e.g. `&lt;script&gt;` or `&lt;style&gt;` it captures
     * the state exit conditions, for example, a `&lt;/script&gt;` exits the special element.
     */
    public int getTagState() {
        return this.tagState;
    }
    /**
     * attribState describes the kind of attribute being parsed.
     * For example, for `src="..."` we might need to know that the `...` is a URL embedded
     * in HTML, or in `onclick="..."` that the value is JavaScript embedded in HTML.
     */
    public int getAttribState() {
        return this.attribState;
    }
    /**
     * delimState identifies the kind of quotation mark for the current attribute value.
     */
    public int getDelimState() {
        return this.delimState;
    }
}
