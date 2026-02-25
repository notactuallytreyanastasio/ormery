package ormery.core;
/**
 * Context encapsulates a state in a streaming parser.  It's used both to
 * parse the fixed part of the content, and to pick escapers to preserve
 * the meaning of subsequent fixed parts.
 */
public interface Context {
    String toString();
}
