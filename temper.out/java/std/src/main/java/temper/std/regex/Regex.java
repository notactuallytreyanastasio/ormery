package temper.std.regex;
import static temper.std.regex.RegexGlobal.regexRefs__162;
import java.util.List;
import temper.core.Nullable;
import java.util.function.Function;
public final class Regex {
    public final RegexNode data;
    public Regex(RegexNode data__262) {
        RegexNode t_419 = data__262;
        this.data = t_419;
        String formatted__264 = Core.regexFormat(data__262);
        Object t_1156 = Core.regexCompiledFormatted(data__262, formatted__264);
        this.compiled = t_1156;
    }
    public boolean found(String text__266) {
        return Core.regexCompiledFound(this, this.compiled, text__266);
    }
    public Match find(String text__269, @Nullable Integer begin__554) {
        int begin__270;
        if (begin__554 == null) {
            begin__270 = 0;
        } else {
            begin__270 = begin__554;
        }
        return Core.regexCompiledFind(this, this.compiled, text__269, begin__270, regexRefs__162);
    }
    public Match find(String text__269) {
        return find(text__269, null);
    }
    public String replace(String text__273, Function<Match, String> format__274) {
        return Core.regexCompiledReplace(this, this.compiled, text__273, format__274, regexRefs__162);
    }
    public List<String> split(String text__277) {
        return Core.regexCompiledSplit(this, this.compiled, text__277, regexRefs__162);
    }
    final Object compiled;
    public RegexNode getData() {
        return this.data;
    }
}
