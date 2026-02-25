package ormery.sql;
/**
 * `SqlSource` represents known-safe SQL source code that doesn't need escaped. This
 * often originates from the raw string content in `sql`-tagged strings. For
 * example, in `sql"select p.name from person p where p.id = $&#123;id&#125;"`, all except
 * the `$&#123;id&#125;` interpolation becomes a `SqlSource` instance.
 */
public final class SqlSource implements SqlPart {
    public final String source;
    public void formatTo(StringBuilder builder__175) {
        builder__175.append(this.source);
    }
    public SqlSource(String source__178) {
        this.source = source__178;
    }
    public String getSource() {
        return this.source;
    }
}
