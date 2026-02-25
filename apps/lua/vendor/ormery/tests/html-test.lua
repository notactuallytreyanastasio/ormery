local temper = require('temper-core');
local percentEscapeOctetTo, propagateOver, entire, oneOrMore, optional, Begin, Dot, End, WordBoundary, Digit, Space, Word, Codec, ContextPropagator, Context, Escaper, EscaperPicker, ContextualAutoescapingAccumulator, AutoescState, Delegate, ContextDelegate, CodeSet, CodePoints, Sequence, Repeat, Or, AfterPropagate, CodeRange, htmlCodec, SafeHtmlBuilder, local_732, local_733, exports;
percentEscapeOctetTo = temper.import('ormery/url', 'percentEscapeOctetTo');
propagateOver = temper.import('ormery/core', 'propagateOver');
entire = temper.import('std/regex', 'entire');
oneOrMore = temper.import('std/regex', 'oneOrMore');
optional = temper.import('std/regex', 'optional');
Begin = temper.import('std/regex', 'Begin');
Dot = temper.import('std/regex', 'Dot');
End = temper.import('std/regex', 'End');
WordBoundary = temper.import('std/regex', 'WordBoundary');
Digit = temper.import('std/regex', 'Digit');
Space = temper.import('std/regex', 'Space');
Word = temper.import('std/regex', 'Word');
Codec = temper.import('ormery/core', 'Codec');
ContextPropagator = temper.import('ormery/core', 'ContextPropagator');
Context = temper.import('ormery/core', 'Context');
Escaper = temper.import('ormery/core', 'Escaper');
EscaperPicker = temper.import('ormery/core', 'EscaperPicker');
ContextualAutoescapingAccumulator = temper.import('ormery/core', 'ContextualAutoescapingAccumulator');
AutoescState = temper.import('ormery/core', 'AutoescState');
Delegate = temper.import('ormery/core', 'Delegate');
ContextDelegate = temper.import('ormery/core', 'ContextDelegate');
CodeSet = temper.import('std/regex', 'CodeSet');
CodePoints = temper.import('std/regex', 'CodePoints');
Sequence = temper.import('std/regex', 'Sequence');
Repeat = temper.import('std/regex', 'Repeat');
Or = temper.import('std/regex', 'Or');
AfterPropagate = temper.import('ormery/core', 'AfterPropagate');
CodeRange = temper.import('std/regex', 'CodeRange');
htmlCodec = temper.import('ormery/html', 'htmlCodec');
SafeHtmlBuilder = temper.import('ormery/html', 'SafeHtmlBuilder');
local_732 = (unpack or table.unpack);
local_733 = require('luaunit');
local_733.FAILURE_PREFIX = temper.test_failure_prefix;
Test_ = {};
Test_.test_htmlDecoding__804 = function()
  temper.test('HTML decoding', function(test_212)
    local actual_213, t_214, fn__5594, actual_215, t_216, fn__5593, actual_217, t_218, fn__5592, actual_219, t_220, fn__5591, actual_221, t_222, fn__5590, actual_223, t_224, fn__5589, actual_225, t_226, fn__5588, actual_227, t_228, fn__5587, actual_229, t_230, fn__5586, actual_231, t_232, fn__5585, actual_233, t_234, fn__5584, actual_235, t_236, fn__5583;
    actual_213 = htmlCodec:decode('');
    t_214 = temper.str_eq(actual_213, '');
    fn__5594 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("") == (', '', ') not (', actual_213, ')');
    end;
    temper.test_assert(test_212, t_214, fn__5594);
    actual_215 = htmlCodec:decode('&l');
    t_216 = temper.str_eq(actual_215, '&l');
    fn__5593 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&l") == (', '&l', ') not (', actual_215, ')');
    end;
    temper.test_assert(test_212, t_216, fn__5593);
    actual_217 = htmlCodec:decode('&lt');
    t_218 = temper.str_eq(actual_217, '<');
    fn__5592 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&lt") == (', '<', ') not (', actual_217, ')');
    end;
    temper.test_assert(test_212, t_218, fn__5592);
    actual_219 = htmlCodec:decode('&lt;');
    t_220 = temper.str_eq(actual_219, '<');
    fn__5591 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&lt;") == (', '<', ') not (', actual_219, ')');
    end;
    temper.test_assert(test_212, t_220, fn__5591);
    actual_221 = htmlCodec:decode('&Bcy;');
    t_222 = temper.str_eq(actual_221, '\xd0\x91');
    fn__5590 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&Bcy;") == (', '\xd0\x91', ') not (', actual_221, ')');
    end;
    temper.test_assert(test_212, t_222, fn__5590);
    actual_223 = htmlCodec:decode('&Bcy');
    t_224 = temper.str_eq(actual_223, '&Bcy');
    fn__5589 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&Bcy") == (', '&Bcy', ') not (', actual_223, ')');
    end;
    temper.test_assert(test_212, t_224, fn__5589);
    actual_225 = htmlCodec:decode('&LT;');
    t_226 = temper.str_eq(actual_225, '<');
    fn__5588 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&LT;") == (', '<', ') not (', actual_225, ')');
    end;
    temper.test_assert(test_212, t_226, fn__5588);
    actual_227 = htmlCodec:decode('&Aacute;');
    t_228 = temper.str_eq(actual_227, '\xc3\x81');
    fn__5587 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&Aacute;") == (', '\xc3\x81', ') not (', actual_227, ')');
    end;
    temper.test_assert(test_212, t_228, fn__5587);
    actual_229 = htmlCodec:decode('&aacute;');
    t_230 = temper.str_eq(actual_229, '\xc3\xa1');
    fn__5586 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&aacute;") == (', '\xc3\xa1', ') not (', actual_229, ')');
    end;
    temper.test_assert(test_212, t_230, fn__5586);
    actual_231 = htmlCodec:decode('&AaCuTe;');
    t_232 = temper.str_eq(actual_231, '&AaCuTe;');
    fn__5585 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&AaCuTe;") == (', '&AaCuTe;', ') not (', actual_231, ')');
    end;
    temper.test_assert(test_212, t_232, fn__5585);
    actual_233 = htmlCodec:decode('&gt;;');
    t_234 = temper.str_eq(actual_233, '>;');
    fn__5584 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&gt;;") == (', '>;', ') not (', actual_233, ')');
    end;
    temper.test_assert(test_212, t_234, fn__5584);
    actual_235 = htmlCodec:decode('&amp;lt;');
    t_236 = temper.str_eq(actual_235, '&lt;');
    fn__5583 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.decode("&amp;lt;") == (', '&lt;', ') not (', actual_235, ')');
    end;
    temper.test_assert(test_212, t_236, fn__5583);
    return nil;
  end);
