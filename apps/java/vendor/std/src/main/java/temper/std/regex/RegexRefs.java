package temper.std.regex;
import temper.core.Nullable;
import java.util.List;
import java.util.Map;
import temper.core.Core;
import java.util.AbstractMap.SimpleImmutableEntry;
final class RegexRefs {
    public final CodePoints codePoints;
    public final Group group;
    public final Match match;
    public final Or orObject;
    public static final class Builder {
        @Nullable CodePoints codePoints;
        public Builder codePoints(@Nullable CodePoints codePoints) {
            this.codePoints = codePoints;
            return this;
        }
        @Nullable Group group;
        public Builder group(@Nullable Group group) {
            this.group = group;
            return this;
        }
        @Nullable Match match;
        public Builder match(@Nullable Match match) {
            this.match = match;
            return this;
        }
        @Nullable Or orObject;
        public Builder orObject(@Nullable Or orObject) {
            this.orObject = orObject;
            return this;
        }
        public RegexRefs build() {
            return new RegexRefs(codePoints, group, match, orObject);
        }
    }
    public RegexRefs(@Nullable CodePoints codePoints__546, @Nullable Group group__548, @Nullable Match match__550, @Nullable Or orObject__552) {
        CodePoints t_1272;
        Group t_1273;
        Map<String, Group> t_1275;
        Match t_1276;
        Or t_1277;
        CodePoints codePoints__256;
        if (codePoints__546 == null) {
            t_1272 = new CodePoints("");
            codePoints__256 = t_1272;
        } else {
            codePoints__256 = codePoints__546;
        }
        Group group__257;
        if (group__548 == null) {
            t_1273 = new Group("", "", 0, 0);
            group__257 = t_1273;
        } else {
            group__257 = group__548;
        }
        Match match__258;
        if (match__550 == null) {
            t_1275 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("", group__257)));
            t_1276 = new Match(group__257, t_1275);
            match__258 = t_1276;
        } else {
            match__258 = match__550;
        }
        Or orObject__259;
        if (orObject__552 == null) {
            t_1277 = new Or(List.of());
            orObject__259 = t_1277;
        } else {
            orObject__259 = orObject__552;
        }
        this.codePoints = codePoints__256;
        this.group = group__257;
        this.match = match__258;
        this.orObject = orObject__259;
    }
    public RegexRefs(@Nullable CodePoints codePoints__546, @Nullable Group group__548, @Nullable Match match__550) {
        this(codePoints__546, group__548, match__550, null);
    }
    public RegexRefs(@Nullable CodePoints codePoints__546, @Nullable Group group__548) {
        this(codePoints__546, group__548, null, null);
    }
    public RegexRefs(@Nullable CodePoints codePoints__546) {
        this(codePoints__546, null, null, null);
    }
    public RegexRefs() {
        this(null, null, null, null);
    }
    public CodePoints getCodePoints() {
        return this.codePoints;
    }
    public Group getGroup() {
        return this.group;
    }
    public Match getMatch() {
        return this.match;
    }
    public Or getOrObject() {
        return this.orObject;
    }
}
