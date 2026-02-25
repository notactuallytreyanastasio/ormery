package ormery.core;
import temper.core.Core;
public final class CoreMain {
    private CoreMain() {
    }
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("ormery.core.CoreGlobal");
        Core.waitUntilTasksComplete();
    }
}
