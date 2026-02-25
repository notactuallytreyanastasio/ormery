using C = Ormery.Core;
namespace Ormery.Html
{
    public interface IUrlEscaper: C::IEscaper
    {
        SafeUrl ApplySafeUrl(SafeUrl x__714);
        SafeUrl ApplyString(string x__717);
    }
}
