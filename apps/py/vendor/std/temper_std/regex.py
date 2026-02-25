from abc import ABCMeta as ABCMeta48
from builtins import str as str15, bool as bool13, int as int20, list as list2, isinstance as isinstance55, len as len5, tuple as tuple0
from typing import Callable as Callable16, Sequence as Sequence17, Union as Union18, Any as Any57, ClassVar as ClassVar50, MutableSequence as MutableSequence14
from types import MappingProxyType as MappingProxyType53
from temper_core import cast_by_type as cast_by_type56, Label as Label21, Pair as Pair3, map_constructor as map_constructor68, generic_eq as generic_eq74, list_get as list_get12, string_from_code_point as string_from_code_point60, string_get as string_get6, string_next as string_next9, int_add as int_add10, int_to_string as int_to_string8, str_cat as str_cat7
from temper_core.regex import regex_compile_formatted as regex_compile_formatted69, regex_compiled_found as regex_compiled_found70, regex_compiled_find as regex_compiled_find71, regex_compiled_replace as regex_compiled_replace72, regex_compiled_split as regex_compiled_split73, regex_formatter_push_capture_name as regex_formatter_push_capture_name75, regex_formatter_push_code_to as regex_formatter_push_code_to76
pair_2610 = Pair3
map_constructor_2611 = map_constructor68
regex_compile_formatted_2612 = regex_compile_formatted69
regex_compiled_found_2613 = regex_compiled_found70
regex_compiled_find_2614 = regex_compiled_find71
regex_compiled_replace_2615 = regex_compiled_replace72
regex_compiled_split_2616 = regex_compiled_split73
generic_eq_2618 = generic_eq74
regex_formatter_push_capture_name_2620 = regex_formatter_push_capture_name75
list_get_2621 = list_get12
string_from_code_point_2622 = string_from_code_point60
regex_formatter_push_code_to_2623 = regex_formatter_push_code_to76
string_get_2625 = string_get6
string_next_2626 = string_next9
len_2627 = len5
int_add_2628 = int_add10
int_to_string_2630 = int_to_string8
str_cat_2631 = str_cat7
list_2633 = list2
tuple_2635 = tuple0
class RegexNode(metaclass = ABCMeta48):
    def compiled(this_42) -> 'Regex':
        return Regex(this_42)
    def found(this_43, text_170: 'str15') -> 'bool13':
        return this_43.compiled().found(text_170)
    def find(this_44, text_173: 'str15') -> 'Match':
        return this_44.compiled().find(text_173)
    def replace(this_45, text_176: 'str15', format_177: 'Callable16[[Match], str15]') -> 'str15':
        'Replace and split functions are also available. Both apply to all matches in\nthe string, replacing all or splitting at all.\n\nthis__45: RegexNode\n\ntext__176: String\n\nformat__177: fn (Match): String\n'
        return this_45.compiled().replace(text_176, format_177)
    def split(this_46, text_180: 'str15') -> 'Sequence17[str15]':
        return this_46.compiled().split(text_180)
class Capture(RegexNode):
    '`Capture` is a [group](#groups) that remembers the matched text for later\naccess. Temper supports only named matches, with current intended syntax\n`/(?name = ...)/`.'
    name_182: 'str15'
    item_183: 'RegexNode'
    __slots__ = ('name_182', 'item_183')
    def __init__(this_87, name_185: 'str15', item_186: 'RegexNode') -> None:
        this_87.name_182 = name_185
        this_87.item_183 = item_186
    @property
    def name(this_444) -> 'str15':
        return this_444.name_182
    @property
    def item(this_447) -> 'RegexNode':
        return this_447.item_183
class CodePart(RegexNode, metaclass = ABCMeta48):
    pass
class CodePoints(CodePart):
    value_187: 'str15'
    __slots__ = ('value_187',)
    def __init__(this_89, value_189: 'str15') -> None:
        this_89.value_187 = value_189
    @property
    def value(this_450) -> 'str15':
        return this_450.value_187
class Special(RegexNode, metaclass = ABCMeta48):
    pass
class SpecialSet(CodePart, Special, metaclass = ABCMeta48):
    pass
