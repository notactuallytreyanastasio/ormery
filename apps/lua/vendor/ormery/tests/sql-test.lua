local temper = require('temper-core');
local SqlBuilder, SqlString, SqlInt32, local_97, local_98, exports;
SqlBuilder = temper.import('ormery/sql', 'SqlBuilder');
SqlString = temper.import('ormery/sql', 'SqlString');
SqlInt32 = temper.import('ormery/sql', 'SqlInt32');
local_97 = (unpack or table.unpack);
local_98 = require('luaunit');
local_98.FAILURE_PREFIX = temper.test_failure_prefix;
Test_ = {};
Test_.test_stringEscaping__268 = function()
  temper.test('string escaping', function(test_24)
    local build__219, buildWrong__220, actual_26, t_27, fn__1102, bobbyTables__225, actual_28, t_29, fn__1101, fn__1100;
    build__219 = function(name__221)
      local t_25;
      t_25 = SqlBuilder();
      t_25:appendSafe('select * from hi where name = ');
      t_25:appendString(name__221);
      return t_25.accumulated:toString();
    end;
    buildWrong__220 = function(name__223)
      return temper.concat("select * from hi where name = '", name__223, "'");
    end;
    actual_26 = build__219('world');
    t_27 = temper.str_eq(actual_26, "select * from hi where name = 'world'");
    fn__1102 = function()
      return temper.concat('expected build("world") == (', "select * from hi where name = 'world'", ') not (', actual_26, ')');
    end;
    temper.test_assert(test_24, t_27, fn__1102);
    bobbyTables__225 = "Robert'); drop table hi;--";
    actual_28 = build__219("Robert'); drop table hi;--");
    t_29 = temper.str_eq(actual_28, "select * from hi where name = 'Robert''); drop table hi;--'");
    fn__1101 = function()
      return temper.concat('expected build(bobbyTables) == (', "select * from hi where name = 'Robert''); drop table hi;--'", ') not (', actual_28, ')');
    end;
    temper.test_assert(test_24, t_29, fn__1101);
    fn__1100 = function()
      return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
    end;
    temper.test_assert(test_24, true, fn__1100);
    return nil;
  end);
end;
Test_.test_stringEdgeCases__276 = function()
  temper.test('string edge cases', function(test_30)
    local t_31, actual_32, t_33, fn__1062, t_34, actual_35, t_36, fn__1061, t_37, actual_38, t_39, fn__1060, t_40, actual_41, t_42, fn__1059;
    t_31 = SqlBuilder();
    t_31:appendSafe('v = ');
    t_31:appendString('');
    actual_32 = t_31.accumulated:toString();
    t_33 = temper.str_eq(actual_32, "v = ''");
    fn__1062 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v = ", \\interpolate, "").toString() == (', "v = ''", ') not (', actual_32, ')');
    end;
    temper.test_assert(test_30, t_33, fn__1062);
    t_34 = SqlBuilder();
    t_34:appendSafe('v = ');
    t_34:appendString("a''b");
    actual_35 = t_34.accumulated:toString();
    t_36 = temper.str_eq(actual_35, "v = 'a''''b'");
    fn__1061 = function()
      return temper.concat("expected stringExpr(`-work/src//sql/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (", "v = 'a''''b'", ') not (', actual_35, ')');
    end;
    temper.test_assert(test_30, t_36, fn__1061);
    t_37 = SqlBuilder();
    t_37:appendSafe('v = ');
    t_37:appendString('Hello \xe4\xb8\x96\xe7\x95\x8c');
    actual_38 = t_37.accumulated:toString();
    t_39 = temper.str_eq(actual_38, "v = 'Hello \xe4\xb8\x96\xe7\x95\x8c'");
    fn__1060 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v = ", \\interpolate, "Hello \xe4\xb8\x96\xe7\x95\x8c").toString() == (', "v = 'Hello \xe4\xb8\x96\xe7\x95\x8c'", ') not (', actual_38, ')');
    end;
    temper.test_assert(test_30, t_39, fn__1060);
    t_40 = SqlBuilder();
    t_40:appendSafe('v = ');
    t_40:appendString('Line1\nLine2');
    actual_41 = t_40.accumulated:toString();
    t_42 = temper.str_eq(actual_41, "v = 'Line1\nLine2'");
    fn__1059 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v = ", \\interpolate, "Line1\\nLine2").toString() == (', "v = 'Line1\nLine2'", ') not (', actual_41, ')');
    end;
    temper.test_assert(test_30, t_42, fn__1059);
    return nil;
  end);
