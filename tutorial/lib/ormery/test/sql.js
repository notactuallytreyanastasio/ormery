import {
  SqlBuilder, SqlInt32, SqlString
} from "../sql.js";
import {
  Test as Test_133
} from "@temperlang/std/testing";
import {
  panic as panic_171
} from "@temperlang/core";
it("string escaping", function () {
    const test_132 = new Test_133();
    try {
      function build_134(name_135) {
        let t_136 = new SqlBuilder();
        t_136.appendSafe("select * from hi where name = ");
        t_136.appendString(name_135);
        return t_136.accumulated.toString();
      }
      function buildWrong_137(name_138) {
        return "select * from hi where name = '" + name_138 + "'";
      }
      const actual_139 = build_134("world");
      let t_140 = actual_139 === "select * from hi where name = 'world'";
      function fn_141() {
        return "expected build(\u0022world\u0022) == (" + "select * from hi where name = 'world'" + ") not (" + actual_139 + ")";
      }
      test_132.assert(t_140, fn_141);
      const bobbyTables_142 = "Robert'); drop table hi;--";
      const actual_143 = build_134("Robert'); drop table hi;--");
      let t_144 = actual_143 === "select * from hi where name = 'Robert''); drop table hi;--'";
      function fn_145() {
        return "expected build(bobbyTables) == (" + "select * from hi where name = 'Robert''); drop table hi;--'" + ") not (" + actual_143 + ")";
      }
      test_132.assert(t_144, fn_145);
      function fn_146() {
        return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
      }
      test_132.assert(true, fn_146);
      return;
    } finally {
      test_132.softFailToHard();
    }
});
it("string edge cases", function () {
    const test_147 = new Test_133();
    try {
      let t_148 = new SqlBuilder();
      t_148.appendSafe("v = ");
      t_148.appendString("");
      const actual_149 = t_148.accumulated.toString();
      let t_150 = actual_149 === "v = ''";
      function fn_151() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022\u0022).toString() == (" + "v = ''" + ") not (" + actual_149 + ")";
      }
      test_147.assert(t_150, fn_151);
      let t_152 = new SqlBuilder();
      t_152.appendSafe("v = ");
      t_152.appendString("a''b");
      const actual_153 = t_152.accumulated.toString();
      let t_154 = actual_153 === "v = 'a''''b'";
      function fn_155() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022a''b\u0022).toString() == (" + "v = 'a''''b'" + ") not (" + actual_153 + ")";
      }
      test_147.assert(t_154, fn_155);
      let t_156 = new SqlBuilder();
      t_156.appendSafe("v = ");
      t_156.appendString("Hello 世界");
      const actual_157 = t_156.accumulated.toString();
      let t_158 = actual_157 === "v = 'Hello 世界'";
      function fn_159() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022Hello 世界\u0022).toString() == (" + "v = 'Hello 世界'" + ") not (" + actual_157 + ")";
      }
      test_147.assert(t_158, fn_159);
      let t_160 = new SqlBuilder();
      t_160.appendSafe("v = ");
      t_160.appendString("Line1\nLine2");
      const actual_161 = t_160.accumulated.toString();
      let t_162 = actual_161 === "v = 'Line1\nLine2'";
      function fn_163() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022Line1\\nLine2\u0022).toString() == (" + "v = 'Line1\nLine2'" + ") not (" + actual_161 + ")";
      }
      test_147.assert(t_162, fn_163);
      return;
    } finally {
      test_147.softFailToHard();
    }
});
it("numbers and booleans", function () {
    const test_164 = new Test_133();
    try {
      let t_165;
      let t_166 = new SqlBuilder();
      t_166.appendSafe("select ");
      t_166.appendInt32(42);
      t_166.appendSafe(", ");
      t_166.appendInt64(BigInt("43"));
      t_166.appendSafe(", ");
      t_166.appendFloat64(19.99);
      t_166.appendSafe(", ");
      t_166.appendBoolean(true);
      t_166.appendSafe(", ");
      t_166.appendBoolean(false);
      const actual_167 = t_166.accumulated.toString();
      let t_168 = actual_167 === "select 42, 43, 19.99, TRUE, FALSE";
      function fn_169() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select \u0022, \\interpolate, 42, \u0022, \u0022, \\interpolate, 43, \u0022, \u0022, \\interpolate, 19.99, \u0022, \u0022, \\interpolate, true, \u0022, \u0022, \\interpolate, false).toString() == (" + "select 42, 43, 19.99, TRUE, FALSE" + ") not (" + actual_167 + ")";
      }
      test_164.assert(t_168, fn_169);
      let date_170;
      try {
        t_165 = new (globalThis.Date)(globalThis.Date.UTC(2024, 12 - 1, 25));
        date_170 = t_165;
      } catch {
        date_170 = panic_171();
      }
      let t_172 = new SqlBuilder();
      t_172.appendSafe("insert into t values (");
      t_172.appendDate(date_170);
      t_172.appendSafe(")");
      const actual_173 = t_172.accumulated.toString();
      let t_174 = actual_173 === "insert into t values ('2024-12-25')";
      function fn_175() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022insert into t values (\u0022, \\interpolate, date, \u0022)\u0022).toString() == (" + "insert into t values ('2024-12-25')" + ") not (" + actual_173 + ")";
      }
      test_164.assert(t_174, fn_175);
      return;
    } finally {
      test_164.softFailToHard();
    }
});
it("lists", function () {
    const test_176 = new Test_133();
    try {
      let t_177;
      let t_178;
      let t_179;
      let t_180;
      let t_181 = new SqlBuilder();
      t_181.appendSafe("v IN (");
      t_181.appendStringList(Object.freeze(["a", "b", "c'd"]));
      t_181.appendSafe(")");
      const actual_182 = t_181.accumulated.toString();
      let t_183 = actual_182 === "v IN ('a', 'b', 'c''d')";
      function fn_184() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(\u0022a\u0022, \u0022b\u0022, \u0022c'd\u0022), \u0022)\u0022).toString() == (" + "v IN ('a', 'b', 'c''d')" + ") not (" + actual_182 + ")";
      }
      test_176.assert(t_183, fn_184);
      let t_185 = new SqlBuilder();
      t_185.appendSafe("v IN (");
      t_185.appendInt32List(Object.freeze([1, 2, 3]));
      t_185.appendSafe(")");
      const actual_186 = t_185.accumulated.toString();
      let t_187 = actual_186 === "v IN (1, 2, 3)";
      function fn_188() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1, 2, 3), \u0022)\u0022).toString() == (" + "v IN (1, 2, 3)" + ") not (" + actual_186 + ")";
      }
      test_176.assert(t_187, fn_188);
      let t_189 = new SqlBuilder();
      t_189.appendSafe("v IN (");
      t_189.appendInt64List(Object.freeze([BigInt("1"), BigInt("2")]));
      t_189.appendSafe(")");
      const actual_190 = t_189.accumulated.toString();
      let t_191 = actual_190 === "v IN (1, 2)";
      function fn_192() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1, 2), \u0022)\u0022).toString() == (" + "v IN (1, 2)" + ") not (" + actual_190 + ")";
      }
      test_176.assert(t_191, fn_192);
      let t_193 = new SqlBuilder();
      t_193.appendSafe("v IN (");
      t_193.appendFloat64List(Object.freeze([1.0, 2.0]));
      t_193.appendSafe(")");
      const actual_194 = t_193.accumulated.toString();
      let t_195 = actual_194 === "v IN (1.0, 2.0)";
      function fn_196() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1.0, 2.0), \u0022)\u0022).toString() == (" + "v IN (1.0, 2.0)" + ") not (" + actual_194 + ")";
      }
      test_176.assert(t_195, fn_196);
      let t_197 = new SqlBuilder();
      t_197.appendSafe("v IN (");
      t_197.appendBooleanList(Object.freeze([true, false]));
      t_197.appendSafe(")");
      const actual_198 = t_197.accumulated.toString();
      let t_199 = actual_198 === "v IN (TRUE, FALSE)";
      function fn_200() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(true, false), \u0022)\u0022).toString() == (" + "v IN (TRUE, FALSE)" + ") not (" + actual_198 + ")";
      }
      test_176.assert(t_199, fn_200);
      try {
        t_177 = new (globalThis.Date)(globalThis.Date.UTC(2024, 1 - 1, 1));
        t_178 = t_177;
      } catch {
        t_178 = panic_171();
      }
      try {
        t_179 = new (globalThis.Date)(globalThis.Date.UTC(2024, 12 - 1, 25));
        t_180 = t_179;
      } catch {
        t_180 = panic_171();
      }
      const dates_201 = Object.freeze([t_178, t_180]);
      let t_202 = new SqlBuilder();
      t_202.appendSafe("v IN (");
      t_202.appendDateList(dates_201);
      t_202.appendSafe(")");
      const actual_203 = t_202.accumulated.toString();
      let t_204 = actual_203 === "v IN ('2024-01-01', '2024-12-25')";
      function fn_205() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022v IN (\u0022, \\interpolate, dates, \u0022)\u0022).toString() == (" + "v IN ('2024-01-01', '2024-12-25')" + ") not (" + actual_203 + ")";
      }
      test_176.assert(t_204, fn_205);
      return;
    } finally {
      test_176.softFailToHard();
    }
});
it("nesting", function () {
    const test_206 = new Test_133();
    try {
      const name_207 = "Someone";
      let t_208 = new SqlBuilder();
      t_208.appendSafe("where p.last_name = ");
      t_208.appendString("Someone");
      const condition_209 = t_208.accumulated;
      let t_210 = new SqlBuilder();
      t_210.appendSafe("select p.id from person p ");
      t_210.appendFragment(condition_209);
      const actual_211 = t_210.accumulated.toString();
      let t_212 = actual_211 === "select p.id from person p where p.last_name = 'Someone'";
      function fn_213() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select p.id from person p \u0022, \\interpolate, condition).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_211 + ")";
      }
      test_206.assert(t_212, fn_213);
      let t_214 = new SqlBuilder();
      t_214.appendSafe("select p.id from person p ");
      t_214.appendPart(condition_209.toSource());
      const actual_215 = t_214.accumulated.toString();
      let t_216 = actual_215 === "select p.id from person p where p.last_name = 'Someone'";
      function fn_217() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select p.id from person p \u0022, \\interpolate, condition.toSource()).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_215 + ")";
      }
      test_206.assert(t_216, fn_217);
      const parts_218 = Object.freeze([new SqlString("a'b"), new SqlInt32(3)]);
      let t_219 = new SqlBuilder();
      t_219.appendSafe("select ");
      t_219.appendPartList(parts_218);
      const actual_220 = t_219.accumulated.toString();
      let t_221 = actual_220 === "select 'a''b', 3";
      function fn_222() {
        return "expected stringExpr(`-work/src//sql/`.sql, true, \u0022select \u0022, \\interpolate, parts).toString() == (" + "select 'a''b', 3" + ") not (" + actual_220 + ")";
      }
      test_206.assert(t_221, fn_222);
      return;
    } finally {
      test_206.softFailToHard();
    }
});
