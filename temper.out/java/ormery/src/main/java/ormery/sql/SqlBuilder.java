package ormery.sql;
import java.util.List;
import temper.core.Core;
import java.util.function.Consumer;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.function.IntConsumer;
import java.util.function.DoubleConsumer;
public final class SqlBuilder {
    final List<SqlPart> buffer;
    public void appendSafe(String sqlSource__96) {
        SqlSource t_1163 = new SqlSource(sqlSource__96);
        Core.listAdd(this.buffer, t_1163);
    }
    public void appendFragment(SqlFragment fragment__99) {
        List<SqlPart> t_1161 = fragment__99.getParts();
        Core.listAddAll(this.buffer, t_1161);
    }
    public void appendPart(SqlPart part__102) {
        Core.listAdd(this.buffer, part__102);
    }
    public void appendPartList(List<SqlPart> values__105) {
        Consumer<SqlPart> fn__1157 = x__107 -> {
            this.appendPart(x__107);
        };
        this.appendList(values__105, fn__1157);
    }
    public void appendBoolean(boolean value__109) {
        SqlBoolean t_1154 = new SqlBoolean(value__109);
        Core.listAdd(this.buffer, t_1154);
    }
    public void appendBooleanList(List<Boolean> values__112) {
        Consumer<Boolean> fn__1151 = x__114 -> {
            this.appendBoolean(x__114);
        };
        this.appendList(values__112, fn__1151);
    }
    public void appendDate(LocalDate value__116) {
        SqlDate t_1148 = new SqlDate(value__116);
        Core.listAdd(this.buffer, t_1148);
    }
    public void appendDateList(List<LocalDate> values__119) {
        Consumer<LocalDate> fn__1145 = x__121 -> {
            this.appendDate(x__121);
        };
        this.appendList(values__119, fn__1145);
    }
    public void appendFloat64(double value__123) {
        SqlFloat64 t_1142 = new SqlFloat64(value__123);
        Core.listAdd(this.buffer, t_1142);
    }
    public void appendFloat64List(List<Double> values__126) {
        DoubleConsumer fn__1139 = x__128 -> {
            this.appendFloat64(x__128);
        };
        this.appendList(values__126, fn__1139 :: accept);
    }
    public void appendInt32(int value__130) {
        SqlInt32 t_1136 = new SqlInt32(value__130);
        Core.listAdd(this.buffer, t_1136);
    }
    public void appendInt32List(List<Integer> values__133) {
        IntConsumer fn__1133 = x__135 -> {
            this.appendInt32(x__135);
        };
        this.appendList(values__133, fn__1133 :: accept);
    }
    public void appendInt64(long value__137) {
        SqlInt64 t_1130 = new SqlInt64(value__137);
        Core.listAdd(this.buffer, t_1130);
    }
    public void appendInt64List(List<Long> values__140) {
        Consumer<Long> fn__1127 = x__142 -> {
            this.appendInt64(x__142);
        };
        this.appendList(values__140, fn__1127);
    }
    public void appendString(String value__144) {
        SqlString t_1124 = new SqlString(value__144);
        Core.listAdd(this.buffer, t_1124);
    }
    public void appendStringList(List<String> values__147) {
        Consumer<String> fn__1121 = x__149 -> {
            this.appendString(x__149);
        };
        this.appendList(values__147, fn__1121);
    }
    <T__27> void appendList(List<T__27> values__151, Consumer<T__27> appendValue__152) {
        int t_1116;
        T__27 t_1118;
        int i__154 = 0;
        while (true) {
            t_1116 = values__151.size();
            if (i__154 >= t_1116) {
                break;
            }
            if (i__154 > 0) {
                this.appendSafe(", ");
            }
            t_1118 = Core.listGet(values__151, i__154);
            appendValue__152.accept(t_1118);
            i__154 = i__154 + 1;
        }
    }
    public SqlFragment getAccumulated() {
        return new SqlFragment(List.copyOf(this.buffer));
    }
    public SqlBuilder() {
        List<SqlPart> t_1113 = new ArrayList<>();
        this.buffer = t_1113;
    }
}
