package ormery.sql;
import temper.core.Core;
public final class SqlFloat64 implements SqlPart {
    public final double value;
    public void formatTo(StringBuilder builder__195) {
        String t_1175 = Core.float64ToString(this.value);
        builder__195.append(t_1175);
    }
    public SqlFloat64(double value__198) {
        this.value = value__198;
    }
    public double getValue() {
        return this.value;
    }
}
