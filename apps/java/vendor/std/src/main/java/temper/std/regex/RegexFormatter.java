package temper.std.regex;
import temper.core.Core;
import java.util.Objects;
import static temper.std.regex.RegexGlobal.Word;
import static temper.std.regex.RegexGlobal.Digit;
import static temper.std.regex.RegexGlobal.Space;
import temper.core.Nullable;
import static temper.std.regex.RegexGlobal.Dot;
import static temper.std.regex.RegexGlobal.End;
import static temper.std.regex.Codes.tab;
import static temper.std.regex.Codes.dash;
import static temper.std.regex.RegexGlobal.Begin;
import static temper.std.regex.Codes.space;
import static temper.std.regex.Codes.digit9;
import static temper.std.regex.Codes.lowerZ;
import static temper.std.regex.Codes.newline;
import static temper.std.regex.Codes.uint16Max;
import static temper.std.regex.RegexGlobal.WordBoundary;
import static temper.std.regex.Codes.surrogateMin;
import static temper.std.regex.Codes.surrogateMax;
import static temper.std.regex.Codes.carriageReturn;
import static temper.std.regex.Codes.highControlMax;
import static temper.std.regex.RegexGlobal.regexRefs__162;
import static temper.std.regex.Codes.supplementalMin;
import static temper.std.regex.RegexGlobal.escapeNeeds__163;
final class RegexFormatter {
    final StringBuilder out;
    public String format(RegexNode regex__310) {
        this.pushRegex(regex__310);
        return this.out.toString();
    }
    void pushRegex(RegexNode regex__313) {
        Capture t_884;
        CodePoints t_885;
        CodeRange t_886;
        CodeSet t_887;
        Or t_888;
        Repeat t_889;
        Sequence t_890;
        if (regex__313 instanceof Capture) {
            t_884 = Core.cast(Capture.class, regex__313);
            this.pushCapture(t_884);
        } else if (regex__313 instanceof CodePoints) {
            t_885 = Core.cast(CodePoints.class, regex__313);
            this.pushCodePoints(t_885, false);
        } else if (regex__313 instanceof CodeRange) {
            t_886 = Core.cast(CodeRange.class, regex__313);
            this.pushCodeRange(t_886);
        } else if (regex__313 instanceof CodeSet) {
            t_887 = Core.cast(CodeSet.class, regex__313);
            this.pushCodeSet(t_887);
        } else if (regex__313 instanceof Or) {
            t_888 = Core.cast(Or.class, regex__313);
            this.pushOr(t_888);
        } else if (regex__313 instanceof Repeat) {
            t_889 = Core.cast(Repeat.class, regex__313);
            this.pushRepeat(t_889);
        } else if (regex__313 instanceof Sequence) {
            t_890 = Core.cast(Sequence.class, regex__313);
            this.pushSequence(t_890);
        } else if (Objects.equals(regex__313, Begin)) {
            this.out.append("^");
        } else if (Objects.equals(regex__313, Dot)) {
            this.out.append(".");
        } else if (Objects.equals(regex__313, End)) {
            this.out.append("$");
        } else if (Objects.equals(regex__313, WordBoundary)) {
            this.out.append("\\b");
        } else if (Objects.equals(regex__313, Digit)) {
            this.out.append("\\d");
        } else if (Objects.equals(regex__313, Space)) {
            this.out.append("\\s");
        } else if (Objects.equals(regex__313, Word)) {
            this.out.append("\\w");
        }
    }
    void pushCapture(Capture capture__316) {
        this.out.append("(");
        StringBuilder t_858 = this.out;
        String t_1242 = capture__316.getName();
        this.pushCaptureName(t_858, t_1242);
        RegexNode t_1244 = capture__316.getItem();
        this.pushRegex(t_1244);
        this.out.append(")");
    }
    void pushCaptureName(StringBuilder out__319, String name__320) {
        out__319.append("?<" + name__320 + ">");
    }
    void pushCode(int code__323, boolean insideCodeSet__324) {
        boolean t_846;
        boolean t_847;
        String t_848;
        String t_850;
        boolean t_851;
        boolean t_852;
        boolean t_853;
        boolean t_854;
        String t_855;
        fn__325: {
            String specialEscape__326;
            if (code__323 == carriageReturn) {
                specialEscape__326 = "r";
            } else if (code__323 == newline) {
                specialEscape__326 = "n";
            } else if (code__323 == tab) {
                specialEscape__326 = "t";
            } else {
                specialEscape__326 = "";
            }
            if (!specialEscape__326.equals("")) {
                this.out.append("\\");
                this.out.append(specialEscape__326);
                break fn__325;
            }
            if (code__323 <= 127) {
                int escapeNeed__327 = Core.listGet(escapeNeeds__163, code__323);
                if (Objects.equals(escapeNeed__327, 2)) {
                    t_847 = true;
                } else {
                    if (insideCodeSet__324) {
                        t_846 = code__323 == dash;
                    } else {
                        t_846 = false;
                    }
                    t_847 = t_846;
                }
                if (t_847) {
                    this.out.append("\\");
                    t_848 = Core.stringFromCodePoint(code__323);
                    this.out.append(t_848);
                    break fn__325;
                } else if (Objects.equals(escapeNeed__327, 0)) {
                    t_850 = Core.stringFromCodePoint(code__323);
                    this.out.append(t_850);
                    break fn__325;
                }
            }
            if (code__323 >= supplementalMin) {
                t_854 = true;
            } else {
                if (code__323 > highControlMax) {
                    if (surrogateMin <= code__323) {
                        t_851 = code__323 <= surrogateMax;
                    } else {
                        t_851 = false;
                    }
                    if (t_851) {
                        t_852 = true;
                    } else {
                        t_852 = code__323 == uint16Max;
                    }
                    t_853 = !t_852;
                } else {
                    t_853 = false;
                }
                t_854 = t_853;
            }
            if (t_854) {
                t_855 = Core.stringFromCodePoint(code__323);
                this.out.append(t_855);
            } else {
                temper.std.regex.Core.regexFormatterPushCodeTo(this, this.out, code__323, insideCodeSet__324);
            }
        }
    }
    void pushCodePoints(CodePoints codePoints__334, boolean insideCodeSet__335) {
        int t_1229;
        int t_1231;
        String value__337 = codePoints__334.getValue();
        int index__338 = 0;
        while (true) {
            if (!Core.stringHasIndex(value__337, index__338)) {
                break;
            }
            t_1229 = value__337.codePointAt(index__338);
            this.pushCode(t_1229, insideCodeSet__335);
            t_1231 = Core.stringNext(value__337, index__338);
            index__338 = t_1231;
        }
    }
    void pushCodeRange(CodeRange codeRange__340) {
        this.out.append("[");
        this.pushCodeRangeUnwrapped(codeRange__340);
        this.out.append("]");
    }
    void pushCodeRangeUnwrapped(CodeRange codeRange__343) {
        int t_1219 = codeRange__343.getMin();
        this.pushCode(t_1219, true);
        this.out.append("-");
        int t_1222 = codeRange__343.getMax();
        this.pushCode(t_1222, true);
    }
    void pushCodeSet(CodeSet codeSet__346) {
        int t_1213;
        CodePart t_1215;
        CodeSet t_831;
        RegexNode adjusted__348 = this.adjustCodeSet(codeSet__346, regexRefs__162);
        if (adjusted__348 instanceof CodeSet) {
            t_831 = Core.cast(CodeSet.class, adjusted__348);
            this.out.append("[");
            if (t_831.isNegated()) {
                this.out.append("^");
            }
            int i__349 = 0;
            while (true) {
                t_1213 = t_831.getItems().size();
                if (i__349 >= t_1213) {
                    break;
                }
                t_1215 = Core.listGet(t_831.getItems(), i__349);
                this.pushCodeSetItem(t_1215);
                i__349 = i__349 + 1;
            }
            this.out.append("]");
        } else {
            this.pushRegex(adjusted__348);
        }
    }
    RegexNode adjustCodeSet(CodeSet codeSet__351, RegexRefs regexRefs__352) {
        return codeSet__351;
    }
    void pushCodeSetItem(CodePart codePart__355) {
        CodePoints t_819;
        CodeRange t_820;
        SpecialSet t_821;
        if (codePart__355 instanceof CodePoints) {
            t_819 = Core.cast(CodePoints.class, codePart__355);
            this.pushCodePoints(t_819, true);
        } else if (codePart__355 instanceof CodeRange) {
            t_820 = Core.cast(CodeRange.class, codePart__355);
            this.pushCodeRangeUnwrapped(t_820);
        } else if (codePart__355 instanceof SpecialSet) {
            t_821 = Core.cast(SpecialSet.class, codePart__355);
            this.pushRegex(t_821);
        }
    }
    void pushOr(Or or__358) {
        RegexNode t_1192;
        int t_1195;
        RegexNode t_1198;
        if (!or__358.getItems().isEmpty()) {
            this.out.append("(?:");
            t_1192 = Core.listGet(or__358.getItems(), 0);
            this.pushRegex(t_1192);
            int i__360 = 1;
            while (true) {
                t_1195 = or__358.getItems().size();
                if (i__360 >= t_1195) {
                    break;
                }
                this.out.append("|");
                t_1198 = Core.listGet(or__358.getItems(), i__360);
                this.pushRegex(t_1198);
                i__360 = i__360 + 1;
            }
            this.out.append(")");
        }
    }
    void pushRepeat(Repeat repeat__362) {
        String t_1180;
        String t_1183;
        boolean t_796;
        boolean t_797;
        boolean t_798;
        this.out.append("(?:");
        RegexNode t_1172 = repeat__362.getItem();
        this.pushRegex(t_1172);
        this.out.append(")");
        int min__364 = repeat__362.getMin();
        @Nullable Integer max__365 = repeat__362.getMax();
        if (min__364 == 0) {
            t_796 = Core.boxedEq(max__365, 1);
        } else {
            t_796 = false;
        }
        if (t_796) {
            this.out.append("?");
        } else {
            if (min__364 == 0) {
                t_797 = max__365 == null;
            } else {
                t_797 = false;
            }
            if (t_797) {
                this.out.append("*");
            } else {
                if (min__364 == 1) {
                    t_798 = max__365 == null;
                } else {
                    t_798 = false;
                }
                if (t_798) {
                    this.out.append("+");
                } else {
                    t_1180 = Integer.toString(min__364);
                    this.out.append("{" + t_1180);
                    if (!Core.boxedEqRev(min__364, max__365)) {
                        this.out.append(",");
                        if (max__365 != null) {
                            t_1183 = Integer.toString(max__365);
                            this.out.append(t_1183);
                        }
                    }
                    this.out.append("}");
                }
            }
        }
        if (repeat__362.isReluctant()) {
            this.out.append("?");
        }
    }
    void pushSequence(Sequence sequence__367) {
        int t_1167;
        RegexNode t_1169;
        int i__369 = 0;
        while (true) {
            t_1167 = sequence__367.getItems().size();
            if (i__369 >= t_1167) {
                break;
            }
            t_1169 = Core.listGet(sequence__367.getItems(), i__369);
            this.pushRegex(t_1169);
            i__369 = i__369 + 1;
        }
    }
    public @Nullable Integer maxCode(CodePart codePart__371) {
        @Nullable Integer return__157;
        int t_1163;
        CodePoints t_784;
        if (codePart__371 instanceof CodePoints) {
            t_784 = Core.cast(CodePoints.class, codePart__371);
            String value__373 = t_784.getValue();
            if (value__373.isEmpty()) {
                return__157 = null;
            } else {
                int max__374 = 0;
                int index__375 = 0;
                while (true) {
                    if (!Core.stringHasIndex(value__373, index__375)) {
                        break;
                    }
                    int next__376 = value__373.codePointAt(index__375);
                    if (next__376 > max__374) {
                        max__374 = next__376;
                    }
                    t_1163 = Core.stringNext(value__373, index__375);
                    index__375 = t_1163;
                }
                return__157 = max__374;
            }
        } else if (codePart__371 instanceof CodeRange) {
            return__157 = Core.cast(CodeRange.class, codePart__371).getMax();
        } else if (Objects.equals(codePart__371, Digit)) {
            return__157 = digit9;
        } else if (Objects.equals(codePart__371, Space)) {
            return__157 = space;
        } else if (Objects.equals(codePart__371, Word)) {
            return__157 = lowerZ;
        } else {
            return__157 = null;
        }
        return return__157;
    }
    public RegexFormatter() {
        StringBuilder t_1157 = new StringBuilder();
        this.out = t_1157;
    }
}
