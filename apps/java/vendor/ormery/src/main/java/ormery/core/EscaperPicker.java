package ormery.core;
/**
 * EscaperPicker returns the escaper that needs to be applied to untrusted parts to render them
 * safe in context.
 */
public interface EscaperPicker<CONTEXT__24 extends ormery.core.Context, ESC__25 extends ormery.core.Escaper> {
    ESC__25 escaperFor(AutoescState<CONTEXT__24> before__153);
}
