package ormery.sql;
public final class SqlBoolean implements SqlPart {
    public final boolean value;
    public void formatTo(StringBuilder builder__181) {
        String t_718;
        if (this.value) {
            t_718 = "TRUE";
        } else {
            t_718 = "FALSE";
        }
        builder__181.append(t_718);
    }
    public SqlBoolean(boolean value__184) {
        this.value = value__184;
    }
    public boolean isValue() {
        return this.value;
    }
}
