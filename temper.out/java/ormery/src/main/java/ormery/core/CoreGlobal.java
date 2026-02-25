package ormery.core;
import temper.core.Core;
import java.util.logging.Logger;
import temper.core.Nullable;
public final class CoreGlobal {
    private CoreGlobal() {
    }
    static String feedSubsidiary__83(Subsidiary subsidiary__176, String adjustedStr__177, boolean prepareForInterp__178) {
        String t_513;
        String str__180 = adjustedStr__177;
        Delegate delegate__181 = subsidiary__176.getDelegate();
        Codec codec__182 = subsidiary__176.getCodec();
        String t_511 = codec__182.decode(str__180);
        str__180 = t_511;
        String t_512 = delegate__181.process(str__180);
        str__180 = t_512;
        if (prepareForInterp__178) {
            t_513 = delegate__181.process(null);
            str__180 = str__180 + t_513;
        }
        String t_514 = codec__182.encode(str__180);
        str__180 = t_514;
        return str__180;
    }
    /**
     * propagateOver is used by both the accumulators and context delegates to propagate context over
     * a known safe chunk.  Propagation is usual defined in terms of transition tables, and each
     * transition consumes some prefix of the remaining content, so this does enough transitions to
     * process the entire chunk.
     */
    public static<CONTEXT__23 extends ormery.core.Context> AfterPropagate<CONTEXT__23> propagateOver(ContextPropagator<CONTEXT__23> contextPropagator__144, AutoescState<CONTEXT__23> before__145, @Nullable String known__146) {
        AfterPropagate<CONTEXT__23> return__66;
        AutoescState<CONTEXT__23> t_488;
        int t_489;
        int t_490;
        String t_491;
        String t_492;
        int t_493;
        if (known__146 == null) {
            return__66 = contextPropagator__144.after(before__145, null);
        } else {
            String known_238 = known__146;
            AutoescState<CONTEXT__23> state__148 = before__145;
            String remainder__149 = known_238;
            StringBuilder adjusted__150 = new StringBuilder();
            while (true) {
                if (remainder__149.isEmpty()) {
                    break;
                }
                AfterPropagate<CONTEXT__23> after__151 = contextPropagator__144.after(state__148, remainder__149);
                adjusted__150.append(after__151.getAdjustedString());
                t_488 = after__151.getStateAfter();
                state__148 = t_488;
                t_489 = after__151.getConsumed();
                t_490 = remainder__149.length();
                t_491 = Core.stringSlice(remainder__149, t_489, t_490);
                remainder__149 = t_491;
            }
            t_492 = adjusted__150.toString();
            t_493 = known_238.length();
            return__66 = new AfterPropagate<>(t_492, t_493, state__148);
        }
        return return__66;
    }
    static {
        Core.getConsole(Logger.getLogger("ormery.core"));
    }
}
