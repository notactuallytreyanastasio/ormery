package temper.std.regex;
import temper.core.Nullable;
public final class Repeat implements RegexNode {
    public final RegexNode item;
    public final int min;
    public final @Nullable Integer max;
    public final boolean reluctant;
    public static final class Builder {
        RegexNode item;
        public Builder item(RegexNode item) {
            this.item = item;
            return this;
        }
        int min;
        boolean min__set;
        public Builder min(int min) {
            min__set = true;
            this.min = min;
            return this;
        }
        @Nullable Integer max;
        boolean max__set;
        public Builder max(@Nullable Integer max) {
            max__set = true;
            this.max = max;
            return this;
        }
        @Nullable Boolean reluctant;
        public Builder reluctant(@Nullable Boolean reluctant) {
            this.reluctant = reluctant;
            return this;
        }
        public Repeat build() {
            if (!min__set || !max__set || item == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!min__set) {
                    _message.append(" min");
                }
                if (!max__set) {
                    _message.append(" max");
                }
                if (item == null) {
                    _message.append(" item");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new Repeat(item, min, max, reluctant);
        }
    }
    public Repeat(RegexNode item__222, int min__223, @Nullable Integer max__224, @Nullable Boolean reluctant__544) {
        boolean reluctant__225;
        if (reluctant__544 == null) {
            reluctant__225 = false;
        } else {
            reluctant__225 = reluctant__544;
        }
        this.item = item__222;
        this.min = min__223;
        this.max = max__224;
        this.reluctant = reluctant__225;
    }
    public Repeat(RegexNode item__222, int min__223, @Nullable Integer max__224) {
        this(item__222, min__223, max__224, null);
    }
    public RegexNode getItem() {
        return this.item;
    }
    public int getMin() {
        return this.min;
    }
    public @Nullable Integer getMax() {
        return this.max;
    }
    public boolean isReluctant() {
        return this.reluctant;
    }
}
