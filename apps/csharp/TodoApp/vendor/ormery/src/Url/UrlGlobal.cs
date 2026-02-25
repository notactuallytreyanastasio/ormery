using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Url
{
    public static class UrlGlobal
    {
        internal static void appendHex__3(int n__7, T::StringBuilder sb__8)
        {
            int t___26;
            int i__10 = n__7 & 15;
            if (i__10 < 10)
            {
                t___26 = 48;
            }
            else
            {
                t___26 = 87;
            }
            C::StringUtil.AppendCodePoint(sb__8, i__10 + t___26);
        }
        public static void PercentEscapeOctetTo(int octet__4, T::StringBuilder sb__5)
        {
            sb__5.Append("%");
            int t___21 = C::Core.DivSafe(octet__4 & 255, 16);
            appendHex__3(t___21, sb__5);
            int t___23 = octet__4 & 15;
            appendHex__3(t___23, sb__5);
        }
    }
}
