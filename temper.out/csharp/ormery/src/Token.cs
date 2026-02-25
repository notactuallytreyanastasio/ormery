using H = Ormery.Html;
namespace Ormery
{
    public class Token
    {
        readonly TokenType tokenType__674;
        readonly string value__675;
        public string CssClass()
        {
            string return__185;
            string name__678 = this.tokenType__674.Name;
            if (name__678 == "keyword")
            {
                return__185 = "kw";
            }
            else if (name__678 == "type")
            {
                return__185 = "typ";
            }
            else if (name__678 == "string")
            {
                return__185 = "str";
            }
            else if (name__678 == "number")
            {
                return__185 = "num";
            }
            else if (name__678 == "comment")
            {
                return__185 = "cmt";
            }
            else if (name__678 == "operator")
            {
                return__185 = "op";
            }
            else
            {
                return__185 = "id";
            }
            return return__185;
        }
        public H::SafeHtml ToHtml()
        {
            string cls__681 = this.CssClass();
            H::SafeHtmlBuilder t___3288 = new H::SafeHtmlBuilder();
            t___3288.AppendSafe("<span class='");
            t___3288.AppendString(cls__681);
            t___3288.AppendSafe("'>");
            t___3288.AppendString(this.value__675);
            t___3288.AppendSafe("</span>");
            return t___3288.Accumulated;
        }
        public Token(TokenType tokenType__683, string value__684)
        {
            this.tokenType__674 = tokenType__683;
            this.value__675 = value__684;
        }
        public TokenType TokenType
        {
            get
            {
                return this.tokenType__674;
            }
        }
        public string Value
        {
            get
            {
                return this.value__675;
            }
        }
    }
}
