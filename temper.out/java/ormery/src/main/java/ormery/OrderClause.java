package ormery;
public final class OrderClause {
    public final String field;
    public final String direction;
    public String describe() {
        return this.field + " " + this.direction;
    }
    public static final class Builder {
        String field;
        public Builder field(String field) {
            this.field = field;
            return this;
        }
        String direction;
        public Builder direction(String direction) {
            this.direction = direction;
            return this;
        }
        public OrderClause build() {
            return new OrderClause(field, direction);
        }
    }
    public OrderClause(String field__374, String direction__375) {
        this.field = field__374;
        this.direction = direction__375;
    }
    public String getField() {
        return this.field;
    }
    public String getDirection() {
        return this.direction;
    }
}
