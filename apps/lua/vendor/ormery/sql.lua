local temper = require('temper-core');
local SqlBuilder, SqlFragment, SqlPart, SqlSource, SqlBoolean, SqlDate, SqlFloat64, SqlInt32, SqlInt64, SqlString, exports;
SqlBuilder = temper.type('SqlBuilder');
SqlBuilder.methods.appendSafe = function(this__9, sqlSource__96)
  local t_0;
  t_0 = SqlSource(sqlSource__96);
  temper.listbuilder_add(this__9.buffer__94, t_0);
  return nil;
end;
SqlBuilder.methods.appendFragment = function(this__10, fragment__99)
  local t_1;
  t_1 = fragment__99.parts;
  temper.listbuilder_addall(this__10.buffer__94, t_1);
  return nil;
end;
SqlBuilder.methods.appendPart = function(this__11, part__102)
  temper.listbuilder_add(this__11.buffer__94, part__102);
  return nil;
end;
SqlBuilder.methods.appendPartList = function(this__12, values__105)
  local fn__1157;
  fn__1157 = function(x__107)
    this__12:appendPart(x__107);
    return nil;
  end;
  this__12:appendList(values__105, fn__1157);
  return nil;
end;
SqlBuilder.methods.appendBoolean = function(this__13, value__109)
  local t_2;
  t_2 = SqlBoolean(value__109);
  temper.listbuilder_add(this__13.buffer__94, t_2);
  return nil;
end;
SqlBuilder.methods.appendBooleanList = function(this__14, values__112)
  local fn__1151;
  fn__1151 = function(x__114)
    this__14:appendBoolean(x__114);
    return nil;
  end;
  this__14:appendList(values__112, fn__1151);
  return nil;
end;
SqlBuilder.methods.appendDate = function(this__15, value__116)
  local t_3;
  t_3 = SqlDate(value__116);
  temper.listbuilder_add(this__15.buffer__94, t_3);
  return nil;
end;
SqlBuilder.methods.appendDateList = function(this__16, values__119)
  local fn__1145;
  fn__1145 = function(x__121)
    this__16:appendDate(x__121);
    return nil;
  end;
  this__16:appendList(values__119, fn__1145);
  return nil;
end;
SqlBuilder.methods.appendFloat64 = function(this__17, value__123)
  local t_4;
  t_4 = SqlFloat64(value__123);
  temper.listbuilder_add(this__17.buffer__94, t_4);
  return nil;
end;
SqlBuilder.methods.appendFloat64List = function(this__18, values__126)
  local fn__1139;
  fn__1139 = function(x__128)
    this__18:appendFloat64(x__128);
    return nil;
  end;
  this__18:appendList(values__126, fn__1139);
  return nil;
end;
SqlBuilder.methods.appendInt32 = function(this__19, value__130)
  local t_5;
  t_5 = SqlInt32(value__130);
  temper.listbuilder_add(this__19.buffer__94, t_5);
  return nil;
end;
SqlBuilder.methods.appendInt32List = function(this__20, values__133)
  local fn__1133;
  fn__1133 = function(x__135)
    this__20:appendInt32(x__135);
    return nil;
  end;
  this__20:appendList(values__133, fn__1133);
  return nil;
end;
SqlBuilder.methods.appendInt64 = function(this__21, value__137)
  local t_6;
  t_6 = SqlInt64(value__137);
  temper.listbuilder_add(this__21.buffer__94, t_6);
  return nil;
end;
SqlBuilder.methods.appendInt64List = function(this__22, values__140)
  local fn__1127;
  fn__1127 = function(x__142)
    this__22:appendInt64(x__142);
    return nil;
  end;
  this__22:appendList(values__140, fn__1127);
  return nil;
end;
SqlBuilder.methods.appendString = function(this__23, value__144)
  local t_7;
  t_7 = SqlString(value__144);
  temper.listbuilder_add(this__23.buffer__94, t_7);
  return nil;
end;
SqlBuilder.methods.appendStringList = function(this__24, values__147)
  local fn__1121;
  fn__1121 = function(x__149)
    this__24:appendString(x__149);
    return nil;
  end;
  this__24:appendList(values__147, fn__1121);
  return nil;
end;
SqlBuilder.methods.appendList = function(this__25, values__151, appendValue__152)
  local t_8, t_9, i__154;
  i__154 = 0;
  while true do
    t_8 = temper.listed_length(values__151);
    if not (i__154 < t_8) then
      break;
    end
    if (i__154 > 0) then
      this__25:appendSafe(', ');
    end
    t_9 = temper.listed_get(values__151, i__154);
    appendValue__152(t_9);
    i__154 = temper.int32_add(i__154, 1);
  end
  return nil;
end;
SqlBuilder.get.accumulated = function(this__26)
  return SqlFragment(temper.listbuilder_tolist(this__26.buffer__94));
end;
SqlBuilder.constructor = function(this__42)
  local t_10;
  t_10 = temper.listbuilder_constructor();
  this__42.buffer__94 = t_10;
  return nil;
end;
SqlFragment = temper.type('SqlFragment');
SqlFragment.methods.toSource = function(this__31)
  return SqlSource(this__31:toString());
end;
SqlFragment.methods.toString = function(this__32)
  local t_11, builder__166, i__167;
  builder__166 = temper.stringbuilder_constructor();
  i__167 = 0;
  while true do
    t_11 = temper.list_length(this__32.parts__161);
    if not (i__167 < t_11) then
      break;
    end
    temper.list_get(this__32.parts__161, i__167):formatTo(builder__166);
    i__167 = temper.int32_add(i__167, 1);
  end
  return temper.stringbuilder_tostring(builder__166);
