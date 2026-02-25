local temper = require('temper-core');
local RegexNode, Capture, CodePart, CodePoints, Special, SpecialSet, CodeRange, CodeSet, Or, Repeat, Sequence, Match, Group, RegexRefs__54, Regex, RegexFormatter__64, Codes__81, Begin__47, return__190, Begin, Dot__48, return__192, Dot, End__49, return__194, End, WordBoundary__50, return__196, WordBoundary, Digit__51, return__198, Digit, Space__52, return__200, Space, Word__53, return__202, Word, buildEscapeNeeds__161, return__990, escapeNeeds__163, return__991, regexRefs__162, entire, oneOrMore, optional, exports;
RegexNode = temper.type('RegexNode');
RegexNode.methods.compiled = function(this__42)
  return Regex(this__42);
end;
RegexNode.methods.found = function(this__43, text__170)
  return this__43:compiled():found(text__170);
end;
RegexNode.methods.find = function(this__44, text__173)
  return this__44:compiled():find(text__173);
end;
RegexNode.methods.replace = function(this__45, text__176, format__177)
  return this__45:compiled():replace(text__176, format__177);
end;
RegexNode.methods.split = function(this__46, text__180)
  return this__46:compiled():split(text__180);
end;
Capture = temper.type('Capture', RegexNode);
Capture.constructor = function(this__87, name__185, item__186)
  this__87.name__182 = name__185;
  this__87.item__183 = item__186;
  return nil;
end;
Capture.get.name = function(this__444)
  return this__444.name__182;
end;
Capture.get.item = function(this__447)
  return this__447.item__183;
end;
CodePart = temper.type('CodePart', RegexNode);
CodePoints = temper.type('CodePoints', CodePart);
CodePoints.constructor = function(this__89, value__189)
  this__89.value__187 = value__189;
  return nil;
end;
CodePoints.get.value = function(this__450)
  return this__450.value__187;
end;
Special = temper.type('Special', RegexNode);
SpecialSet = temper.type('SpecialSet', CodePart, Special);
CodeRange = temper.type('CodeRange', CodePart);
CodeRange.constructor = function(this__105, min__207, max__208)
  this__105.min__204 = min__207;
  this__105.max__205 = max__208;
  return nil;
end;
CodeRange.get.min = function(this__453)
  return this__453.min__204;
end;
CodeRange.get.max = function(this__456)
  return this__456.max__205;
end;
CodeSet = temper.type('CodeSet', RegexNode);
CodeSet.constructor = function(this__107, items__212, negated__542)
  local negated__213;
  if temper.is_null(negated__542) then
    negated__213 = false;
  else
    negated__213 = negated__542;
  end
  this__107.items__209 = items__212;
  this__107.negated__210 = negated__213;
  return nil;
end;
CodeSet.get.items = function(this__459)
  return this__459.items__209;
end;
CodeSet.get.negated = function(this__462)
  return this__462.negated__210;
end;
Or = temper.type('Or', RegexNode);
Or.constructor = function(this__110, items__216)
  this__110.items__214 = items__216;
  return nil;
end;
Or.get.items = function(this__465)
  return this__465.items__214;
end;
Repeat = temper.type('Repeat', RegexNode);
Repeat.constructor = function(this__113, item__222, min__223, max__224, reluctant__544)
  local reluctant__225;
  if (max__224 == nil) then
    max__224 = temper.null;
  end
  if temper.is_null(reluctant__544) then
    reluctant__225 = false;
  else
    reluctant__225 = reluctant__544;
  end
  this__113.item__217 = item__222;
  this__113.min__218 = min__223;
  this__113.max__219 = max__224;
  this__113.reluctant__220 = reluctant__225;
  return nil;
end;
Repeat.get.item = function(this__468)
  return this__468.item__217;
end;
Repeat.get.min = function(this__471)
  return this__471.min__218;
end;
Repeat.get.max = function(this__474)
  return this__474.max__219;
end;
Repeat.get.reluctant = function(this__477)
  return this__477.reluctant__220;
end;
Sequence = temper.type('Sequence', RegexNode);
Sequence.constructor = function(this__119, items__236)
  this__119.items__234 = items__236;
  return nil;
end;
Sequence.get.items = function(this__480)
  return this__480.items__234;
