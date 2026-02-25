package ormery;
public final class TokenType {
    public final String name;
    public boolean isKeyword() {
        return this.name.equals("keyword");
    }
    public boolean isType() {
        return this.name.equals("type");
    }
    public boolean isString() {
        return this.name.equals("string");
    }
    public boolean isNumber() {
        return this.name.equals("number");
    }
    public boolean isComment() {
        return this.name.equals("comment");
    }
    public boolean isOperator() {
        return this.name.equals("operator");
    }
    public boolean isIdentifier() {
        return this.name.equals("identifier");
    }
    public TokenType(String name__673) {
        this.name = name__673;
    }
    public String getName() {
        return this.name;
    }
}
