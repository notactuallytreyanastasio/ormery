package ormery;
import temper.core.Core;
public final class OrmeryMain {
    private OrmeryMain() {
    }
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("ormery.OrmeryGlobal");
        Core.waitUntilTasksComplete();
    }
}
