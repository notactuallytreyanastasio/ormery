import {
  SafeHtmlBuilder, htmlCodec
} from "../html.js";
import {
  Test as Test_711
} from "@temperlang/std/testing";
it("HTML decoding", function () {
    const test_710 = new Test_711();
    try {
      const actual_712 = htmlCodec.decode("");
      let t_713 = actual_712 === "";
      function fn_714() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0022) == (" + "" + ") not (" + actual_712 + ")";
      }
      test_710.assert(t_713, fn_714);
      const actual_715 = htmlCodec.decode("\u0026l");
      let t_716 = actual_715 === "\u0026l";
      function fn_717() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026l\u0022) == (" + "\u0026l" + ") not (" + actual_715 + ")";
      }
      test_710.assert(t_716, fn_717);
      const actual_718 = htmlCodec.decode("\u0026lt");
      let t_719 = actual_718 === "\u003c";
      function fn_720() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026lt\u0022) == (" + "\u003c" + ") not (" + actual_718 + ")";
      }
      test_710.assert(t_719, fn_720);
      const actual_721 = htmlCodec.decode("\u0026lt;");
      let t_722 = actual_721 === "\u003c";
      function fn_723() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026lt;\u0022) == (" + "\u003c" + ") not (" + actual_721 + ")";
      }
      test_710.assert(t_722, fn_723);
      const actual_724 = htmlCodec.decode("\u0026Bcy;");
      let t_725 = actual_724 === "Б";
      function fn_726() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026Bcy;\u0022) == (" + "Б" + ") not (" + actual_724 + ")";
      }
      test_710.assert(t_725, fn_726);
      const actual_727 = htmlCodec.decode("\u0026Bcy");
      let t_728 = actual_727 === "\u0026Bcy";
      function fn_729() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026Bcy\u0022) == (" + "\u0026Bcy" + ") not (" + actual_727 + ")";
      }
      test_710.assert(t_728, fn_729);
      const actual_730 = htmlCodec.decode("\u0026LT;");
      let t_731 = actual_730 === "\u003c";
      function fn_732() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026LT;\u0022) == (" + "\u003c" + ") not (" + actual_730 + ")";
      }
      test_710.assert(t_731, fn_732);
      const actual_733 = htmlCodec.decode("\u0026Aacute;");
      let t_734 = actual_733 === "Á";
      function fn_735() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026Aacute;\u0022) == (" + "Á" + ") not (" + actual_733 + ")";
      }
      test_710.assert(t_734, fn_735);
      const actual_736 = htmlCodec.decode("\u0026aacute;");
      let t_737 = actual_736 === "á";
      function fn_738() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026aacute;\u0022) == (" + "á" + ") not (" + actual_736 + ")";
      }
      test_710.assert(t_737, fn_738);
      const actual_739 = htmlCodec.decode("\u0026AaCuTe;");
      let t_740 = actual_739 === "\u0026AaCuTe;";
      function fn_741() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026AaCuTe;\u0022) == (" + "\u0026AaCuTe;" + ") not (" + actual_739 + ")";
      }
      test_710.assert(t_740, fn_741);
      const actual_742 = htmlCodec.decode("\u0026gt;;");
      let t_743 = actual_742 === "\u003e;";
      function fn_744() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026gt;;\u0022) == (" + "\u003e;" + ") not (" + actual_742 + ")";
      }
      test_710.assert(t_743, fn_744);
      const actual_745 = htmlCodec.decode("\u0026amp;lt;");
      let t_746 = actual_745 === "\u0026lt;";
      function fn_747() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026amp;lt;\u0022) == (" + "\u0026lt;" + ") not (" + actual_745 + ")";
      }
      test_710.assert(t_746, fn_747);
      return;
    } finally {
      test_710.softFailToHard();
    }
});
it("HTML encoding", function () {
    const test_748 = new Test_711();
    try {
      const actual_749 = htmlCodec.encode("");
      let t_750 = actual_749 === "";
      function fn_751() {
        return "expected `-work/src//html/`.htmlCodec.encode(\u0022\u0022) == (" + "" + ") not (" + actual_749 + ")";
      }
      test_748.assert(t_750, fn_751);
      const actual_752 = htmlCodec.encode("Hello, World!");
      let t_753 = actual_752 === "Hello, World!";
      function fn_754() {
        return "expected `-work/src//html/`.htmlCodec.encode(\u0022Hello, World!\u0022) == (" + "Hello, World!" + ") not (" + actual_752 + ")";
      }
      test_748.assert(t_753, fn_754);
      const actual_755 = htmlCodec.encode("\u003cfoo\u003e \u0026 \u003cbar baz='b\u0022oo'\u003e far");
      let t_756 = actual_755 === "\u0026lt;foo\u0026gt; \u0026amp; \u0026lt;bar baz=\u0026#39;b\u0026#34;oo\u0026#39;\u0026gt; far";
      function fn_757() {
        return "expected `-work/src//html/`.htmlCodec.encode(\u0022\u003cfoo\u003e \u0026 \u003cbar baz='b\\\u0022oo'\u003e far\u0022) == (" + "\u0026lt;foo\u0026gt; \u0026amp; \u0026lt;bar baz=\u0026#39;b\u0026#34;oo\u0026#39;\u0026gt; far" + ") not (" + actual_755 + ")";
      }
      test_748.assert(t_756, fn_757);
      return;
    } finally {
      test_748.softFailToHard();
    }
});
it("hello world, html style", function () {
    const test_1138 = new Test_711();
    try {
      let t_1139 = new SafeHtmlBuilder();
      t_1139.appendSafe("Hello, \u003cb\u003e");
      t_1139.appendString("World");
      t_1139.appendSafe("\u003c/b\u003e!");
      const actual_1140 = t_1139.accumulated.toString();
      let t_1141 = actual_1140 === "Hello, \u003cb\u003eWorld\u003c/b\u003e!";
      function fn_1142() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022Hello, \u003cb\u003e\u0022, \\interpolate, \u0022World\u0022, \u0022\u003c/b\u003e!\u0022).toString() == (" + "Hello, \u003cb\u003eWorld\u003c/b\u003e!" + ") not (" + actual_1140 + ")";
      }
      test_1138.assert(t_1141, fn_1142);
      return;
    } finally {
      test_1138.softFailToHard();
    }
});
it("autoescaped", function () {
    const test_1143 = new Test_711();
    try {
      let t_1144 = new SafeHtmlBuilder();
      t_1144.appendSafe("1 + 1 ");
      t_1144.appendString("\u003c");
      t_1144.appendSafe(" 3.");
      const actual_1145 = t_1144.accumulated.toString();
      let t_1146 = actual_1145 === "1 + 1 \u0026lt; 3.";
      function fn_1147() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u00221 + 1 \u0022, \\interpolate, \u0022\u003c\u0022, \u0022 3.\u0022).toString() == (" + "1 + 1 \u0026lt; 3." + ") not (" + actual_1145 + ")";
      }
      test_1143.assert(t_1146, fn_1147);
      return;
    } finally {
      test_1143.softFailToHard();
    }
});
it("context matters -- URLs embed", function () {
    const test_1148 = new Test_711();
    try {
      function okUrl_1149() {
        return "https://example.com/isn't-a-problem";
      }
      function evilUrl_1150() {
        return "javascript:alert('evil done')";
      }
      let t_1151 = new SafeHtmlBuilder();
      t_1151.appendSafe("\u003ca href='");
      t_1151.appendString("https://example.com/isn't-a-problem");
      t_1151.appendSafe("'\u003e");
      t_1151.appendString("https://example.com/isn't-a-problem");
      t_1151.appendSafe("\u003c/a\u003e");
      const actual_1152 = t_1151.accumulated.toString();
      let t_1153 = actual_1152 === "\u003ca href='https://example.com/isn\u0026#39;t-a-problem'\u003ehttps://example.com/isn\u0026#39;t-a-problem\u003c/a\u003e";
      function fn_1154() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003ca href='\u0022, \\interpolate, okUrl(), \u0022'\u003e\u0022, \\interpolate, okUrl(), \u0022\u003c/a\u003e\u0022).toString() == (" + "\u003ca href='https://example.com/isn\u0026#39;t-a-problem'\u003ehttps://example.com/isn\u0026#39;t-a-problem\u003c/a\u003e" + ") not (" + actual_1152 + ")";
      }
      test_1148.assert(t_1153, fn_1154);
      let t_1155 = new SafeHtmlBuilder();
      t_1155.appendSafe("\u003ca href='");
      t_1155.appendString("javascript:alert('evil done')");
      t_1155.appendSafe("'\u003e");
      t_1155.appendString("javascript:alert('evil done')");
      t_1155.appendSafe("\u003c/a\u003e");
      const actual_1156 = t_1155.accumulated.toString();
      let t_1157 = actual_1156 === "\u003ca href='about:zz_Temper_zz#'\u003ejavascript:alert(\u0026#39;evil done\u0026#39;)\u003c/a\u003e";
      function fn_1158() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003ca href='\u0022, \\interpolate, evilUrl(), \u0022'\u003e\u0022, \\interpolate, evilUrl(), \u0022\u003c/a\u003e\u0022).toString() == (" + "\u003ca href='about:zz_Temper_zz#'\u003ejavascript:alert(\u0026#39;evil done\u0026#39;)\u003c/a\u003e" + ") not (" + actual_1156 + ")";
      }
      test_1148.assert(t_1157, fn_1158);
      return;
    } finally {
      test_1148.softFailToHard();
    }
});
it("quote adjustments", function () {
    const test_1159 = new Test_711();
    try {
      const className_1160 = "some-class";
      let t_1161 = new SafeHtmlBuilder();
      t_1161.appendSafe("\u003chr class=");
      t_1161.appendString("some-class");
      t_1161.appendSafe("\u003e\u003chr class='");
      t_1161.appendString("some-class");
      t_1161.appendSafe("'\u003e\u003chr class=other-class\u003e");
      const actual_1162 = t_1161.accumulated.toString();
      let t_1163 = actual_1162 === "\u003chr class=\u0022some-class\u0022\u003e\u003chr class='some-class'\u003e\u003chr class=\u0022other-class\u0022\u003e";
      function fn_1164() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003chr class=\u0022, \\interpolate, className, \u0022\u003e\u003chr class='\u0022, \\interpolate, className, \u0022'\u003e\u003chr class=other-class\u003e\u0022).toString() == (" + "\u003chr class=\u0022some-class\u0022\u003e\u003chr class='some-class'\u003e\u003chr class=\u0022other-class\u0022\u003e" + ") not (" + actual_1162 + ")";
      }
      test_1159.assert(t_1163, fn_1164);
      return;
    } finally {
      test_1159.softFailToHard();
    }
});
it("safehtml injected in tag and attribute context", function () {
    const test_1165 = new Test_711();
    try {
      let t_1166 = new SafeHtmlBuilder();
      t_1166.appendSafe("I \u003c3 \u003cb\u003ePonies\u003c/b\u003e!");
      const love_1167 = t_1166.accumulated;
      let t_1168 = new SafeHtmlBuilder();
      t_1168.appendSafe("\u003cb\u003e");
      t_1168.appendSafeHtml(love_1167);
      t_1168.appendSafe("\u003c/b\u003e\u003cimg alt='");
      t_1168.appendSafeHtml(love_1167);
      t_1168.appendSafe("' src='ponies'\u003e");
      const actual_1169 = t_1168.accumulated.toString();
      let t_1170 = actual_1169 === "\u003cb\u003eI \u0026lt;3 \u003cb\u003ePonies\u003c/b\u003e!\u003c/b\u003e\u003cimg alt='I \u0026lt;3 \u0026lt;b\u0026gt;Ponies\u0026lt;/b\u0026gt;!' src='ponies'\u003e";
      function fn_1171() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003cb\u003e\u0022, \\interpolate, love, \u0022\u003c/b\u003e\u003cimg alt='\u0022, \\interpolate, love, \u0022' src='ponies'\u003e\u0022).toString() == (" + "\u003cb\u003eI \u0026lt;3 \u003cb\u003ePonies\u003c/b\u003e!\u003c/b\u003e\u003cimg alt='I \u0026lt;3 \u0026lt;b\u0026gt;Ponies\u0026lt;/b\u0026gt;!' src='ponies'\u003e" + ") not (" + actual_1169 + ")";
      }
      test_1165.assert(t_1170, fn_1171);
      return;
    } finally {
      test_1165.softFailToHard();
    }
});
it("looping inside an HTML expression", function () {
    const test_1172 = new Test_711();
    try {
      const items_1173 = Object.freeze(["One", "\u003cTwo\u003e", "Three"]);
      const accumulator_1174 = new SafeHtmlBuilder();
      accumulator_1174.appendSafe("\u003cul\u003e\n");
      function fn_1175(item_1176) {
        accumulator_1174.appendSafe("  \u003cli\u003e");
        accumulator_1174.appendString(item_1176);
        accumulator_1174.appendSafe("\u003c/li\u003e\n");
        return;
      }
      items_1173.forEach(fn_1175);
      accumulator_1174.appendSafe("\u003c/ul\u003e");
      const got_1177 = accumulator_1174.accumulated;
      const actual_1178 = got_1177.text;
      let t_1179 = actual_1178 === "\u003cul\u003e\n  \u003cli\u003eOne\u003c/li\u003e\n  \u003cli\u003e\u0026lt;Two\u0026gt;\u003c/li\u003e\n  \u003cli\u003eThree\u003c/li\u003e\n\u003c/ul\u003e";
      function fn_1180() {
        return "expected got.text == (" + "\u003cul\u003e\n  \u003cli\u003eOne\u003c/li\u003e\n  \u003cli\u003e\u0026lt;Two\u0026gt;\u003c/li\u003e\n  \u003cli\u003eThree\u003c/li\u003e\n\u003c/ul\u003e" + ") not (" + actual_1178 + ")";
      }
      test_1172.assert(t_1179, fn_1180);
      return;
    } finally {
      test_1172.softFailToHard();
    }
});
it("double quotes in attribute value with inserted quotes", function () {
    const test_1181 = new Test_711();
    try {
      let t_1182 = new SafeHtmlBuilder();
      t_1182.appendSafe("\u003cdiv id=a\u0022b\u003e");
      const actual_1183 = t_1182.accumulated.text;
      let t_1184 = actual_1183 === "\u003cdiv id=\u0022a\u0026#34;b\u0022\u003e";
      function fn_1185() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003cdiv id=a\\\u0022b\u003e\u0022).text == (" + "\u003cdiv id=\u0022a\u0026#34;b\u0022\u003e" + ") not (" + actual_1183 + ")";
      }
      test_1181.assert(t_1184, fn_1185);
      return;
    } finally {
      test_1181.softFailToHard();
    }
});