end;
Match = temper.type('Match');
Match.constructor = function(this__122, full__240, groups__241)
  this__122.full__237 = full__240;
  this__122.groups__238 = groups__241;
  return nil;
end;
Match.get.full = function(this__495)
  return this__495.full__237;
end;
Match.get.groups = function(this__498)
  return this__498.groups__238;
end;
Group = temper.type('Group');
Group.constructor = function(this__125, name__247, value__248, begin__249, end__250)
  this__125.name__242 = name__247;
  this__125.value__243 = value__248;
  this__125.begin__244 = begin__249;
  this__125.end__245 = end__250;
  return nil;
end;
Group.get.name = function(this__483)
  return this__483.name__242;
end;
Group.get.value = function(this__486)
  return this__486.value__243;
end;
Group.get.begin = function(this__489)
  return this__489.begin__244;
end;
Group.get.end_ = function(this__492)
  return this__492.end__245;
end;
RegexRefs__54 = temper.type('RegexRefs__54');
RegexRefs__54.constructor = function(this__127, codePoints__546, group__548, match__550, orObject__552)
  local t_276, t_277, t_278, t_279, t_280, codePoints__256, group__257, match__258, orObject__259;
  if temper.is_null(codePoints__546) then
    t_276 = CodePoints('');
    codePoints__256 = t_276;
  else
    codePoints__256 = codePoints__546;
  end
  if temper.is_null(group__548) then
    t_277 = Group('', '', 1.0, 1.0);
    group__257 = t_277;
  else
    group__257 = group__548;
  end
  if temper.is_null(match__550) then
    t_278 = temper.map_constructor(temper.listof(temper.pair_constructor('', group__257)));
    t_279 = Match(group__257, t_278);
    match__258 = t_279;
  else
    match__258 = match__550;
  end
  if temper.is_null(orObject__552) then
    t_280 = Or(temper.listof());
    orObject__259 = t_280;
  else
    orObject__259 = orObject__552;
  end
  this__127.codePoints__251 = codePoints__256;
  this__127.group__252 = group__257;
  this__127.match__253 = match__258;
  this__127.orObject__254 = orObject__259;
  return nil;
end;
RegexRefs__54.get.codePoints = function(this__501)
  return this__501.codePoints__251;
end;
RegexRefs__54.get.group = function(this__504)
  return this__504.group__252;
end;
RegexRefs__54.get.match = function(this__507)
  return this__507.match__253;
end;
RegexRefs__54.get.orObject = function(this__510)
  return this__510.orObject__254;
end;
Regex = temper.type('Regex');
Regex.constructor = function(this__55, data__262)
  local t_281, formatted__264, t_282;
  t_281 = data__262;
  this__55.data__260 = t_281;
  formatted__264 = temper.regex_format(data__262);
  t_282 = temper.regex_compileformatted(data__262, formatted__264);
  this__55.compiled__279 = t_282;
  return nil;
end;
Regex.methods.found = function(this__56, text__266)
  return temper.regex_compiledfound(this__56, this__56.compiled__279, text__266);
end;
Regex.methods.find = function(this__57, text__269, begin__554)
  local begin__270;
  if temper.is_null(begin__554) then
    begin__270 = 1.0;
  else
    begin__270 = begin__554;
  end
  return temper.regex_compiledfind(this__57, this__57.compiled__279, text__269, begin__270, regexRefs__162);
end;
Regex.methods.replace = function(this__58, text__273, format__274)
  return temper.regex_compiledreplace(this__58, this__58.compiled__279, text__273, format__274, regexRefs__162);
end;
Regex.methods.split = function(this__59, text__277)
  return temper.regex_compiledsplit(this__59, this__59.compiled__279, text__277, regexRefs__162);
end;
Regex.get.data = function(this__537)
  return this__537.data__260;
end;
RegexFormatter__64 = temper.type('RegexFormatter__64');
RegexFormatter__64.methods.format = function(this__65, regex__310)
  this__65:pushRegex(regex__310);
  return temper.stringbuilder_tostring(this__65.out__301);