end;
Test_.test_htmlEncoding__829 = function()
  temper.test('HTML encoding', function(test_237)
    local actual_238, t_239, fn__5570, actual_240, t_241, fn__5569, actual_242, t_243, fn__5568;
    actual_238 = htmlCodec:encode('');
    t_239 = temper.str_eq(actual_238, '');
    fn__5570 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.encode("") == (', '', ') not (', actual_238, ')');
    end;
    temper.test_assert(test_237, t_239, fn__5570);
    actual_240 = htmlCodec:encode('Hello, World!');
    t_241 = temper.str_eq(actual_240, 'Hello, World!');
    fn__5569 = function()
      return temper.concat('expected `-work/src//html/`.htmlCodec.encode("Hello, World!") == (', 'Hello, World!', ') not (', actual_240, ')');
    end;
    temper.test_assert(test_237, t_241, fn__5569);
    actual_242 = htmlCodec:encode("<foo> & <bar baz='b\"oo'> far");
    t_243 = temper.str_eq(actual_242, '&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far');
    fn__5568 = function()
      return temper.concat("expected `-work/src//html/`.htmlCodec.encode(\"<foo> & <bar baz='b\\\"oo'> far\") == (", '&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far', ') not (', actual_242, ')');
    end;
    temper.test_assert(test_237, t_243, fn__5568);
    return nil;
  end);
