using C = Ormery.Core;
namespace Ormery.Html
{
    public interface IHtmlEscaper: C::IEscaper
    {
        string ApplySafeHtml(SafeHtml x__535);
        string ApplySafeUrl(SafeUrl x__538);
        string ApplyInt32(int x__541);
        string ApplyInt64(long x__544);
        string ApplyFloat64(double x__547);
        string ApplyString(string x__550);
    }
}
