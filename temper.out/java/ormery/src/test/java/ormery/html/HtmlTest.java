package ormery.html;
import java.util.function.Supplier;
import temper.std.testing.Test;
import static ormery.html.HtmlGlobal.htmlCodec;
import java.util.List;
import java.util.function.Consumer;
public final class HtmlTest {
    private HtmlTest() {
    }
    @org.junit.jupiter.api.Test public void htmlDecoding__804() {
        Test test_9 = new Test();
        try {
            String actual_805 = htmlCodec.decode("");
            boolean t_5597 = actual_805.equals("");
            Supplier<String> fn__5594 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"\") == (" + "" + ") not (" + actual_805 + ")";
            test_9.assert_(t_5597, fn__5594);
            String actual_807 = htmlCodec.decode("&l");
            boolean t_5601 = actual_807.equals("&l");
            Supplier<String> fn__5593 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&l\") == (" + "&l" + ") not (" + actual_807 + ")";
            test_9.assert_(t_5601, fn__5593);
            String actual_809 = htmlCodec.decode("&lt");
            boolean t_5605 = actual_809.equals("<");
            Supplier<String> fn__5592 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&lt\") == (" + "<" + ") not (" + actual_809 + ")";
            test_9.assert_(t_5605, fn__5592);
            String actual_811 = htmlCodec.decode("&lt;");
            boolean t_5609 = actual_811.equals("<");
            Supplier<String> fn__5591 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&lt;\") == (" + "<" + ") not (" + actual_811 + ")";
            test_9.assert_(t_5609, fn__5591);
            String actual_813 = htmlCodec.decode("&Bcy;");
            boolean t_5613 = actual_813.equals("\u0411");
            Supplier<String> fn__5590 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&Bcy;\") == (" + "\u0411" + ") not (" + actual_813 + ")";
            test_9.assert_(t_5613, fn__5590);
            String actual_815 = htmlCodec.decode("&Bcy");
            boolean t_5617 = actual_815.equals("&Bcy");
            Supplier<String> fn__5589 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&Bcy\") == (" + "&Bcy" + ") not (" + actual_815 + ")";
            test_9.assert_(t_5617, fn__5589);
            String actual_817 = htmlCodec.decode("&LT;");
            boolean t_5621 = actual_817.equals("<");
            Supplier<String> fn__5588 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&LT;\") == (" + "<" + ") not (" + actual_817 + ")";
            test_9.assert_(t_5621, fn__5588);
            String actual_819 = htmlCodec.decode("&Aacute;");
            boolean t_5625 = actual_819.equals("\u00c1");
            Supplier<String> fn__5587 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&Aacute;\") == (" + "\u00c1" + ") not (" + actual_819 + ")";
            test_9.assert_(t_5625, fn__5587);
            String actual_821 = htmlCodec.decode("&aacute;");
            boolean t_5629 = actual_821.equals("\u00e1");
            Supplier<String> fn__5586 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&aacute;\") == (" + "\u00e1" + ") not (" + actual_821 + ")";
            test_9.assert_(t_5629, fn__5586);
            String actual_823 = htmlCodec.decode("&AaCuTe;");
            boolean t_5633 = actual_823.equals("&AaCuTe;");
            Supplier<String> fn__5585 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&AaCuTe;\") == (" + "&AaCuTe;" + ") not (" + actual_823 + ")";
            test_9.assert_(t_5633, fn__5585);
            String actual_825 = htmlCodec.decode("&gt;;");
            boolean t_5637 = actual_825.equals(">;");
            Supplier<String> fn__5584 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&gt;;\") == (" + ">;" + ") not (" + actual_825 + ")";
            test_9.assert_(t_5637, fn__5584);
            String actual_827 = htmlCodec.decode("&amp;lt;");
            boolean t_5641 = actual_827.equals("&lt;");
            Supplier<String> fn__5583 = () -> "expected `-work/src//html/`.htmlCodec.decode(\"&amp;lt;\") == (" + "&lt;" + ") not (" + actual_827 + ")";
            test_9.assert_(t_5641, fn__5583);
        } finally {
            test_9.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void htmlEncoding__829() {
        Test test_10 = new Test();
        try {
            String actual_830 = htmlCodec.encode("");
            boolean t_5573 = actual_830.equals("");
            Supplier<String> fn__5570 = () -> "expected `-work/src//html/`.htmlCodec.encode(\"\") == (" + "" + ") not (" + actual_830 + ")";
            test_10.assert_(t_5573, fn__5570);
            String actual_832 = htmlCodec.encode("Hello, World!");
            boolean t_5577 = actual_832.equals("Hello, World!");
            Supplier<String> fn__5569 = () -> "expected `-work/src//html/`.htmlCodec.encode(\"Hello, World!\") == (" + "Hello, World!" + ") not (" + actual_832 + ")";
            test_10.assert_(t_5577, fn__5569);
            String actual_834 = htmlCodec.encode("<foo> & <bar baz='b\"oo'> far");
            boolean t_5581 = actual_834.equals("&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far");
            Supplier<String> fn__5568 = () -> "expected `-work/src//html/`.htmlCodec.encode(\"<foo> & <bar baz='b\\\"oo'> far\") == (" + "&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far" + ") not (" + actual_834 + ")";
            test_10.assert_(t_5581, fn__5568);
        } finally {
            test_10.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void helloWorldHtmlStyle__899() {
        Test test_32 = new Test();
        try {
            SafeHtmlBuilder t_4675 = new SafeHtmlBuilder();
            t_4675.appendSafe("Hello, <b>");
            t_4675.appendString("World");
            t_4675.appendSafe("</b>!");
            String actual_900 = t_4675.getAccumulated().toString();
            boolean t_4682 = actual_900.equals("Hello, <b>World</b>!");
            Supplier<String> fn__4674 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"Hello, <b>\", \\interpolate, \"World\", \"</b>!\").toString() == (" + "Hello, <b>World</b>!" + ") not (" + actual_900 + ")";
            test_32.assert_(t_4682, fn__4674);
        } finally {
            test_32.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void autoescaped__903() {
        Test test_33 = new Test();
        try {
            SafeHtmlBuilder t_4665 = new SafeHtmlBuilder();
            t_4665.appendSafe("1 + 1 ");
            t_4665.appendString("<");
            t_4665.appendSafe(" 3.");
            String actual_904 = t_4665.getAccumulated().toString();
            boolean t_4672 = actual_904.equals("1 + 1 &lt; 3.");
            Supplier<String> fn__4664 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"1 + 1 \", \\interpolate, \"<\", \" 3.\").toString() == (" + "1 + 1 &lt; 3." + ") not (" + actual_904 + ")";
            test_33.assert_(t_4672, fn__4664);
        } finally {
            test_33.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void contextMattersUrlsEmbed__907() {
        Test test_34 = new Test();
        try {
            Supplier<String> okUrl__483 = () -> "https://example.com/isn't-a-problem";
            Supplier<String> evilUrl__484 = () -> "javascript:alert('evil done')";
            SafeHtmlBuilder t_4642 = new SafeHtmlBuilder();
            t_4642.appendSafe("<a href='");
            t_4642.appendString("https://example.com/isn't-a-problem");
            t_4642.appendSafe("'>");
            t_4642.appendString("https://example.com/isn't-a-problem");
            t_4642.appendSafe("</a>");
            String actual_908 = t_4642.getAccumulated().toString();
            boolean t_4651 = actual_908.equals("<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>");
            Supplier<String> fn__4641 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"<a href='\", \\interpolate, okUrl(), \"'>\", \\interpolate, okUrl(), \"</a>\").toString() == (" + "<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>" + ") not (" + actual_908 + ")";
            test_34.assert_(t_4651, fn__4641);
            SafeHtmlBuilder t_4653 = new SafeHtmlBuilder();
            t_4653.appendSafe("<a href='");
            t_4653.appendString("javascript:alert('evil done')");
            t_4653.appendSafe("'>");
            t_4653.appendString("javascript:alert('evil done')");
            t_4653.appendSafe("</a>");
            String actual_911 = t_4653.getAccumulated().toString();
            boolean t_4662 = actual_911.equals("<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>");
            Supplier<String> fn__4640 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"<a href='\", \\interpolate, evilUrl(), \"'>\", \\interpolate, evilUrl(), \"</a>\").toString() == (" + "<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>" + ") not (" + actual_911 + ")";
            test_34.assert_(t_4662, fn__4640);
        } finally {
            test_34.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void quoteAdjustments__914() {
        Test test_35 = new Test();
        try {
            String className__488 = "some-class";
            SafeHtmlBuilder t_4629 = new SafeHtmlBuilder();
            t_4629.appendSafe("<hr class=");
            t_4629.appendString("some-class");
            t_4629.appendSafe("><hr class='");
            t_4629.appendString("some-class");
            t_4629.appendSafe("'><hr class=other-class>");
            String actual_915 = t_4629.getAccumulated().toString();
            boolean t_4638 = actual_915.equals("<hr class=\"some-class\"><hr class='some-class'><hr class=\"other-class\">");
            Supplier<String> fn__4628 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"<hr class=\", \\interpolate, className, \"><hr class='\", \\interpolate, className, \"'><hr class=other-class>\").toString() == (" + "<hr class=\"some-class\"><hr class='some-class'><hr class=\"other-class\">" + ") not (" + actual_915 + ")";
            test_35.assert_(t_4638, fn__4628);
        } finally {
            test_35.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safehtmlInjectedInTagAndAttributeContext__918() {
        Test test_36 = new Test();
        try {
            SafeHtmlBuilder t_4614 = new SafeHtmlBuilder();
            t_4614.appendSafe("I <3 <b>Ponies</b>!");
            SafeHtml love__490 = t_4614.getAccumulated();
            SafeHtmlBuilder t_4617 = new SafeHtmlBuilder();
            t_4617.appendSafe("<b>");
            t_4617.appendSafeHtml(love__490);
            t_4617.appendSafe("</b><img alt='");
            t_4617.appendSafeHtml(love__490);
            t_4617.appendSafe("' src='ponies'>");
            String actual_920 = t_4617.getAccumulated().toString();
            boolean t_4626 = actual_920.equals("<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>");
            Supplier<String> fn__4613 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"<b>\", \\interpolate, love, \"</b><img alt='\", \\interpolate, love, \"' src='ponies'>\").toString() == (" + "<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>" + ") not (" + actual_920 + ")";
            test_36.assert_(t_4626, fn__4613);
        } finally {
            test_36.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void loopingInsideAnHtmlExpression__923() {
        Test test_37 = new Test();
        try {
            List<String> items__492 = List.of("One", "<Two>", "Three");
            SafeHtmlBuilder accumulator_38 = new SafeHtmlBuilder();
            accumulator_38.appendSafe("<ul>\n");
            Consumer<String> fn__4602 = item__495 -> {
                accumulator_38.appendSafe("  <li>");
                accumulator_38.appendString(item__495);
                accumulator_38.appendSafe("</li>\n");
            };
            items__492.forEach(fn__4602);
            accumulator_38.appendSafe("</ul>");
            SafeHtml got__493 = accumulator_38.getAccumulated();
            String actual_924 = got__493.getText();
            boolean t_4611 = actual_924.equals("<ul>\n  <li>One</li>\n  <li>&lt;Two&gt;</li>\n  <li>Three</li>\n</ul>");
            Supplier<String> fn__4601 = () -> "expected got.text == (" + "<ul>\n  <li>One</li>\n  <li>&lt;Two&gt;</li>\n  <li>Three</li>\n</ul>" + ") not (" + actual_924 + ")";
            test_37.assert_(t_4611, fn__4601);
        } finally {
            test_37.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void doubleQuotesInAttributeValueWithInsertedQuotes__935() {
        Test test_52 = new Test();
        try {
            SafeHtmlBuilder t_4589 = new SafeHtmlBuilder();
            t_4589.appendSafe("<div id=a\"b>");
            String actual_936 = t_4589.getAccumulated().getText();
            boolean t_4594 = actual_936.equals("<div id=\"a&#34;b\">");
            Supplier<String> fn__4588 = () -> "expected stringExpr(`-work/src//html/`.html, true, \"<div id=a\\\"b>\").text == (" + "<div id=\"a&#34;b\">" + ") not (" + actual_936 + ")";
            test_52.assert_(t_4594, fn__4588);
        } finally {
            test_52.softFailToHard();
        }
    }
}
