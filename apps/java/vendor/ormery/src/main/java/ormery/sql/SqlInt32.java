package ormery.sql;
public final class SqlInt32 implements SqlPart {
    public final int value;
    public void formatTo(StringBuilder builder__201) {
        String t_1173 = Integer.toString(this.value);
        builder__201.append(t_1173);
    }
    public SqlInt32(int value__204) {
        this.value = value__204;
    }
    public int getValue() {
        return this.value;
    }
}
