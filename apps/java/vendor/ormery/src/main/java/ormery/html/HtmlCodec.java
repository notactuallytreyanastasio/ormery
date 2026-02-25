package ormery.html;
import temper.core.Core;
import ormery.core.Codec;
import java.util.Objects;
import static ormery.html.HtmlGlobal.htmlNamedCharacters__267;
/**
 * HtmlCodec deals with HTML entity syntax: `&amp;lt;` for example.
 */
public final class HtmlCodec implements Codec {
    public String encode(String s__345) {
        String return__127;
        int t_5670;
        int t_5673;
        int t_5674;
        String t_3392;
        StringBuilder sb__347 = new StringBuilder();
        int end__348 = s__345.length();
        int encodedTo__349 = 0;
        int i__350 = 0;
        while (i__350 < end__348) {
            continue_5752: {
                t_5670 = s__345.codePointAt(i__350);
                if (t_5670 == 38) {
                    t_3392 = "&amp;";
                } else if (t_5670 == 60) {
                    t_3392 = "&lt;";
                } else if (t_5670 == 62) {
                    t_3392 = "&gt;";
                } else if (t_5670 == 39) {
                    t_3392 = "&#39;";
                } else if (t_5670 == 34) {
                    t_3392 = "&#34;";
                } else if (t_5670 == 0) {
                    t_3392 = "&#0;";
                } else {
                    break continue_5752;
                }
                String replacement__351 = t_3392;
                Core.stringBuilderAppendBetween(sb__347, s__345, encodedTo__349, i__350);
                sb__347.append(replacement__351);
                t_5673 = Core.stringNext(s__345, i__350);
                encodedTo__349 = t_5673;
            }
            t_5674 = Core.stringNext(s__345, i__350);
            i__350 = t_5674;
        }
        if (encodedTo__349 > 0) {
            Core.stringBuilderAppendBetween(sb__347, s__345, encodedTo__349, end__348);
            return__127 = sb__347.toString();
        } else {
            return__127 = s__345;
        }
        return return__127;
    }
    public String decode(String s__353) {
        String return__128;
        int t_5647;
        int t_5648;
        int t_5650;
        int t_5652;
        int t_5653;
        int t_5654;
        int t_5659;
        int t_5665;
        boolean t_3360;
        boolean t_3366;
        boolean t_3367;
        boolean t_3370;
        int t_3373;
        boolean t_3375;
        boolean t_3376;
        boolean t_3380;
        StringBuilder sb__355 = new StringBuilder();
        int end__356 = s__353.length();
        int decodedTo__357 = 0;
        int i__358 = 0;
        while (i__358 < end__356) {
            continue_5753: if (s__353.codePointAt(i__358) == 38) {
                int startOfEntity__359 = Core.stringNext(s__353, i__358);
                int endOfEntity__360 = startOfEntity__359;
                if (startOfEntity__359 < end__356) {
                    t_5647 = s__353.codePointAt(startOfEntity__359);
                    t_3360 = Objects.equals(t_5647, "#");
                } else {
                    t_3360 = false;
                }
                if (t_3360) {
                    t_5648 = Core.stringNext(s__353, startOfEntity__359);
                    endOfEntity__360 = t_5648;
                    if (endOfEntity__360 >= end__356) {
                        break continue_5753;
                    }
                    int base__361 = 10;
                    if ((s__353.codePointAt(endOfEntity__360) | 32) == 120) {
                        t_5650 = Core.stringNext(s__353, endOfEntity__360);
                        endOfEntity__360 = t_5650;
                        base__361 = 16;
                    }
                    int digitQuota__362 = 7;
                    int startOfDigits__363 = endOfEntity__360;
                    while (true) {
                        if (endOfEntity__360 >= end__356) {
                            break;
                        }
                        int cp__364 = s__353.codePointAt(endOfEntity__360);
                        if (48 <= cp__364) {
                            t_3366 = cp__364 <= 57;
                        } else {
                            t_3366 = false;
                        }
                        if (!t_3366) {
                            if (base__361 == 16) {
                                int lcp__365 = cp__364 | 32;
                                if (97 <= lcp__365) {
                                    t_3367 = lcp__365 <= 102;
                                } else {
                                    t_3367 = false;
                                }
                                if (!t_3367) {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        t_5652 = Core.stringNext(s__353, endOfEntity__360);
                        endOfEntity__360 = t_5652;
                    }
                    int endOfDigits__366 = endOfEntity__360;
                    if (endOfDigits__366 == startOfDigits__363) {
                        break continue_5753;
                    }
                    if (endOfEntity__360 < end__356) {
                        t_5653 = s__353.codePointAt(endOfEntity__360);
                        t_3370 = t_5653 == 59;
                    } else {
                        t_3370 = false;
                    }
                    if (t_3370) {
                        t_5654 = Core.stringNext(s__353, endOfEntity__360);
                        endOfEntity__360 = t_5654;
                    }
                    try {
                        t_3373 = Core.stringToInt(Core.stringSlice(s__353, startOfDigits__363, endOfDigits__366), base__361);
                    } catch (RuntimeException ignored$1) {
                        break continue_5753;
                    }
                    int decodedCp__367 = t_3373;
                    if (0 <= decodedCp__367) {
                        t_3375 = decodedCp__367 <= 1114111;
                    } else {
                        t_3375 = false;
                    }
                    if (t_3375) {
                        Core.stringBuilderAppendBetween(sb__355, s__353, decodedTo__357, i__358);
                        if (55296 <= decodedCp__367) {
                            t_3376 = decodedCp__367 <= 57343;
                        } else {
                            t_3376 = false;
                        }
                        if (t_3376) {
                            sb__355.append("\ufffd");
                        } else {
                            try {
                                Core.stringBuilderAppendCodePoint(sb__355, decodedCp__367);
                            } catch (RuntimeException ignored$2) {
                                break continue_5753;
                            }
                        }
                        decodedTo__357 = endOfEntity__360;
                    }
                } else {
                    while (endOfEntity__360 < end__356) {
                        int cp__368 = s__353.codePointAt(endOfEntity__360);
                        t_5659 = Core.stringNext(s__353, endOfEntity__360);
                        endOfEntity__360 = t_5659;
                        if (cp__368 == 59) {
                            break;
                        }
                        int lcp__369 = cp__368 | 32;
                        if (97 <= lcp__369) {
                            t_3380 = lcp__369 <= 122;
                        } else {
                            t_3380 = false;
                        }
                        if (!t_3380) {
                            break;
                        }
                    }
                    if (startOfEntity__359 < endOfEntity__360) {
                        String entityName__370 = Core.stringSlice(s__353, startOfEntity__359, endOfEntity__360);
                        String entityValue__371 = htmlNamedCharacters__267.getOrDefault(entityName__370, "");
                        if (!entityValue__371.isEmpty()) {
                            Core.stringBuilderAppendBetween(sb__355, s__353, decodedTo__357, i__358);
                            sb__355.append(entityValue__371);
                            decodedTo__357 = endOfEntity__360;
                        }
                    }
                }
            }
            t_5665 = Core.stringNext(s__353, i__358);
            i__358 = t_5665;
        }
        if (decodedTo__357 > 0) {
            Core.stringBuilderAppendBetween(sb__355, s__353, decodedTo__357, end__356);
            return__128 = sb__355.toString();
        } else {
            return__128 = s__353;
        }
        return return__128;
    }
    public HtmlCodec() {
    }
}
