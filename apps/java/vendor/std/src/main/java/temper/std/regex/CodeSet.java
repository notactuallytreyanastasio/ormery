package temper.std.regex;
import java.util.List;
import temper.core.Nullable;
public final class CodeSet implements RegexNode {
    public final List<CodePart> items;
    public final boolean negated;
    public static final class Builder {
        List<CodePart> items;
        public Builder items(List<CodePart> items) {
            this.items = items;
            return this;
        }
        @Nullable Boolean negated;
        public Builder negated(@Nullable Boolean negated) {
            this.negated = negated;
            return this;
        }
        public CodeSet build() {
            return new CodeSet(items, negated);
        }
    }
    public CodeSet(List<CodePart> items__212, @Nullable Boolean negated__542) {
        boolean negated__213;
        if (negated__542 == null) {
            negated__213 = false;
        } else {
            negated__213 = negated__542;
        }
        this.items = items__212;
        this.negated = negated__213;
    }
    public CodeSet(List<CodePart> items__212) {
        this(items__212, null);
    }
    public List<CodePart> getItems() {
        return this.items;
    }
    public boolean isNegated() {
        return this.negated;
    }
}
