using S = Ormery.Support;
using T = System.Text;
using C = TemperLang.Core;
namespace Ormery.Core
{
    public static class CoreGlobal
    {
        internal static string feedSubsidiary__83(Subsidiary subsidiary__176, string adjustedStr__177, bool prepareForInterp__178)
        {
            string t___513;
            string str__180 = adjustedStr__177;
            IDelegate delegate__181 = subsidiary__176.Delegate;
            ICodec codec__182 = subsidiary__176.Codec;
            string t___511 = codec__182.Decode(str__180);
            str__180 = t___511;
            string t___512 = delegate__181.Process(str__180);
            str__180 = t___512;
            if (prepareForInterp__178)
            {
                t___513 = delegate__181.Process(null);
                str__180 = str__180 + t___513;
            }
            string t___514 = codec__182.Encode(str__180);
            str__180 = t___514;
            return str__180;
        }
        public static AfterPropagate<CONTEXT__23> PropagateOver<CONTEXT__23>(IContextPropagator<CONTEXT__23> contextPropagator__144, AutoescState<CONTEXT__23> before__145, string ? known__146) where CONTEXT__23: IContext
        {
            AfterPropagate<CONTEXT__23> return__66;
            AutoescState<CONTEXT__23> t___488;
            int t___489;
            int t___490;
            string t___491;
            string t___492;
            int t___493;
            if (known__146 == null)
            {
                return__66 = contextPropagator__144.After(before__145, null);
            }
            else
            {
                string known___238 = known__146!;
                AutoescState<CONTEXT__23> state__148 = before__145;
                string remainder__149 = known___238;
                T::StringBuilder adjusted__150 = new T::StringBuilder();
                while (true)
                {
                    if (string.IsNullOrEmpty(remainder__149)) break;
                    AfterPropagate<CONTEXT__23> after__151 = contextPropagator__144.After(state__148, remainder__149);
                    adjusted__150.Append(after__151.AdjustedString);
                    t___488 = after__151.StateAfter;
                    state__148 = t___488;
                    t___489 = after__151.Consumed;
                    t___490 = remainder__149.Length;
                    t___491 = C::StringUtil.Slice(remainder__149, t___489, t___490);
                    remainder__149 = t___491;
                }
                t___492 = adjusted__150.ToString();
                t___493 = known___238.Length;
                return__66 = new AfterPropagate<CONTEXT__23>(t___492, t___493, state__148);
            }
            return return__66;
        }
        static CoreGlobal()
        {
            S::Logging.LoggingConsoleFactory.CreateConsole("Ormery.Core");
        }
    }
}
