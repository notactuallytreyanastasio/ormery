package ormery.core;
import temper.core.Nullable;
/**
 * AfterPropagate bundles up information about propagating context across a fixed part,
 * and context transitions needed to move into a state where an interpolation is valid.
 */
public final class AfterPropagate<CONTEXT__11 extends ormery.core.Context> {
    public final String adjustedString;
    public final int consumed;
    public final AutoescState<CONTEXT__11> stateAfter;
    /**
     * push specifies a delegate for a nested language.
     * The &#42;Codec&#42; is used to decode content before feeding it to the delegate, and to
     * reencode the delegate's adjusted output.
     */
    public AfterPropagate<CONTEXT__11> push(Delegate delegate__112, Codec codec__113) {
        return new AfterPropagate<>(this.adjustedString, this.consumed, new AutoescState<>(this.stateAfter.getContext(), new Subsidiary(delegate__112, codec__113)));
    }
    /**
     * pop undoes a push by removing a previously pushed delegate when a
     * nested language region ends.
     */
    public AfterPropagate<CONTEXT__11> pop() {
        return new AfterPropagate<>(this.adjustedString, this.consumed, new AutoescState<>(this.stateAfter.getContext(), null));
    }
    /**
     * feed gives a nested language delegate the adjustedString, so that it
     * can reconsider it in the context of the nested language.  This may
     * involve applying a codec to decode the content before the nested
     * language delegate handles it, and then re-encoding if there are any
     * adjustements made by the delegate.
     */
    public AfterPropagate<CONTEXT__11> feed(boolean prepareForInterp__118) {
        AfterPropagate<CONTEXT__11> return__57;
        @Nullable Subsidiary subsidiary__120 = this.stateAfter.getSubsidiary();
        if (subsidiary__120 != null) {
            Subsidiary subsidiary_237 = subsidiary__120;
            String adjustedFromDelegate__121 = CoreGlobal.feedSubsidiary__83(subsidiary_237, this.adjustedString, prepareForInterp__118);
            return__57 = new AfterPropagate<>(adjustedFromDelegate__121, this.consumed, this.stateAfter);
        } else {
            return__57 = this;
        }
        return return__57;
    }
    public static final class Builder<CONTEXT__11 extends ormery.core.Context> {
        String adjustedString;
        public Builder<CONTEXT__11> adjustedString(String adjustedString) {
            this.adjustedString = adjustedString;
            return this;
        }
        int consumed;
        boolean consumed__set;
        public Builder<CONTEXT__11> consumed(int consumed) {
            consumed__set = true;
            this.consumed = consumed;
            return this;
        }
        AutoescState<CONTEXT__11> stateAfter;
        public Builder<CONTEXT__11> stateAfter(AutoescState<CONTEXT__11> stateAfter) {
            this.stateAfter = stateAfter;
            return this;
        }
        public AfterPropagate<CONTEXT__11> build() {
            if (!consumed__set || adjustedString == null || stateAfter == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!consumed__set) {
                    _message.append(" consumed");
                }
                if (adjustedString == null) {
                    _message.append(" adjustedString");
                }
                if (stateAfter == null) {
                    _message.append(" stateAfter");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new AfterPropagate<>(adjustedString, consumed, stateAfter);
        }
    }
    public AfterPropagate(String adjustedString__123, int consumed__124, AutoescState<CONTEXT__11> stateAfter__125) {
        this.adjustedString = adjustedString__123;
        this.consumed = consumed__124;
        this.stateAfter = stateAfter__125;
    }
    public String getAdjustedString() {
        return this.adjustedString;
    }
    public int getConsumed() {
        return this.consumed;
    }
    public AutoescState<CONTEXT__11> getStateAfter() {
        return this.stateAfter;
    }
}
