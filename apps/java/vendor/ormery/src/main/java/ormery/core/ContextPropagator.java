package ormery.core;
import temper.core.Nullable;
/**
 * ContextPropagator knows how to consumes a chunk of context from a literalPart and transition
 * to the Context after it.
 */
public interface ContextPropagator<CONTEXT__15 extends ormery.core.Context> {
    /**
     * after computes the context after processing the literalPart and any
     * adjustments made to the literal part.
     *
     * If literalPart is null, then we're doing a context transition to get
     * into a state where we're ready to interpolate an untrusted value.
     */
    AfterPropagate<CONTEXT__15> after(AutoescState<CONTEXT__15> before__127, @Nullable String literalPart__128);
}
