package ormery.sql;
public final class SqlInt64 implements SqlPart {
    public final long value;
    public void formatTo(StringBuilder builder__207) {
        String t_1171 = Long.toString(this.value);
        builder__207.append(t_1171);
    }
    public SqlInt64(long value__210) {
        this.value = value__210;
    }
    public long getValue() {
        return this.value;
    }
}
