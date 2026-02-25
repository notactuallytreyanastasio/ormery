package ormery;
public final class Field {
    public final String name;
    public final String fieldType;
    public final boolean primaryKey;
    public final boolean nullable;
    public String getDescription() {
        String pk__258;
        if (this.primaryKey) {
            pk__258 = " (PK)";
        } else {
            pk__258 = "";
        }
        String null__259;
        if (this.nullable) {
            null__259 = " (nullable)";
        } else {
            null__259 = "";
        }
        return this.name + ": " + this.fieldType + pk__258 + null__259;
    }
    public static final class Builder {
        String name;
        public Builder name(String name) {
            this.name = name;
            return this;
        }
        String fieldType;
        public Builder fieldType(String fieldType) {
            this.fieldType = fieldType;
            return this;
        }
        boolean primaryKey;
        boolean primaryKey__set;
        public Builder primaryKey(boolean primaryKey) {
            primaryKey__set = true;
            this.primaryKey = primaryKey;
            return this;
        }
        boolean nullable;
        boolean nullable__set;
        public Builder nullable(boolean nullable) {
            nullable__set = true;
            this.nullable = nullable;
            return this;
        }
        public Field build() {
            if (!primaryKey__set || !nullable__set || name == null || fieldType == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!primaryKey__set) {
                    _message.append(" primaryKey");
                }
                if (!nullable__set) {
                    _message.append(" nullable");
                }
                if (name == null) {
                    _message.append(" name");
                }
                if (fieldType == null) {
                    _message.append(" fieldType");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new Field(name, fieldType, primaryKey, nullable);
        }
    }
    public Field(String name__261, String fieldType__262, boolean primaryKey__263, boolean nullable__264) {
        this.name = name__261;
        this.fieldType = fieldType__262;
        this.primaryKey = primaryKey__263;
        this.nullable = nullable__264;
    }
    public String getName() {
        return this.name;
    }
    public String getFieldType() {
        return this.fieldType;
    }
    public boolean isPrimaryKey() {
        return this.primaryKey;
    }
    public boolean isNullable() {
        return this.nullable;
    }
}
