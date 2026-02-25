package ormery.core;
import temper.core.Nullable;
/**
 * ContextDelegate makes clear that a delegate uses the Context and AutoescState mechanism for parsing.
 */
public interface ContextDelegate<CONTEXT__18 extends ormery.core.Context> extends Delegate {
    AutoescState<CONTEXT__18> getState();
    void setState(AutoescState<CONTEXT__18> x__136);
    ContextPropagator<CONTEXT__18> getContextPropagator();
    default String process(@Nullable String known__141) {
        AfterPropagate<CONTEXT__18> after__143 = CoreGlobal.propagateOver(this.getContextPropagator(), this.getState(), known__141);
        AutoescState<CONTEXT__18> t_479 = after__143.getStateAfter();
        this.setState(t_479);
        return after__143.getAdjustedString();
    }
}
