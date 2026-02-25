package ormery.html;
import temper.core.Core;
public final class HtmlMain {
    private HtmlMain() {
    }
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("ormery.html.HtmlGlobal");
        Core.waitUntilTasksComplete();
    }
}
