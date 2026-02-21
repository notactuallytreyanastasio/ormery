# Contextual Autoescaping Framework

This defines a framework for building contextual autoescaping
accumulators that can be highly statically optimized because the
separate steps, listed below, are all pure functional operators that
can be inlined and simplified at compile time.

Context encapsulates a state in a streaming parser.  It's used both to
parse the fixed part of the content, and to pick escapers to preserve
the meaning of subsequent fixed parts.

    export interface Context {
      public toString(): String;
    }

Autoesc bundles together a context and a subsidiary automaton.

    export class AutoescState<CONTEXT extends Context>(
      public context: CONTEXT,
      public subsidiary: Subsidiary?,
    ) {}

AfterPropagate bundles up information about propagating context across a fixed part,
and context transitions needed to move into a state where an interpolation is valid.

    export class AfterPropagate<CONTEXT extends Context>(
      public adjustedString: String,
      public consumed: StringIndex,
      public stateAfter: AutoescState<CONTEXT>,
    ) {

push specifies a delegate for a nested language.
The *Codec* is used to decode content before feeding it to the delegate, and to
reencode the delegate's adjusted output.

      public push(delegate: Delegate, codec: Codec): AfterPropagate<CONTEXT> {
        new AfterPropagate<CONTEXT>(
          adjustedString,
          consumed,
          new AutoescState<CONTEXT>(stateAfter.context, new Subsidiary(delegate, codec)),
        )
      }

pop undoes a push by removing a previously pushed delegate when a
nested language region ends.

      public pop(): AfterPropagate<CONTEXT> {
        new AfterPropagate<CONTEXT>(
          adjustedString,
          consumed,
          new AutoescState<CONTEXT>(stateAfter.context, null),
        )
      }

feed gives a nested language delegate the adjustedString, so that it
can reconsider it in the context of the nested language.  This may
involve applying a codec to decode the content before the nested
language delegate handles it, and then re-encoding if there are any
adjustements made by the delegate.

      public feed(prepareForInterp: Boolean): AfterPropagate<CONTEXT> {
        let subsidiary = stateAfter.subsidiary;
        if (subsidiary != null) {
          let adjustedFromDelegate = feedSubsidiary(subsidiary, adjustedString, prepareForInterp);
          new AfterPropagate<CONTEXT>(adjustedFromDelegate, consumed, stateAfter)
        } else {
          this
        }
      }
    }

ContextPropagator knows how to consumes a chunk of context from a literalPart and transition
to the Context after it.

    export interface ContextPropagator<CONTEXT extends Context> {

after computes the context after processing the literalPart and any
adjustments made to the literal part.

If literalPart is null, then we're doing a context transition to get
into a state where we're ready to interpolate an untrusted value.

      after(
        before: AutoescState<CONTEXT>,
        literalPart: String?,
      ): AfterPropagate<CONTEXT>;
    }

Delegate can represent an automaton for a nested language that can propagates its own state over
strings that relate to chunks of content from the outer language.  The content can be
decoded via a *Codec*.

    export interface Delegate {
      process(fixed: String?): String;
    }

ContextDelegate makes clear that a delegate uses the Context and AutoescState mechanism for parsing.

    export interface ContextDelegate<CONTEXT extends Context> extends Delegate {
      protected get state(): AutoescState<CONTEXT>;
      protected set state(x: AutoescState<CONTEXT>): Void;
      protected get contextPropagator(): ContextPropagator<CONTEXT>;

      process(known: String?): String {
        let after = propagateOver<CONTEXT>(contextPropagator, this.state, known);
        this.state = after.stateAfter;
        after.adjustedString
      }
    }

Escaper is for transforms from untrusted values to trustworthy values.

They should be stateless, only useful for their behaviour, unless they
represent the composition of two more escapers in which case they should
have two properties: `first` and `second` indicating the application order.

    export interface Escaper {
      // One or more apply methods like
      // apply(interpolation: String): OUT;
    }

