namespace Ormery.Html
{
    public class HtmlCssDelegate: IHtmlDelegate
    {
        public string Process(string ? s__765)
        {
            string return__254;
            if (!(s__765 == null))
            {
                return__254 = s__765!;
            }
            else
            {
                return__254 = "";
            }
            return return__254;
        }
        public IHtmlEscaper ? Escaper(IHtmlEscaper outer__768)
        {
            return outer__768;
        }
        public HtmlCssDelegate()
        {
        }
    }
}