class CodeRange(CodePart):
    min_204: 'int20'
    max_205: 'int20'
    __slots__ = ('min_204', 'max_205')
    def __init__(this_105, min_207: 'int20', max_208: 'int20') -> None:
        this_105.min_204 = min_207
        this_105.max_205 = max_208
    @property
    def min(this_453) -> 'int20':
        return this_453.min_204
    @property
    def max(this_456) -> 'int20':
        return this_456.max_205
class CodeSet(RegexNode):
    items_209: 'Sequence17[CodePart]'
    negated_210: 'bool13'
    __slots__ = ('items_209', 'negated_210')
    def __init__(this_107, items_212: 'Sequence17[CodePart]', negated_542: 'Union18[bool13, None]' = None) -> None:
        _negated_542: 'Union18[bool13, None]' = negated_542
        negated_213: 'bool13'
        if _negated_542 is None:
            negated_213 = False
        else:
            negated_213 = _negated_542
        this_107.items_209 = items_212
        this_107.negated_210 = negated_213
    @property
    def items(this_459) -> 'Sequence17[CodePart]':
        return this_459.items_209
    @property
    def negated(this_462) -> 'bool13':
        return this_462.negated_210
class Or(RegexNode):
    '`Or` matches any one of multiple options, such as `/ab|cd|e*/`.'
    items_214: 'Sequence17[RegexNode]'
    __slots__ = ('items_214',)
    def __init__(this_110, items_216: 'Sequence17[RegexNode]') -> None:
        this_110.items_214 = items_216
    @property
    def items(this_465) -> 'Sequence17[RegexNode]':
        return this_465.items_214
class Repeat(RegexNode):
    item_217: 'RegexNode'
    min_218: 'int20'
    max_219: 'Union18[int20, None]'
    reluctant_220: 'bool13'
    __slots__ = ('item_217', 'min_218', 'max_219', 'reluctant_220')
    def __init__(this_113, item_222: 'RegexNode', min_223: 'int20', max_224: 'Union18[int20, None]', reluctant_544: 'Union18[bool13, None]' = None) -> None:
        _reluctant_544: 'Union18[bool13, None]' = reluctant_544
        reluctant_225: 'bool13'
        if _reluctant_544 is None:
            reluctant_225 = False
        else:
            reluctant_225 = _reluctant_544
        this_113.item_217 = item_222
        this_113.min_218 = min_223
        this_113.max_219 = max_224
        this_113.reluctant_220 = reluctant_225
    @property
    def item(this_468) -> 'RegexNode':
        return this_468.item_217
    @property
    def min(this_471) -> 'int20':
        return this_471.min_218
    @property
    def max(this_474) -> 'Union18[int20, None]':
        return this_474.max_219
    @property
    def reluctant(this_477) -> 'bool13':
        return this_477.reluctant_220
class Sequence(RegexNode):
    '`Sequence` strings along multiple other regexes in order.'
    items_234: 'Sequence17[RegexNode]'
    __slots__ = ('items_234',)
    def __init__(this_119, items_236: 'Sequence17[RegexNode]') -> None:
        this_119.items_234 = items_236
    @property
    def items(this_480) -> 'Sequence17[RegexNode]':
        return this_480.items_234
class Match:
    full_237: 'Group'
    groups_238: 'MappingProxyType53[str15, Group]'
    __slots__ = ('full_237', 'groups_238')
    def __init__(this_122, full_240: 'Group', groups_241: 'MappingProxyType53[str15, Group]') -> None:
        this_122.full_237 = full_240
        this_122.groups_238 = groups_241
    @property
    def full(this_495) -> 'Group':
        return this_495.full_237
    @property
    def groups(this_498) -> 'MappingProxyType53[str15, Group]':
        return this_498.groups_238
class Group:
    name_242: 'str15'
    value_243: 'str15'
    begin_244: 'int20'
    end_245: 'int20'
    __slots__ = ('name_242', 'value_243', 'begin_244', 'end_245')
    def __init__(this_125, name_247: 'str15', value_248: 'str15', begin_249: 'int20', end_250: 'int20') -> None:
        this_125.name_242 = name_247
        this_125.value_243 = value_248
        this_125.begin_244 = begin_249
        this_125.end_245 = end_250
    @property
    def name(this_483) -> 'str15':
        return this_483.name_242
    @property
    def value(this_486) -> 'str15':
        return this_486.value_243
    @property
    def begin(this_489) -> 'int20':
        return this_489.begin_244
    @property
    def end(this_492) -> 'int20':
        return this_492.end_245
