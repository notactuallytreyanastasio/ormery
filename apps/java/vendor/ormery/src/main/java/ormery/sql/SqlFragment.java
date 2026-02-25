package ormery.sql;
import java.util.List;
import temper.core.Core;
public final class SqlFragment {
    public final List<SqlPart> parts;
    public SqlSource toSource() {
        return new SqlSource(this.toString());
    }
    public String toString() {
        int t_1186;
        StringBuilder builder__166 = new StringBuilder();
        int i__167 = 0;
        while (true) {
            t_1186 = this.parts.size();
            if (i__167 >= t_1186) {
                break;
            }
            Core.listGet(this.parts, i__167).formatTo(builder__166);
            i__167 = i__167 + 1;
        }
        return builder__166.toString();
    }
    public SqlFragment(List<SqlPart> parts__169) {
        this.parts = parts__169;
    }
    public List<SqlPart> getParts() {
        return this.parts;
    }
}
