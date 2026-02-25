package ormery;
import java.util.List;
import temper.core.Core;
import java.util.Map;
import java.util.ArrayList;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Predicate;
import ormery.sql.SqlFragment;
import java.util.LinkedHashMap;
import java.util.function.ToIntBiFunction;
public final class Query {
    public final Schema schema;
    public final InMemoryStore store;
    List<WhereClause> whereClauses;
    List<String> selectFields;
    List<OrderClause> orderByClauses;
    int limitValue;
    int offsetValue;
    public static final class Builder {
        Schema schema;
        public Builder schema(Schema schema) {
            this.schema = schema;
            return this;
        }
        InMemoryStore store;
        public Builder store(InMemoryStore store) {
            this.store = store;
            return this;
        }
        public Query build() {
            return new Query(schema, store);
        }
    }
    public Query(Schema schema__384, InMemoryStore store__385) {
        this.schema = schema__384;
        this.store = store__385;
        List<WhereClause> t_3898 = new ArrayList<>();
        this.whereClauses = t_3898;
        List<String> t_2220 = List.of();
        this.selectFields = t_2220;
        List<OrderClause> t_3899 = new ArrayList<>();
        this.orderByClauses = t_3899;
        this.limitValue = -1;
        this.offsetValue = 0;
    }
    public Query where(String field__388, String operator__389, String value__390) {
        WhereClause t_3896 = new WhereClause(field__388, operator__389, value__390);
        Core.listAdd(this.whereClauses, t_3896);
        return this;
    }
    public Query select(List<String> fields__393) {
        this.selectFields = fields__393;
        return this;
    }
    public Query orderBy(String field__396, String direction__397) {
        OrderClause t_3894 = new OrderClause(field__396, direction__397);
        Core.listAdd(this.orderByClauses, t_3894);
        return this;
    }
    public Query limit(int n__400) {
        int t_2209;
        if (n__400 < 0) {
            t_2209 = 0;
        } else {
            t_2209 = n__400;
        }
        this.limitValue = t_2209;
        return this;
    }
    public Query offset(int n__403) {
        this.offsetValue = n__403;
        return this;
    }
    boolean matchesWhere(Record record__406) {
        boolean return__152;
        String t_3883;
        String t_3885;
        String t_3887;
        String t_3889;
        String t_3890;
        boolean t_3891;
        String t_3892;
        String t_3893;
        fn__407: {
            List<WhereClause> this__2521 = List.copyOf(this.whereClauses);
            int n__2522 = this__2521.size();
            int i__2523 = 0;
            while (i__2523 < n__2522) {
                WhereClause el__2524 = Core.listGet(this__2521, i__2523);
                i__2523 = i__2523 + 1;
                WhereClause clause__408 = el__2524;
                t_3883 = clause__408.getField();
                String recordValue__409 = record__406.getOr(t_3883, "");
                t_3885 = clause__408.getField();
                if (!this.schema.hasField(t_3885)) {
                    return__152 = false;
                    break fn__407;
                }
                t_3887 = clause__408.getField();
                Field t_2198;
                t_2198 = this.schema.getField(t_3887);
                Field fieldInfo__410 = t_2198;
                String fieldType__411 = fieldInfo__410.getFieldType();
                boolean matches__412;
                if (fieldType__411.equals("Int")) {
                    t_3889 = clause__408.getOperator();
                    t_3890 = clause__408.getValue();
                    t_3891 = OrmeryGlobal.compareInt__195(recordValue__409, t_3889, t_3890);
                    matches__412 = t_3891;
                } else if (fieldType__411.equals("String")) {
                    t_3892 = clause__408.getOperator();
                    t_3893 = clause__408.getValue();
                    matches__412 = OrmeryGlobal.compareString__196(recordValue__409, t_3892, t_3893);
                } else {
                    matches__412 = false;
                }
                if (!matches__412) {
                    return__152 = false;
                    break fn__407;
                }
            }
            return__152 = true;
        }
        return return__152;
    }
    Record projectRecord(Record record__414) {
        Record return__153;
        Map<String, String> t_3878;
        fn__415: {
            if (this.selectFields.size() == 0) {
                return__153 = record__414;
                break fn__415;
            }
            Map<String, String> builder__416 = new LinkedHashMap<>();
            Consumer<String> fn__3873 = fieldName__417 -> {
                String value__418 = record__414.getOr(fieldName__417, "");
                builder__416.put(fieldName__417, value__418);
            };
            this.selectFields.forEach(fn__3873);
            t_3878 = Core.mappedToMap(builder__416);
            return__153 = new Record(t_3878);
        }
        return return__153;
    }
    int compareRecords(Record a__420, Record b__421, List<OrderClause> orderClauses__422) {
        int return__154;
        String t_3862;
        String t_3864;
        String t_3866;
        String t_3868;
        fn__423: {
            List<OrderClause> this__2526 = orderClauses__422;
            int n__2527 = this__2526.size();
            int i__2528 = 0;
            while (i__2528 < n__2527) {
                OrderClause el__2529 = Core.listGet(this__2526, i__2528);
                i__2528 = i__2528 + 1;
                OrderClause clause__424 = el__2529;
                int t_2177;
                int t_2179;
                t_3862 = clause__424.getField();
                String aVal__425 = a__420.getOr(t_3862, "");
                t_3864 = clause__424.getField();
                String bVal__426 = b__421.getOr(t_3864, "");
                t_3866 = clause__424.getField();
                if (!this.schema.hasField(t_3866)) {
                    continue;
                }
                t_3868 = clause__424.getField();
                Field t_2174;
                t_2174 = this.schema.getField(t_3868);
                Field fieldInfo__427 = t_2174;
                String fieldType__428 = fieldInfo__427.getFieldType();
                int cmp__429;
                if (fieldType__428.equals("Int")) {
                    int aInt__430;
                    int aInt_5869;
                    try {
                        t_2177 = Core.stringToInt(aVal__425);
                        aInt_5869 = t_2177;
                    } catch (RuntimeException ignored$1) {
                        aInt_5869 = 0;
                    }
                    aInt__430 = aInt_5869;
                    int bInt__431;
                    int bInt_5870;
                    try {
                        t_2179 = Core.stringToInt(bVal__426);
                        bInt_5870 = t_2179;
                    } catch (RuntimeException ignored$2) {
                        bInt_5870 = 0;
                    }
                    bInt__431 = bInt_5870;
                    cmp__429 = Integer.compare(aInt__430, bInt__431);
                } else if (fieldType__428.equals("String")) {
                    cmp__429 = Core.genericCmp(aVal__425, bVal__426);
                } else {
                    cmp__429 = 0;
                }
                if (cmp__429 != 0) {
                    if (clause__424.getDirection().equals("desc")) {
                        return__154 = -cmp__429;
                    } else {
                        return__154 = cmp__429;
                    }
                    break fn__423;
                }
            }
            return__154 = 0;
        }
        return return__154;
    }
    public List<Record> all() {
        List<Record> t_3853;
        List<Record> t_3854;
        int t_3855;
        int t_3856;
        List<Record> t_3857;
        String t_3846 = this.schema.getTableName();
        List<Record> allRecords__434 = this.store.all(t_3846);
        Predicate<Record> fn__3845 = r__436 -> this.matchesWhere(r__436);
        List<Record> filtered__435 = Core.listFilterObj(allRecords__434, fn__3845);
        List<Record> sorted__438;
        if (this.orderByClauses.size() > 0) {
            List<OrderClause> clauses__439 = List.copyOf(this.orderByClauses);
            ToIntBiFunction<Record, Record> fn__3844 = (a__440, b__441) -> this.compareRecords(a__440, b__441, clauses__439);
            t_3853 = Core.listSorted(filtered__435, fn__3844 :: applyAsInt);
            sorted__438 = t_3853;
        } else {
            sorted__438 = filtered__435;
        }
        List<Record> sliced__443;
        if (this.limitValue >= 0) {
            int start__444 = this.offsetValue;
            int end__445 = this.offsetValue + this.limitValue;
            t_3854 = Core.listSlice(sorted__438, start__444, end__445);
            sliced__443 = t_3854;
        } else if (this.offsetValue > 0) {
            t_3856 = this.offsetValue;
            t_3855 = sorted__438.size();
            t_3857 = Core.listSlice(sorted__438, t_3856, t_3855);
            sliced__443 = t_3857;
        } else {
            sliced__443 = sorted__438;
        }
        Function<Record, Record> fn__3843 = r__446 -> this.projectRecord(r__446);
        return Core.listMapObjToObj(sliced__443, fn__3843);
    }
    public SqlFragment toSql() {
        return OrmeryGlobal.toSqlQuery(this.schema, this.selectFields, List.copyOf(this.whereClauses), List.copyOf(this.orderByClauses), this.limitValue, this.offsetValue);
    }
    public Schema getSchema() {
        return this.schema;
    }
    public InMemoryStore getStore() {
        return this.store;
    }
}