class RegexRefs_54:
    code_points_251: 'CodePoints'
    group_252: 'Group'
    match_253: 'Match'
    or_object_254: 'Or'
    __slots__ = ('code_points_251', 'group_252', 'match_253', 'or_object_254')
    def __init__(this_127, code_points_546: 'Union18[CodePoints, None]' = None, group_548: 'Union18[Group, None]' = None, match_550: 'Union18[Match, None]' = None, or_object_552: 'Union18[Or, None]' = None) -> None:
        _code_points_546: 'Union18[CodePoints, None]' = code_points_546
        _group_548: 'Union18[Group, None]' = group_548
        _match_550: 'Union18[Match, None]' = match_550
        _or_object_552: 'Union18[Or, None]' = or_object_552
        t_1272: 'CodePoints'
        t_1273: 'Group'
        t_1275: 'MappingProxyType53[str15, Group]'
        t_1276: 'Match'
        t_1277: 'Or'
        code_points_256: 'CodePoints'
        if _code_points_546 is None:
            t_1272 = CodePoints('')
            code_points_256 = t_1272
        else:
            code_points_256 = _code_points_546
        group_257: 'Group'
        if _group_548 is None:
            t_1273 = Group('', '', 0, 0)
            group_257 = t_1273
        else:
            group_257 = _group_548
        match_258: 'Match'
        if _match_550 is None:
            t_1275 = map_constructor_2611((pair_2610('', group_257),))
            t_1276 = Match(group_257, t_1275)
            match_258 = t_1276
        else:
            match_258 = _match_550
        or_object_259: 'Or'
        if _or_object_552 is None:
            t_1277 = Or(())
            or_object_259 = t_1277
        else:
            or_object_259 = _or_object_552
        this_127.code_points_251 = code_points_256
        this_127.group_252 = group_257
        this_127.match_253 = match_258
        this_127.or_object_254 = or_object_259
    @property
    def code_points(this_501) -> 'CodePoints':
        return this_501.code_points_251
    @property
    def group(this_504) -> 'Group':
        return this_504.group_252
    @property
    def match_(this_507) -> 'Match':
        return this_507.match_253
    @property
    def or_object(this_510) -> 'Or':
        return this_510.or_object_254
class Regex:
    data_260: 'RegexNode'
    compiled_279: 'Any57'
    __slots__ = ('data_260', 'compiled_279')
    def __init__(this_55, data_262: 'RegexNode') -> None:
        t_419: 'RegexNode' = data_262
        this_55.data_260 = t_419
        formatted_264: 'str15' = RegexFormatter_64.regex_format(data_262)
        t_1156: 'Any57' = regex_compile_formatted_2612(data_262, formatted_264)
        this_55.compiled_279 = t_1156
    def found(this_56, text_266: 'str15') -> 'bool13':
        return regex_compiled_found_2613(this_56, this_56.compiled_279, text_266)
    def find(this_57, text_269: 'str15', begin_554: 'Union18[int20, None]' = None) -> 'Match':
        _begin_554: 'Union18[int20, None]' = begin_554
        begin_270: 'int20'
        if _begin_554 is None:
            begin_270 = 0
        else:
            begin_270 = _begin_554
        return regex_compiled_find_2614(this_57, this_57.compiled_279, text_269, begin_270, regex_refs_162)
    def replace(this_58, text_273: 'str15', format_274: 'Callable16[[Match], str15]') -> 'str15':
        return regex_compiled_replace_2615(this_58, this_58.compiled_279, text_273, format_274, regex_refs_162)
    def split(this_59, text_277: 'str15') -> 'Sequence17[str15]':
        return regex_compiled_split_2616(this_59, this_59.compiled_279, text_277, regex_refs_162)
    @property
    def data(this_537) -> 'RegexNode':
        return this_537.data_260
