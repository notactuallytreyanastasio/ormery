package ormery;
import temper.core.Core;
import java.util.Map;
import java.util.List;
import java.util.function.Function;
import java.util.function.BiFunction;
public final class Record {
    public final Map<String, String> data;
    public String get(String field__308) {
        return Core.mappedGet(this.data, field__308);
    }
    public String getOr(String field__311, String fallback__312) {
        return this.data.getOrDefault(field__311, fallback__312);
    }
    public boolean has(String field__315) {
        return this.data.containsKey(field__315);
    }
    public int getId() {
        String idStr__319;
        idStr__319 = Core.mappedGet(this.data, "id");
        return Core.stringToInt(idStr__319);
    }
    public String describe() {
        BiFunction<String, String, String> fn__4061 = (k__323, v__324) -> k__323 + ": " + v__324;
        List<String> pairs__322 = Core.mappedToListWith(this.data, fn__4061);
        Function<String, String> fn__4060 = s__326 -> s__326;
        return Core.listJoinObj(pairs__322, ", ", fn__4060);
    }
    public Record(Map<String, String> data__329) {
        this.data = data__329;
    }
    public Map<String, String> getData() {
        return this.data;
    }
}
