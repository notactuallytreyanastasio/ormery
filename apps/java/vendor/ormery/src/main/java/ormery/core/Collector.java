package ormery.core;
import java.util.List;
import temper.core.Core;
import java.util.ArrayList;
public final class Collector<PART__2> {
    final List<Collected<PART__2>> partsBuilder;
    public void appendSafe(String fixed__87) {
        CollectedFixed<PART__2> t_519 = new CollectedFixed<>(fixed__87);
        Core.listAdd(this.partsBuilder, t_519);
    }
    public void append(PART__2 part__90) {
        CollectedPart<PART__2> t_517 = new CollectedPart<>(part__90);
        Core.listAdd(this.partsBuilder, t_517);
    }
    public List<Collected<PART__2>> getParts() {
        return List.copyOf(this.partsBuilder);
    }
    public Collector() {
        List<Collected<PART__2>> t_515 = new ArrayList<>();
        this.partsBuilder = t_515;
    }
}