class RegexFormatter_64:
    out_301: 'list2[str15]'
    __slots__ = ('out_301',)
    @staticmethod
    def regex_format(data_307: 'RegexNode') -> 'str15':
        return RegexFormatter_64().format(data_307)
    def format(this_65, regex_310: 'RegexNode') -> 'str15':
        this_65.push_regex_312(regex_310)
        return ''.join(this_65.out_301)
    def push_regex_312(this_66, regex_313: 'RegexNode') -> 'None':
        t_884: 'Capture'
        t_885: 'CodePoints'
        t_886: 'CodeRange'
        t_887: 'CodeSet'
        t_888: 'Or'
        t_889: 'Repeat'
        t_890: 'Sequence'
        if isinstance55(regex_313, Capture):
            t_884 = cast_by_type56(regex_313, Capture)
            this_66.push_capture_315(t_884)
        elif isinstance55(regex_313, CodePoints):
            t_885 = cast_by_type56(regex_313, CodePoints)
            this_66.push_code_points_333(t_885, False)
        elif isinstance55(regex_313, CodeRange):
            t_886 = cast_by_type56(regex_313, CodeRange)
            this_66.push_code_range_339(t_886)
        elif isinstance55(regex_313, CodeSet):
            t_887 = cast_by_type56(regex_313, CodeSet)
            this_66.push_code_set_345(t_887)
        elif isinstance55(regex_313, Or):
            t_888 = cast_by_type56(regex_313, Or)
            this_66.push_or_357(t_888)
        elif isinstance55(regex_313, Repeat):
            t_889 = cast_by_type56(regex_313, Repeat)
            this_66.push_repeat_361(t_889)
        elif isinstance55(regex_313, Sequence):
            t_890 = cast_by_type56(regex_313, Sequence)
            this_66.push_sequence_366(t_890)
        elif generic_eq_2618(regex_313, begin):
            this_66.out_301.append('^')
        elif generic_eq_2618(regex_313, dot):
            this_66.out_301.append('.')
        elif generic_eq_2618(regex_313, end):
            this_66.out_301.append('$')
        elif generic_eq_2618(regex_313, word_boundary):
            this_66.out_301.append('\\b')
        elif generic_eq_2618(regex_313, digit):
            this_66.out_301.append('\\d')
        elif generic_eq_2618(regex_313, space):
            this_66.out_301.append('\\s')
        elif generic_eq_2618(regex_313, word):
            this_66.out_301.append('\\w')
    def push_capture_315(this_67, capture_316: 'Capture') -> 'None':
        this_67.out_301.append('(')
        t_858: 'list2[str15]' = this_67.out_301
        t_1242: 'str15' = capture_316.name
        regex_formatter_push_capture_name_2620(this_67, t_858, t_1242)
        t_1244: 'RegexNode' = capture_316.item
        this_67.push_regex_312(t_1244)
        this_67.out_301.append(')')
    def push_code_322(this_69, code_323: 'int20', inside_code_set_324: 'bool13') -> 'None':
        t_846: 'bool13'
        t_847: 'bool13'
        t_848: 'str15'
        t_850: 'str15'
        t_851: 'bool13'
        t_852: 'bool13'
        t_853: 'bool13'
        t_854: 'bool13'
        t_855: 'str15'
        with Label21() as fn_325:
            special_escape_326: 'str15'
            if code_323 == Codes_81.carriage_return:
                special_escape_326 = 'r'
            elif code_323 == Codes_81.newline:
                special_escape_326 = 'n'
            elif code_323 == Codes_81.tab:
                special_escape_326 = 't'
            else:
                special_escape_326 = ''
            if special_escape_326 != '':
                this_69.out_301.append('\\')
                this_69.out_301.append(special_escape_326)
                fn_325.break_()
            if code_323 <= 127:
                escape_need_327: 'int20' = list_get_2621(escape_needs_163, code_323)
                if generic_eq_2618(escape_need_327, 2):
                    t_847 = True
                else:
                    if inside_code_set_324:
                        t_846 = code_323 == Codes_81.dash
                    else:
                        t_846 = False
                    t_847 = t_846
                if t_847:
                    this_69.out_301.append('\\')
                    t_848 = string_from_code_point_2622(code_323)
                    this_69.out_301.append(t_848)
                    fn_325.break_()
                elif generic_eq_2618(escape_need_327, 0):
                    t_850 = string_from_code_point_2622(code_323)
                    this_69.out_301.append(t_850)
                    fn_325.break_()
            if code_323 >= Codes_81.supplemental_min:
                t_854 = True
            else:
                if code_323 > Codes_81.high_control_max:
                    if Codes_81.surrogate_min <= code_323:
                        t_851 = code_323 <= Codes_81.surrogate_max
                    else:
                        t_851 = False
                    if t_851:
                        t_852 = True
                    else:
                        t_852 = code_323 == Codes_81.uint16_max
                    t_853 = not t_852
                else:
                    t_853 = False
                t_854 = t_853
            if t_854:
                t_855 = string_from_code_point_2622(code_323)
                this_69.out_301.append(t_855)
            else:
                regex_formatter_push_code_to_2623(this_69, this_69.out_301, code_323, inside_code_set_324)
    def push_code_points_333(this_71, code_points_334: 'CodePoints', inside_code_set_335: 'bool13') -> 'None':
        t_1229: 'int20'
        t_1231: 'int20'
        value_337: 'str15' = code_points_334.value
        index_338: 'int20' = 0
        while True:
            if not len5(value_337) > index_338:
                break
            t_1229 = string_get_2625(value_337, index_338)
            this_71.push_code_322(t_1229, inside_code_set_335)
            t_1231 = string_next_2626(value_337, index_338)
            index_338 = t_1231
    def push_code_range_339(this_72, code_range_340: 'CodeRange') -> 'None':
        this_72.out_301.append('[')
        this_72.push_code_range_unwrapped_342(code_range_340)
        this_72.out_301.append(']')
    def push_code_range_unwrapped_342(this_73, code_range_343: 'CodeRange') -> 'None':
        t_1219: 'int20' = code_range_343.min
        this_73.push_code_322(t_1219, True)
        this_73.out_301.append('-')
        t_1222: 'int20' = code_range_343.max
        this_73.push_code_322(t_1222, True)
    def push_code_set_345(this_74, code_set_346: 'CodeSet') -> 'None':
        t_1213: 'int20'
        t_1215: 'CodePart'
        t_831: 'CodeSet'
        adjusted_348: 'RegexNode' = this_74.adjust_code_set_350(code_set_346, regex_refs_162)
        if isinstance55(adjusted_348, CodeSet):
            t_831 = cast_by_type56(adjusted_348, CodeSet)
            this_74.out_301.append('[')
            if t_831.negated:
                this_74.out_301.append('^')
            i_349: 'int20' = 0
            while True:
                t_1213 = len_2627(t_831.items)
                if not i_349 < t_1213:
                    break
                t_1215 = list_get_2621(t_831.items, i_349)
                this_74.push_code_set_item_354(t_1215)
                i_349 = int_add_2628(i_349, 1)
            this_74.out_301.append(']')
        else:
            this_74.push_regex_312(adjusted_348)
    def adjust_code_set_350(this_75, code_set_351: 'CodeSet', regex_refs_352: 'RegexRefs_54') -> 'RegexNode':
        return code_set_351
    def push_code_set_item_354(this_76, code_part_355: 'CodePart') -> 'None':
        t_819: 'CodePoints'
        t_820: 'CodeRange'
        t_821: 'SpecialSet'
        if isinstance55(code_part_355, CodePoints):
            t_819 = cast_by_type56(code_part_355, CodePoints)
            this_76.push_code_points_333(t_819, True)
        elif isinstance55(code_part_355, CodeRange):
            t_820 = cast_by_type56(code_part_355, CodeRange)
            this_76.push_code_range_unwrapped_342(t_820)
        elif isinstance55(code_part_355, SpecialSet):
            t_821 = cast_by_type56(code_part_355, SpecialSet)
            this_76.push_regex_312(t_821)
    def push_or_357(this_77, or_358: 'Or') -> 'None':
        t_1192: 'RegexNode'
        t_1195: 'int20'
        t_1198: 'RegexNode'
        if not (not or_358.items):
            this_77.out_301.append('(?:')
            t_1192 = list_get_2621(or_358.items, 0)
            this_77.push_regex_312(t_1192)
            i_360: 'int20' = 1
            while True:
                t_1195 = len_2627(or_358.items)
                if not i_360 < t_1195:
                    break
                this_77.out_301.append('|')
                t_1198 = list_get_2621(or_358.items, i_360)
                this_77.push_regex_312(t_1198)
                i_360 = int_add_2628(i_360, 1)
            this_77.out_301.append(')')
    def push_repeat_361(this_78, repeat_362: 'Repeat') -> 'None':
        t_1180: 'str15'
        t_1183: 'str15'
        t_796: 'bool13'
        t_797: 'bool13'
        t_798: 'bool13'
        this_78.out_301.append('(?:')
        t_1172: 'RegexNode' = repeat_362.item
        this_78.push_regex_312(t_1172)
        this_78.out_301.append(')')
        min_364: 'int20' = repeat_362.min
        max_365: 'Union18[int20, None]' = repeat_362.max
        if min_364 == 0:
            t_796 = max_365 == 1
        else:
            t_796 = False
        if t_796:
            this_78.out_301.append('?')
        else:
            if min_364 == 0:
                t_797 = max_365 is None
            else:
                t_797 = False
            if t_797:
                this_78.out_301.append('*')
            else:
                if min_364 == 1:
                    t_798 = max_365 is None
                else:
                    t_798 = False
                if t_798:
                    this_78.out_301.append('+')
                else:
                    t_1180 = int_to_string_2630(min_364)
                    this_78.out_301.append(str_cat_2631('{', t_1180))
                    if min_364 != max_365:
                        this_78.out_301.append(',')
                        if not max_365 is None:
                            t_1183 = int_to_string_2630(max_365)
                            this_78.out_301.append(t_1183)
                    this_78.out_301.append('}')
        if repeat_362.reluctant:
            this_78.out_301.append('?')
    def push_sequence_366(this_79, sequence_367: 'Sequence') -> 'None':
        t_1167: 'int20'
        t_1169: 'RegexNode'
        i_369: 'int20' = 0
        while True:
            t_1167 = len_2627(sequence_367.items)
            if not i_369 < t_1167:
                break
            t_1169 = list_get_2621(sequence_367.items, i_369)
            this_79.push_regex_312(t_1169)
            i_369 = int_add_2628(i_369, 1)
    def max_code(this_80, code_part_371: 'CodePart') -> 'Union18[int20, None]':
        return_157: 'Union18[int20, None]'
        t_1163: 'int20'
        t_784: 'CodePoints'
        if isinstance55(code_part_371, CodePoints):
            t_784 = cast_by_type56(code_part_371, CodePoints)
            value_373: 'str15' = t_784.value
            if not value_373:
                return_157 = None
            else:
                max_374: 'int20' = 0
                index_375: 'int20' = 0
                while True:
                    if not len5(value_373) > index_375:
                        break
                    next_376: 'int20' = string_get_2625(value_373, index_375)
                    if next_376 > max_374:
                        max_374 = next_376
                    t_1163 = string_next_2626(value_373, index_375)
                    index_375 = t_1163
                return_157 = max_374
        elif isinstance55(code_part_371, CodeRange):
            return_157 = cast_by_type56(code_part_371, CodeRange).max
        elif generic_eq_2618(code_part_371, digit):
            return_157 = Codes_81.digit9
        elif generic_eq_2618(code_part_371, space):
            return_157 = Codes_81.space
        elif generic_eq_2618(code_part_371, word):
            return_157 = Codes_81.lower_z
        else:
            return_157 = None
        return return_157
    def __init__(this_138) -> None:
        t_1157: 'list2[str15]' = ['']
        this_138.out_301 = t_1157
