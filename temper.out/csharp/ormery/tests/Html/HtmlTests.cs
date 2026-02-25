using U = Microsoft.VisualStudio.TestTools.UnitTesting;
using H = Ormery.Html;
using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
using T = TemperLang.Std.Testing;
namespace Ormery.Html
{
    [U::TestClass]
    public class HtmlTests
    {
        [U::TestMethod]
        public void htmlDecoding__804()
        {
            T::Test test___9 = new T::Test();
            try
            {
                string actual___805 = H::HtmlGlobal.HtmlCodec.Decode("");
                bool t___5597 = actual___805 == "";
                string fn__5594()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022\u0022) == (" + "" + ") not (" + actual___805 + ")";
                }
                test___9.Assert(t___5597, (S::Func<string>) fn__5594);
                string actual___807 = H::HtmlGlobal.HtmlCodec.Decode("&l");
                bool t___5601 = actual___807 == "&l";
                string fn__5593()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&l\u0022) == (" + "&l" + ") not (" + actual___807 + ")";
                }
                test___9.Assert(t___5601, (S::Func<string>) fn__5593);
                string actual___809 = H::HtmlGlobal.HtmlCodec.Decode("&lt");
                bool t___5605 = actual___809 == "<";
                string fn__5592()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&lt\u0022) == (" + "<" + ") not (" + actual___809 + ")";
                }
                test___9.Assert(t___5605, (S::Func<string>) fn__5592);
                string actual___811 = H::HtmlGlobal.HtmlCodec.Decode("&lt;");
                bool t___5609 = actual___811 == "<";
                string fn__5591()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&lt;\u0022) == (" + "<" + ") not (" + actual___811 + ")";
                }
                test___9.Assert(t___5609, (S::Func<string>) fn__5591);
                string actual___813 = H::HtmlGlobal.HtmlCodec.Decode("&Bcy;");
                bool t___5613 = actual___813 == "Б";
                string fn__5590()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&Bcy;\u0022) == (" + "Б" + ") not (" + actual___813 + ")";
                }
                test___9.Assert(t___5613, (S::Func<string>) fn__5590);
                string actual___815 = H::HtmlGlobal.HtmlCodec.Decode("&Bcy");
                bool t___5617 = actual___815 == "&Bcy";
                string fn__5589()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&Bcy\u0022) == (" + "&Bcy" + ") not (" + actual___815 + ")";
                }
                test___9.Assert(t___5617, (S::Func<string>) fn__5589);
                string actual___817 = H::HtmlGlobal.HtmlCodec.Decode("&LT;");
                bool t___5621 = actual___817 == "<";
                string fn__5588()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&LT;\u0022) == (" + "<" + ") not (" + actual___817 + ")";
                }
                test___9.Assert(t___5621, (S::Func<string>) fn__5588);
                string actual___819 = H::HtmlGlobal.HtmlCodec.Decode("&Aacute;");
                bool t___5625 = actual___819 == "Á";
                string fn__5587()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&Aacute;\u0022) == (" + "Á" + ") not (" + actual___819 + ")";
                }
                test___9.Assert(t___5625, (S::Func<string>) fn__5587);
                string actual___821 = H::HtmlGlobal.HtmlCodec.Decode("&aacute;");
                bool t___5629 = actual___821 == "á";
                string fn__5586()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&aacute;\u0022) == (" + "á" + ") not (" + actual___821 + ")";
                }
                test___9.Assert(t___5629, (S::Func<string>) fn__5586);
                string actual___823 = H::HtmlGlobal.HtmlCodec.Decode("&AaCuTe;");
                bool t___5633 = actual___823 == "&AaCuTe;";
                string fn__5585()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&AaCuTe;\u0022) == (" + "&AaCuTe;" + ") not (" + actual___823 + ")";
                }
                test___9.Assert(t___5633, (S::Func<string>) fn__5585);
                string actual___825 = H::HtmlGlobal.HtmlCodec.Decode("&gt;;");
                bool t___5637 = actual___825 == ">;";
                string fn__5584()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&gt;;\u0022) == (" + ">;" + ") not (" + actual___825 + ")";
                }
                test___9.Assert(t___5637, (S::Func<string>) fn__5584);
                string actual___827 = H::HtmlGlobal.HtmlCodec.Decode("&amp;lt;");
                bool t___5641 = actual___827 == "&lt;";
                string fn__5583()
                {
                    return "expected `-work/src//html/`.htmlCodec.decode(\u0022&amp;lt;\u0022) == (" + "&lt;" + ") not (" + actual___827 + ")";
                }
                test___9.Assert(t___5641, (S::Func<string>) fn__5583);
            }
            finally
            {
                test___9.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void htmlEncoding__829()
        {
            T::Test test___10 = new T::Test();
            try
            {
                string actual___830 = H::HtmlGlobal.HtmlCodec.Encode("");
                bool t___5573 = actual___830 == "";
                string fn__5570()
                {
                    return "expected `-work/src//html/`.htmlCodec.encode(\u0022\u0022) == (" + "" + ") not (" + actual___830 + ")";
                }
                test___10.Assert(t___5573, (S::Func<string>) fn__5570);
                string actual___832 = H::HtmlGlobal.HtmlCodec.Encode("Hello, World!");
                bool t___5577 = actual___832 == "Hello, World!";
                string fn__5569()
                {
                    return "expected `-work/src//html/`.htmlCodec.encode(\u0022Hello, World!\u0022) == (" + "Hello, World!" + ") not (" + actual___832 + ")";
                }
                test___10.Assert(t___5577, (S::Func<string>) fn__5569);
                string actual___834 = H::HtmlGlobal.HtmlCodec.Encode("<foo> & <bar baz='b\u0022oo'> far");
                bool t___5581 = actual___834 == "&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far";
                string fn__5568()
                {
                    return "expected `-work/src//html/`.htmlCodec.encode(\u0022<foo> & <bar baz='b\\\u0022oo'> far\u0022) == (" + "&lt;foo&gt; &amp; &lt;bar baz=&#39;b&#34;oo&#39;&gt; far" + ") not (" + actual___834 + ")";
                }
                test___10.Assert(t___5581, (S::Func<string>) fn__5568);
            }
            finally
            {
                test___10.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void helloWorldHtmlStyle__899()
        {
            T::Test test___32 = new T::Test();
            try
            {
                SafeHtmlBuilder t___4675 = new SafeHtmlBuilder();
                t___4675.AppendSafe("Hello, <b>");
                t___4675.AppendString("World");
                t___4675.AppendSafe("</b>!");
                string actual___900 = t___4675.Accumulated.ToString();
                bool t___4682 = actual___900 == "Hello, <b>World</b>!";
                string fn__4674()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u0022Hello, <b>\u0022, \\interpolate, \u0022World\u0022, \u0022</b>!\u0022).toString() == (" + "Hello, <b>World</b>!" + ") not (" + actual___900 + ")";
                }
                test___32.Assert(t___4682, (S::Func<string>) fn__4674);
            }
            finally
            {
                test___32.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void autoescaped__903()
        {
            T::Test test___33 = new T::Test();
            try
            {
                SafeHtmlBuilder t___4665 = new SafeHtmlBuilder();
                t___4665.AppendSafe("1 + 1 ");
                t___4665.AppendString("<");
                t___4665.AppendSafe(" 3.");
                string actual___904 = t___4665.Accumulated.ToString();
                bool t___4672 = actual___904 == "1 + 1 &lt; 3.";
                string fn__4664()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u00221 + 1 \u0022, \\interpolate, \u0022<\u0022, \u0022 3.\u0022).toString() == (" + "1 + 1 &lt; 3." + ") not (" + actual___904 + ")";
                }
                test___33.Assert(t___4672, (S::Func<string>) fn__4664);
            }
            finally
            {
                test___33.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void contextMattersUrlsEmbed__907()
        {
            T::Test test___34 = new T::Test();
            try
            {
                string okUrl__483()
                {
                    return "https://example.com/isn't-a-problem";
                }
                string evilUrl__484()
                {
                    return "javascript:alert('evil done')";
                }
                SafeHtmlBuilder t___4642 = new SafeHtmlBuilder();
                t___4642.AppendSafe("<a href='");
                t___4642.AppendString("https://example.com/isn't-a-problem");
                t___4642.AppendSafe("'>");
                t___4642.AppendString("https://example.com/isn't-a-problem");
                t___4642.AppendSafe("</a>");
                string actual___908 = t___4642.Accumulated.ToString();
                bool t___4651 = actual___908 == "<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>";
                string fn__4641()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u0022<a href='\u0022, \\interpolate, okUrl(), \u0022'>\u0022, \\interpolate, okUrl(), \u0022</a>\u0022).toString() == (" + "<a href='https://example.com/isn&#39;t-a-problem'>https://example.com/isn&#39;t-a-problem</a>" + ") not (" + actual___908 + ")";
                }
                test___34.Assert(t___4651, (S::Func<string>) fn__4641);
                SafeHtmlBuilder t___4653 = new SafeHtmlBuilder();
                t___4653.AppendSafe("<a href='");
                t___4653.AppendString("javascript:alert('evil done')");
                t___4653.AppendSafe("'>");
                t___4653.AppendString("javascript:alert('evil done')");
                t___4653.AppendSafe("</a>");
                string actual___911 = t___4653.Accumulated.ToString();
                bool t___4662 = actual___911 == "<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>";
                string fn__4640()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u0022<a href='\u0022, \\interpolate, evilUrl(), \u0022'>\u0022, \\interpolate, evilUrl(), \u0022</a>\u0022).toString() == (" + "<a href='about:zz_Temper_zz#'>javascript:alert(&#39;evil done&#39;)</a>" + ") not (" + actual___911 + ")";
                }
                test___34.Assert(t___4662, (S::Func<string>) fn__4640);
            }
            finally
            {
                test___34.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void quoteAdjustments__914()
        {
            T::Test test___35 = new T::Test();
            try
            {
                string className__488 = "some-class";
                SafeHtmlBuilder t___4629 = new SafeHtmlBuilder();
                t___4629.AppendSafe("<hr class=");
                t___4629.AppendString("some-class");
                t___4629.AppendSafe("><hr class='");
                t___4629.AppendString("some-class");
                t___4629.AppendSafe("'><hr class=other-class>");
                string actual___915 = t___4629.Accumulated.ToString();
                bool t___4638 = actual___915 == "<hr class=\u0022some-class\u0022><hr class='some-class'><hr class=\u0022other-class\u0022>";
                string fn__4628()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u0022<hr class=\u0022, \\interpolate, className, \u0022><hr class='\u0022, \\interpolate, className, \u0022'><hr class=other-class>\u0022).toString() == (" + "<hr class=\u0022some-class\u0022><hr class='some-class'><hr class=\u0022other-class\u0022>" + ") not (" + actual___915 + ")";
                }
                test___35.Assert(t___4638, (S::Func<string>) fn__4628);
            }
            finally
            {
                test___35.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safehtmlInjectedInTagAndAttributeContext__918()
        {
            T::Test test___36 = new T::Test();
            try
            {
                SafeHtmlBuilder t___4614 = new SafeHtmlBuilder();
                t___4614.AppendSafe("I <3 <b>Ponies</b>!");
                SafeHtml love__490 = t___4614.Accumulated;
                SafeHtmlBuilder t___4617 = new SafeHtmlBuilder();
                t___4617.AppendSafe("<b>");
                t___4617.AppendSafeHtml(love__490);
                t___4617.AppendSafe("</b><img alt='");
                t___4617.AppendSafeHtml(love__490);
                t___4617.AppendSafe("' src='ponies'>");
                string actual___920 = t___4617.Accumulated.ToString();
                bool t___4626 = actual___920 == "<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>";
                string fn__4613()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u0022<b>\u0022, \\interpolate, love, \u0022</b><img alt='\u0022, \\interpolate, love, \u0022' src='ponies'>\u0022).toString() == (" + "<b>I &lt;3 <b>Ponies</b>!</b><img alt='I &lt;3 &lt;b&gt;Ponies&lt;/b&gt;!' src='ponies'>" + ") not (" + actual___920 + ")";
                }
                test___36.Assert(t___4626, (S::Func<string>) fn__4613);
            }
            finally
            {
                test___36.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void loopingInsideAnHtmlExpression__923()
        {
            T::Test test___37 = new T::Test();
            try
            {
                G::IReadOnlyList<string> items__492 = C::Listed.CreateReadOnlyList<string>("One", "<Two>", "Three");
                SafeHtmlBuilder accumulator___38 = new SafeHtmlBuilder();
                accumulator___38.AppendSafe("<ul>\n");
                void fn__4602(string item__495)
                {
                    accumulator___38.AppendSafe("  <li>");
                    accumulator___38.AppendString(item__495);
                    accumulator___38.AppendSafe("</li>\n");
                }
                C::Listed.ForEach(items__492, (S::Action<string>) fn__4602);
                accumulator___38.AppendSafe("</ul>");
                SafeHtml got__493 = accumulator___38.Accumulated;
                string actual___924 = got__493.Text;
                bool t___4611 = actual___924 == "<ul>\n  <li>One</li>\n  <li>&lt;Two&gt;</li>\n  <li>Three</li>\n</ul>";
                string fn__4601()
                {
                    return "expected got.text == (" + "<ul>\n  <li>One</li>\n  <li>&lt;Two&gt;</li>\n  <li>Three</li>\n</ul>" + ") not (" + actual___924 + ")";
                }
                test___37.Assert(t___4611, (S::Func<string>) fn__4601);
            }
            finally
            {
                test___37.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void doubleQuotesInAttributeValueWithInsertedQuotes__935()
        {
            T::Test test___52 = new T::Test();
            try
            {
                SafeHtmlBuilder t___4589 = new SafeHtmlBuilder();
                t___4589.AppendSafe("<div id=a\u0022b>");
                string actual___936 = t___4589.Accumulated.Text;
                bool t___4594 = actual___936 == "<div id=\u0022a&#34;b\u0022>";
                string fn__4588()
                {
                    return "expected stringExpr(`-work/src//html/`.html, true, \u0022<div id=a\\\u0022b>\u0022).text == (" + "<div id=\u0022a&#34;b\u0022>" + ") not (" + actual___936 + ")";
                }
                test___52.Assert(t___4594, (S::Func<string>) fn__4588);
            }
            finally
            {
                test___52.SoftFailToHard();
            }
        }
    }
}