end;
Test_.test_helloWorldHtmlStyle__899 = function()
  temper.test('hello world, html style', function(test_700)
    local t_701, actual_702, t_703, fn__4674;
    t_701 = SafeHtmlBuilder();
    t_701:appendSafe('Hello, <b>');
    t_701:appendString('World');
    t_701:appendSafe('</b>!');
    actual_702 = t_701.accumulated:toString();
    t_703 = temper.str_eq(actual_702, 'Hello, <b>World</b>!');
    fn__4674 = function()
      return temper.concat('expected stringExpr(`-work/src//html/`.html, true, "Hello, <b>", \\interpolate, "World", "</b>!").toString() == (', 'Hello, <b>World</b>!', ') not (', actual_702, ')');
    end;
    temper.test_assert(test_700, t_703, fn__4674);
    return nil;
  end);
end;
Test_.test_autoescaped__903 = function()
  temper.test('autoescaped', function(test_704)
    local t_705, actual_706, t_707, fn__4664;
    t_705 = SafeHtmlBuilder();
    t_705:appendSafe('1 + 1 ');
    t_705:appendString('<');
    t_705:appendSafe(' 3.');
    actual_706 = t_705.accumulated:toString();
    t_707 = temper.str_eq(actual_706, '1 + 1 &lt; 3.');
    fn__4664 = function()
      return temper.concat('expected stringExpr(`-work/src//html/`.html, true, "1 + 1 ", \\interpolate, "<", " 3.").toString() == (', '1 + 1 &lt; 3.', ') not (', actual_706, ')');
    end;
    temper.test_assert(test_704, t_707, fn__4664);
    return nil;
  end);
end;
Test_.test_contextMattersUrlsEmbed__907 = function()
  temper.test('context matters -- URLs embed', function(test_708)
    local okUrl__483, evilUrl__484, t_709, actual_710, t_711, fn__4641, t_712, actual_713, t_714, fn__4640;
    okUrl__483 = function()
      return "https://example.com/isn't-a-problem";
    end;
    evilUrl__484 = function()
      return "javascript:alert('evil done')";
    end;
    t_709 = SafeHtmlBuilder();
    t_709:appendSafe("<a href='");
    t_709:appendString("https://example.com/isn't-a-problem");
    t_709:appendSafe("'>");
    t_709:appendString("https://example.com/isn't-a-problem");
    t_709:appendSafe('</a>');
    actual_710 = t_709.accumulated:toString();
    t_711 = temper.str_eq(actual_710, "<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>");
    fn__4641 = function()
      return temper.concat("expected stringExpr(`-work/src//html/`.html, true, \"<a href='\", \\interpolate, okUrl(), \"'>\", \\interpolate, okUrl(), \"</a>\").toString() == (", "<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>", ') not (', actual_710, ')');
    end;
    temper.test_assert(test_708, t_711, fn__4641);
    t_712 = SafeHtmlBuilder();
    t_712:appendSafe("<a href='");
    t_712:appendString("javascript:alert('evil done')");
    t_712:appendSafe("'>");
    t_712:appendString("javascript:alert('evil done')");
    t_712:appendSafe('</a>');
    actual_713 = t_712.accumulated:toString();
    t_714 = temper.str_eq(actual_713, "<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>");
    fn__4640 = function()
      return temper.concat("expected stringExpr(`-work/src//html/`.html, true, \"<a href='\", \\interpolate, evilUrl(), \"'>\", \\interpolate, evilUrl(), \"</a>\").toString() == (", "<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>", ') not (', actual_713, ')');
    end;
    temper.test_assert(test_708, t_714, fn__4640);
    return nil;
  end);
