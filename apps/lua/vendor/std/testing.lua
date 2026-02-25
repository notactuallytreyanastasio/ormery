local temper = require('temper-core');
local Test, processTestCases, escapeXml__41, reportTestResults, runTestCases, runTest, exports;
Test = temper.type('Test');
Test.methods.softFailToHard = function(this__15)
  if this__15.hasUnhandledFail then
    this__15._failedOnAssert__65 = true;
    temper.test_bail(this__15);
  end
  return nil;
end;
Test.get.hasUnhandledFail = function(this__20)
  local t_0;
  if this__20._failedOnAssert__65 then
    t_0 = true;
  else
    t_0 = this__20._passing__66;
  end
  return not t_0;
end;
Test.methods.messagesCombined = function(this__21)
  local return__35;
  if temper.listed_isempty(this__21._messages__67) then
    return__35 = temper.null;
  else
    local fn__399;
    fn__399 = function(it__64)
      return it__64;
    end;
    return__35 = temper.listed_join(this__21._messages__67, ', ', fn__399);
  end
  return return__35;
end;
Test.constructor = function(this__25)
  local t_1;
  this__25._failedOnAssert__65 = false;
  this__25._passing__66 = true;
  t_1 = temper.listbuilder_constructor();
  this__25._messages__67 = t_1;
  return nil;
end;
processTestCases = function(testCases__69)
  local fn__395;
  fn__395 = function(testCase__71)
    local t_2, t_3, t_4, t_5, key__73, fun__74, test__75, hadBubble__76, local_6, local_7, local_8, messages__77, failures__78;
    key__73 = testCase__71.key;
    fun__74 = testCase__71.value;
    test__75 = Test();
    hadBubble__76 = false;
    local_6, local_7, local_8 = temper.pcall(function()
      fun__74(test__75);
    end);
    if local_6 then
    else
      hadBubble__76 = true;
    end
    messages__77 = temper.test_messages(test__75);
    if temper.test_passing(test__75) then
      t_4 = not hadBubble__76;
    else
      t_4 = false;
    end
    if t_4 then
      failures__78 = temper.listof();
    else
      if hadBubble__76 then
        t_2 = temper.test_failedonassert(test__75);
        t_5 = not t_2;
      else
        t_5 = false;
      end
      if t_5 then
        local allMessages__79;
        allMessages__79 = temper.list_tolistbuilder(messages__77);
        temper.listbuilder_add(allMessages__79, 'Bubble');
        t_3 = temper.listbuilder_tolist(allMessages__79);
        failures__78 = t_3;
      else
        failures__78 = messages__77;
      end
    end
    return temper.pair_constructor(key__73, failures__78);
  end;
  return temper.listed_map(testCases__69, fn__395);
end;
escapeXml__41 = function(s__103)
  local return__40, t_10, t_11, t_12, t_13, t_14, t_15, t_16, t_17, sb__105, end__106, emitted__107, i__108;
  sb__105 = temper.stringbuilder_constructor();
  end__106 = temper.string_end(s__103);
  emitted__107 = 1.0;
  i__108 = 1.0;
  while temper.stringindexoption_compareto_lt(i__108, end__106) do
    local c__109, esc__110;
    ::continue_1::c__109 = temper.string_get(s__103, i__108);
    if (c__109 == 38) then
      t_17 = '&amp;';
    elseif (c__109 == 60) then
      t_17 = '&lt;';
    elseif (c__109 == 62) then
      t_17 = '&gt;';
    elseif (c__109 == 39) then
      t_17 = '&#39;';
    elseif (c__109 == 34) then
      t_17 = '&#34;';
    else
      if (c__109 == 10) then
        t_13 = true;
      else
        if (c__109 == 13) then
          t_12 = true;
        else
          t_12 = (c__109 == 9);
        end
        t_13 = t_12;
      end
      if t_13 then
        goto break_0;
      else
        if (c__109 < 32) then
          t_15 = true;
        else
          if (c__109 == 65534) then
            t_14 = true;
          else
            t_14 = (c__109 == 65535);
          end
          t_15 = t_14;
        end
        if t_15 then
          t_16 = temper.concat('[0x', temper.int32_tostring(c__109, 16), ']');
        else
          goto break_0;
        end
        t_17 = t_16;
      end
    end
    esc__110 = t_17;
    temper.stringbuilder_appendbetween(sb__105, s__103, emitted__107, i__108);
    temper.stringbuilder_append(sb__105, esc__110);
    t_10 = temper.string_next(s__103, i__108);
    emitted__107 = t_10;
    ::break_0::t_11 = temper.string_next(s__103, i__108);
    i__108 = t_11;
  end
  if temper.stringindexoption_compareto_eq(emitted__107, 1.0) then
    return__40 = s__103;
  else
    temper.stringbuilder_appendbetween(sb__105, s__103, emitted__107, end__106);
    return__40 = temper.stringbuilder_tostring(sb__105);
  end
  return return__40;
