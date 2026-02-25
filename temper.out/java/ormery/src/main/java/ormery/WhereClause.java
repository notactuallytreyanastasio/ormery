package ormery;
public final class WhereClause {
    public final String field;
    public final String operator;
    public final String value;
    public String describe() {
        return this.field + " " + this.operator + " " + this.value;
    }
    public static final class Builder {
        String field;
        public Builder field(String field) {
            this.field = field;
            return this;
        }
        String operator;
        public Builder operator(String operator) {
            this.operator = operator;
            return this;
        }
        String value;
        public Builder value(String value) {
            this.value = value;
            return this;
        }
        public WhereClause build() {
            return new WhereClause(field, operator, value);
        }
    }
    public WhereClause(String field__366, String operator__367, String value__368) {
        this.field = field__366;
        this.operator = operator__367;
        this.value = value__368;
    }
    public String getField() {
        return this.field;
    }
    public String getOperator() {
        return this.operator;
    }
    public String getValue() {
        return this.value;
    }
}
