package ormery.sql;
import temper.core.Core;
public final class SqlMain {
    private SqlMain() {
    }
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("ormery.sql.SqlGlobal");
        Core.waitUntilTasksComplete();
    }
}