end;
RegexFormatter__64.methods.pushRegex = function(this__66, regex__313)
  local t_283, t_284, t_285, t_286, t_287, t_288, t_289;
  if temper.instance_of(regex__313, Capture) then
    t_283 = temper.cast_to(regex__313, Capture);
    this__66:pushCapture(t_283);
  elseif temper.instance_of(regex__313, CodePoints) then
    t_284 = temper.cast_to(regex__313, CodePoints);
    this__66:pushCodePoints(t_284, false);
  elseif temper.instance_of(regex__313, CodeRange) then
    t_285 = temper.cast_to(regex__313, CodeRange);
    this__66:pushCodeRange(t_285);
  elseif temper.instance_of(regex__313, CodeSet) then
    t_286 = temper.cast_to(regex__313, CodeSet);
    this__66:pushCodeSet(t_286);
  elseif temper.instance_of(regex__313, Or) then
    t_287 = temper.cast_to(regex__313, Or);
    this__66:pushOr(t_287);
  elseif temper.instance_of(regex__313, Repeat) then
    t_288 = temper.cast_to(regex__313, Repeat);
    this__66:pushRepeat(t_288);
  elseif temper.instance_of(regex__313, Sequence) then
    t_289 = temper.cast_to(regex__313, Sequence);
    this__66:pushSequence(t_289);
  elseif temper.generic_eq(regex__313, Begin) then
    temper.stringbuilder_append(this__66.out__301, '^');
  elseif temper.generic_eq(regex__313, Dot) then
    temper.stringbuilder_append(this__66.out__301, '.');
  elseif temper.generic_eq(regex__313, End) then
    temper.stringbuilder_append(this__66.out__301, '$');
  elseif temper.generic_eq(regex__313, WordBoundary) then
    temper.stringbuilder_append(this__66.out__301, '\\b');
  elseif temper.generic_eq(regex__313, Digit) then
    temper.stringbuilder_append(this__66.out__301, '\\d');
  elseif temper.generic_eq(regex__313, Space) then
    temper.stringbuilder_append(this__66.out__301, '\\s');
  elseif temper.generic_eq(regex__313, Word) then
    temper.stringbuilder_append(this__66.out__301, '\\w');
  end
  return nil;
end;
RegexFormatter__64.methods.pushCapture = function(this__67, capture__316)
  local t_290, t_291, t_292;
  temper.stringbuilder_append(this__67.out__301, '(');
  t_290 = this__67.out__301;
  t_291 = capture__316.name;
  temper.regexformatter_pushcapturename(this__67, t_290, t_291);
  t_292 = capture__316.item;
  this__67:pushRegex(t_292);
  temper.stringbuilder_append(this__67.out__301, ')');
  return nil;
end;
RegexFormatter__64.methods.pushCode = function(this__69, code__323, insideCodeSet__324)
  local return__146, t_293, t_294, t_295, t_296, t_297, t_298, t_299, t_300, t_301, local_302, local_303, local_304;
  ::continue_25::local_302, local_303, local_304 = temper.pcall(function()
    local specialEscape__326;
    if (code__323 == Codes__81.carriageReturn) then
      specialEscape__326 = 'r';
    elseif (code__323 == Codes__81.newline) then
      specialEscape__326 = 'n';
    elseif (code__323 == Codes__81.tab) then
      specialEscape__326 = 't';
    else
      specialEscape__326 = '';
    end
    if temper.str_ne(specialEscape__326, '') then
      temper.stringbuilder_append(this__69.out__301, '\\');
      temper.stringbuilder_append(this__69.out__301, specialEscape__326);
      return__146 = nil;
      return 'break_24', 'flow';
    end
    if (code__323 <= 127) then
      local escapeNeed__327;
      escapeNeed__327 = temper.list_get(escapeNeeds__163, code__323);
      if temper.generic_eq(escapeNeed__327, 2) then
        t_294 = true;
      else
        if insideCodeSet__324 then
          t_293 = (code__323 == Codes__81.dash);
        else
          t_293 = false;
        end
        t_294 = t_293;
      end
      if t_294 then
        temper.stringbuilder_append(this__69.out__301, '\\');
        t_295 = temper.string_fromcodepoint(code__323);
        temper.stringbuilder_append(this__69.out__301, t_295);
        return__146 = nil;
        return 'break_24', 'flow';
      elseif temper.generic_eq(escapeNeed__327, 0) then
        t_296 = temper.string_fromcodepoint(code__323);
        temper.stringbuilder_append(this__69.out__301, t_296);
        return__146 = nil;
        return 'break_24', 'flow';
      end
    end
    if (code__323 >= Codes__81.supplementalMin) then
      t_300 = true;
    else
      if (code__323 > Codes__81.highControlMax) then
        if (Codes__81.surrogateMin <= code__323) then
          t_297 = (code__323 <= Codes__81.surrogateMax);
        else
          t_297 = false;
        end
        if t_297 then
          t_298 = true;
        else
          t_298 = (code__323 == Codes__81.uint16Max);
        end
        t_299 = not t_298;
      else
        t_299 = false;
      end
      t_300 = t_299;
    end
    if t_300 then
      t_301 = temper.string_fromcodepoint(code__323);
      temper.stringbuilder_append(this__69.out__301, t_301);
    else
      temper.regexformatter_pushcodeto(this__69, this__69.out__301, code__323, insideCodeSet__324);
    end
  end);
  if local_302 then
    if (local_304 == 'flow') then
      if (local_303 == nil) then
      elseif (local_303 == 'break_24') then
        goto break_24;
      end
    end
  else
    temper.bubble();
  end
  return__146 = nil;
  ::break_24::return return__146;