end;
reportTestResults = function(testResults__80, writeLine__81)
  local t_18, t_19, t_20, total__83, fn__352, fails__84, totals__88, i__89;
  writeLine__81('<testsuites>');
  total__83 = temper.int32_tostring(temper.list_length(testResults__80));
  fn__352 = function(fails__85, testResult__86)
    local t_21;
    if temper.listed_isempty(testResult__86.value) then
      t_21 = 0;
    else
      t_21 = 1;
    end
    return temper.int32_add(fails__85, t_21);
  end;
  fails__84 = temper.int32_tostring(temper.listed_reducefrom(testResults__80, 0, fn__352));
  totals__88 = temper.concat("tests='", total__83, "' failures='", fails__84, "'");
  writeLine__81(temper.concat("  <testsuite name='suite' ", totals__88, " time='0.0'>"));
  i__89 = 0;
  while true do
    local testResult__90, failureMessages__91, name__92, basics__93;
    t_18 = temper.list_length(testResults__80);
    if not (i__89 < t_18) then
      break;
    end
    testResult__90 = temper.list_get(testResults__80, i__89);
    failureMessages__91 = testResult__90.value;
    t_19 = testResult__90.key;
    name__92 = escapeXml__41(t_19);
    basics__93 = temper.concat("name='", name__92, "' classname='", name__92, "' time='0.0'");
    if temper.listed_isempty(failureMessages__91) then
      writeLine__81(temper.concat('    <testcase ', basics__93, ' />'));
    else
      local fn__351, message__94;
      writeLine__81(temper.concat('    <testcase ', basics__93, '>'));
      fn__351 = function(it__95)
        return it__95;
      end;
      t_20 = temper.listed_join(failureMessages__91, ', ', fn__351);
      message__94 = escapeXml__41(t_20);
      writeLine__81(temper.concat("      <failure message='", message__94, "' />"));
      writeLine__81('    </testcase>');
    end
    i__89 = temper.int32_add(i__89, 1);
  end
  writeLine__81('  </testsuite>');
  writeLine__81('</testsuites>');
  return nil;
end;
runTestCases = function(testCases__96)
  local report__98, t_22, fn__343;
  report__98 = temper.stringbuilder_constructor();
  t_22 = temper._processtestcases(testCases__96);
  fn__343 = function(line__99)
    temper.stringbuilder_append(report__98, line__99);
    temper.stringbuilder_append(report__98, '\n');
    return nil;
  end;
  temper._reporttestresults(t_22, fn__343);
  return temper.stringbuilder_tostring(report__98);
end;
runTest = function(testFun__100)
  local test__102, local_23, local_24, local_25;
  test__102 = Test();
  local_23, local_24, local_25 = temper.pcall(function()
    testFun__100(test__102);
  end);
  if local_23 then
  else
    local fn__337;
    fn__337 = function()
      return 'bubble during test running';
    end;
    temper.test_assert(test__102, false, fn__337);
  end
  test__102:softFailToHard();
  return nil;
end;
exports = {};
exports.Test = Test;
exports.processTestCases = processTestCases;
exports.reportTestResults = reportTestResults;
exports.runTestCases = runTestCases;
exports.runTest = runTest;
return exports;
