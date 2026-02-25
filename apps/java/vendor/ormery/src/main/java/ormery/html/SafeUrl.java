package ormery.html;
/**
 * SafeUrl is a string wrapper for URL content that is trusted.
 * It should only be constructed when it is known safe by construction.
 */
public final class SafeUrl {
    public final String text;
    public String toString() {
        return this.text;
    }
    public SafeUrl(String text__533) {
        this.text = text__533;
    }
    public String getText() {
        return this.text;
    }
}