end;
RegexFormatter__64.methods.pushCodePoints = function(this__71, codePoints__334, insideCodeSet__335)
  local t_306, t_307, value__337, index__338;
  value__337 = codePoints__334.value;
  index__338 = 1.0;
  while true do
    if not temper.string_hasindex(value__337, index__338) then
      break;
    end
    t_306 = temper.string_get(value__337, index__338);
    this__71:pushCode(t_306, insideCodeSet__335);
    t_307 = temper.string_next(value__337, index__338);
    index__338 = t_307;
  end
  return nil;
end;
RegexFormatter__64.methods.pushCodeRange = function(this__72, codeRange__340)
  temper.stringbuilder_append(this__72.out__301, '[');
  this__72:pushCodeRangeUnwrapped(codeRange__340);
  temper.stringbuilder_append(this__72.out__301, ']');
  return nil;
end;
RegexFormatter__64.methods.pushCodeRangeUnwrapped = function(this__73, codeRange__343)
  local t_308, t_309;
  t_308 = codeRange__343.min;
  this__73:pushCode(t_308, true);
  temper.stringbuilder_append(this__73.out__301, '-');
  t_309 = codeRange__343.max;
  this__73:pushCode(t_309, true);
  return nil;
end;
RegexFormatter__64.methods.pushCodeSet = function(this__74, codeSet__346)
  local t_310, t_311, t_312, adjusted__348;
  adjusted__348 = temper.regexformatter_adjustcodeset(this__74, codeSet__346, regexRefs__162);
  if temper.instance_of(adjusted__348, CodeSet) then
    local i__349;
    t_312 = temper.cast_to(adjusted__348, CodeSet);
    temper.stringbuilder_append(this__74.out__301, '[');
    if t_312.negated then
      temper.stringbuilder_append(this__74.out__301, '^');
    end
    i__349 = 0;
    while true do
      t_310 = temper.list_length(t_312.items);
      if not (i__349 < t_310) then
        break;
      end
      t_311 = temper.list_get(t_312.items, i__349);
      this__74:pushCodeSetItem(t_311);
      i__349 = temper.int32_add(i__349, 1);
    end
    temper.stringbuilder_append(this__74.out__301, ']');
  else
    this__74:pushRegex(adjusted__348);
  end
  return nil;
end;
RegexFormatter__64.methods.pushCodeSetItem = function(this__76, codePart__355)
  local t_313, t_314, t_315;
  if temper.instance_of(codePart__355, CodePoints) then
    t_313 = temper.cast_to(codePart__355, CodePoints);
    this__76:pushCodePoints(t_313, true);
  elseif temper.instance_of(codePart__355, CodeRange) then
    t_314 = temper.cast_to(codePart__355, CodeRange);
    this__76:pushCodeRangeUnwrapped(t_314);
  elseif temper.instance_of(codePart__355, SpecialSet) then
    t_315 = temper.cast_to(codePart__355, SpecialSet);
    this__76:pushRegex(t_315);
  end
  return nil;
