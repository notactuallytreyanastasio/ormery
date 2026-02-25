package temper.std.regex;
import java.util.List;
import java.util.function.Function;
public interface RegexNode {
    default Regex compiled() {
        return new Regex(this);
    }
    default boolean found(String text__170) {
        return this.compiled().found(text__170);
    }
    default Match find(String text__173) {
        return this.compiled().find(text__173);
    }
    /**
     * Replace and split functions are also available. Both apply to all matches in
     * the string, replacing all or splitting at all.
     */
    default String replace(String text__176, Function<Match, String> format__177) {
        return this.compiled().replace(text__176, format__177);
    }
    default List<String> split(String text__180) {
        return this.compiled().split(text__180);
    }
}
