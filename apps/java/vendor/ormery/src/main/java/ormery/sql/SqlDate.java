package ormery.sql;
import java.time.LocalDate;
import temper.core.Core;
import java.util.function.IntConsumer;
public final class SqlDate implements SqlPart {
    public final LocalDate value;
    public void formatTo(StringBuilder builder__187) {
        builder__187.append("'");
        String s__189 = this.value.toString();
        IntConsumer fn__1178 = c__190 -> {
            if (c__190 == 39) {
                builder__187.append("''");
            } else {
                Core.stringBuilderAppendCodePoint(builder__187, c__190);
            }
        };
        Core.stringForEach(s__189, fn__1178);
        builder__187.append("'");
    }
    public SqlDate(LocalDate value__192) {
        this.value = value__192;
    }
    public LocalDate getValue() {
        return this.value;
    }
}