end;
RegexFormatter__64.methods.pushOr = function(this__77, or__358)
  local t_316, t_317, t_318;
  if not temper.listed_isempty(or__358.items) then
    local i__360;
    temper.stringbuilder_append(this__77.out__301, '(?:');
    t_316 = temper.list_get(or__358.items, 0);
    this__77:pushRegex(t_316);
    i__360 = 1;
    while true do
      t_317 = temper.list_length(or__358.items);
      if not (i__360 < t_317) then
        break;
      end
      temper.stringbuilder_append(this__77.out__301, '|');
      t_318 = temper.list_get(or__358.items, i__360);
      this__77:pushRegex(t_318);
      i__360 = temper.int32_add(i__360, 1);
    end
    temper.stringbuilder_append(this__77.out__301, ')');
  end
  return nil;
end;
RegexFormatter__64.methods.pushRepeat = function(this__78, repeat__362)
  local t_319, t_320, t_321, t_322, t_323, t_324, min__364, max__365;
  temper.stringbuilder_append(this__78.out__301, '(?:');
  t_324 = repeat__362.item;
  this__78:pushRegex(t_324);
  temper.stringbuilder_append(this__78.out__301, ')');
  min__364 = repeat__362.min;
  max__365 = repeat__362.max;
  if (min__364 == 0) then
    t_321 = (max__365 == 1);
  else
    t_321 = false;
  end
  if t_321 then
    temper.stringbuilder_append(this__78.out__301, '?');
  else
    if (min__364 == 0) then
      t_322 = temper.is_null(max__365);
    else
      t_322 = false;
    end
    if t_322 then
      temper.stringbuilder_append(this__78.out__301, '*');
    else
      if (min__364 == 1) then
        t_323 = temper.is_null(max__365);
      else
        t_323 = false;
      end
      if t_323 then
        temper.stringbuilder_append(this__78.out__301, '+');
      else
        t_319 = temper.int32_tostring(min__364);
        temper.stringbuilder_append(this__78.out__301, temper.concat('{', t_319));
        if (min__364 ~= max__365) then
          temper.stringbuilder_append(this__78.out__301, ',');
          if not temper.is_null(max__365) then
            t_320 = temper.int32_tostring(max__365);
            temper.stringbuilder_append(this__78.out__301, t_320);
          end
        end
        temper.stringbuilder_append(this__78.out__301, '}');
      end
    end
  end
  if repeat__362.reluctant then
    temper.stringbuilder_append(this__78.out__301, '?');
  end
  return nil;
end;
RegexFormatter__64.methods.pushSequence = function(this__79, sequence__367)
  local t_325, t_326, i__369;
  i__369 = 0;
  while true do
    t_325 = temper.list_length(sequence__367.items);
    if not (i__369 < t_325) then
      break;
    end
    t_326 = temper.list_get(sequence__367.items, i__369);
    this__79:pushRegex(t_326);
    i__369 = temper.int32_add(i__369, 1);
  end
  return nil;
end;
RegexFormatter__64.methods.maxCode = function(this__80, codePart__371)
  local return__157, t_327, t_328;
  if temper.instance_of(codePart__371, CodePoints) then
    local value__373;
    t_328 = temper.cast_to(codePart__371, CodePoints);
    value__373 = t_328.value;
    if temper.string_isempty(value__373) then
      return__157 = temper.null;
    else
      local max__374, index__375;
      max__374 = 0;
      index__375 = 1.0;
      while true do
        local next__376;
        if not temper.string_hasindex(value__373, index__375) then
          break;
        end
        next__376 = temper.string_get(value__373, index__375);
        if (next__376 > max__374) then
          max__374 = next__376;
        end
        t_327 = temper.string_next(value__373, index__375);
        index__375 = t_327;
      end
      return__157 = max__374;
    end
  elseif temper.instance_of(codePart__371, CodeRange) then
    return__157 = (temper.cast_to(codePart__371, CodeRange)).max;
  elseif temper.generic_eq(codePart__371, Digit) then
    return__157 = Codes__81.digit9;
  elseif temper.generic_eq(codePart__371, Space) then
    return__157 = Codes__81.space;
  elseif temper.generic_eq(codePart__371, Word) then
    return__157 = Codes__81.lowerZ;
  else
    return__157 = temper.null;
  end
  return return__157;