class Codes_81:
    ampersand: ClassVar50['int20']
    backslash: ClassVar50['int20']
    caret: ClassVar50['int20']
    carriage_return: ClassVar50['int20']
    curly_left: ClassVar50['int20']
    curly_right: ClassVar50['int20']
    dash: ClassVar50['int20']
    dot: ClassVar50['int20']
    high_control_min: ClassVar50['int20']
    high_control_max: ClassVar50['int20']
    digit0: ClassVar50['int20']
    digit9: ClassVar50['int20']
    lower_a: ClassVar50['int20']
    lower_z: ClassVar50['int20']
    newline: ClassVar50['int20']
    peso: ClassVar50['int20']
    pipe: ClassVar50['int20']
    plus: ClassVar50['int20']
    question: ClassVar50['int20']
    round_left: ClassVar50['int20']
    round_right: ClassVar50['int20']
    slash: ClassVar50['int20']
    square_left: ClassVar50['int20']
    square_right: ClassVar50['int20']
    star: ClassVar50['int20']
    tab: ClassVar50['int20']
    tilde: ClassVar50['int20']
    upper_a: ClassVar50['int20']
    upper_z: ClassVar50['int20']
    space: ClassVar50['int20']
    surrogate_min: ClassVar50['int20']
    surrogate_max: ClassVar50['int20']
    supplemental_min: ClassVar50['int20']
    uint16_max: ClassVar50['int20']
    underscore: ClassVar50['int20']
    __slots__ = ()
    def __init__(this_159) -> None:
        pass
