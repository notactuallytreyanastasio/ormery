using C = Ormery.Core;
using H = Ormery.Html;
namespace Ormery.Html
{
    public class HtmlEscaperContext: C::IContext
    {
        readonly int htmlState__496;
        readonly int tagState__497;
        readonly int attribState__498;
        readonly int delimState__499;
        public string ToString()
        {
            return "HtmlEscaperContext(" + H::HtmlGlobal.htmlStateStr__260(this.htmlState__496) + ", " + H::HtmlGlobal.tagStateStr__261(this.tagState__497) + ", " + H::HtmlGlobal.attribStateStr__262(this.attribState__498) + ", " + H::HtmlGlobal.delimStateStr__263(this.delimState__499) + ")";
        }
        public HtmlEscaperContext(int htmlState__503, int tagState__504, int attribState__505, int delimState__506)
        {
            this.htmlState__496 = htmlState__503;
            this.tagState__497 = tagState__504;
            this.attribState__498 = attribState__505;
            this.delimState__499 = delimState__506;
        }
        public int HtmlState
        {
            get
            {
                return this.htmlState__496;
            }
        }
        public int TagState
        {
            get
            {
                return this.tagState__497;
            }
        }
        public int AttribState
        {
            get
            {
                return this.attribState__498;
            }
        }
        public int DelimState
        {
            get
            {
                return this.delimState__499;
            }
        }
    }
}