end;
Test_.test_numbersAndBooleans__289 = function()
  temper.test('numbers and booleans', function(test_43)
    local t_44, t_45, actual_46, t_47, fn__1033, date__228, local_48, local_49, local_50, t_52, actual_53, t_54, fn__1032;
    t_45 = SqlBuilder();
    t_45:appendSafe('select ');
    t_45:appendInt32(42);
    t_45:appendSafe(', ');
    t_45:appendInt64(temper.int64_constructor(43));
    t_45:appendSafe(', ');
    t_45:appendFloat64(19.99);
    t_45:appendSafe(', ');
    t_45:appendBoolean(true);
    t_45:appendSafe(', ');
    t_45:appendBoolean(false);
    actual_46 = t_45.accumulated:toString();
    t_47 = temper.str_eq(actual_46, 'select 42, 43, 19.99, TRUE, FALSE');
    fn__1033 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "select ", \\interpolate, 42, ", ", \\interpolate, 43, ", ", \\interpolate, 19.99, ", ", \\interpolate, true, ", ", \\interpolate, false).toString() == (', 'select 42, 43, 19.99, TRUE, FALSE', ') not (', actual_46, ')');
    end;
    temper.test_assert(test_43, t_47, fn__1033);
    local_48, local_49, local_50 = temper.pcall(function()
      t_44 = temper.date_constructor(2024, 12, 25);
      date__228 = t_44;
    end);
    if local_48 then
    else
      date__228 = temper.bubble();
    end
    t_52 = SqlBuilder();
    t_52:appendSafe('insert into t values (');
    t_52:appendDate(date__228);
    t_52:appendSafe(')');
    actual_53 = t_52.accumulated:toString();
    t_54 = temper.str_eq(actual_53, "insert into t values ('2024-12-25')");
    fn__1032 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "insert into t values (", \\interpolate, date, ")").toString() == (', "insert into t values ('2024-12-25')", ') not (', actual_53, ')');
    end;
    temper.test_assert(test_43, t_54, fn__1032);
    return nil;
  end);
end;
Test_.test_lists__296 = function()
  temper.test('lists', function(test_55)
    local t_56, t_57, t_58, t_59, t_60, actual_61, t_62, fn__977, t_63, actual_64, t_65, fn__976, t_66, actual_67, t_68, fn__975, t_69, actual_70, t_71, fn__974, t_72, actual_73, t_74, fn__973, local_75, local_76, local_77, local_79, local_80, local_81, dates__230, t_83, actual_84, t_85, fn__972;
    t_60 = SqlBuilder();
    t_60:appendSafe('v IN (');
    t_60:appendStringList(temper.listof('a', 'b', "c'd"));
    t_60:appendSafe(')');
    actual_61 = t_60.accumulated:toString();
    t_62 = temper.str_eq(actual_61, "v IN ('a', 'b', 'c''d')");
    fn__977 = function()
      return temper.concat("expected stringExpr(`-work/src//sql/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (", "v IN ('a', 'b', 'c''d')", ') not (', actual_61, ')');
    end;
    temper.test_assert(test_55, t_62, fn__977);
    t_63 = SqlBuilder();
    t_63:appendSafe('v IN (');
    t_63:appendInt32List(temper.listof(1, 2, 3));
    t_63:appendSafe(')');
    actual_64 = t_63.accumulated:toString();
    t_65 = temper.str_eq(actual_64, 'v IN (1, 2, 3)');
    fn__976 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v IN (", \\interpolate, list(1, 2, 3), ")").toString() == (', 'v IN (1, 2, 3)', ') not (', actual_64, ')');
    end;
    temper.test_assert(test_55, t_65, fn__976);
    t_66 = SqlBuilder();
    t_66:appendSafe('v IN (');
    t_66:appendInt64List(temper.listof(temper.int64_constructor(1), temper.int64_constructor(2)));
    t_66:appendSafe(')');
    actual_67 = t_66.accumulated:toString();
    t_68 = temper.str_eq(actual_67, 'v IN (1, 2)');
    fn__975 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v IN (", \\interpolate, list(1, 2), ")").toString() == (', 'v IN (1, 2)', ') not (', actual_67, ')');
    end;
    temper.test_assert(test_55, t_68, fn__975);
    t_69 = SqlBuilder();
    t_69:appendSafe('v IN (');
    t_69:appendFloat64List(temper.listof(1.0, 2.0));
    t_69:appendSafe(')');
    actual_70 = t_69.accumulated:toString();
    t_71 = temper.str_eq(actual_70, 'v IN (1.0, 2.0)');
    fn__974 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v IN (", \\interpolate, list(1.0, 2.0), ")").toString() == (', 'v IN (1.0, 2.0)', ') not (', actual_70, ')');
    end;
    temper.test_assert(test_55, t_71, fn__974);
    t_72 = SqlBuilder();
    t_72:appendSafe('v IN (');
    t_72:appendBooleanList(temper.listof(true, false));
    t_72:appendSafe(')');
    actual_73 = t_72.accumulated:toString();
    t_74 = temper.str_eq(actual_73, 'v IN (TRUE, FALSE)');
    fn__973 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v IN (", \\interpolate, list(true, false), ")").toString() == (', 'v IN (TRUE, FALSE)', ') not (', actual_73, ')');
    end;
    temper.test_assert(test_55, t_74, fn__973);
    local_75, local_76, local_77 = temper.pcall(function()
      t_56 = temper.date_constructor(2024, 1, 1);
      t_57 = t_56;
    end);
    if local_75 then
    else
      t_57 = temper.bubble();
    end
    local_79, local_80, local_81 = temper.pcall(function()
      t_58 = temper.date_constructor(2024, 12, 25);
      t_59 = t_58;
    end);
    if local_79 then
    else
      t_59 = temper.bubble();
    end
    dates__230 = temper.listof(t_57, t_59);
    t_83 = SqlBuilder();
    t_83:appendSafe('v IN (');
    t_83:appendDateList(dates__230);
    t_83:appendSafe(')');
    actual_84 = t_83.accumulated:toString();
    t_85 = temper.str_eq(actual_84, "v IN ('2024-01-01', '2024-12-25')");
    fn__972 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "v IN (", \\interpolate, dates, ")").toString() == (', "v IN ('2024-01-01', '2024-12-25')", ') not (', actual_84, ')');
    end;
    temper.test_assert(test_55, t_85, fn__972);
    return nil;
  end);