Codes_81.ampersand = 38
Codes_81.backslash = 92
Codes_81.caret = 94
Codes_81.carriage_return = 13
Codes_81.curly_left = 123
Codes_81.curly_right = 125
Codes_81.dash = 45
Codes_81.dot = 46
Codes_81.high_control_min = 127
Codes_81.high_control_max = 159
Codes_81.digit0 = 48
Codes_81.digit9 = 57
Codes_81.lower_a = 97
Codes_81.lower_z = 122
Codes_81.newline = 10
Codes_81.peso = 36
Codes_81.pipe = 124
Codes_81.plus = 43
Codes_81.question = 63
Codes_81.round_left = 40
Codes_81.round_right = 41
Codes_81.slash = 47
Codes_81.square_left = 91
Codes_81.square_right = 93
Codes_81.star = 42
Codes_81.tab = 9
Codes_81.tilde = 42
Codes_81.upper_a = 65
Codes_81.upper_z = 90
Codes_81.space = 32
Codes_81.surrogate_min = 55296
Codes_81.surrogate_max = 57343
Codes_81.supplemental_min = 65536
Codes_81.uint16_max = 65535
Codes_81.underscore = 95
class Begin_47(Special):
    __slots__ = ()
    def __init__(this_91) -> None:
        pass
return_190: 'Special' = Begin_47()
begin: 'Special' = return_190
class Dot_48(Special):
    __slots__ = ()
    def __init__(this_93) -> None:
        pass