end;
Test_.test_quoteAdjustments__914 = function()
  temper.test('quote adjustments', function(test_715)
    local className__488, t_716, actual_717, t_718, fn__4628;
    className__488 = 'some-class';
    t_716 = SafeHtmlBuilder();
    t_716:appendSafe('<hr class=');
    t_716:appendString('some-class');
    t_716:appendSafe("><hr class='");
    t_716:appendString('some-class');
    t_716:appendSafe("'><hr class=other-class>");
    actual_717 = t_716.accumulated:toString();
    t_718 = temper.str_eq(actual_717, "<hr class=\"some-class\"><hr class='some-class'><hr class=\"other-class\">");
    fn__4628 = function()
      return temper.concat("expected stringExpr(`-work/src//html/`.html, true, \"<hr class=\", \\interpolate, className, \"><hr class='\", \\interpolate, className, \"'><hr class=other-class>\").toString() == (", "<hr class=\"some-class\"><hr class='some-class'><hr class=\"other-class\">", ') not (', actual_717, ')');
    end;
    temper.test_assert(test_715, t_718, fn__4628);
    return nil;
  end);
end;
Test_.test_safehtmlInjectedInTagAndAttributeContext__918 = function()
  temper.test('safehtml injected in tag and attribute context', function(test_719)
    local t_720, love__490, t_721, actual_722, t_723, fn__4613;
    t_720 = SafeHtmlBuilder();
    t_720:appendSafe('I <3 <b>Ponies</b>!');
    love__490 = t_720.accumulated;
    t_721 = SafeHtmlBuilder();
    t_721:appendSafe('<b>');
    t_721:appendSafeHtml(love__490);
    t_721:appendSafe("</b><img alt='");
    t_721:appendSafeHtml(love__490);
    t_721:appendSafe("' src='ponies'>");
    actual_722 = t_721.accumulated:toString();
    t_723 = temper.str_eq(actual_722, "<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>");
    fn__4613 = function()
      return temper.concat("expected stringExpr(`-work/src//html/`.html, true, \"<b>\", \\interpolate, love, \"</b><img alt='\", \\interpolate, love, \"' src='ponies'>\").toString() == (", "<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>", ') not (', actual_722, ')');
    end;
    temper.test_assert(test_719, t_723, fn__4613);
    return nil;
  end);
end;
Test_.test_loopingInsideAnHtmlExpression__923 = function()
  temper.test('looping inside an HTML expression', function(test_724)
    local items__492, accumulator_725, fn__4602, got__493, actual_726, t_727, fn__4601;
    items__492 = temper.listof('One', '<Two>', 'Three');
    accumulator_725 = SafeHtmlBuilder();
    accumulator_725:appendSafe('<ul>\n');
    fn__4602 = function(item__495)
      accumulator_725:appendSafe('  <li>');
      accumulator_725:appendString(item__495);
      accumulator_725:appendSafe('</li>\n');
      return nil;
    end;
    temper.list_foreach(items__492, fn__4602);
    accumulator_725:appendSafe('</ul>');
    got__493 = accumulator_725.accumulated;
    actual_726 = got__493.text;
    t_727 = temper.str_eq(actual_726, '<ul>\n  <li>One</li>\n  <li>&lt;Two&gt;</li>\n  <li>Three</li>\n</ul>');
    fn__4601 = function()
      return temper.concat('expected got.text == (', '<ul>\n  <li>One</li>\n  <li>&lt;Two&gt;</li>\n  <li>Three</li>\n</ul>', ') not (', actual_726, ')');
    end;
    temper.test_assert(test_724, t_727, fn__4601);
    return nil;
  end);
end;
Test_.test_doubleQuotesInAttributeValueWithInsertedQuotes__935 = function()
  temper.test('double quotes in attribute value with inserted quotes', function(test_728)
    local t_729, actual_730, t_731, fn__4588;
    t_729 = SafeHtmlBuilder();
    t_729:appendSafe('<div id=a"b>');
    actual_730 = t_729.accumulated.text;
    t_731 = temper.str_eq(actual_730, '<div id="a&#34;b">');
    fn__4588 = function()
      return temper.concat('expected stringExpr(`-work/src//html/`.html, true, "<div id=a\\"b>").text == (', '<div id="a&#34;b">', ') not (', actual_730, ')');
    end;
    temper.test_assert(test_728, t_731, fn__4588);
    return nil;
  end);
end;
exports = {};
local_733.LuaUnit.run(local_732({'--pattern', '^Test_%.', local_732(arg)}));
return exports;
