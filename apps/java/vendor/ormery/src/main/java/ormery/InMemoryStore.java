package ormery;
import java.util.List;
import java.util.Map;
import temper.core.Core;
import java.util.ArrayList;
import java.util.LinkedHashMap;
public final class InMemoryStore {
    Map<String, List<Record>> tables;
    Map<String, Integer> nextIds;
    public InMemoryStore() {
        Map<String, List<Record>> t_4058 = new LinkedHashMap<>();
        this.tables = t_4058;
        Map<String, Integer> t_4059 = new LinkedHashMap<>();
        this.nextIds = t_4059;
    }
    void ensureTable(String tableName__335) {
        List<Record> t_4055;
        if (!this.tables.containsKey(tableName__335)) {
            t_4055 = new ArrayList<>();
            this.tables.put(tableName__335, t_4055);
            this.nextIds.put(tableName__335, 1);
        }
    }
    public Record insert(String tableName__338, Map<String, String> data__339) {
        this.ensureTable(tableName__338);
        int id__341 = this.nextIds.getOrDefault(tableName__338, 1);
        this.nextIds.put(tableName__338, id__341 + 1);
        Map<String, String> dataBuilder__342 = new LinkedHashMap<>(data__339);
        dataBuilder__342.put("id", Integer.toString(id__341));
        Record record__343 = new Record(Core.mappedToMap(dataBuilder__342));
        List<Record> t_4051 = new ArrayList<>();
        List<Record> table__344 = this.tables.getOrDefault(tableName__338, t_4051);
        Core.listAdd(table__344, record__343);
        return record__343;
    }
    public List<Record> all(String tableName__346) {
        this.ensureTable(tableName__346);
        List<Record> t_4040 = new ArrayList<>();
        List<Record> table__348 = this.tables.getOrDefault(tableName__346, t_4040);
        return List.copyOf(table__348);
    }
    public Record get(String tableName__350, int id__351) {
        Record return__138;
        List<Record> t_4034;
        fn__352: {
            this.ensureTable(tableName__350);
            t_4034 = new ArrayList<>();
            List<Record> table__353 = this.tables.getOrDefault(tableName__350, t_4034);
            List<Record> this__2516 = List.copyOf(table__353);
            int n__2517 = this__2516.size();
            int i__2518 = 0;
            while (i__2518 < n__2517) {
                Record el__2519 = Core.listGet(this__2516, i__2518);
                i__2518 = i__2518 + 1;
                Record record__354 = el__2519;
                int recordId__355;
                recordId__355 = record__354.getId();
                if (recordId__355 == id__351) {
                    return__138 = record__354;
                    break fn__352;
                }
            }
            throw Core.bubble();
        }
        return return__138;
    }
    public int count(String tableName__357) {
        this.ensureTable(tableName__357);
        List<Record> t_4030 = new ArrayList<>();
        List<Record> table__359 = this.tables.getOrDefault(tableName__357, t_4030);
        return table__359.size();
    }
}
