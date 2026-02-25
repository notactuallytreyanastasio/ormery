package ormery.url;
import temper.core.Core;
public final class UrlMain {
    private UrlMain() {
    }
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("ormery.url.UrlGlobal");
        Core.waitUntilTasksComplete();
    }
}
