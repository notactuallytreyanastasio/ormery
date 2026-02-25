import {
  SafeHtmlBuilder, htmlCodec
} from "../html.js";
import {
  Test as Test_713
} from "@temperlang/std/testing";
it("HTML decoding", function () {
    const test_712 = new Test_713();
    try {
      const actual_714 = htmlCodec.decode("");
      let t_715 = actual_714 === "";
      function fn_716() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0022) == (" + "" + ") not (" + actual_714 + ")";
      }
      test_712.assert(t_715, fn_716);
      const actual_717 = htmlCodec.decode("\u0026l");
      let t_718 = actual_717 === "\u0026l";
      function fn_719() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026l\u0022) == (" + "\u0026l" + ") not (" + actual_717 + ")";
      }
      test_712.assert(t_718, fn_719);
      const actual_720 = htmlCodec.decode("\u0026lt");
      let t_721 = actual_720 === "\u003c";
      function fn_722() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026lt\u0022) == (" + "\u003c" + ") not (" + actual_720 + ")";
      }
      test_712.assert(t_721, fn_722);
      const actual_723 = htmlCodec.decode("\u0026lt;");
      let t_724 = actual_723 === "\u003c";
      function fn_725() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026lt;\u0022) == (" + "\u003c" + ") not (" + actual_723 + ")";
      }
      test_712.assert(t_724, fn_725);
      const actual_726 = htmlCodec.decode("\u0026Bcy;");
      let t_727 = actual_726 === "Б";
      function fn_728() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026Bcy;\u0022) == (" + "Б" + ") not (" + actual_726 + ")";
      }
      test_712.assert(t_727, fn_728);
      const actual_729 = htmlCodec.decode("\u0026Bcy");
      let t_730 = actual_729 === "\u0026Bcy";
      function fn_731() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026Bcy\u0022) == (" + "\u0026Bcy" + ") not (" + actual_729 + ")";
      }
      test_712.assert(t_730, fn_731);
      const actual_732 = htmlCodec.decode("\u0026LT;");
      let t_733 = actual_732 === "\u003c";
      function fn_734() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026LT;\u0022) == (" + "\u003c" + ") not (" + actual_732 + ")";
      }
      test_712.assert(t_733, fn_734);
      const actual_735 = htmlCodec.decode("\u0026Aacute;");
      let t_736 = actual_735 === "Á";
      function fn_737() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026Aacute;\u0022) == (" + "Á" + ") not (" + actual_735 + ")";
      }
      test_712.assert(t_736, fn_737);
      const actual_738 = htmlCodec.decode("\u0026aacute;");
      let t_739 = actual_738 === "á";
      function fn_740() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026aacute;\u0022) == (" + "á" + ") not (" + actual_738 + ")";
      }
      test_712.assert(t_739, fn_740);
      const actual_741 = htmlCodec.decode("\u0026AaCuTe;");
      let t_742 = actual_741 === "\u0026AaCuTe;";
      function fn_743() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026AaCuTe;\u0022) == (" + "\u0026AaCuTe;" + ") not (" + actual_741 + ")";
      }
      test_712.assert(t_742, fn_743);
      const actual_744 = htmlCodec.decode("\u0026gt;;");
      let t_745 = actual_744 === "\u003e;";
      function fn_746() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026gt;;\u0022) == (" + "\u003e;" + ") not (" + actual_744 + ")";
      }
      test_712.assert(t_745, fn_746);
      const actual_747 = htmlCodec.decode("\u0026amp;lt;");
      let t_748 = actual_747 === "\u0026lt;";
      function fn_749() {
        return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0026amp;lt;\u0022) == (" + "\u0026lt;" + ") not (" + actual_747 + ")";
      }
      test_712.assert(t_748, fn_749);
      return;
    } finally {
      test_712.softFailToHard();
    }
});
it("HTML encoding", function () {
    const test_750 = new Test_713();
    try {
      const actual_751 = htmlCodec.encode("");
      let t_752 = actual_751 === "";
      function fn_753() {
        return "expected `-work/src//html/`.htmlCodec.encode(\u0022\u0022) == (" + "" + ") not (" + actual_751 + ")";
      }
      test_750.assert(t_752, fn_753);
      const actual_754 = htmlCodec.encode("Hello, World!");
      let t_755 = actual_754 === "Hello, World!";
      function fn_756() {
        return "expected `-work/src//html/`.htmlCodec.encode(\u0022Hello, World!\u0022) == (" + "Hello, World!" + ") not (" + actual_754 + ")";
      }
      test_750.assert(t_755, fn_756);
      const actual_757 = htmlCodec.encode("\u003cfoo\u003e \u0026 \u003cbar baz='b\u0022oo'\u003e far");
      let t_758 = actual_757 === "\u0026lt;foo\u0026gt; \u0026amp; \u0026lt;bar baz=\u0026#39;b\u0026#34;oo\u0026#39;\u0026gt; far";
      function fn_759() {
        return "expected `-work/src//html/`.htmlCodec.encode(\u0022\u003cfoo\u003e \u0026 \u003cbar baz='b\\\u0022oo'\u003e far\u0022) == (" + "\u0026lt;foo\u0026gt; \u0026amp; \u0026lt;bar baz=\u0026#39;b\u0026#34;oo\u0026#39;\u0026gt; far" + ") not (" + actual_757 + ")";
      }
      test_750.assert(t_758, fn_759);
      return;
    } finally {
      test_750.softFailToHard();
    }
});
it("hello world, html style", function () {
    const test_1140 = new Test_713();
    try {
      let t_1141 = new SafeHtmlBuilder();
      t_1141.appendSafe("Hello, \u003cb\u003e");
      t_1141.appendString("World");
      t_1141.appendSafe("\u003c/b\u003e!");
      const actual_1142 = t_1141.accumulated.toString();
      let t_1143 = actual_1142 === "Hello, \u003cb\u003eWorld\u003c/b\u003e!";
      function fn_1144() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022Hello, \u003cb\u003e\u0022, \\interpolate, \u0022World\u0022, \u0022\u003c/b\u003e!\u0022).toString() == (" + "Hello, \u003cb\u003eWorld\u003c/b\u003e!" + ") not (" + actual_1142 + ")";
      }
      test_1140.assert(t_1143, fn_1144);
      return;
    } finally {
      test_1140.softFailToHard();
    }
});
it("autoescaped", function () {
    const test_1145 = new Test_713();
    try {
      let t_1146 = new SafeHtmlBuilder();
      t_1146.appendSafe("1 + 1 ");
      t_1146.appendString("\u003c");
      t_1146.appendSafe(" 3.");
      const actual_1147 = t_1146.accumulated.toString();
      let t_1148 = actual_1147 === "1 + 1 \u0026lt; 3.";
      function fn_1149() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u00221 + 1 \u0022, \\interpolate, \u0022\u003c\u0022, \u0022 3.\u0022).toString() == (" + "1 + 1 \u0026lt; 3." + ") not (" + actual_1147 + ")";
      }
      test_1145.assert(t_1148, fn_1149);
      return;
    } finally {
      test_1145.softFailToHard();
    }
});
it("context matters -- URLs embed", function () {
    const test_1150 = new Test_713();
    try {
      function okUrl_1151() {
        return "https://example.com/isn't-a-problem";
      }
      function evilUrl_1152() {
        return "javascript:alert('evil done')";
      }
      let t_1153 = new SafeHtmlBuilder();
      t_1153.appendSafe("\u003ca href='");
      t_1153.appendString("https://example.com/isn't-a-problem");
      t_1153.appendSafe("'\u003e");
      t_1153.appendString("https://example.com/isn't-a-problem");
      t_1153.appendSafe("\u003c/a\u003e");
      const actual_1154 = t_1153.accumulated.toString();
      let t_1155 = actual_1154 === "\u003ca href='https://example.com/isn\u0026#39;t-a-problem'\u003ehttps://example.com/isn\u0026#39;t-a-problem\u003c/a\u003e";
      function fn_1156() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003ca href='\u0022, \\interpolate, okUrl(), \u0022'\u003e\u0022, \\interpolate, okUrl(), \u0022\u003c/a\u003e\u0022).toString() == (" + "\u003ca href='https://example.com/isn\u0026#39;t-a-problem'\u003ehttps://example.com/isn\u0026#39;t-a-problem\u003c/a\u003e" + ") not (" + actual_1154 + ")";
      }
      test_1150.assert(t_1155, fn_1156);
      let t_1157 = new SafeHtmlBuilder();
      t_1157.appendSafe("\u003ca href='");
      t_1157.appendString("javascript:alert('evil done')");
      t_1157.appendSafe("'\u003e");
      t_1157.appendString("javascript:alert('evil done')");
      t_1157.appendSafe("\u003c/a\u003e");
      const actual_1158 = t_1157.accumulated.toString();
      let t_1159 = actual_1158 === "\u003ca href='about:zz_Temper_zz#'\u003ejavascript:alert(\u0026#39;evil done\u0026#39;)\u003c/a\u003e";
      function fn_1160() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003ca href='\u0022, \\interpolate, evilUrl(), \u0022'\u003e\u0022, \\interpolate, evilUrl(), \u0022\u003c/a\u003e\u0022).toString() == (" + "\u003ca href='about:zz_Temper_zz#'\u003ejavascript:alert(\u0026#39;evil done\u0026#39;)\u003c/a\u003e" + ") not (" + actual_1158 + ")";
      }
      test_1150.assert(t_1159, fn_1160);
      return;
    } finally {
      test_1150.softFailToHard();
    }
});
it("quote adjustments", function () {
    const test_1161 = new Test_713();
    try {
      const className_1162 = "some-class";
      let t_1163 = new SafeHtmlBuilder();
      t_1163.appendSafe("\u003chr class=");
      t_1163.appendString("some-class");
      t_1163.appendSafe("\u003e\u003chr class='");
      t_1163.appendString("some-class");
      t_1163.appendSafe("'\u003e\u003chr class=other-class\u003e");
      const actual_1164 = t_1163.accumulated.toString();
      let t_1165 = actual_1164 === "\u003chr class=\u0022some-class\u0022\u003e\u003chr class='some-class'\u003e\u003chr class=\u0022other-class\u0022\u003e";
      function fn_1166() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003chr class=\u0022, \\interpolate, className, \u0022\u003e\u003chr class='\u0022, \\interpolate, className, \u0022'\u003e\u003chr class=other-class\u003e\u0022).toString() == (" + "\u003chr class=\u0022some-class\u0022\u003e\u003chr class='some-class'\u003e\u003chr class=\u0022other-class\u0022\u003e" + ") not (" + actual_1164 + ")";
      }
      test_1161.assert(t_1165, fn_1166);
      return;
    } finally {
      test_1161.softFailToHard();
    }
});
it("safehtml injected in tag and attribute context", function () {
    const test_1167 = new Test_713();
    try {
      let t_1168 = new SafeHtmlBuilder();
      t_1168.appendSafe("I \u003c3 \u003cb\u003ePonies\u003c/b\u003e!");
      const love_1169 = t_1168.accumulated;
      let t_1170 = new SafeHtmlBuilder();
      t_1170.appendSafe("\u003cb\u003e");
      t_1170.appendSafeHtml(love_1169);
      t_1170.appendSafe("\u003c/b\u003e\u003cimg alt='");
      t_1170.appendSafeHtml(love_1169);
      t_1170.appendSafe("' src='ponies'\u003e");
      const actual_1171 = t_1170.accumulated.toString();
      let t_1172 = actual_1171 === "\u003cb\u003eI \u0026lt;3 \u003cb\u003ePonies\u003c/b\u003e!\u003c/b\u003e\u003cimg alt='I \u0026lt;3 \u0026lt;b\u0026gt;Ponies\u0026lt;/b\u0026gt;!' src='ponies'\u003e";
      function fn_1173() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003cb\u003e\u0022, \\interpolate, love, \u0022\u003c/b\u003e\u003cimg alt='\u0022, \\interpolate, love, \u0022' src='ponies'\u003e\u0022).toString() == (" + "\u003cb\u003eI \u0026lt;3 \u003cb\u003ePonies\u003c/b\u003e!\u003c/b\u003e\u003cimg alt='I \u0026lt;3 \u0026lt;b\u0026gt;Ponies\u0026lt;/b\u0026gt;!' src='ponies'\u003e" + ") not (" + actual_1171 + ")";
      }
      test_1167.assert(t_1172, fn_1173);
      return;
    } finally {
      test_1167.softFailToHard();
    }
});
it("looping inside an HTML expression", function () {
    const test_1174 = new Test_713();
    try {
      const items_1175 = Object.freeze(["One", "\u003cTwo\u003e", "Three"]);
      const accumulator_1176 = new SafeHtmlBuilder();
      accumulator_1176.appendSafe("\u003cul\u003e\n");
      function fn_1177(item_1178) {
        accumulator_1176.appendSafe("  \u003cli\u003e");
        accumulator_1176.appendString(item_1178);
        accumulator_1176.appendSafe("\u003c/li\u003e\n");
        return;
      }
      items_1175.forEach(fn_1177);
      accumulator_1176.appendSafe("\u003c/ul\u003e");
      const got_1179 = accumulator_1176.accumulated;
      const actual_1180 = got_1179.text;
      let t_1181 = actual_1180 === "\u003cul\u003e\n  \u003cli\u003eOne\u003c/li\u003e\n  \u003cli\u003e\u0026lt;Two\u0026gt;\u003c/li\u003e\n  \u003cli\u003eThree\u003c/li\u003e\n\u003c/ul\u003e";
      function fn_1182() {
        return "expected got.text == (" + "\u003cul\u003e\n  \u003cli\u003eOne\u003c/li\u003e\n  \u003cli\u003e\u0026lt;Two\u0026gt;\u003c/li\u003e\n  \u003cli\u003eThree\u003c/li\u003e\n\u003c/ul\u003e" + ") not (" + actual_1180 + ")";
      }
      test_1174.assert(t_1181, fn_1182);
      return;
    } finally {
      test_1174.softFailToHard();
    }
});
it("double quotes in attribute value with inserted quotes", function () {
    const test_1183 = new Test_713();
    try {
      let t_1184 = new SafeHtmlBuilder();
      t_1184.appendSafe("\u003cdiv id=a\u0022b\u003e");
      const actual_1185 = t_1184.accumulated.text;
      let t_1186 = actual_1185 === "\u003cdiv id=\u0022a\u0026#34;b\u0022\u003e";
      function fn_1187() {
        return "expected stringExpr(`-work/src//html/`.html, true, \u0022\u003cdiv id=a\\\u0022b\u003e\u0022).text == (" + "\u003cdiv id=\u0022a\u0026#34;b\u0022\u003e" + ") not (" + actual_1185 + ")";
      }
      test_1183.assert(t_1186, fn_1187);
      return;
    } finally {
      test_1183.softFailToHard();
    }
});
