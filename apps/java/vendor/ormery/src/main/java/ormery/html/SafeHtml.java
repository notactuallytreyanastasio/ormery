package ormery.html;
/**
 * SafeHtml is a string wrapper for HTML text that is trusted.
 * It should only be constructed when it is known safe by construction.
 * For example, the auto-escaping HTML tag produces it, but it may be created by other reliable sources:
 * e.g. an escaping function that defangs all HTML meta-characters, and a carefully reviewed HTML sanitizer.
 */
public final class SafeHtml {
    public final String text;
    public String toString() {
        return this.text;
    }
    public SafeHtml(String text__528) {
        this.text = text__528;
    }
    public String getText() {
        return this.text;
    }
}
