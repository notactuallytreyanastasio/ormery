package ormery.core;
public interface ContextualAutoescapingAccumulator<CONTEXT__27 extends ormery.core.Context, ESC__28 extends ormery.core.Escaper> {
    AutoescState<CONTEXT__27> getState();
    void setState(AutoescState<CONTEXT__27> newState__158);
    EscaperPicker<CONTEXT__27, ESC__28> getEscaperPicker();
    ContextPropagator<CONTEXT__27> getContextPropagator();
    default ESC__28 prepareForAppend() {
        AfterPropagate<CONTEXT__27> after__166 = CoreGlobal.propagateOver(this.getContextPropagator(), this.getState(), null);
        AutoescState<CONTEXT__27> t_467 = after__166.getStateAfter();
        this.setState(t_467);
        String adjusted__167 = after__166.getAdjustedString();
        if (!adjusted__167.isEmpty()) {
            this.collectFixed(adjusted__167);
        }
        return this.getEscaperPicker().escaperFor(this.getState());
    }
    default void appendSafe(String known__169) {
        AfterPropagate<CONTEXT__27> after__171 = CoreGlobal.propagateOver(this.getContextPropagator(), this.getState(), known__169);
        AutoescState<CONTEXT__27> t_458 = after__171.getStateAfter();
        this.setState(t_458);
        String adjusted__172 = after__171.getAdjustedString();
        if (!adjusted__172.isEmpty()) {
            this.collectFixed(adjusted__172);
        }
    }
    /**
     * Appends the fixed, trusted fragment to the collector.
     */
    void collectFixed(String fixedFragment__174);
}