return_192: 'Special' = Dot_48()
dot: 'Special' = return_192
class End_49(Special):
    __slots__ = ()
    def __init__(this_95) -> None:
        pass
return_194: 'Special' = End_49()
end: 'Special' = return_194
class WordBoundary_50(Special):
    __slots__ = ()
    def __init__(this_97) -> None:
        pass
return_196: 'Special' = WordBoundary_50()
word_boundary: 'Special' = return_196
class Digit_51(SpecialSet):
    __slots__ = ()
    def __init__(this_99) -> None:
        pass
return_198: 'SpecialSet' = Digit_51()
digit: 'SpecialSet' = return_198
class Space_52(SpecialSet):
    __slots__ = ()
    def __init__(this_101) -> None:
        pass
return_200: 'SpecialSet' = Space_52()
space: 'SpecialSet' = return_200
class Word_53(SpecialSet):
    __slots__ = ()
    def __init__(this_103) -> None:
        pass
return_202: 'SpecialSet' = Word_53()
word: 'SpecialSet' = return_202
def build_escape_needs_161() -> 'Sequence17[int20]':
    t_925: 'bool13'
    t_926: 'bool13'
    t_927: 'bool13'
    t_928: 'bool13'
    t_929: 'bool13'
    t_930: 'bool13'
    t_931: 'bool13'
    t_932: 'bool13'
    t_933: 'bool13'
    t_934: 'bool13'
    t_935: 'bool13'
    t_936: 'bool13'
    t_937: 'bool13'
    t_938: 'bool13'
    t_939: 'bool13'
    t_940: 'bool13'
    t_941: 'bool13'
    t_942: 'bool13'
    t_943: 'bool13'
    t_944: 'bool13'
    t_945: 'bool13'
    t_946: 'bool13'
    t_947: 'bool13'
    t_948: 'bool13'
    t_949: 'int20'
    escape_needs_379: 'MutableSequence14[int20]' = list_2633()
    code_380: 'int20' = 0
    while code_380 <= 127:
        if code_380 == Codes_81.dash:
            t_932 = True
        else:
            if code_380 == Codes_81.space:
                t_931 = True
            else:
                if code_380 == Codes_81.underscore:
                    t_930 = True
                else:
                    if Codes_81.digit0 <= code_380:
                        t_925 = code_380 <= Codes_81.digit9
                    else:
                        t_925 = False
                    if t_925:
                        t_929 = True
                    else:
                        if Codes_81.upper_a <= code_380:
                            t_926 = code_380 <= Codes_81.upper_z
                        else:
                            t_926 = False
                        if t_926:
                            t_928 = True
                        else:
                            if Codes_81.lower_a <= code_380:
                                t_927 = code_380 <= Codes_81.lower_z
                            else:
                                t_927 = False
                            t_928 = t_927
                        t_929 = t_928
                    t_930 = t_929
                t_931 = t_930
            t_932 = t_931
        if t_932:
            t_949 = 0
        else:
            if code_380 == Codes_81.ampersand:
                t_948 = True
            else:
                if code_380 == Codes_81.backslash:
                    t_947 = True
                else:
                    if code_380 == Codes_81.caret:
                        t_946 = True
                    else:
                        if code_380 == Codes_81.curly_left:
                            t_945 = True
                        else:
                            if code_380 == Codes_81.curly_right:
                                t_944 = True
                            else:
                                if code_380 == Codes_81.dot:
                                    t_943 = True
                                else:
                                    if code_380 == Codes_81.peso:
                                        t_942 = True
                                    else:
                                        if code_380 == Codes_81.pipe:
                                            t_941 = True
                                        else:
                                            if code_380 == Codes_81.plus:
                                                t_940 = True
                                            else:
                                                if code_380 == Codes_81.question:
                                                    t_939 = True
                                                else:
                                                    if code_380 == Codes_81.round_left:
                                                        t_938 = True
                                                    else:
                                                        if code_380 == Codes_81.round_right:
                                                            t_937 = True
                                                        else:
                                                            if code_380 == Codes_81.slash:
                                                                t_936 = True
                                                            else:
                                                                if code_380 == Codes_81.square_left:
                                                                    t_935 = True
                                                                else:
                                                                    if code_380 == Codes_81.square_right:
                                                                        t_934 = True
                                                                    else:
                                                                        if code_380 == Codes_81.star:
                                                                            t_933 = True
                                                                        else:
                                                                            t_933 = code_380 == Codes_81.tilde
                                                                        t_934 = t_933
                                                                    t_935 = t_934
                                                                t_936 = t_935
                                                            t_937 = t_936
                                                        t_938 = t_937
                                                    t_939 = t_938
                                                t_940 = t_939
                                            t_941 = t_940
                                        t_942 = t_941
                                    t_943 = t_942
                                t_944 = t_943
                            t_945 = t_944
                        t_946 = t_945
                    t_947 = t_946
                t_948 = t_947
            if t_948:
                t_949 = 2
            else:
                t_949 = 1
        escape_needs_379.append(t_949)
        code_380 = int_add_2628(code_380, 1)
    return tuple_2635(escape_needs_379)
return_990: 'Sequence17[int20]' = build_escape_needs_161()
escape_needs_163: 'Sequence17[int20]' = return_990
return_991: 'RegexRefs_54' = RegexRefs_54()
regex_refs_162: 'RegexRefs_54' = return_991
def entire(item_226: 'RegexNode') -> 'RegexNode':
    return Sequence((begin, item_226, end))
def one_or_more(item_228: 'RegexNode', reluctant_556: 'Union18[bool13, None]' = None) -> 'Repeat':
    _reluctant_556: 'Union18[bool13, None]' = reluctant_556
    reluctant_229: 'bool13'
    if _reluctant_556 is None:
        reluctant_229 = False
    else:
        reluctant_229 = _reluctant_556
    return Repeat(item_228, 1, None, reluctant_229)
def optional(item_231: 'RegexNode', reluctant_558: 'Union18[bool13, None]' = None) -> 'Repeat':
    _reluctant_558: 'Union18[bool13, None]' = reluctant_558
    reluctant_232: 'bool13'
    if _reluctant_558 is None:
        reluctant_232 = False
    else:
        reluctant_232 = _reluctant_558
    return Repeat(item_231, 0, 1, reluctant_232)
