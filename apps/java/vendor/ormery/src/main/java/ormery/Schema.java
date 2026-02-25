package ormery;
import java.util.List;
import temper.core.Core;
import java.util.function.Function;
public final class Schema {
    public final String tableName;
    public final List<Field> fields;
    public Field getField(String name__268) {
        Field return__117;
        fn__269: {
            List<Field> this__2501 = this.fields;
            int n__2502 = this__2501.size();
            int i__2503 = 0;
            while (i__2503 < n__2502) {
                Field el__2504 = Core.listGet(this__2501, i__2503);
                i__2503 = i__2503 + 1;
                Field field__270 = el__2504;
                if (field__270.getName().equals(name__268)) {
                    return__117 = field__270;
                    break fn__269;
                }
            }
            throw Core.bubble();
        }
        return return__117;
    }
    public boolean hasField(String name__272) {
        boolean return__118;
        fn__273: {
            List<Field> this__2506 = this.fields;
            int n__2507 = this__2506.size();
            int i__2508 = 0;
            while (i__2508 < n__2507) {
                Field el__2509 = Core.listGet(this__2506, i__2508);
                i__2508 = i__2508 + 1;
                Field field__274 = el__2509;
                if (field__274.getName().equals(name__272)) {
                    return__118 = true;
                    break fn__273;
                }
            }
            return__118 = false;
        }
        return return__118;
    }
    public Field getPrimaryKeyField() {
        Field return__119;
        fn__276: {
            List<Field> this__2511 = this.fields;
            int n__2512 = this__2511.size();
            int i__2513 = 0;
            while (i__2513 < n__2512) {
                Field el__2514 = Core.listGet(this__2511, i__2513);
                i__2513 = i__2513 + 1;
                Field field__277 = el__2514;
                if (field__277.isPrimaryKey()) {
                    return__119 = field__277;
                    break fn__276;
                }
            }
            throw Core.bubble();
        }
        return return__119;
    }
    public List<String> getFieldNames() {
        Function<Field, String> fn__4076 = f__280 -> f__280.getName();
        return Core.listMapObjToObj(this.fields, fn__4076);
    }
    public String describe() {
        String header__284 = "Schema: " + this.tableName + "\n";
        Function<Field, String> fn__4070 = f__286 -> "  - " + f__286.getDescription();
        List<String> t_4072 = Core.listMapObjToObj(this.fields, fn__4070);
        Function<String, String> fn__4069 = s__288 -> s__288;
        String fieldList__285 = Core.listJoinObj(t_4072, "\n", fn__4069);
        return header__284 + fieldList__285;
    }
    public static final class Builder {
        String tableName;
        public Builder tableName(String tableName) {
            this.tableName = tableName;
            return this;
        }
        List<Field> fields;
        public Builder fields(List<Field> fields) {
            this.fields = fields;
            return this;
        }
        public Schema build() {
            return new Schema(tableName, fields);
        }
    }
    public Schema(String tableName__291, List<Field> fields__292) {
        this.tableName = tableName__291;
        this.fields = fields__292;
    }
    public String getTableName() {
        return this.tableName;
    }
    public List<Field> getFields() {
        return this.fields;
    }
}