(*OUT* is the representation of the trustworthy output and must be
accepted by with the collector type's *append* method.)

propagateOver is used by both the accumulators and context delegates to propagate context over
a known safe chunk.  Propagation is usual defined in terms of transition tables, and each
transition consumes some prefix of the remaining content, so this does enough transitions to
process the entire chunk.

    export let propagateOver<CONTEXT extends Context>(
      contextPropagator: ContextPropagator<CONTEXT>,
      before: AutoescState<CONTEXT>,
      known: String?
    ): AfterPropagate<CONTEXT> {
      if (known == null) {
        contextPropagator.after(before, null)
      } else {
        var state = before;
        var remainder = known;
        let adjusted = new StringBuilder();
        while (!remainder.isEmpty) {
          let after = contextPropagator.after(state, remainder);
          adjusted.append(after.adjustedString);
          state = after.stateAfter;
          remainder = remainder.slice(after.consumed, remainder.end);
        }
        new AfterPropagate(adjusted.toString(), known.end, state)
      }
    }

EscaperPicker returns the escaper that needs to be applied to untrusted parts to render them
safe in context.

    export interface EscaperPicker<CONTEXT extends Context, ESC extends Escaper> {
      escaperFor(before: AutoescState<CONTEXT>): ESC;
    }

CONTEXT is a type for context values that track the parser context of
the accumulated prefix.  ESC is the type used to represent escapers
that convert interpolated expression values to PARTS.

Additionally, there are two names below that are mentioned in static
conventions but which need not be represented as type parameters.
COLLECTOR is a mutable type that collects safe and unsafe parts and
which is used to produce the output.  OUT is the type of the final
accumulated value.

    export interface ContextualAutoescapingAccumulator<CONTEXT extends Context, ESC extends Escaper> {
      // By convention, these are defined in concrete accumulator types.
      // public static initialState(): AutoescState<CONTEXT> { ... }
      // public static newCollector(): COLLECTOR { ... }
      // public static propagator(): ContextPropagator<CONTEXT> { ... }
      // public static picker(): EscaperPicker<CONTEXT, PART, ESC> { ... }
      // public static fromCollector(collector: COLLECTOR): OUT;
      // public static mergeStates(a: AutoescState<CONTEXT>, b: AutoescState<CONTEXT>): AutoescState<CONTEXT>;
      // - state is initialized to initialState()
      // - The internal collector property is initialized to newCollector()
      // - contextPropagator() just returns propagator().
      // - escaperPicker() just returns picker().
      // - get accumulated() just returns fromCollector(collector)

      protected get state(): AutoescState<CONTEXT>;
      protected set state(newState: AutoescState<CONTEXT>): Void;

      public get escaperPicker(): EscaperPicker<CONTEXT, ESC>;
      public get contextPropagator(): ContextPropagator<CONTEXT>;

      public prepareForAppend(): ESC {
        if (DEBUG) {
          console.log("prepare, before: ${context.toString()}");
        }
        let after = propagateOver<CONTEXT>(contextPropagator, this.state, null);
        this.state = after.stateAfter;
        let adjusted = after.adjustedString;
        if (!adjusted.isEmpty) {
          collectFixed(adjusted);
        }
        if (DEBUG) {
          console.log("-> ${context.toString()}");
        }
        escaperPicker.escaperFor(this.state)
      }

      // Need one or more append methods

      public appendSafe(known: String): Void {
        if (DEBUG) {
          console.log("appendSafe: `${known}`, before: ${context}");
        }
        let after = propagateOver<CONTEXT>(contextPropagator, this.state, known);
        this.state = after.stateAfter;
        let adjusted = after.adjustedString;
        if (!adjusted.isEmpty) {
          collectFixed(adjusted);
        }
        if (DEBUG) {
          console.log("-> ${context}");
        }
      }

      /** Appends the fixed, trusted fragment to the collector. */
      protected collectFixed(fixedFragment: String): Void;
    }

feed allows a subsidiary language handler a chance to update its own
context and perhaps offer its own adjustments.

For example, an HTML state machine might use a subsidiary CSS state
machine to process the body of a `<style>` element, but still retain
control over identifying the `</style>` end tag which ends the
element.

    let feedSubsidiary(
      subsidiary: Subsidiary,
      adjustedStr: String,
      prepareForInterp: Boolean,
    ): String {
      var str = adjustedStr;

      let delegate = subsidiary.delegate;
      let codec = subsidiary.codec;

      str = codec.decode(str);

      str = delegate.process(str);
      if (prepareForInterp) {
        str = "${str}${delegate.process(null)}";
      }

      if (codec != null) {
        str = codec.encode(str);
      }

      str
    }

Codecs allow encoding and decoding a string according to some escaping
convention.
Sometimes we need to decode strings before passing to a subsidiary state
machine and re-encode any strings that they adjust.

    export interface Codec {
      encode(s: String): String;
      decode(s: String): String;
    }

    export class Subsidiary(
      public delegate: Delegate,
      public codec: Codec,
    ) {}

    let DEBUG = false;
