import {
  globalConsole as globalConsole__223, type as type__236, listBuilderAdd as listBuilderAdd_229, listBuilderToList as listBuilderToList_234
} from "@temperlang/core";
globalConsole__223;
/** @template PART_224 */
export class Collector extends type__236() {
  /** @type {Array<Collected<PART_224>>} */
  #partsBuilder_225;
  /** @param {string} fixed_227 */
  appendSafe(fixed_227) {
    let t_228 = new CollectedFixed(fixed_227);
    listBuilderAdd_229(this.#partsBuilder_225, t_228);
    return;
  }
  /** @param {PART_224} part_231 */
  append(part_231) {
    let t_232 = new CollectedPart(part_231);
    listBuilderAdd_229(this.#partsBuilder_225, t_232);
    return;
  }
  /** @returns {Array<Collected<PART_224>>} */
  get parts() {
    return listBuilderToList_234(this.#partsBuilder_225);
  }
  constructor() {
    super ();
    let t_235 = [];
    this.#partsBuilder_225 = t_235;
    return;
  }
};
/** @template PART_237 */
export class Collected extends type__236() {
};
/** @template PART_238 */
export class CollectedFixed extends type__236(Collected) {
  /** @type {string} */
  #safeText_239;
  /** @param {string} safeText_240 */
  constructor(safeText_240) {
    super ();
    this.#safeText_239 = safeText_240;
    return;
  }
  /** @returns {string} */
  get safeText() {
    return this.#safeText_239;
  }
};
/** @template PART_242 */
export class CollectedPart extends type__236(Collected) {
  /** @type {PART_242} */
  #part_243;
  /** @param {PART_242} part_244 */
  constructor(part_244) {
    super ();
    this.#part_243 = part_244;
    return;
  }
  /** @returns {PART_242} */
  get part() {
    return this.#part_243;
  }
};
export class Context extends type__236() {
  /** @returns {string} */
  toString() {
    null;
  }
};
/** @template CONTEXT_247 */
export class AutoescState extends type__236() {
  /** @type {CONTEXT_247} */
  #context_248;
  /** @type {Subsidiary | null} */
  #subsidiary_249;
  /**
   * @template CONTEXT_247
   * @param {{
   *   context: CONTEXT_247, subsidiary: Subsidiary | null
   * }}
   * props
   * @returns {AutoescState<CONTEXT_247>}
   */
  static["new"](props) {
    return new AutoescState(props.context, props.subsidiary);
  }
  /**
   * @param {CONTEXT_247} context_250
   * @param {Subsidiary | null} subsidiary_251
   */
  constructor(context_250, subsidiary_251) {
    super ();
    this.#context_248 = context_250;
    this.#subsidiary_249 = subsidiary_251;
    return;
  }
  /** @returns {CONTEXT_247} */
  get context() {
    return this.#context_248;
  }
  /** @returns {Subsidiary | null} */
  get subsidiary() {
    return this.#subsidiary_249;
  }
};
/** @template CONTEXT_254 */
export class AfterPropagate extends type__236() {
  /** @type {string} */
  #adjustedString_255;
  /** @type {globalThis.number} */
  #consumed_256;
  /** @type {AutoescState<CONTEXT_254>} */
  #stateAfter_257;
  /**
   * @param {Delegate} delegate_259
   * @param {Codec} codec_260
   * @returns {AfterPropagate<CONTEXT_254>}
   */
  push(delegate_259, codec_260) {
    return new AfterPropagate(this.#adjustedString_255, this.#consumed_256, new AutoescState(this.#stateAfter_257.context, new Subsidiary(delegate_259, codec_260)));
  }
  /** @returns {AfterPropagate<CONTEXT_254>} */
  pop() {
    return new AfterPropagate(this.#adjustedString_255, this.#consumed_256, new AutoescState(this.#stateAfter_257.context, null));
  }
  /**
   * @param {boolean} prepareForInterp_263
   * @returns {AfterPropagate<CONTEXT_254>}
   */
  feed(prepareForInterp_263) {
    let return_264;
    const subsidiary_265 = this.#stateAfter_257.subsidiary;
    if (!(subsidiary_265 == null)) {
      const subsidiary_266 = subsidiary_265;
      const adjustedFromDelegate_267 = feedSubsidiary_268(subsidiary_266, this.#adjustedString_255, prepareForInterp_263);
      return_264 = new AfterPropagate(adjustedFromDelegate_267, this.#consumed_256, this.#stateAfter_257);
    } else {
      return_264 = this;
    }
    return return_264;
  }
  /**
   * @template CONTEXT_254
   * @param {{
   *   adjustedString: string, consumed: globalThis.number, stateAfter: AutoescState<CONTEXT_254>
   * }}
   * props
   * @returns {AfterPropagate<CONTEXT_254>}
   */
  static["new"](props) {
    return new AfterPropagate(props.adjustedString, props.consumed, props.stateAfter);
  }
  /**
   * @param {string} adjustedString_269
   * @param {globalThis.number} consumed_270
   * @param {AutoescState<CONTEXT_254>} stateAfter_271
   */
  constructor(adjustedString_269, consumed_270, stateAfter_271) {
    super ();
    this.#adjustedString_255 = adjustedString_269;
    this.#consumed_256 = consumed_270;
    this.#stateAfter_257 = stateAfter_271;
    return;
  }
  /** @returns {string} */
  get adjustedString() {
    return this.#adjustedString_255;
  }
  /** @returns {globalThis.number} */
  get consumed() {
    return this.#consumed_256;
  }
  /** @returns {AutoescState<CONTEXT_254>} */
  get stateAfter() {
    return this.#stateAfter_257;
  }
};
/** @template CONTEXT_275 */
export class ContextPropagator extends type__236() {
  /**
   * @param {AutoescState<CONTEXT_275>} before_277
   * @param {string | null} literalPart_278
   * @returns {AfterPropagate<CONTEXT_275>}
   */
  after(before_277, literalPart_278) {
    null;
  }
};
export class Delegate extends type__236() {
  /**
   * @param {string | null} fixed_280
   * @returns {string}
   */
  process(fixed_280) {
    null;
  }
};
/** @template CONTEXT_281 */
export class ContextDelegate extends type__236(Delegate) {
  /** @returns {AutoescState<CONTEXT_281>} */
  get state() {
    null;
  }
  /** @param {AutoescState<CONTEXT_281>} x_284 */
  set state(x_284) {
    null;
  }
  /** @returns {ContextPropagator<CONTEXT_281>} */
  get contextPropagator() {
    null;
  }
  /**
   * @param {string | null} known_287
   * @returns {string}
   */
  process(known_287) {
    const after_288 = propagateOver(this.contextPropagator, this.state, known_287);
    let t_289 = after_288.stateAfter;
    this.state = t_289;
    return after_288.adjustedString;
  }
};
export class Escaper extends type__236() {
};
/**
 * @template CONTEXT_290
 * @template ESC_291
 */
export class EscaperPicker extends type__236() {
  /**
   * @param {AutoescState<CONTEXT_290>} before_293
   * @returns {ESC_291}
   */
  escaperFor(before_293) {
    null;
  }
};
/**
 * @template CONTEXT_294
 * @template ESC_295
 */
export class ContextualAutoescapingAccumulator extends type__236() {
  /** @returns {AutoescState<CONTEXT_294>} */
  get state() {
    null;
  }
  /** @param {AutoescState<CONTEXT_294>} newState_298 */
  set state(newState_298) {
    null;
  }
  /** @returns {EscaperPicker<CONTEXT_294, ESC_295>} */
  get escaperPicker() {
    null;
  }
  /** @returns {ContextPropagator<CONTEXT_294>} */
  get contextPropagator() {
    null;
  }
  /** @returns {ESC_295} */
  prepareForAppend() {
    const after_302 = propagateOver(this.contextPropagator, this.state, null);
    let t_303 = after_302.stateAfter;
    this.state = t_303;
    const adjusted_304 = after_302.adjustedString;
    if (! ! adjusted_304) {
      this.collectFixed(adjusted_304);
    }
    return this.escaperPicker.escaperFor(this.state);
  }
  /** @param {string} known_306 */
  appendSafe(known_306) {
    const after_307 = propagateOver(this.contextPropagator, this.state, known_306);
    let t_308 = after_307.stateAfter;
    this.state = t_308;
    const adjusted_309 = after_307.adjustedString;
    if (! ! adjusted_309) {
      this.collectFixed(adjusted_309);
    }
    return;
  }
  /** @param {string} fixedFragment_311 */
  collectFixed(fixedFragment_311) {
    null;
  }
};
export class Codec extends type__236() {
  /**
   * @param {string} s_313
   * @returns {string}
   */
  encode(s_313) {
    null;
  }
  /**
   * @param {string} s_315
   * @returns {string}
   */
  decode(s_315) {
    null;
  }
};
export class Subsidiary extends type__236() {
  /** @type {Delegate} */
  #delegate_316;
  /** @type {Codec} */
  #codec_317;
  /**
   * @param {{
   *   delegate: Delegate, codec: Codec
   * }}
   * props
   * @returns {Subsidiary}
   */
  static["new"](props) {
    return new Subsidiary(props.delegate, props.codec);
  }
  /**
   * @param {Delegate} delegate_318
   * @param {Codec} codec_319
   */
  constructor(delegate_318, codec_319) {
    super ();
    this.#delegate_316 = delegate_318;
    this.#codec_317 = codec_319;
    return;
  }
  /** @returns {Delegate} */
  get delegate() {
    return this.#delegate_316;
  }
  /** @returns {Codec} */
  get codec() {
    return this.#codec_317;
  }
};
/**
 * @param {Subsidiary} subsidiary_322
 * @param {string} adjustedStr_323
 * @param {boolean} prepareForInterp_324
 * @returns {string}
 */
function feedSubsidiary_268(subsidiary_322, adjustedStr_323, prepareForInterp_324) {
  let t_325;
  let str_326 = adjustedStr_323;
  const delegate_327 = subsidiary_322.delegate;
  const codec_328 = subsidiary_322.codec;
  let t_329 = codec_328.decode(str_326);
  str_326 = t_329;
  let t_330 = delegate_327.process(str_326);
  str_326 = t_330;
  if (prepareForInterp_324) {
    t_325 = delegate_327.process(null);
    str_326 = String(str_326) + t_325;
  }
  let t_331 = codec_328.encode(str_326);
  str_326 = t_331;
  return str_326;
}
/**
 * @template {Context} CONTEXT_347
 * @param {ContextPropagator<CONTEXT_347>} contextPropagator_332
 * @param {AutoescState<CONTEXT_347>} before_333
 * @param {string | null} known_334
 * @returns {AfterPropagate<CONTEXT_347>}
 */
export function propagateOver(contextPropagator_332, before_333, known_334) {
  let return_335;
  let t_336;
  let t_337;
  let t_338;
  let t_339;
  let t_340;
  let t_341;
  if (known_334 == null) {
    return_335 = contextPropagator_332.after(before_333, null);
  } else {
    const known_342 = known_334;
    let state_343 = before_333;
    let remainder_344 = known_342;
    const adjusted_345 = [""];
    while (true) {
      if (! ! ! remainder_344) {
        break;
      }
      const after_346 = contextPropagator_332.after(state_343, remainder_344);
      adjusted_345[0] += after_346.adjustedString;
      t_336 = after_346.stateAfter;
      state_343 = t_336;
      t_337 = after_346.consumed;
      t_338 = remainder_344.length;
      t_339 = remainder_344.substring(t_337, t_338);
      remainder_344 = t_339;
    }
    t_340 = adjusted_345[0];
    t_341 = known_342.length;
    return_335 = new AfterPropagate(t_340, t_341, state_343);
  }
  return return_335;
};