end;
RegexFormatter__64.constructor = function(this__138)
  local t_329;
  t_329 = temper.stringbuilder_constructor();
  this__138.out__301 = t_329;
  return nil;
end;
Codes__81 = temper.type('Codes__81');
Codes__81.constructor = function(this__159)
  return nil;
end;
Codes__81.ampersand = 38;
Codes__81.backslash = 92;
Codes__81.caret = 94;
Codes__81.carriageReturn = 13;
Codes__81.curlyLeft = 123;
Codes__81.curlyRight = 125;
Codes__81.dash = 45;
Codes__81.dot = 46;
Codes__81.highControlMin = 127;
Codes__81.highControlMax = 159;
Codes__81.digit0 = 48;
Codes__81.digit9 = 57;
Codes__81.lowerA = 97;
Codes__81.lowerZ = 122;
Codes__81.newline = 10;
Codes__81.peso = 36;
Codes__81.pipe = 124;
Codes__81.plus = 43;
Codes__81.question = 63;
Codes__81.roundLeft = 40;
Codes__81.roundRight = 41;
Codes__81.slash = 47;
Codes__81.squareLeft = 91;
Codes__81.squareRight = 93;
Codes__81.star = 42;
Codes__81.tab = 9;
Codes__81.tilde = 42;
Codes__81.upperA = 65;
Codes__81.upperZ = 90;
Codes__81.space = 32;
Codes__81.surrogateMin = 55296;
Codes__81.surrogateMax = 57343;
Codes__81.supplementalMin = 65536;
Codes__81.uint16Max = 65535;
Codes__81.underscore = 95;
Begin__47 = temper.type('Begin__47', Special);
Begin__47.constructor = function(this__91)
  return nil;
end;
return__190 = Begin__47();
Begin = return__190;
Dot__48 = temper.type('Dot__48', Special);
Dot__48.constructor = function(this__93)
  return nil;
end;
return__192 = Dot__48();
Dot = return__192;
End__49 = temper.type('End__49', Special);
End__49.constructor = function(this__95)
  return nil;
end;
return__194 = End__49();
End = return__194;
WordBoundary__50 = temper.type('WordBoundary__50', Special);
WordBoundary__50.constructor = function(this__97)
  return nil;
end;
return__196 = WordBoundary__50();
WordBoundary = return__196;
Digit__51 = temper.type('Digit__51', SpecialSet);
Digit__51.constructor = function(this__99)
  return nil;
end;
return__198 = Digit__51();
Digit = return__198;
Space__52 = temper.type('Space__52', SpecialSet);
Space__52.constructor = function(this__101)
  return nil;
end;
return__200 = Space__52();
Space = return__200;
Word__53 = temper.type('Word__53', SpecialSet);
Word__53.constructor = function(this__103)
  return nil;
