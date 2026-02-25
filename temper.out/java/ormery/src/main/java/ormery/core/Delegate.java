package ormery.core;
import temper.core.Nullable;
/**
 * Delegate can represent an automaton for a nested language that can propagates its own state over
 * strings that relate to chunks of content from the outer language.  The content can be
 * decoded via a &#42;Codec&#42;.
 */
public interface Delegate {
    String process(@Nullable String fixed__131);
}
