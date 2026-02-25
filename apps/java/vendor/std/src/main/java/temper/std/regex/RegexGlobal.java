package temper.std.regex;
import java.util.List;
import temper.core.Nullable;
import temper.core.Core;
import java.util.ArrayList;
import static temper.std.regex.Codes.dot;
import static temper.std.regex.Codes.dash;
import static temper.std.regex.Codes.peso;
import static temper.std.regex.Codes.pipe;
import static temper.std.regex.Codes.plus;
import static temper.std.regex.Codes.star;
import static temper.std.regex.Codes.space;
import static temper.std.regex.Codes.caret;
import static temper.std.regex.Codes.slash;
import static temper.std.regex.Codes.tilde;
import static temper.std.regex.Codes.digit0;
import static temper.std.regex.Codes.digit9;
import static temper.std.regex.Codes.upperA;
import static temper.std.regex.Codes.upperZ;
import static temper.std.regex.Codes.lowerA;
import static temper.std.regex.Codes.lowerZ;
import static temper.std.regex.Codes.question;
import static temper.std.regex.Codes.ampersand;
import static temper.std.regex.Codes.backslash;
import static temper.std.regex.Codes.curlyLeft;
import static temper.std.regex.Codes.roundLeft;
import static temper.std.regex.Codes.underscore;
import static temper.std.regex.Codes.curlyRight;
import static temper.std.regex.Codes.roundRight;
import static temper.std.regex.Codes.squareLeft;
import static temper.std.regex.Codes.squareRight;
public final class RegexGlobal {
    private RegexGlobal() {
    }
    static final Special return__190;
    public static final Special Begin;
    static final Special return__192;
    public static final Special Dot;
    static final Special return__194;
    public static final Special End;
    static final Special return__196;
    public static final Special WordBoundary;
    static final SpecialSet return__198;
    public static final SpecialSet Digit;
    static final SpecialSet return__200;
    public static final SpecialSet Space;
    static final SpecialSet return__202;
    public static final SpecialSet Word;
    static final List<Integer> return__990;
    static final List<Integer> escapeNeeds__163;
    static final RegexRefs return__991;
    static final RegexRefs regexRefs__162;
    static List<Integer> buildEscapeNeeds__161() {
        boolean t_925;
        boolean t_926;
        boolean t_927;
        boolean t_928;
        boolean t_929;
        boolean t_930;
        boolean t_931;
        boolean t_932;
        boolean t_933;
        boolean t_934;
        boolean t_935;
        boolean t_936;
        boolean t_937;
        boolean t_938;
        boolean t_939;
        boolean t_940;
        boolean t_941;
        boolean t_942;
        boolean t_943;
        boolean t_944;
        boolean t_945;
        boolean t_946;
        boolean t_947;
        boolean t_948;
        int t_949;
        List<Integer> escapeNeeds__379 = new ArrayList<>();
        int code__380 = 0;
        while (code__380 <= 127) {
            if (code__380 == dash) {
                t_932 = true;
            } else {
                if (code__380 == space) {
                    t_931 = true;
                } else {
                    if (code__380 == underscore) {
                        t_930 = true;
                    } else {
                        if (digit0 <= code__380) {
                            t_925 = code__380 <= digit9;
                        } else {
                            t_925 = false;
                        }
                        if (t_925) {
                            t_929 = true;
                        } else {
                            if (upperA <= code__380) {
                                t_926 = code__380 <= upperZ;
                            } else {
                                t_926 = false;
                            }
                            if (t_926) {
                                t_928 = true;
                            } else {
                                if (lowerA <= code__380) {
                                    t_927 = code__380 <= lowerZ;
                                } else {
                                    t_927 = false;
                                }
                                t_928 = t_927;
                            }
                            t_929 = t_928;
                        }
                        t_930 = t_929;
                    }
                    t_931 = t_930;
                }
                t_932 = t_931;
            }
            if (t_932) {
                t_949 = 0;
            } else {
                if (code__380 == ampersand) {
                    t_948 = true;
                } else {
                    if (code__380 == backslash) {
                        t_947 = true;
                    } else {
                        if (code__380 == caret) {
                            t_946 = true;
                        } else {
                            if (code__380 == curlyLeft) {
                                t_945 = true;
                            } else {
                                if (code__380 == curlyRight) {
                                    t_944 = true;
                                } else {
                                    if (code__380 == dot) {
                                        t_943 = true;
                                    } else {
                                        if (code__380 == peso) {
                                            t_942 = true;
                                        } else {
                                            if (code__380 == pipe) {
                                                t_941 = true;
                                            } else {
                                                if (code__380 == plus) {
                                                    t_940 = true;
                                                } else {
                                                    if (code__380 == question) {
                                                        t_939 = true;
                                                    } else {
                                                        if (code__380 == roundLeft) {
                                                            t_938 = true;
                                                        } else {
                                                            if (code__380 == roundRight) {
                                                                t_937 = true;
                                                            } else {
                                                                if (code__380 == slash) {
                                                                    t_936 = true;
                                                                } else {
                                                                    if (code__380 == squareLeft) {
                                                                        t_935 = true;
                                                                    } else {
                                                                        if (code__380 == squareRight) {
                                                                            t_934 = true;
                                                                        } else {
                                                                            if (code__380 == star) {
                                                                                t_933 = true;
                                                                            } else {
                                                                                t_933 = code__380 == tilde;
                                                                            }
                                                                            t_934 = t_933;
                                                                        }
                                                                        t_935 = t_934;
                                                                    }
                                                                    t_936 = t_935;
                                                                }
                                                                t_937 = t_936;
                                                            }
                                                            t_938 = t_937;
                                                        }
                                                        t_939 = t_938;
                                                    }
                                                    t_940 = t_939;
                                                }
                                                t_941 = t_940;
                                            }
                                            t_942 = t_941;
                                        }
                                        t_943 = t_942;
                                    }
                                    t_944 = t_943;
                                }
                                t_945 = t_944;
                            }
                            t_946 = t_945;
                        }
                        t_947 = t_946;
                    }
                    t_948 = t_947;
                }
                if (t_948) {
                    t_949 = 2;
                } else {
                    t_949 = 1;
                }
            }
            Core.listAdd(escapeNeeds__379, t_949);
            code__380 = code__380 + 1;
        }
        return List.copyOf(escapeNeeds__379);
    }
    public static RegexNode entire(RegexNode item__226) {
        return new Sequence(List.of(Begin, item__226, End));
    }
    public static Repeat oneOrMore(RegexNode item__228, @Nullable Boolean reluctant__556) {
        boolean reluctant__229;
        if (reluctant__556 == null) {
            reluctant__229 = false;
        } else {
            reluctant__229 = reluctant__556;
        }
        return new Repeat(item__228, 1, null, reluctant__229);
    }
    public static Repeat oneOrMore(RegexNode item__228) {
        return oneOrMore(item__228, null);
    }
    public static Repeat optional(RegexNode item__231, @Nullable Boolean reluctant__558) {
        boolean reluctant__232;
        if (reluctant__558 == null) {
            reluctant__232 = false;
        } else {
            reluctant__232 = reluctant__558;
        }
        return new Repeat(item__231, 0, 1, reluctant__232);
    }
    public static Repeat optional(RegexNode item__231) {
        return optional(item__231, null);
    }
    static {
        return__190 = new Begin();
        Begin = return__190;
        return__192 = new Dot();
        Dot = return__192;
        return__194 = new End();
        End = return__194;
        return__196 = new WordBoundary();
        WordBoundary = return__196;
        return__198 = new Digit();
        Digit = return__198;
        return__200 = new Space();
        Space = return__200;
        return__202 = new Word();
        Word = return__202;
        return__990 = RegexGlobal.buildEscapeNeeds__161();
        escapeNeeds__163 = return__990;
        return__991 = new RegexRefs();
        regexRefs__162 = return__991;
    }
}
