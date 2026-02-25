namespace Ormery.Html
{
    public class HtmlJsDelegate: IHtmlDelegate
    {
        public string Process(string ? s__772)
        {
            string return__258;
            if (!(s__772 == null))
            {
                return__258 = s__772!;
            }
            else
            {
                return__258 = "";
            }
            return return__258;
        }
        public IHtmlEscaper ? Escaper(IHtmlEscaper outer__775)
        {
            return outer__775;
        }
        public HtmlJsDelegate()
        {
        }
    }
}