end;
return__202 = Word__53();
Word = return__202;
buildEscapeNeeds__161 = function()
  local t_330, t_331, t_332, t_333, t_334, t_335, t_336, t_337, t_338, t_339, t_340, t_341, t_342, t_343, t_344, t_345, t_346, t_347, t_348, t_349, t_350, t_351, t_352, t_353, t_354, escapeNeeds__379, code__380;
  escapeNeeds__379 = temper.listbuilder_constructor();
  code__380 = 0;
  while (code__380 <= 127) do
    if (code__380 == Codes__81.dash) then
      t_337 = true;
    else
      if (code__380 == Codes__81.space) then
        t_336 = true;
      else
        if (code__380 == Codes__81.underscore) then
          t_335 = true;
        else
          if (Codes__81.digit0 <= code__380) then
            t_330 = (code__380 <= Codes__81.digit9);
          else
            t_330 = false;
          end
          if t_330 then
            t_334 = true;
          else
            if (Codes__81.upperA <= code__380) then
              t_331 = (code__380 <= Codes__81.upperZ);
            else
              t_331 = false;
            end
            if t_331 then
              t_333 = true;
            else
              if (Codes__81.lowerA <= code__380) then
                t_332 = (code__380 <= Codes__81.lowerZ);
              else
                t_332 = false;
              end
              t_333 = t_332;
            end
            t_334 = t_333;
          end
          t_335 = t_334;
        end
        t_336 = t_335;
      end
      t_337 = t_336;
    end
    if t_337 then
      t_354 = 0;
    else
      if (code__380 == Codes__81.ampersand) then
        t_353 = true;
      else
        if (code__380 == Codes__81.backslash) then
          t_352 = true;
        else
          if (code__380 == Codes__81.caret) then
            t_351 = true;
          else
            if (code__380 == Codes__81.curlyLeft) then
              t_350 = true;
            else
              if (code__380 == Codes__81.curlyRight) then
                t_349 = true;
              else
                if (code__380 == Codes__81.dot) then
                  t_348 = true;
                else
                  if (code__380 == Codes__81.peso) then
                    t_347 = true;
                  else
                    if (code__380 == Codes__81.pipe) then
                      t_346 = true;
                    else
                      if (code__380 == Codes__81.plus) then
                        t_345 = true;
                      else
                        if (code__380 == Codes__81.question) then
                          t_344 = true;
                        else
                          if (code__380 == Codes__81.roundLeft) then
                            t_343 = true;
                          else
                            if (code__380 == Codes__81.roundRight) then
                              t_342 = true;
                            else
                              if (code__380 == Codes__81.slash) then
                                t_341 = true;
                              else
                                if (code__380 == Codes__81.squareLeft) then
                                  t_340 = true;
                                else
                                  if (code__380 == Codes__81.squareRight) then
                                    t_339 = true;
                                  else
                                    if (code__380 == Codes__81.star) then
                                      t_338 = true;
                                    else
                                      t_338 = (code__380 == Codes__81.tilde);
                                    end
                                    t_339 = t_338;
                                  end
                                  t_340 = t_339;
                                end
                                t_341 = t_340;
                              end
                              t_342 = t_341;
                            end
                            t_343 = t_342;
                          end
                          t_344 = t_343;
                        end
                        t_345 = t_344;
                      end
                      t_346 = t_345;
                    end
                    t_347 = t_346;
                  end
                  t_348 = t_347;
                end
                t_349 = t_348;
              end
              t_350 = t_349;
            end
            t_351 = t_350;
          end
          t_352 = t_351;
        end
        t_353 = t_352;
      end
      if t_353 then
        t_354 = 2;
      else
        t_354 = 1;
      end
    end
    temper.listbuilder_add(escapeNeeds__379, t_354);
    code__380 = temper.int32_add(code__380, 1);
  end
  return temper.listbuilder_tolist(escapeNeeds__379);
end;
return__990 = buildEscapeNeeds__161();
escapeNeeds__163 = return__990;
return__991 = RegexRefs__54();
regexRefs__162 = return__991;
entire = function(item__226)
  return Sequence(temper.listof(Begin, item__226, End));
end;
oneOrMore = function(item__228, reluctant__556)
  local reluctant__229;
  if temper.is_null(reluctant__556) then
    reluctant__229 = false;
  else
    reluctant__229 = reluctant__556;
  end
  return Repeat(item__228, 1, temper.null, reluctant__229);
end;
optional = function(item__231, reluctant__558)
  local reluctant__232;
  if temper.is_null(reluctant__558) then
    reluctant__232 = false;
  else
    reluctant__232 = reluctant__558;
  end
  return Repeat(item__231, 0, 1, reluctant__232);
end;
exports = {};
exports.RegexNode = RegexNode;
exports.Capture = Capture;
exports.CodePart = CodePart;
exports.CodePoints = CodePoints;
exports.Special = Special;
exports.SpecialSet = SpecialSet;
exports.CodeRange = CodeRange;
exports.CodeSet = CodeSet;
exports.Or = Or;
exports.Repeat = Repeat;
exports.Sequence = Sequence;
exports.Match = Match;
exports.Group = Group;
exports.Regex = Regex;
exports.Begin = Begin;
exports.Dot = Dot;
exports.End = End;
exports.WordBoundary = WordBoundary;
exports.Digit = Digit;
exports.Space = Space;
exports.Word = Word;
exports.entire = entire;
exports.oneOrMore = oneOrMore;
exports.optional = optional;
return exports;
