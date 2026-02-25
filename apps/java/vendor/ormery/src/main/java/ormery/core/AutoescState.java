package ormery.core;
import temper.core.Nullable;
public final class AutoescState<CONTEXT__10 extends ormery.core.Context> {
    public final CONTEXT__10 context;
    public final @Nullable Subsidiary subsidiary;
    public static final class Builder<CONTEXT__10 extends ormery.core.Context> {
        CONTEXT__10 context;
        public Builder<CONTEXT__10> context(CONTEXT__10 context) {
            this.context = context;
            return this;
        }
        @Nullable Subsidiary subsidiary;
        boolean subsidiary__set;
        public Builder<CONTEXT__10> subsidiary(@Nullable Subsidiary subsidiary) {
            subsidiary__set = true;
            this.subsidiary = subsidiary;
            return this;
        }
        public AutoescState<CONTEXT__10> build() {
            if (!subsidiary__set || context == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!subsidiary__set) {
                    _message.append(" subsidiary");
                }
                if (context == null) {
                    _message.append(" context");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new AutoescState<>(context, subsidiary);
        }
    }
    public AutoescState(CONTEXT__10 context__106, @Nullable Subsidiary subsidiary__107) {
        this.context = context__106;
        this.subsidiary = subsidiary__107;
    }
    public CONTEXT__10 getContext() {
        return this.context;
    }
    public @Nullable Subsidiary getSubsidiary() {
        return this.subsidiary;
    }
}
