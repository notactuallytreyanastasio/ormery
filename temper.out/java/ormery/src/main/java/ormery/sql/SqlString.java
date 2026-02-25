package ormery.sql;
import temper.core.Core;
import java.util.function.IntConsumer;
/**
 * `SqlString` represents text data that needs escaped.
 */
public final class SqlString implements SqlPart {
    public final String value;
    public void formatTo(StringBuilder builder__213) {
        builder__213.append("'");
        IntConsumer fn__1166 = c__215 -> {
            if (c__215 == 39) {
                builder__213.append("''");
            } else {
                Core.stringBuilderAppendCodePoint(builder__213, c__215);
            }
        };
        Core.stringForEach(this.value, fn__1166);
        builder__213.append("'");
    }
    public SqlString(String value__217) {
        this.value = value__217;
    }
    public String getValue() {
        return this.value;
    }
}