end;
Test_.test_nesting__315 = function()
  temper.test('nesting', function(test_86)
    local name__232, t_87, condition__233, t_88, actual_89, t_90, fn__940, t_91, actual_92, t_93, fn__939, parts__234, t_94, actual_95, t_96, fn__938;
    name__232 = 'Someone';
    t_87 = SqlBuilder();
    t_87:appendSafe('where p.last_name = ');
    t_87:appendString('Someone');
    condition__233 = t_87.accumulated;
    t_88 = SqlBuilder();
    t_88:appendSafe('select p.id from person p ');
    t_88:appendFragment(condition__233);
    actual_89 = t_88.accumulated:toString();
    t_90 = temper.str_eq(actual_89, "select p.id from person p where p.last_name = 'Someone'");
    fn__940 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "select p.id from person p ", \\interpolate, condition).toString() == (', "select p.id from person p where p.last_name = 'Someone'", ') not (', actual_89, ')');
    end;
    temper.test_assert(test_86, t_90, fn__940);
    t_91 = SqlBuilder();
    t_91:appendSafe('select p.id from person p ');
    t_91:appendPart(condition__233:toSource());
    actual_92 = t_91.accumulated:toString();
    t_93 = temper.str_eq(actual_92, "select p.id from person p where p.last_name = 'Someone'");
    fn__939 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "select p.id from person p ", \\interpolate, condition.toSource()).toString() == (', "select p.id from person p where p.last_name = 'Someone'", ') not (', actual_92, ')');
    end;
    temper.test_assert(test_86, t_93, fn__939);
    parts__234 = temper.listof(SqlString("a'b"), SqlInt32(3));
    t_94 = SqlBuilder();
    t_94:appendSafe('select ');
    t_94:appendPartList(parts__234);
    actual_95 = t_94.accumulated:toString();
    t_96 = temper.str_eq(actual_95, "select 'a''b', 3");
    fn__938 = function()
      return temper.concat('expected stringExpr(`-work/src//sql/`.sql, true, "select ", \\interpolate, parts).toString() == (', "select 'a''b', 3", ') not (', actual_95, ')');
    end;
    temper.test_assert(test_86, t_96, fn__938);
    return nil;
  end);
end;
exports = {};
local_98.LuaUnit.run(local_97({'--pattern', '^Test_%.', local_97(arg)}));
return exports;
