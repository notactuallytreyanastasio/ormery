namespace Ormery.Html
{
    public class OutputHtmlSpaceEscaper: IHtmlEscaper
    {
        public static readonly OutputHtmlSpaceEscaper Instance = new OutputHtmlSpaceEscaper();
        public string ApplySafeHtml(SafeHtml x__554)
        {
            return " ";
        }
        public string ApplySafeUrl(SafeUrl x__557)
        {
            return " ";
        }
        public string ApplyInt32(int x__560)
        {
            return " ";
        }
        public string ApplyInt64(long x__563)
        {
            return " ";
        }
        public string ApplyFloat64(double x__566)
        {
            return " ";
        }
        public string ApplyString(string x__569)
        {
            return " ";
        }
        public OutputHtmlSpaceEscaper()
        {
        }
    }
}