end;
SqlFragment.constructor = function(this__63, parts__169)
  this__63.parts__161 = parts__169;
  return nil;
end;
SqlFragment.get.parts = function(this__248)
  return this__248.parts__161;
end;
SqlPart = temper.type('SqlPart');
SqlPart.methods.formatTo = function(this__33, builder__171)
  temper.virtual();
end;
SqlSource = temper.type('SqlSource', SqlPart);
SqlSource.methods.formatTo = function(this__34, builder__175)
  temper.stringbuilder_append(builder__175, this__34.source__173);
  return nil;
end;
SqlSource.constructor = function(this__69, source__178)
  this__69.source__173 = source__178;
  return nil;
end;
SqlSource.get.source = function(this__245)
  return this__245.source__173;
end;
SqlBoolean = temper.type('SqlBoolean', SqlPart);
SqlBoolean.methods.formatTo = function(this__35, builder__181)
  local t_12;
  if this__35.value__179 then
    t_12 = 'TRUE';
  else
    t_12 = 'FALSE';
  end
  temper.stringbuilder_append(builder__181, t_12);
  return nil;
end;
SqlBoolean.constructor = function(this__72, value__184)
  this__72.value__179 = value__184;
  return nil;
end;
SqlBoolean.get.value = function(this__251)
  return this__251.value__179;
end;
SqlDate = temper.type('SqlDate', SqlPart);
SqlDate.methods.formatTo = function(this__36, builder__187)
  local s__189, fn__1178;
  temper.stringbuilder_append(builder__187, "'");
  s__189 = temper.date_tostring(this__36.value__185);
  fn__1178 = function(c__190)
    if (c__190 == 39) then
      temper.stringbuilder_append(builder__187, "''");
    else
      local local_13, local_14, local_15;
      local_13, local_14, local_15 = temper.pcall(function()
        temper.stringbuilder_appendcodepoint(builder__187, c__190);
      end);
      if local_13 then
      else
        temper.bubble();
      end
    end
    return nil;
  end;
  temper.string_foreach(s__189, fn__1178);
  temper.stringbuilder_append(builder__187, "'");
  return nil;
end;
SqlDate.constructor = function(this__75, value__192)
  this__75.value__185 = value__192;
  return nil;
end;
SqlDate.get.value = function(this__254)
  return this__254.value__185;
end;
SqlFloat64 = temper.type('SqlFloat64', SqlPart);
SqlFloat64.methods.formatTo = function(this__37, builder__195)
  local t_17;
  t_17 = temper.float64_tostring(this__37.value__193);
  temper.stringbuilder_append(builder__195, t_17);
  return nil;
end;
SqlFloat64.constructor = function(this__78, value__198)
  this__78.value__193 = value__198;
  return nil;
end;
SqlFloat64.get.value = function(this__257)
  return this__257.value__193;
end;
SqlInt32 = temper.type('SqlInt32', SqlPart);
SqlInt32.methods.formatTo = function(this__38, builder__201)
  local t_18;
  t_18 = temper.int32_tostring(this__38.value__199);
  temper.stringbuilder_append(builder__201, t_18);
  return nil;
end;
SqlInt32.constructor = function(this__81, value__204)
  this__81.value__199 = value__204;
  return nil;
end;
SqlInt32.get.value = function(this__260)
  return this__260.value__199;
end;
SqlInt64 = temper.type('SqlInt64', SqlPart);
SqlInt64.methods.formatTo = function(this__39, builder__207)
  local t_19;
  t_19 = temper.int64_tostring(this__39.value__205);
  temper.stringbuilder_append(builder__207, t_19);
  return nil;
end;
SqlInt64.constructor = function(this__84, value__210)
  this__84.value__205 = value__210;
  return nil;
end;
SqlInt64.get.value = function(this__263)
  return this__263.value__205;
end;
SqlString = temper.type('SqlString', SqlPart);
SqlString.methods.formatTo = function(this__40, builder__213)
  local fn__1166;
  temper.stringbuilder_append(builder__213, "'");
  fn__1166 = function(c__215)
    if (c__215 == 39) then
      temper.stringbuilder_append(builder__213, "''");
    else
      local local_20, local_21, local_22;
      local_20, local_21, local_22 = temper.pcall(function()
        temper.stringbuilder_appendcodepoint(builder__213, c__215);
      end);
      if local_20 then
      else
        temper.bubble();
      end
    end
    return nil;
  end;
  temper.string_foreach(this__40.value__211, fn__1166);
  temper.stringbuilder_append(builder__213, "'");
  return nil;
end;
SqlString.constructor = function(this__87, value__217)
  this__87.value__211 = value__217;
  return nil;
end;
SqlString.get.value = function(this__266)
  return this__266.value__211;
end;
exports = {};
exports.SqlBuilder = SqlBuilder;
exports.SqlFragment = SqlFragment;
exports.SqlPart = SqlPart;
exports.SqlSource = SqlSource;
exports.SqlBoolean = SqlBoolean;
exports.SqlDate = SqlDate;
exports.SqlFloat64 = SqlFloat64;
exports.SqlInt32 = SqlInt32;
exports.SqlInt64 = SqlInt64;
exports.SqlString = SqlString;
return exports;
