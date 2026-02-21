import {
  percentEscapeOctetTo as percentEscapeOctetTo_358
} from "./url.js";
import {
  Codec as Codec_437, AutoescState as AutoescState_443, AfterPropagate as AfterPropagate_444, ContextPropagator as ContextPropagator_445, Context as Context_467, Escaper as Escaper_493, EscaperPicker as EscaperPicker_545, ContextualAutoescapingAccumulator as ContextualAutoescapingAccumulator_585, Delegate as Delegate_588, Subsidiary as Subsidiary_591, ContextDelegate as ContextDelegate_607
} from "./core.js";
import {
  Begin as Begin_363, End as End_365, CodeSet as CodeSet_807, CodePoints as CodePoints_808, Sequence as Sequence_810, Repeat as Repeat_812, Or as Or_814, Regex as Regex_834, CodeRange as CodeRange_1033
} from "@temperlang/std/regex";
import {
  type as type__438, requireInstanceOf as requireInstanceOf__1137, stringGet as stringGet_393, stringNext as stringNext_395, stringToInt32 as stringToInt32_428, stringBuilderAppendCodePoint as stringBuilderAppendCodePoint_430, mappedGetOr as mappedGetOr_435, float64ToString as float64ToString_522, panic as panic_605, listedGet as listedGet_673, mapBuilderConstructor as mapBuilderConstructor_703, mapBuilderSet as mapBuilderSet_706, mappedToMap as mappedToMap_708, listBuilderAdd as listBuilderAdd_837, listBuilderToList as listBuilderToList_838
} from "@temperlang/core";
/** @type {boolean} */
let t_370;
/** @type {boolean} */
let t_371;
/** @type {boolean} */
let t_372;
/** @type {boolean} */
let t_373;
/** @type {boolean} */
let t_374;
/** @type {boolean} */
let t_375;
/** @type {boolean} */
let t_376;
/** @type {boolean} */
let t_377;
/** @type {boolean} */
let t_378;
/** @type {boolean} */
let t_379;
/** @type {boolean} */
let t_380;
export class HtmlCodec extends type__438(Codec_437) {
  /**
   * @param {string} s_382
   * @returns {string}
   */
  encode(s_382) {
    let return_383;
    let t_384;
    let t_385;
    let t_386;
    let t_387;
    const sb_388 = [""];
    const end_389 = s_382.length;
    let encodedTo_390 = 0;
    let i_391 = 0;
    while (i_391 < end_389) {
      continue_392: {
        t_384 = stringGet_393(s_382, i_391);
        if (t_384 === 38) {
          t_387 = "\u0026amp;";
        } else if (t_384 === 60) {
          t_387 = "\u0026lt;";
        } else if (t_384 === 62) {
          t_387 = "\u0026gt;";
        } else if (t_384 === 39) {
          t_387 = "\u0026#39;";
        } else if (t_384 === 34) {
          t_387 = "\u0026#34;";
        } else if (t_384 === 0) {
          t_387 = "\u0026#0;";
        } else {
          break continue_392;
        }
        const replacement_394 = t_387;
        sb_388[0] += s_382.substring(encodedTo_390, i_391);
        sb_388[0] += replacement_394;
        t_385 = stringNext_395(s_382, i_391);
        encodedTo_390 = t_385;
      }
      t_386 = stringNext_395(s_382, i_391);
      i_391 = t_386;
    }
    if (encodedTo_390 > 0) {
      sb_388[0] += s_382.substring(encodedTo_390, end_389);
      return_383 = sb_388[0];
    } else {
      return_383 = s_382;
    }
    return return_383;
  }
  /**
   * @param {string} s_397
   * @returns {string}
   */
  decode(s_397) {
    let return_398;
    let t_399;
    let t_400;
    let t_401;
    let t_402;
    let t_403;
    let t_404;
    let t_405;
    let t_406;
    let t_407;
    let t_408;
    let t_409;
    let t_410;
    let t_411;
    let t_412;
    let t_413;
    let t_414;
    const sb_415 = [""];
    const end_416 = s_397.length;
    let decodedTo_417 = 0;
    let i_418 = 0;
    while (i_418 < end_416) {
      continue_419: {
        if (stringGet_393(s_397, i_418) === 38) {
          const startOfEntity_420 = stringNext_395(s_397, i_418);
          let endOfEntity_421 = startOfEntity_420;
          if (startOfEntity_420 < end_416) {
            t_399 = stringGet_393(s_397, startOfEntity_420);
            t_407 = Object.is(t_399, "#");
          } else {
            t_407 = false;
          }
          if (t_407) {
            t_400 = stringNext_395(s_397, startOfEntity_420);
            endOfEntity_421 = t_400;
            if (endOfEntity_421 >= end_416) {
              break continue_419;
            }
            let base_422 = 10;
            if ((stringGet_393(s_397, endOfEntity_421) | 32) === 120) {
              t_401 = stringNext_395(s_397, endOfEntity_421);
              endOfEntity_421 = t_401;
              base_422 = 16;
            }
            let digitQuota_423 = 7;
            const startOfDigits_424 = endOfEntity_421;
            while (true) {
              if (!(endOfEntity_421 < end_416)) {
                break;
              }
              const cp_425 = stringGet_393(s_397, endOfEntity_421);
              if (48 <= cp_425) {
                t_408 = cp_425 <= 57;
              } else {
                t_408 = false;
              }
              if (! t_408) {
                if (base_422 === 16) {
                  const lcp_426 = cp_425 | 32;
                  if (97 <= lcp_426) {
                    t_409 = lcp_426 <= 102;
                  } else {
                    t_409 = false;
                  }
                  if (! t_409) {
                    break;
                  }
                } else {
                  break;
                }
              }
              t_402 = stringNext_395(s_397, endOfEntity_421);
              endOfEntity_421 = t_402;
            }
            const endOfDigits_427 = endOfEntity_421;
            if (endOfDigits_427 === startOfDigits_424) {
              break continue_419;
            }
            if (endOfEntity_421 < end_416) {
              t_403 = stringGet_393(s_397, endOfEntity_421);
              t_410 = t_403 === 59;
            } else {
              t_410 = false;
            }
            if (t_410) {
              t_404 = stringNext_395(s_397, endOfEntity_421);
              endOfEntity_421 = t_404;
            }
            try {
              t_411 = stringToInt32_428(s_397.substring(startOfDigits_424, endOfDigits_427), base_422);
            } catch {
              break continue_419;
            }
            const decodedCp_429 = t_411;
            if (0 <= decodedCp_429) {
              t_412 = decodedCp_429 <= 1114111;
            } else {
              t_412 = false;
            }
            if (t_412) {
              sb_415[0] += s_397.substring(decodedTo_417, i_418);
              if (55296 <= decodedCp_429) {
                t_413 = decodedCp_429 <= 57343;
              } else {
                t_413 = false;
              }
              if (t_413) {
                sb_415[0] += "�";
              } else {
                try {
                  stringBuilderAppendCodePoint_430(sb_415, decodedCp_429);
                } catch {
                  break continue_419;
                }
              }
              decodedTo_417 = endOfEntity_421;
            }
          } else {
            while (endOfEntity_421 < end_416) {
              const cp_431 = stringGet_393(s_397, endOfEntity_421);
              t_405 = stringNext_395(s_397, endOfEntity_421);
              endOfEntity_421 = t_405;
              if (cp_431 === 59) {
                break;
              }
              const lcp_432 = cp_431 | 32;
              if (97 <= lcp_432) {
                t_414 = lcp_432 <= 122;
              } else {
                t_414 = false;
              }
              if (! t_414) {
                break;
              }
            }
            if (startOfEntity_420 < endOfEntity_421) {
              const entityName_433 = s_397.substring(startOfEntity_420, endOfEntity_421);
              const entityValue_434 = mappedGetOr_435(htmlNamedCharacters_436, entityName_433, "");
              if (! ! entityValue_434) {
                sb_415[0] += s_397.substring(decodedTo_417, i_418);
                sb_415[0] += entityValue_434;
                decodedTo_417 = endOfEntity_421;
              }
            }
          }
        }
      }
      t_406 = stringNext_395(s_397, i_418);
      i_418 = t_406;
    }
    if (decodedTo_417 > 0) {
      sb_415[0] += s_397.substring(decodedTo_417, end_416);
      return_398 = sb_415[0];
    } else {
      return_398 = s_397;
    }
    return return_398;
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlContextPropagator extends type__438(ContextPropagator_445) {
  /**
   * @param {AutoescState_443<HtmlEscaperContext>} before_440
   * @param {string | null} literalPart_441
   * @returns {AfterPropagate_444<HtmlEscaperContext>}
   */
  after(before_440, literalPart_441) {
    return htmlPropagateContext_442(before_440, literalPart_441);
  }
  constructor() {
    super ();
    return;
  }
};
export class UrlContextPropagator extends type__438(ContextPropagator_445) {
  /**
   * @param {AutoescState_443<UrlEscaperContext>} before_447
   * @param {string | null} literalPart_448
   * @returns {AfterPropagate_444<UrlEscaperContext>}
   */
  after(before_447, literalPart_448) {
    return urlPropagateContext_449(before_447, literalPart_448);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlEscaperContext extends type__438(Context_467) {
  /** @type {number} */
  #htmlState_450;
  /** @type {number} */
  #tagState_451;
  /** @type {number} */
  #attribState_452;
  /** @type {number} */
  #delimState_453;
  /** @returns {string} */
  toString() {
    return "HtmlEscaperContext(" + htmlStateStr_455(this.#htmlState_450) + ", " + tagStateStr_456(this.#tagState_451) + ", " + attribStateStr_457(this.#attribState_452) + ", " + delimStateStr_458(this.#delimState_453) + ")";
  }
  /**
   * @param {{
   *   htmlState: number, tagState: number, attribState: number, delimState: number
   * }}
   * props
   * @returns {HtmlEscaperContext}
   */
  static["new"](props) {
    return new HtmlEscaperContext(props.htmlState, props.tagState, props.attribState, props.delimState);
  }
  /**
   * @param {number} htmlState_459
   * @param {number} tagState_460
   * @param {number} attribState_461
   * @param {number} delimState_462
   */
  constructor(htmlState_459, tagState_460, attribState_461, delimState_462) {
    super ();
    this.#htmlState_450 = htmlState_459;
    this.#tagState_451 = tagState_460;
    this.#attribState_452 = attribState_461;
    this.#delimState_453 = delimState_462;
    return;
  }
  /** @returns {number} */
  get htmlState() {
    return this.#htmlState_450;
  }
  /** @returns {number} */
  get tagState() {
    return this.#tagState_451;
  }
  /** @returns {number} */
  get attribState() {
    return this.#attribState_452;
  }
  /** @returns {number} */
  get delimState() {
    return this.#delimState_453;
  }
};
export class UrlEscaperContext extends type__438(Context_467) {
  /** @type {number} */
  #urlState_468;
  /** @returns {string} */
  toString() {
    return "UrlEscaperContext(" + urlStateStr_470(this.#urlState_468) + ")";
  }
  /** @param {number} urlState_471 */
  constructor(urlState_471) {
    super ();
    this.#urlState_468 = urlState_471;
    return;
  }
  /** @returns {number} */
  get urlState() {
    return this.#urlState_468;
  }
};
export class SafeHtml extends type__438() {
  /** @type {string} */
  #text_473;
  /** @returns {string} */
  toString() {
    return this.#text_473;
  }
  /** @param {string} text_475 */
  constructor(text_475) {
    super ();
    this.#text_473 = text_475;
    return;
  }
  /** @returns {string} */
  get text() {
    return this.#text_473;
  }
};
export class SafeUrl extends type__438() {
  /** @type {string} */
  #text_477;
  /** @returns {string} */
  toString() {
    return this.#text_477;
  }
  /** @param {string} text_479 */
  constructor(text_479) {
    super ();
    this.#text_477 = text_479;
    return;
  }
  /** @returns {string} */
  get text() {
    return this.#text_477;
  }
};
export class HtmlEscaper extends type__438(Escaper_493) {
  /**
   * @param {SafeHtml} x_482
   * @returns {string}
   */
  applySafeHtml(x_482) {
    null;
  }
  /**
   * @param {SafeUrl} x_484
   * @returns {string}
   */
  applySafeUrl(x_484) {
    null;
  }
  /**
   * @param {number} x_486
   * @returns {string}
   */
  applyInt32(x_486) {
    null;
  }
  /**
   * @param {bigint} x_488
   * @returns {string}
   */
  applyInt64(x_488) {
    null;
  }
  /**
   * @param {number} x_490
   * @returns {string}
   */
  applyFloat64(x_490) {
    null;
  }
  /**
   * @param {string} x_492
   * @returns {string}
   */
  applyString(x_492) {
    null;
  }
};
export class OutputHtmlSpaceEscaper extends type__438(HtmlEscaper) {
  /** @type {OutputHtmlSpaceEscaper} */
  static #instance_494 = new OutputHtmlSpaceEscaper();
  /** @returns {OutputHtmlSpaceEscaper} */
  static get instance() {
    return this.#instance_494;
  }
  /**
   * @param {SafeHtml} x_496
   * @returns {string}
   */
  applySafeHtml(x_496) {
    return " ";
  }
  /**
   * @param {SafeUrl} x_498
   * @returns {string}
   */
  applySafeUrl(x_498) {
    return " ";
  }
  /**
   * @param {number} x_500
   * @returns {string}
   */
  applyInt32(x_500) {
    return " ";
  }
  /**
   * @param {bigint} x_502
   * @returns {string}
   */
  applyInt64(x_502) {
    return " ";
  }
  /**
   * @param {number} x_504
   * @returns {string}
   */
  applyFloat64(x_504) {
    return " ";
  }
  /**
   * @param {string} x_506
   * @returns {string}
   */
  applyString(x_506) {
    return " ";
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlPcdataEscaper extends type__438(HtmlEscaper) {
  /** @type {HtmlPcdataEscaper} */
  static #instance_507 = new HtmlPcdataEscaper();
  /** @returns {HtmlPcdataEscaper} */
  static get instance() {
    return this.#instance_507;
  }
  /**
   * @param {SafeHtml} x_509
   * @returns {string}
   */
  applySafeHtml(x_509) {
    return x_509.toString();
  }
  /**
   * @param {SafeUrl} x_511
   * @returns {string}
   */
  applySafeUrl(x_511) {
    let t_512 = x_511.text;
    return this.applyString(t_512);
  }
  /**
   * @param {number} x_514
   * @returns {string}
   */
  applyInt32(x_514) {
    let t_515 = x_514.toString();
    return this.applyString(t_515);
  }
  /**
   * @param {bigint} x_517
   * @returns {string}
   */
  applyInt64(x_517) {
    let t_518 = x_517.toString();
    return this.applyString(t_518);
  }
  /**
   * @param {number} x_520
   * @returns {string}
   */
  applyFloat64(x_520) {
    let t_521 = float64ToString_522(x_520);
    return this.applyString(t_521);
  }
  /**
   * @param {string} x_524
   * @returns {string}
   */
  applyString(x_524) {
    return htmlCodec.encode(x_524);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlAttributeEscaper extends type__438(HtmlEscaper) {
  /** @type {HtmlAttributeEscaper} */
  static #instance_525 = new HtmlAttributeEscaper();
  /** @returns {HtmlAttributeEscaper} */
  static get instance() {
    return this.#instance_525;
  }
  /**
   * @param {SafeHtml} x_527
   * @returns {string}
   */
  applySafeHtml(x_527) {
    let t_528 = htmlCodec.decode(x_527.text);
    return this.applyString(t_528);
  }
  /**
   * @param {SafeUrl} x_530
   * @returns {string}
   */
  applySafeUrl(x_530) {
    let t_531 = x_530.text;
    return this.applyString(t_531);
  }
  /**
   * @param {number} x_533
   * @returns {string}
   */
  applyInt32(x_533) {
    let t_534 = x_533.toString();
    return this.applyString(t_534);
  }
  /**
   * @param {bigint} x_536
   * @returns {string}
   */
  applyInt64(x_536) {
    let t_537 = x_536.toString();
    return this.applyString(t_537);
  }
  /**
   * @param {number} x_539
   * @returns {string}
   */
  applyFloat64(x_539) {
    let t_540 = float64ToString_522(x_539);
    return this.applyString(t_540);
  }
  /**
   * @param {string} x_542
   * @returns {string}
   */
  applyString(x_542) {
    return htmlCodec.encode(x_542);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlEscaperPicker extends type__438(EscaperPicker_545) {
  /**
   * @param {AutoescState_443<HtmlEscaperContext>} stateBefore_544
   * @returns {HtmlEscaper}
   */
  escaperFor(stateBefore_544) {
    return pickHtmlEscaper(stateBefore_544);
  }
  constructor() {
    super ();
    return;
  }
};
export class SafeHtmlBuilder extends type__438(ContextualAutoescapingAccumulator_585) {
  /** @returns {globalThis.Array<string>} */
  static newCollector() {
    return[""];
  }
  /** @returns {AutoescState_443<HtmlEscaperContext>} */
  static initialState() {
    return new AutoescState_443(new HtmlEscaperContext(0, 0, 0, 0), null);
  }
  /** @returns {HtmlContextPropagator} */
  static propagator() {
    return new HtmlContextPropagator();
  }
  /** @returns {EscaperPicker_545<HtmlEscaperContext, HtmlEscaper>} */
  static picker() {
    return new HtmlEscaperPicker();
  }
  /**
   * @param {globalThis.Array<string>} collector_551
   * @returns {SafeHtml}
   */
  static fromCollector(collector_551) {
    return new SafeHtml(collector_551[0]);
  }
  /**
   * @param {AutoescState_443<HtmlEscaperContext>} a_553
   * @param {AutoescState_443<HtmlEscaperContext>} b_554
   * @returns {AutoescState_443<HtmlEscaperContext>}
   */
  static mergeStates(a_553, b_554) {
    return a_553;
  }
  /** @type {AutoescState_443<HtmlEscaperContext>} */
  #_state_555;
  /** @type {globalThis.Array<string>} */
  #collector_556;
  /** @returns {AutoescState_443<HtmlEscaperContext>} */
  get state() {
    return this.#_state_555;
  }
  /** @param {AutoescState_443<HtmlEscaperContext>} x_559 */
  set state(x_559) {
    this.#_state_555 = x_559;
    return;
  }
  /** @returns {EscaperPicker_545<HtmlEscaperContext, HtmlEscaper>} */
  get escaperPicker() {
    return SafeHtmlBuilder.picker();
  }
  /** @returns {ContextPropagator_445<HtmlEscaperContext>} */
  get contextPropagator() {
    return SafeHtmlBuilder.propagator();
  }
  /** @returns {SafeHtml} */
  get accumulated() {
    return SafeHtmlBuilder.fromCollector(this.#collector_556);
  }
  /** @param {string} fixed_564 */
  collectFixed(fixed_564) {
    this.#collector_556[0] += fixed_564;
    return;
  }
  /** @param {SafeHtml} x_566 */
  appendSafeHtml(x_566) {
    let t_567 = this.prepareForAppend().applySafeHtml(x_566);
    this.#collector_556[0] += t_567;
    return;
  }
  /** @param {SafeUrl} x_569 */
  appendSafeUrl(x_569) {
    let t_570 = this.prepareForAppend().applySafeUrl(x_569);
    this.#collector_556[0] += t_570;
    return;
  }
  /** @param {number} x_572 */
  appendInt32(x_572) {
    let t_573 = this.prepareForAppend().applyInt32(x_572);
    this.#collector_556[0] += t_573;
    return;
  }
  /** @param {bigint} x_575 */
  appendInt64(x_575) {
    let t_576 = this.prepareForAppend().applyInt64(x_575);
    this.#collector_556[0] += t_576;
    return;
  }
  /** @param {number} x_578 */
  appendFloat64(x_578) {
    let t_579 = this.prepareForAppend().applyFloat64(x_578);
    this.#collector_556[0] += t_579;
    return;
  }
  /** @param {string} x_581 */
  appendString(x_581) {
    let t_582 = this.prepareForAppend().applyString(x_581);
    this.#collector_556[0] += t_582;
    return;
  }
  constructor() {
    super ();
    let t_583 = SafeHtmlBuilder.initialState();
    this.#_state_555 = t_583;
    let t_584 = SafeHtmlBuilder.newCollector();
    this.#collector_556 = t_584;
    return;
  }
};
export class HtmlDelegate extends type__438(Delegate_588) {
  /**
   * @param {HtmlEscaper} outer_587
   * @returns {HtmlEscaper}
   */
  escaper(outer_587) {
    null;
  }
};
export class HtmlUrlDelegate extends type__438(ContextDelegate_607, HtmlDelegate) {
  /** @type {AutoescState_443<UrlEscaperContext>} */
  #_state_589;
  /** @type {Subsidiary_591 | null} */
  #_subsidiary_590;
  /** @returns {AutoescState_443<UrlEscaperContext>} */
  get state() {
    return this.#_state_589;
  }
  /** @param {AutoescState_443<UrlEscaperContext>} x_594 */
  set state(x_594) {
    this.#_state_589 = x_594;
    return;
  }
  /** @returns {ContextPropagator_445<UrlEscaperContext>} */
  get contextPropagator() {
    return urlContextPropagator_596;
  }
  /**
   * @param {HtmlEscaper} outer_598
   * @returns {HtmlEscaper}
   */
  escaper(outer_598) {
    let return_599;
    let t_600;
    let t_601 = this.state.context.urlState;
    if (t_601 === 0) {
      return_599 = new HtmlUrlEscaperAdapter(htmlProtocolFilteringUrlEscaper_602, outer_598);
    } else if (t_601 === 1) {
      return_599 = new HtmlUrlEscaperAdapter(htmlUrlPartUrlEscaper_603, outer_598);
    } else {
      if (t_601 === 2) {
        t_600 = true;
      } else {
        t_600 = t_601 === 3;
      }
      if (t_600) {
        return_599 = new HtmlUrlEscaperAdapter(htmlAsIfQueryUrlEscaper_604, outer_598);
      } else {
        return_599 = panic_605();
      }
    }
    return return_599;
  }
  constructor() {
    super ();
    let t_606 = new AutoescState_443(new UrlEscaperContext(0), null);
    this.#_state_589 = t_606;
    this.#_subsidiary_590 = null;
    return;
  }
};
export class HtmlUrlEscaperAdapter extends type__438(HtmlEscaper) {
  /** @type {UrlEscaper} */
  #first_608;
  /** @type {HtmlEscaper} */
  #second_609;
  /**
   * @param {SafeHtml} x_611
   * @returns {string}
   */
  applySafeHtml(x_611) {
    let t_612 = x_611.text;
    let t_613 = this.#first_608.applyString(t_612);
    return this.#second_609.applySafeUrl(t_613);
  }
  /**
   * @param {SafeUrl} x_615
   * @returns {string}
   */
  applySafeUrl(x_615) {
    let t_616 = this.#first_608.applySafeUrl(x_615);
    return this.#second_609.applySafeUrl(t_616);
  }
  /**
   * @param {number} x_618
   * @returns {string}
   */
  applyInt32(x_618) {
    let t_619 = x_618.toString();
    let t_620 = this.#first_608.applyString(t_619);
    return this.#second_609.applySafeUrl(t_620);
  }
  /**
   * @param {bigint} x_622
   * @returns {string}
   */
  applyInt64(x_622) {
    let t_623 = x_622.toString();
    let t_624 = this.#first_608.applyString(t_623);
    return this.#second_609.applySafeUrl(t_624);
  }
  /**
   * @param {number} x_626
   * @returns {string}
   */
  applyFloat64(x_626) {
    let t_627 = float64ToString_522(x_626);
    let t_628 = this.#first_608.applyString(t_627);
    return this.#second_609.applySafeUrl(t_628);
  }
  /**
   * @param {string} x_630
   * @returns {string}
   */
  applyString(x_630) {
    let t_631 = this.#first_608.applyString(x_630);
    return this.#second_609.applySafeUrl(t_631);
  }
  /**
   * @param {{
   *   first: UrlEscaper, second: HtmlEscaper
   * }}
   * props
   * @returns {HtmlUrlEscaperAdapter}
   */
  static["new"](props) {
    return new HtmlUrlEscaperAdapter(props.first, props.second);
  }
  /**
   * @param {UrlEscaper} first_632
   * @param {HtmlEscaper} second_633
   */
  constructor(first_632, second_633) {
    super ();
    this.#first_608 = first_632;
    this.#second_609 = second_633;
    return;
  }
  /** @returns {UrlEscaper} */
  get first() {
    return this.#first_608;
  }
  /** @returns {HtmlEscaper} */
  get second() {
    return this.#second_609;
  }
};
export class UrlEscaper extends type__438(Escaper_493) {
  /**
   * @param {SafeUrl} x_637
   * @returns {SafeUrl}
   */
  applySafeUrl(x_637) {
    null;
  }
  /**
   * @param {string} x_639
   * @returns {SafeUrl}
   */
  applyString(x_639) {
    null;
  }
};
export class HtmlProtocolFilteringUrlEscaper extends type__438(UrlEscaper) {
  /** @type {HtmlProtocolFilteringUrlEscaper} */
  static #instance_640 = new HtmlProtocolFilteringUrlEscaper();
  /** @returns {HtmlProtocolFilteringUrlEscaper} */
  static get instance() {
    return this.#instance_640;
  }
  /**
   * @param {SafeUrl} x_642
   * @returns {SafeUrl}
   */
  applySafeUrl(x_642) {
    return x_642;
  }
  /**
   * @param {string} x_644
   * @returns {SafeUrl}
   */
  applyString(x_644) {
    let return_645;
    let t_646;
    let t_647;
    let t_648;
    fn_649: {
      let protocolEnd_650 = 0;
      const end_651 = x_644.length;
      while (protocolEnd_650 < end_651) {
        const cp_652 = stringGet_393(x_644, protocolEnd_650);
        if (cp_652 === 58) {
          const protocol_653 = x_644.substring(0, protocolEnd_650);
          try {
            t_647 = protocolAllowList_654.find(protocol_653);
            t_648 = t_647;
          } catch {
            t_648 = null;
          }
          if (!(t_648 == null)) {
            return_645 = new SafeUrl(x_644);
          } else {
            return_645 = fallbackSafeUrl_655;
          }
          break fn_649;
        }
        t_646 = stringNext_395(x_644, protocolEnd_650);
        protocolEnd_650 = t_646;
      }
      return_645 = htmlUrlPartUrlEscaper_603.applyString(x_644);
    }
    return return_645;
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlUrlPartUrlEscaper extends type__438(UrlEscaper) {
  /** @type {HtmlUrlPartUrlEscaper} */
  static #instance_656 = new HtmlUrlPartUrlEscaper();
  /** @returns {HtmlUrlPartUrlEscaper} */
  static get instance() {
    return this.#instance_656;
  }
  /**
   * @param {SafeUrl} x_658
   * @returns {SafeUrl}
   */
  applySafeUrl(x_658) {
    return x_658;
  }
  /**
   * @param {string} x_660
   * @returns {SafeUrl}
   */
  applyString(x_660) {
    let t_661;
    let t_662;
    let t_663;
    let t_664;
    let t_665;
    let t_666;
    let i_667 = 0;
    const end_668 = x_660.length;
    let emitted_669 = 0;
    const sb_670 = [""];
    while (i_667 < end_668) {
      const cp_671 = stringGet_393(x_660, i_667);
      if (cp_671 < urlSafe_672.length) {
        t_661 = listedGet_673(urlSafe_672, cp_671);
        t_665 = ! t_661;
      } else {
        t_665 = false;
      }
      if (t_665) {
        sb_670[0] += x_660.substring(emitted_669, i_667);
        percentEscapeOctetTo_358(cp_671, sb_670);
        t_662 = stringNext_395(x_660, i_667);
        emitted_669 = t_662;
      }
      t_663 = stringNext_395(x_660, i_667);
      i_667 = t_663;
    }
    if (emitted_669 > 0) {
      sb_670[0] += x_660.substring(emitted_669, end_668);
      t_664 = sb_670[0];
      t_666 = t_664;
    } else {
      t_666 = x_660;
    }
    return new SafeUrl(t_666);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlAsIfQueryUrlEscaper extends type__438(UrlEscaper) {
  /** @type {HtmlAsIfQueryUrlEscaper} */
  static #instance_674 = new HtmlAsIfQueryUrlEscaper();
  /** @returns {HtmlAsIfQueryUrlEscaper} */
  static get instance() {
    return this.#instance_674;
  }
  /**
   * @param {SafeUrl} x_676
   * @returns {SafeUrl}
   */
  applySafeUrl(x_676) {
    return x_676;
  }
  /**
   * @param {string} x_678
   * @returns {SafeUrl}
   */
  applyString(x_678) {
    let t_679;
    let t_680;
    let t_681;
    let t_682;
    let t_683;
    let t_684;
    let i_685 = 0;
    const end_686 = x_678.length;
    let emitted_687 = 0;
    const sb_688 = [""];
    while (i_685 < end_686) {
      const cp_689 = stringGet_393(x_678, i_685);
      if (cp_689 < urlQuerySafe_690.length) {
        t_679 = listedGet_673(urlQuerySafe_690, cp_689);
        t_683 = ! t_679;
      } else {
        t_683 = false;
      }
      if (t_683) {
        sb_688[0] += x_678.substring(emitted_687, i_685);
        percentEscapeOctetTo_358(cp_689, sb_688);
        t_680 = stringNext_395(x_678, i_685);
        emitted_687 = t_680;
      }
      t_681 = stringNext_395(x_678, i_685);
      i_685 = t_681;
    }
    if (emitted_687 > 0) {
      sb_688[0] += x_678.substring(emitted_687, end_686);
      t_682 = sb_688[0];
      t_684 = t_682;
    } else {
      t_684 = x_678;
    }
    return new SafeUrl(t_684);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlCssDelegate extends type__438(HtmlDelegate) {
  /**
   * @param {string | null} s_692
   * @returns {string}
   */
  process(s_692) {
    let return_693;
    if (!(s_692 == null)) {
      return_693 = s_692;
    } else {
      return_693 = "";
    }
    return return_693;
  }
  /**
   * @param {HtmlEscaper} outer_695
   * @returns {HtmlEscaper | null}
   */
  escaper(outer_695) {
    return outer_695;
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlJsDelegate extends type__438(HtmlDelegate) {
  /**
   * @param {string | null} s_697
   * @returns {string}
   */
  process(s_697) {
    let return_698;
    if (!(s_697 == null)) {
      return_698 = s_697;
    } else {
      return_698 = "";
    }
    return return_698;
  }
  /**
   * @param {HtmlEscaper} outer_700
   * @returns {HtmlEscaper | null}
   */
  escaper(outer_700) {
    return outer_700;
  }
  constructor() {
    super ();
    return;
  }
};
/** @type {Array<string>} */
const strs_701 = Object.freeze(["AElig", "Æ", "AElig;", "Æ", "AMP", "\u0026", "AMP;", "\u0026", "Aacute", "Á", "Aacute;", "Á", "Abreve;", "Ă", "Acirc", "Â", "Acirc;", "Â", "Acy;", "А", "Afr;", "\ud835\udd04", "Agrave", "À", "Agrave;", "À", "Alpha;", "Α", "Amacr;", "Ā", "And;", "⩓", "Aogon;", "Ą", "Aopf;", "\ud835\udd38", "ApplyFunction;", "⁡", "Aring", "Å", "Aring;", "Å", "Ascr;", "\ud835\udc9c", "Assign;", "≔", "Atilde", "Ã", "Atilde;", "Ã", "Auml", "Ä", "Auml;", "Ä", "Backslash;", "∖", "Barv;", "⫧", "Barwed;", "⌆", "Bcy;", "Б", "Because;", "∵", "Bernoullis;", "ℬ", "Beta;", "Β", "Bfr;", "\ud835\udd05", "Bopf;", "\ud835\udd39", "Breve;", "˘", "Bscr;", "ℬ", "Bumpeq;", "≎", "CHcy;", "Ч", "COPY", "©", "COPY;", "©", "Cacute;", "Ć", "Cap;", "⋒", "CapitalDifferentialD;", "ⅅ", "Cayleys;", "ℭ", "Ccaron;", "Č", "Ccedil", "Ç", "Ccedil;", "Ç", "Ccirc;", "Ĉ", "Cconint;", "∰", "Cdot;", "Ċ", "Cedilla;", "¸", "CenterDot;", "·", "Cfr;", "ℭ", "Chi;", "Χ", "CircleDot;", "⊙", "CircleMinus;", "⊖", "CirclePlus;", "⊕", "CircleTimes;", "⊗", "ClockwiseContourIntegral;", "∲", "CloseCurlyDoubleQuote;", "”", "CloseCurlyQuote;", "’", "Colon;", "∷", "Colone;", "⩴", "Congruent;", "≡", "Conint;", "∯", "ContourIntegral;", "∮", "Copf;", "ℂ", "Coproduct;", "∐", "CounterClockwiseContourIntegral;", "∳", "Cross;", "⨯", "Cscr;", "\ud835\udc9e", "Cup;", "⋓", "CupCap;", "≍", "DD;", "ⅅ", "DDotrahd;", "⤑", "DJcy;", "Ђ", "DScy;", "Ѕ", "DZcy;", "Џ", "Dagger;", "‡", "Darr;", "↡", "Dashv;", "⫤", "Dcaron;", "Ď", "Dcy;", "Д", "Del;", "∇", "Delta;", "Δ", "Dfr;", "\ud835\udd07", "DiacriticalAcute;", "´", "DiacriticalDot;", "˙", "DiacriticalDoubleAcute;", "˝", "DiacriticalGrave;", "`", "DiacriticalTilde;", "˜", "Diamond;", "⋄", "DifferentialD;", "ⅆ", "Dopf;", "\ud835\udd3b", "Dot;", "¨", "DotDot;", "⃜", "DotEqual;", "≐", "DoubleContourIntegral;", "∯", "DoubleDot;", "¨", "DoubleDownArrow;", "⇓", "DoubleLeftArrow;", "⇐", "DoubleLeftRightArrow;", "⇔", "DoubleLeftTee;", "⫤", "DoubleLongLeftArrow;", "⟸", "DoubleLongLeftRightArrow;", "⟺", "DoubleLongRightArrow;", "⟹", "DoubleRightArrow;", "⇒", "DoubleRightTee;", "⊨", "DoubleUpArrow;", "⇑", "DoubleUpDownArrow;", "⇕", "DoubleVerticalBar;", "∥", "DownArrow;", "↓", "DownArrowBar;", "⤓", "DownArrowUpArrow;", "⇵", "DownBreve;", "̑", "DownLeftRightVector;", "⥐", "DownLeftTeeVector;", "⥞", "DownLeftVector;", "↽", "DownLeftVectorBar;", "⥖", "DownRightTeeVector;", "⥟", "DownRightVector;", "⇁", "DownRightVectorBar;", "⥗", "DownTee;", "⊤", "DownTeeArrow;", "↧", "Downarrow;", "⇓", "Dscr;", "\ud835\udc9f", "Dstrok;", "Đ", "ENG;", "Ŋ", "ETH", "Ð", "ETH;", "Ð", "Eacute", "É", "Eacute;", "É", "Ecaron;", "Ě", "Ecirc", "Ê", "Ecirc;", "Ê", "Ecy;", "Э", "Edot;", "Ė", "Efr;", "\ud835\udd08", "Egrave", "È", "Egrave;", "È", "Element;", "∈", "Emacr;", "Ē", "EmptySmallSquare;", "◻", "EmptyVerySmallSquare;", "▫", "Eogon;", "Ę", "Eopf;", "\ud835\udd3c", "Epsilon;", "Ε", "Equal;", "⩵", "EqualTilde;", "≂", "Equilibrium;", "⇌", "Escr;", "ℰ", "Esim;", "⩳", "Eta;", "Η", "Euml", "Ë", "Euml;", "Ë", "Exists;", "∃", "ExponentialE;", "ⅇ", "Fcy;", "Ф", "Ffr;", "\ud835\udd09", "FilledSmallSquare;", "◼", "FilledVerySmallSquare;", "▪", "Fopf;", "\ud835\udd3d", "ForAll;", "∀", "Fouriertrf;", "ℱ", "Fscr;", "ℱ", "GJcy;", "Ѓ", "GT", "\u003e", "GT;", "\u003e", "Gamma;", "Γ", "Gammad;", "Ϝ", "Gbreve;", "Ğ", "Gcedil;", "Ģ", "Gcirc;", "Ĝ", "Gcy;", "Г", "Gdot;", "Ġ", "Gfr;", "\ud835\udd0a", "Gg;", "⋙", "Gopf;", "\ud835\udd3e", "GreaterEqual;", "≥", "GreaterEqualLess;", "⋛", "GreaterFullEqual;", "≧", "GreaterGreater;", "⪢", "GreaterLess;", "≷", "GreaterSlantEqual;", "⩾", "GreaterTilde;", "≳", "Gscr;", "\ud835\udca2", "Gt;", "≫", "HARDcy;", "Ъ", "Hacek;", "ˇ", "Hat;", "^", "Hcirc;", "Ĥ", "Hfr;", "ℌ", "HilbertSpace;", "ℋ", "Hopf;", "ℍ", "HorizontalLine;", "─", "Hscr;", "ℋ", "Hstrok;", "Ħ", "HumpDownHump;", "≎", "HumpEqual;", "≏", "IEcy;", "Е", "IJlig;", "Ĳ", "IOcy;", "Ё", "Iacute", "Í", "Iacute;", "Í", "Icirc", "Î", "Icirc;", "Î", "Icy;", "И", "Idot;", "İ", "Ifr;", "ℑ", "Igrave", "Ì", "Igrave;", "Ì", "Im;", "ℑ", "Imacr;", "Ī", "ImaginaryI;", "ⅈ", "Implies;", "⇒", "Int;", "∬", "Integral;", "∫", "Intersection;", "⋂", "InvisibleComma;", "⁣", "InvisibleTimes;", "⁢", "Iogon;", "Į", "Iopf;", "\ud835\udd40", "Iota;", "Ι", "Iscr;", "ℐ", "Itilde;", "Ĩ", "Iukcy;", "І", "Iuml", "Ï", "Iuml;", "Ï", "Jcirc;", "Ĵ", "Jcy;", "Й", "Jfr;", "\ud835\udd0d", "Jopf;", "\ud835\udd41", "Jscr;", "\ud835\udca5", "Jsercy;", "Ј", "Jukcy;", "Є", "KHcy;", "Х", "KJcy;", "Ќ", "Kappa;", "Κ", "Kcedil;", "Ķ", "Kcy;", "К", "Kfr;", "\ud835\udd0e", "Kopf;", "\ud835\udd42", "Kscr;", "\ud835\udca6", "LJcy;", "Љ", "LT", "\u003c", "LT;", "\u003c", "Lacute;", "Ĺ", "Lambda;", "Λ", "Lang;", "⟪", "Laplacetrf;", "ℒ", "Larr;", "↞", "Lcaron;", "Ľ", "Lcedil;", "Ļ", "Lcy;", "Л", "LeftAngleBracket;", "⟨", "LeftArrow;", "←", "LeftArrowBar;", "⇤", "LeftArrowRightArrow;", "⇆", "LeftCeiling;", "⌈", "LeftDoubleBracket;", "⟦", "LeftDownTeeVector;", "⥡", "LeftDownVector;", "⇃", "LeftDownVectorBar;", "⥙", "LeftFloor;", "⌊", "LeftRightArrow;", "↔", "LeftRightVector;", "⥎", "LeftTee;", "⊣", "LeftTeeArrow;", "↤", "LeftTeeVector;", "⥚", "LeftTriangle;", "⊲", "LeftTriangleBar;", "⧏", "LeftTriangleEqual;", "⊴", "LeftUpDownVector;", "⥑", "LeftUpTeeVector;", "⥠", "LeftUpVector;", "↿", "LeftUpVectorBar;", "⥘", "LeftVector;", "↼", "LeftVectorBar;", "⥒", "Leftarrow;", "⇐", "Leftrightarrow;", "⇔", "LessEqualGreater;", "⋚", "LessFullEqual;", "≦", "LessGreater;", "≶", "LessLess;", "⪡", "LessSlantEqual;", "⩽", "LessTilde;", "≲", "Lfr;", "\ud835\udd0f", "Ll;", "⋘", "Lleftarrow;", "⇚", "Lmidot;", "Ŀ", "LongLeftArrow;", "⟵", "LongLeftRightArrow;", "⟷", "LongRightArrow;", "⟶", "Longleftarrow;", "⟸", "Longleftrightarrow;", "⟺", "Longrightarrow;", "⟹", "Lopf;", "\ud835\udd43", "LowerLeftArrow;", "↙", "LowerRightArrow;", "↘", "Lscr;", "ℒ", "Lsh;", "↰", "Lstrok;", "Ł", "Lt;", "≪", "Map;", "⤅", "Mcy;", "М", "MediumSpace;", " ", "Mellintrf;", "ℳ", "Mfr;", "\ud835\udd10", "MinusPlus;", "∓", "Mopf;", "\ud835\udd44", "Mscr;", "ℳ", "Mu;", "Μ", "NJcy;", "Њ", "Nacute;", "Ń", "Ncaron;", "Ň", "Ncedil;", "Ņ", "Ncy;", "Н", "NegativeMediumSpace;", "​", "NegativeThickSpace;", "​", "NegativeThinSpace;", "​", "NegativeVeryThinSpace;", "​", "NestedGreaterGreater;", "≫", "NestedLessLess;", "≪", "NewLine;", "\n", "Nfr;", "\ud835\udd11", "NoBreak;", "⁠", "NonBreakingSpace;", " ", "Nopf;", "ℕ", "Not;", "⫬", "NotCongruent;", "≢", "NotCupCap;", "≭", "NotDoubleVerticalBar;", "∦", "NotElement;", "∉", "NotEqual;", "≠", "NotEqualTilde;", "≂̸", "NotExists;", "∄", "NotGreater;", "≯", "NotGreaterEqual;", "≱", "NotGreaterFullEqual;", "≧̸", "NotGreaterGreater;", "≫̸", "NotGreaterLess;", "≹", "NotGreaterSlantEqual;", "⩾̸", "NotGreaterTilde;", "≵", "NotHumpDownHump;", "≎̸", "NotHumpEqual;", "≏̸", "NotLeftTriangle;", "⋪", "NotLeftTriangleBar;", "⧏̸", "NotLeftTriangleEqual;", "⋬", "NotLess;", "≮", "NotLessEqual;", "≰", "NotLessGreater;", "≸", "NotLessLess;", "≪̸", "NotLessSlantEqual;", "⩽̸", "NotLessTilde;", "≴", "NotNestedGreaterGreater;", "⪢̸", "NotNestedLessLess;", "⪡̸", "NotPrecedes;", "⊀", "NotPrecedesEqual;", "⪯̸", "NotPrecedesSlantEqual;", "⋠", "NotReverseElement;", "∌", "NotRightTriangle;", "⋫", "NotRightTriangleBar;", "⧐̸", "NotRightTriangleEqual;", "⋭", "NotSquareSubset;", "⊏̸", "NotSquareSubsetEqual;", "⋢", "NotSquareSuperset;", "⊐̸", "NotSquareSupersetEqual;", "⋣", "NotSubset;", "⊂⃒", "NotSubsetEqual;", "⊈", "NotSucceeds;", "⊁", "NotSucceedsEqual;", "⪰̸", "NotSucceedsSlantEqual;", "⋡", "NotSucceedsTilde;", "≿̸", "NotSuperset;", "⊃⃒", "NotSupersetEqual;", "⊉", "NotTilde;", "≁", "NotTildeEqual;", "≄", "NotTildeFullEqual;", "≇", "NotTildeTilde;", "≉", "NotVerticalBar;", "∤", "Nscr;", "\ud835\udca9", "Ntilde", "Ñ", "Ntilde;", "Ñ", "Nu;", "Ν", "OElig;", "Œ", "Oacute", "Ó", "Oacute;", "Ó", "Ocirc", "Ô", "Ocirc;", "Ô", "Ocy;", "О", "Odblac;", "Ő", "Ofr;", "\ud835\udd12", "Ograve", "Ò", "Ograve;", "Ò", "Omacr;", "Ō", "Omega;", "Ω", "Omicron;", "Ο", "Oopf;", "\ud835\udd46", "OpenCurlyDoubleQuote;", "“", "OpenCurlyQuote;", "‘", "Or;", "⩔", "Oscr;", "\ud835\udcaa", "Oslash", "Ø", "Oslash;", "Ø", "Otilde", "Õ", "Otilde;", "Õ", "Otimes;", "⨷", "Ouml", "Ö", "Ouml;", "Ö", "OverBar;", "‾", "OverBrace;", "⏞", "OverBracket;", "⎴", "OverParenthesis;", "⏜", "PartialD;", "∂", "Pcy;", "П", "Pfr;", "\ud835\udd13", "Phi;", "Φ", "Pi;", "Π", "PlusMinus;", "±", "Poincareplane;", "ℌ", "Popf;", "ℙ", "Pr;", "⪻", "Precedes;", "≺", "PrecedesEqual;", "⪯", "PrecedesSlantEqual;", "≼", "PrecedesTilde;", "≾", "Prime;", "″", "Product;", "∏", "Proportion;", "∷", "Proportional;", "∝", "Pscr;", "\ud835\udcab", "Psi;", "Ψ", "QUOT", "\u0022", "QUOT;", "\u0022", "Qfr;", "\ud835\udd14", "Qopf;", "ℚ", "Qscr;", "\ud835\udcac", "RBarr;", "⤐", "REG", "®", "REG;", "®", "Racute;", "Ŕ", "Rang;", "⟫", "Rarr;", "↠", "Rarrtl;", "⤖", "Rcaron;", "Ř", "Rcedil;", "Ŗ", "Rcy;", "Р", "Re;", "ℜ", "ReverseElement;", "∋", "ReverseEquilibrium;", "⇋", "ReverseUpEquilibrium;", "⥯", "Rfr;", "ℜ", "Rho;", "Ρ", "RightAngleBracket;", "⟩", "RightArrow;", "→", "RightArrowBar;", "⇥", "RightArrowLeftArrow;", "⇄", "RightCeiling;", "⌉", "RightDoubleBracket;", "⟧", "RightDownTeeVector;", "⥝", "RightDownVector;", "⇂", "RightDownVectorBar;", "⥕", "RightFloor;", "⌋", "RightTee;", "⊢", "RightTeeArrow;", "↦", "RightTeeVector;", "⥛", "RightTriangle;", "⊳", "RightTriangleBar;", "⧐", "RightTriangleEqual;", "⊵", "RightUpDownVector;", "⥏", "RightUpTeeVector;", "⥜", "RightUpVector;", "↾", "RightUpVectorBar;", "⥔", "RightVector;", "⇀", "RightVectorBar;", "⥓", "Rightarrow;", "⇒", "Ropf;", "ℝ", "RoundImplies;", "⥰", "Rrightarrow;", "⇛", "Rscr;", "ℛ", "Rsh;", "↱", "RuleDelayed;", "⧴", "SHCHcy;", "Щ", "SHcy;", "Ш", "SOFTcy;", "Ь", "Sacute;", "Ś", "Sc;", "⪼", "Scaron;", "Š", "Scedil;", "Ş", "Scirc;", "Ŝ", "Scy;", "С", "Sfr;", "\ud835\udd16", "ShortDownArrow;", "↓", "ShortLeftArrow;", "←", "ShortRightArrow;", "→", "ShortUpArrow;", "↑", "Sigma;", "Σ", "SmallCircle;", "∘", "Sopf;", "\ud835\udd4a", "Sqrt;", "√", "Square;", "□", "SquareIntersection;", "⊓", "SquareSubset;", "⊏", "SquareSubsetEqual;", "⊑", "SquareSuperset;", "⊐", "SquareSupersetEqual;", "⊒", "SquareUnion;", "⊔", "Sscr;", "\ud835\udcae", "Star;", "⋆", "Sub;", "⋐", "Subset;", "⋐", "SubsetEqual;", "⊆", "Succeeds;", "≻", "SucceedsEqual;", "⪰", "SucceedsSlantEqual;", "≽", "SucceedsTilde;", "≿", "SuchThat;", "∋", "Sum;", "∑", "Sup;", "⋑", "Superset;", "⊃", "SupersetEqual;", "⊇", "Supset;", "⋑", "THORN", "Þ", "THORN;", "Þ", "TRADE;", "™", "TSHcy;", "Ћ", "TScy;", "Ц", "Tab;", "\t", "Tau;", "Τ", "Tcaron;", "Ť", "Tcedil;", "Ţ", "Tcy;", "Т", "Tfr;", "\ud835\udd17", "Therefore;", "∴", "Theta;", "Θ", "ThickSpace;", "  ", "ThinSpace;", " ", "Tilde;", "∼", "TildeEqual;", "≃", "TildeFullEqual;", "≅", "TildeTilde;", "≈", "Topf;", "\ud835\udd4b", "TripleDot;", "⃛", "Tscr;", "\ud835\udcaf", "Tstrok;", "Ŧ", "Uacute", "Ú", "Uacute;", "Ú", "Uarr;", "↟", "Uarrocir;", "⥉", "Ubrcy;", "Ў", "Ubreve;", "Ŭ", "Ucirc", "Û", "Ucirc;", "Û", "Ucy;", "У", "Udblac;", "Ű", "Ufr;", "\ud835\udd18", "Ugrave", "Ù", "Ugrave;", "Ù", "Umacr;", "Ū", "UnderBar;", "_", "UnderBrace;", "⏟", "UnderBracket;", "⎵", "UnderParenthesis;", "⏝", "Union;", "⋃", "UnionPlus;", "⊎", "Uogon;", "Ų", "Uopf;", "\ud835\udd4c", "UpArrow;", "↑", "UpArrowBar;", "⤒", "UpArrowDownArrow;", "⇅", "UpDownArrow;", "↕", "UpEquilibrium;", "⥮", "UpTee;", "⊥", "UpTeeArrow;", "↥", "Uparrow;", "⇑", "Updownarrow;", "⇕", "UpperLeftArrow;", "↖", "UpperRightArrow;", "↗", "Upsi;", "ϒ", "Upsilon;", "Υ", "Uring;", "Ů", "Uscr;", "\ud835\udcb0", "Utilde;", "Ũ", "Uuml", "Ü", "Uuml;", "Ü", "VDash;", "⊫", "Vbar;", "⫫", "Vcy;", "В", "Vdash;", "⊩", "Vdashl;", "⫦", "Vee;", "⋁", "Verbar;", "‖", "Vert;", "‖", "VerticalBar;", "∣", "VerticalLine;", "|", "VerticalSeparator;", "❘", "VerticalTilde;", "≀", "VeryThinSpace;", " ", "Vfr;", "\ud835\udd19", "Vopf;", "\ud835\udd4d", "Vscr;", "\ud835\udcb1", "Vvdash;", "⊪", "Wcirc;", "Ŵ", "Wedge;", "⋀", "Wfr;", "\ud835\udd1a", "Wopf;", "\ud835\udd4e", "Wscr;", "\ud835\udcb2", "Xfr;", "\ud835\udd1b", "Xi;", "Ξ", "Xopf;", "\ud835\udd4f", "Xscr;", "\ud835\udcb3", "YAcy;", "Я", "YIcy;", "Ї", "YUcy;", "Ю", "Yacute", "Ý", "Yacute;", "Ý", "Ycirc;", "Ŷ", "Ycy;", "Ы", "Yfr;", "\ud835\udd1c", "Yopf;", "\ud835\udd50", "Yscr;", "\ud835\udcb4", "Yuml;", "Ÿ", "ZHcy;", "Ж", "Zacute;", "Ź", "Zcaron;", "Ž", "Zcy;", "З", "Zdot;", "Ż", "ZeroWidthSpace;", "​", "Zeta;", "Ζ", "Zfr;", "ℨ", "Zopf;", "ℤ", "Zscr;", "\ud835\udcb5", "aacute", "á", "aacute;", "á", "abreve;", "ă", "ac;", "∾", "acE;", "∾̳", "acd;", "∿", "acirc", "â", "acirc;", "â", "acute", "´", "acute;", "´", "acy;", "а", "aelig", "æ", "aelig;", "æ", "af;", "⁡", "afr;", "\ud835\udd1e", "agrave", "à", "agrave;", "à", "alefsym;", "ℵ", "aleph;", "ℵ", "alpha;", "α", "amacr;", "ā", "amalg;", "⨿", "amp", "\u0026", "amp;", "\u0026", "and;", "∧", "andand;", "⩕", "andd;", "⩜", "andslope;", "⩘", "andv;", "⩚", "ang;", "∠", "ange;", "⦤", "angle;", "∠", "angmsd;", "∡", "angmsdaa;", "⦨", "angmsdab;", "⦩", "angmsdac;", "⦪", "angmsdad;", "⦫", "angmsdae;", "⦬", "angmsdaf;", "⦭", "angmsdag;", "⦮", "angmsdah;", "⦯", "angrt;", "∟", "angrtvb;", "⊾", "angrtvbd;", "⦝", "angsph;", "∢", "angst;", "Å", "angzarr;", "⍼", "aogon;", "ą", "aopf;", "\ud835\udd52", "ap;", "≈", "apE;", "⩰", "apacir;", "⩯", "ape;", "≊", "apid;", "≋", "apos;", "'", "approx;", "≈", "approxeq;", "≊", "aring", "å", "aring;", "å", "ascr;", "\ud835\udcb6", "ast;", "*", "asymp;", "≈", "asympeq;", "≍", "atilde", "ã", "atilde;", "ã", "auml", "ä", "auml;", "ä", "awconint;", "∳", "awint;", "⨑", "bNot;", "⫭", "backcong;", "≌", "backepsilon;", "϶", "backprime;", "‵", "backsim;", "∽", "backsimeq;", "⋍", "barvee;", "⊽", "barwed;", "⌅", "barwedge;", "⌅", "bbrk;", "⎵", "bbrktbrk;", "⎶", "bcong;", "≌", "bcy;", "б", "bdquo;", "„", "becaus;", "∵", "because;", "∵", "bemptyv;", "⦰", "bepsi;", "϶", "bernou;", "ℬ", "beta;", "β", "beth;", "ℶ", "between;", "≬", "bfr;", "\ud835\udd1f", "bigcap;", "⋂", "bigcirc;", "◯", "bigcup;", "⋃", "bigodot;", "⨀", "bigoplus;", "⨁", "bigotimes;", "⨂", "bigsqcup;", "⨆", "bigstar;", "★", "bigtriangledown;", "▽", "bigtriangleup;", "△", "biguplus;", "⨄", "bigvee;", "⋁", "bigwedge;", "⋀", "bkarow;", "⤍", "blacklozenge;", "⧫", "blacksquare;", "▪", "blacktriangle;", "▴", "blacktriangledown;", "▾", "blacktriangleleft;", "◂", "blacktriangleright;", "▸", "blank;", "␣", "blk12;", "▒", "blk14;", "░", "blk34;", "▓", "block;", "█", "bne;", "=⃥", "bnequiv;", "≡⃥", "bnot;", "⌐", "bopf;", "\ud835\udd53", "bot;", "⊥", "bottom;", "⊥", "bowtie;", "⋈", "boxDL;", "╗", "boxDR;", "╔", "boxDl;", "╖", "boxDr;", "╓", "boxH;", "═", "boxHD;", "╦", "boxHU;", "╩", "boxHd;", "╤", "boxHu;", "╧", "boxUL;", "╝", "boxUR;", "╚", "boxUl;", "╜", "boxUr;", "╙", "boxV;", "║", "boxVH;", "╬", "boxVL;", "╣", "boxVR;", "╠", "boxVh;", "╫", "boxVl;", "╢", "boxVr;", "╟", "boxbox;", "⧉", "boxdL;", "╕", "boxdR;", "╒", "boxdl;", "┐", "boxdr;", "┌", "boxh;", "─", "boxhD;", "╥", "boxhU;", "╨", "boxhd;", "┬", "boxhu;", "┴", "boxminus;", "⊟", "boxplus;", "⊞", "boxtimes;", "⊠", "boxuL;", "╛", "boxuR;", "╘", "boxul;", "┘", "boxur;", "└", "boxv;", "│", "boxvH;", "╪", "boxvL;", "╡", "boxvR;", "╞", "boxvh;", "┼", "boxvl;", "┤", "boxvr;", "├", "bprime;", "‵", "breve;", "˘", "brvbar", "¦", "brvbar;", "¦", "bscr;", "\ud835\udcb7", "bsemi;", "⁏", "bsim;", "∽", "bsime;", "⋍", "bsol;", "\\", "bsolb;", "⧅", "bsolhsub;", "⟈", "bull;", "•", "bullet;", "•", "bump;", "≎", "bumpE;", "⪮", "bumpe;", "≏", "bumpeq;", "≏", "cacute;", "ć", "cap;", "∩", "capand;", "⩄", "capbrcup;", "⩉", "capcap;", "⩋", "capcup;", "⩇", "capdot;", "⩀", "caps;", "∩︀", "caret;", "⁁", "caron;", "ˇ", "ccaps;", "⩍", "ccaron;", "č", "ccedil", "ç", "ccedil;", "ç", "ccirc;", "ĉ", "ccups;", "⩌", "ccupssm;", "⩐", "cdot;", "ċ", "cedil", "¸", "cedil;", "¸", "cemptyv;", "⦲", "cent", "¢", "cent;", "¢", "centerdot;", "·", "cfr;", "\ud835\udd20", "chcy;", "ч", "check;", "✓", "checkmark;", "✓", "chi;", "χ", "cir;", "○", "cirE;", "⧃", "circ;", "ˆ", "circeq;", "≗", "circlearrowleft;", "↺", "circlearrowright;", "↻", "circledR;", "®", "circledS;", "Ⓢ", "circledast;", "⊛", "circledcirc;", "⊚", "circleddash;", "⊝", "cire;", "≗", "cirfnint;", "⨐", "cirmid;", "⫯", "cirscir;", "⧂", "clubs;", "♣", "clubsuit;", "♣", "colon;", ":", "colone;", "≔", "coloneq;", "≔", "comma;", ",", "commat;", "@", "comp;", "∁", "compfn;", "∘", "complement;", "∁", "complexes;", "ℂ", "cong;", "≅", "congdot;", "⩭", "conint;", "∮", "copf;", "\ud835\udd54", "coprod;", "∐", "copy", "©", "copy;", "©", "copysr;", "℗", "crarr;", "↵", "cross;", "✗", "cscr;", "\ud835\udcb8", "csub;", "⫏", "csube;", "⫑", "csup;", "⫐", "csupe;", "⫒", "ctdot;", "⋯", "cudarrl;", "⤸", "cudarrr;", "⤵", "cuepr;", "⋞", "cuesc;", "⋟", "cularr;", "↶", "cularrp;", "⤽", "cup;", "∪", "cupbrcap;", "⩈", "cupcap;", "⩆", "cupcup;", "⩊", "cupdot;", "⊍", "cupor;", "⩅", "cups;", "∪︀", "curarr;", "↷", "curarrm;", "⤼", "curlyeqprec;", "⋞", "curlyeqsucc;", "⋟", "curlyvee;", "⋎", "curlywedge;", "⋏", "curren", "¤", "curren;", "¤", "curvearrowleft;", "↶", "curvearrowright;", "↷", "cuvee;", "⋎", "cuwed;", "⋏", "cwconint;", "∲", "cwint;", "∱", "cylcty;", "⌭", "dArr;", "⇓", "dHar;", "⥥", "dagger;", "†", "daleth;", "ℸ", "darr;", "↓", "dash;", "‐", "dashv;", "⊣", "dbkarow;", "⤏", "dblac;", "˝", "dcaron;", "ď", "dcy;", "д", "dd;", "ⅆ", "ddagger;", "‡", "ddarr;", "⇊", "ddotseq;", "⩷", "deg", "°", "deg;", "°", "delta;", "δ", "demptyv;", "⦱", "dfisht;", "⥿", "dfr;", "\ud835\udd21", "dharl;", "⇃", "dharr;", "⇂", "diam;", "⋄", "diamond;", "⋄", "diamondsuit;", "♦", "diams;", "♦", "die;", "¨", "digamma;", "ϝ", "disin;", "⋲", "div;", "÷", "divide", "÷", "divide;", "÷", "divideontimes;", "⋇", "divonx;", "⋇", "djcy;", "ђ", "dlcorn;", "⌞", "dlcrop;", "⌍", "dollar;", "\u0024", "dopf;", "\ud835\udd55", "dot;", "˙", "doteq;", "≐", "doteqdot;", "≑", "dotminus;", "∸", "dotplus;", "∔", "dotsquare;", "⊡", "doublebarwedge;", "⌆", "downarrow;", "↓", "downdownarrows;", "⇊", "downharpoonleft;", "⇃", "downharpoonright;", "⇂", "drbkarow;", "⤐", "drcorn;", "⌟", "drcrop;", "⌌", "dscr;", "\ud835\udcb9", "dscy;", "ѕ", "dsol;", "⧶", "dstrok;", "đ", "dtdot;", "⋱", "dtri;", "▿", "dtrif;", "▾", "duarr;", "⇵", "duhar;", "⥯", "dwangle;", "⦦", "dzcy;", "џ", "dzigrarr;", "⟿", "eDDot;", "⩷", "eDot;", "≑", "eacute", "é", "eacute;", "é", "easter;", "⩮", "ecaron;", "ě", "ecir;", "≖", "ecirc", "ê", "ecirc;", "ê", "ecolon;", "≕", "ecy;", "э", "edot;", "ė", "ee;", "ⅇ", "efDot;", "≒", "efr;", "\ud835\udd22", "eg;", "⪚", "egrave", "è", "egrave;", "è", "egs;", "⪖", "egsdot;", "⪘", "el;", "⪙", "elinters;", "⏧", "ell;", "ℓ", "els;", "⪕", "elsdot;", "⪗", "emacr;", "ē", "empty;", "∅", "emptyset;", "∅", "emptyv;", "∅", "emsp13;", " ", "emsp14;", " ", "emsp;", " ", "eng;", "ŋ", "ensp;", " ", "eogon;", "ę", "eopf;", "\ud835\udd56", "epar;", "⋕", "eparsl;", "⧣", "eplus;", "⩱", "epsi;", "ε", "epsilon;", "ε", "epsiv;", "ϵ", "eqcirc;", "≖", "eqcolon;", "≕", "eqsim;", "≂", "eqslantgtr;", "⪖", "eqslantless;", "⪕", "equals;", "=", "equest;", "≟", "equiv;", "≡", "equivDD;", "⩸", "eqvparsl;", "⧥", "erDot;", "≓", "erarr;", "⥱", "escr;", "ℯ", "esdot;", "≐", "esim;", "≂", "eta;", "η", "eth", "ð", "eth;", "ð", "euml", "ë", "euml;", "ë", "euro;", "€", "excl;", "!", "exist;", "∃", "expectation;", "ℰ", "exponentiale;", "ⅇ", "fallingdotseq;", "≒", "fcy;", "ф", "female;", "♀", "ffilig;", "ﬃ", "fflig;", "ﬀ", "ffllig;", "ﬄ", "ffr;", "\ud835\udd23", "filig;", "ﬁ", "fjlig;", "fj", "flat;", "♭", "fllig;", "ﬂ", "fltns;", "▱", "fnof;", "ƒ", "fopf;", "\ud835\udd57", "forall;", "∀", "fork;", "⋔", "forkv;", "⫙", "fpartint;", "⨍", "frac12", "½", "frac12;", "½", "frac13;", "⅓", "frac14", "¼", "frac14;", "¼", "frac15;", "⅕", "frac16;", "⅙", "frac18;", "⅛", "frac23;", "⅔", "frac25;", "⅖", "frac34", "¾", "frac34;", "¾", "frac35;", "⅗", "frac38;", "⅜", "frac45;", "⅘", "frac56;", "⅚", "frac58;", "⅝", "frac78;", "⅞", "frasl;", "⁄", "frown;", "⌢", "fscr;", "\ud835\udcbb", "gE;", "≧", "gEl;", "⪌", "gacute;", "ǵ", "gamma;", "γ", "gammad;", "ϝ", "gap;", "⪆", "gbreve;", "ğ", "gcirc;", "ĝ", "gcy;", "г", "gdot;", "ġ", "ge;", "≥", "gel;", "⋛", "geq;", "≥", "geqq;", "≧", "geqslant;", "⩾", "ges;", "⩾", "gescc;", "⪩", "gesdot;", "⪀", "gesdoto;", "⪂", "gesdotol;", "⪄", "gesl;", "⋛︀", "gesles;", "⪔", "gfr;", "\ud835\udd24", "gg;", "≫", "ggg;", "⋙", "gimel;", "ℷ", "gjcy;", "ѓ", "gl;", "≷", "glE;", "⪒", "gla;", "⪥", "glj;", "⪤", "gnE;", "≩", "gnap;", "⪊", "gnapprox;", "⪊", "gne;", "⪈", "gneq;", "⪈", "gneqq;", "≩", "gnsim;", "⋧", "gopf;", "\ud835\udd58", "grave;", "`", "gscr;", "ℊ", "gsim;", "≳", "gsime;", "⪎", "gsiml;", "⪐", "gt", "\u003e", "gt;", "\u003e", "gtcc;", "⪧", "gtcir;", "⩺", "gtdot;", "⋗", "gtlPar;", "⦕", "gtquest;", "⩼", "gtrapprox;", "⪆", "gtrarr;", "⥸", "gtrdot;", "⋗", "gtreqless;", "⋛", "gtreqqless;", "⪌", "gtrless;", "≷", "gtrsim;", "≳", "gvertneqq;", "≩︀", "gvnE;", "≩︀", "hArr;", "⇔", "hairsp;", " ", "half;", "½", "hamilt;", "ℋ", "hardcy;", "ъ", "harr;", "↔", "harrcir;", "⥈", "harrw;", "↭", "hbar;", "ℏ", "hcirc;", "ĥ", "hearts;", "♥", "heartsuit;", "♥", "hellip;", "…", "hercon;", "⊹", "hfr;", "\ud835\udd25", "hksearow;", "⤥", "hkswarow;", "⤦", "hoarr;", "⇿", "homtht;", "∻", "hookleftarrow;", "↩", "hookrightarrow;", "↪", "hopf;", "\ud835\udd59", "horbar;", "―", "hscr;", "\ud835\udcbd", "hslash;", "ℏ", "hstrok;", "ħ", "hybull;", "⁃", "hyphen;", "‐", "iacute", "í", "iacute;", "í", "ic;", "⁣", "icirc", "î", "icirc;", "î", "icy;", "и", "iecy;", "е", "iexcl", "¡", "iexcl;", "¡", "iff;", "⇔", "ifr;", "\ud835\udd26", "igrave", "ì", "igrave;", "ì", "ii;", "ⅈ", "iiiint;", "⨌", "iiint;", "∭", "iinfin;", "⧜", "iiota;", "℩", "ijlig;", "ĳ", "imacr;", "ī", "image;", "ℑ", "imagline;", "ℐ", "imagpart;", "ℑ", "imath;", "ı", "imof;", "⊷", "imped;", "Ƶ", "in;", "∈", "incare;", "℅", "infin;", "∞", "infintie;", "⧝", "inodot;", "ı", "int;", "∫", "intcal;", "⊺", "integers;", "ℤ", "intercal;", "⊺", "intlarhk;", "⨗", "intprod;", "⨼", "iocy;", "ё", "iogon;", "į", "iopf;", "\ud835\udd5a", "iota;", "ι", "iprod;", "⨼", "iquest", "¿", "iquest;", "¿", "iscr;", "\ud835\udcbe", "isin;", "∈", "isinE;", "⋹", "isindot;", "⋵", "isins;", "⋴", "isinsv;", "⋳", "isinv;", "∈", "it;", "⁢", "itilde;", "ĩ", "iukcy;", "і", "iuml", "ï", "iuml;", "ï", "jcirc;", "ĵ", "jcy;", "й", "jfr;", "\ud835\udd27", "jmath;", "ȷ", "jopf;", "\ud835\udd5b", "jscr;", "\ud835\udcbf", "jsercy;", "ј", "jukcy;", "є", "kappa;", "κ", "kappav;", "ϰ", "kcedil;", "ķ", "kcy;", "к", "kfr;", "\ud835\udd28", "kgreen;", "ĸ", "khcy;", "х", "kjcy;", "ќ", "kopf;", "\ud835\udd5c", "kscr;", "\ud835\udcc0", "lAarr;", "⇚", "lArr;", "⇐", "lAtail;", "⤛", "lBarr;", "⤎", "lE;", "≦", "lEg;", "⪋", "lHar;", "⥢", "lacute;", "ĺ", "laemptyv;", "⦴", "lagran;", "ℒ", "lambda;", "λ", "lang;", "⟨", "langd;", "⦑", "langle;", "⟨", "lap;", "⪅", "laquo", "«", "laquo;", "«", "larr;", "←", "larrb;", "⇤", "larrbfs;", "⤟", "larrfs;", "⤝", "larrhk;", "↩", "larrlp;", "↫", "larrpl;", "⤹", "larrsim;", "⥳", "larrtl;", "↢", "lat;", "⪫", "latail;", "⤙", "late;", "⪭", "lates;", "⪭︀", "lbarr;", "⤌", "lbbrk;", "❲", "lbrace;", "{", "lbrack;", "[", "lbrke;", "⦋", "lbrksld;", "⦏", "lbrkslu;", "⦍", "lcaron;", "ľ", "lcedil;", "ļ", "lceil;", "⌈", "lcub;", "{", "lcy;", "л", "ldca;", "⤶", "ldquo;", "“", "ldquor;", "„", "ldrdhar;", "⥧", "ldrushar;", "⥋", "ldsh;", "↲", "le;", "≤", "leftarrow;", "←", "leftarrowtail;", "↢", "leftharpoondown;", "↽", "leftharpoonup;", "↼", "leftleftarrows;", "⇇", "leftrightarrow;", "↔", "leftrightarrows;", "⇆", "leftrightharpoons;", "⇋", "leftrightsquigarrow;", "↭", "leftthreetimes;", "⋋", "leg;", "⋚", "leq;", "≤", "leqq;", "≦", "leqslant;", "⩽", "les;", "⩽", "lescc;", "⪨", "lesdot;", "⩿", "lesdoto;", "⪁", "lesdotor;", "⪃", "lesg;", "⋚︀", "lesges;", "⪓", "lessapprox;", "⪅", "lessdot;", "⋖", "lesseqgtr;", "⋚", "lesseqqgtr;", "⪋", "lessgtr;", "≶", "lesssim;", "≲", "lfisht;", "⥼", "lfloor;", "⌊", "lfr;", "\ud835\udd29", "lg;", "≶", "lgE;", "⪑", "lhard;", "↽", "lharu;", "↼", "lharul;", "⥪", "lhblk;", "▄", "ljcy;", "љ", "ll;", "≪", "llarr;", "⇇", "llcorner;", "⌞", "llhard;", "⥫", "lltri;", "◺", "lmidot;", "ŀ", "lmoust;", "⎰", "lmoustache;", "⎰", "lnE;", "≨", "lnap;", "⪉", "lnapprox;", "⪉", "lne;", "⪇", "lneq;", "⪇", "lneqq;", "≨", "lnsim;", "⋦", "loang;", "⟬", "loarr;", "⇽", "lobrk;", "⟦", "longleftarrow;", "⟵", "longleftrightarrow;", "⟷", "longmapsto;", "⟼", "longrightarrow;", "⟶", "looparrowleft;", "↫", "looparrowright;", "↬", "lopar;", "⦅", "lopf;", "\ud835\udd5d", "loplus;", "⨭", "lotimes;", "⨴", "lowast;", "∗", "lowbar;", "_", "loz;", "◊", "lozenge;", "◊", "lozf;", "⧫", "lpar;", "(", "lparlt;", "⦓", "lrarr;", "⇆", "lrcorner;", "⌟", "lrhar;", "⇋", "lrhard;", "⥭", "lrm;", "‎", "lrtri;", "⊿", "lsaquo;", "‹", "lscr;", "\ud835\udcc1", "lsh;", "↰", "lsim;", "≲", "lsime;", "⪍", "lsimg;", "⪏", "lsqb;", "[", "lsquo;", "‘", "lsquor;", "‚", "lstrok;", "ł", "lt", "\u003c", "lt;", "\u003c", "ltcc;", "⪦", "ltcir;", "⩹", "ltdot;", "⋖", "lthree;", "⋋", "ltimes;", "⋉", "ltlarr;", "⥶", "ltquest;", "⩻", "ltrPar;", "⦖", "ltri;", "◃", "ltrie;", "⊴", "ltrif;", "◂", "lurdshar;", "⥊", "luruhar;", "⥦", "lvertneqq;", "≨︀", "lvnE;", "≨︀", "mDDot;", "∺", "macr", "¯", "macr;", "¯", "male;", "♂", "malt;", "✠", "maltese;", "✠", "map;", "↦", "mapsto;", "↦", "mapstodown;", "↧", "mapstoleft;", "↤", "mapstoup;", "↥", "marker;", "▮", "mcomma;", "⨩", "mcy;", "м", "mdash;", "—", "measuredangle;", "∡", "mfr;", "\ud835\udd2a", "mho;", "℧", "micro", "µ", "micro;", "µ", "mid;", "∣", "midast;", "*", "midcir;", "⫰", "middot", "·", "middot;", "·", "minus;", "−", "minusb;", "⊟", "minusd;", "∸", "minusdu;", "⨪", "mlcp;", "⫛", "mldr;", "…", "mnplus;", "∓", "models;", "⊧", "mopf;", "\ud835\udd5e", "mp;", "∓", "mscr;", "\ud835\udcc2", "mstpos;", "∾", "mu;", "μ", "multimap;", "⊸", "mumap;", "⊸", "nGg;", "⋙̸", "nGt;", "≫⃒", "nGtv;", "≫̸", "nLeftarrow;", "⇍", "nLeftrightarrow;", "⇎", "nLl;", "⋘̸", "nLt;", "≪⃒", "nLtv;", "≪̸", "nRightarrow;", "⇏", "nVDash;", "⊯", "nVdash;", "⊮", "nabla;", "∇", "nacute;", "ń", "nang;", "∠⃒", "nap;", "≉", "napE;", "⩰̸", "napid;", "≋̸", "napos;", "ŉ", "napprox;", "≉", "natur;", "♮", "natural;", "♮", "naturals;", "ℕ", "nbsp", " ", "nbsp;", " ", "nbump;", "≎̸", "nbumpe;", "≏̸", "ncap;", "⩃", "ncaron;", "ň", "ncedil;", "ņ", "ncong;", "≇", "ncongdot;", "⩭̸", "ncup;", "⩂", "ncy;", "н", "ndash;", "–", "ne;", "≠", "neArr;", "⇗", "nearhk;", "⤤", "nearr;", "↗", "nearrow;", "↗", "nedot;", "≐̸", "nequiv;", "≢", "nesear;", "⤨", "nesim;", "≂̸", "nexist;", "∄", "nexists;", "∄", "nfr;", "\ud835\udd2b", "ngE;", "≧̸", "nge;", "≱", "ngeq;", "≱", "ngeqq;", "≧̸", "ngeqslant;", "⩾̸", "nges;", "⩾̸", "ngsim;", "≵", "ngt;", "≯", "ngtr;", "≯", "nhArr;", "⇎", "nharr;", "↮", "nhpar;", "⫲", "ni;", "∋", "nis;", "⋼", "nisd;", "⋺", "niv;", "∋", "njcy;", "њ", "nlArr;", "⇍", "nlE;", "≦̸", "nlarr;", "↚", "nldr;", "‥", "nle;", "≰", "nleftarrow;", "↚", "nleftrightarrow;", "↮", "nleq;", "≰", "nleqq;", "≦̸", "nleqslant;", "⩽̸", "nles;", "⩽̸", "nless;", "≮", "nlsim;", "≴", "nlt;", "≮", "nltri;", "⋪", "nltrie;", "⋬", "nmid;", "∤", "nopf;", "\ud835\udd5f", "not", "¬", "not;", "¬", "notin;", "∉", "notinE;", "⋹̸", "notindot;", "⋵̸", "notinva;", "∉", "notinvb;", "⋷", "notinvc;", "⋶", "notni;", "∌", "notniva;", "∌", "notnivb;", "⋾", "notnivc;", "⋽", "npar;", "∦", "nparallel;", "∦", "nparsl;", "⫽⃥", "npart;", "∂̸", "npolint;", "⨔", "npr;", "⊀", "nprcue;", "⋠", "npre;", "⪯̸", "nprec;", "⊀", "npreceq;", "⪯̸", "nrArr;", "⇏", "nrarr;", "↛", "nrarrc;", "⤳̸", "nrarrw;", "↝̸", "nrightarrow;", "↛", "nrtri;", "⋫", "nrtrie;", "⋭", "nsc;", "⊁", "nsccue;", "⋡", "nsce;", "⪰̸", "nscr;", "\ud835\udcc3", "nshortmid;", "∤", "nshortparallel;", "∦", "nsim;", "≁", "nsime;", "≄", "nsimeq;", "≄", "nsmid;", "∤", "nspar;", "∦", "nsqsube;", "⋢", "nsqsupe;", "⋣", "nsub;", "⊄", "nsubE;", "⫅̸", "nsube;", "⊈", "nsubset;", "⊂⃒", "nsubseteq;", "⊈", "nsubseteqq;", "⫅̸", "nsucc;", "⊁", "nsucceq;", "⪰̸", "nsup;", "⊅", "nsupE;", "⫆̸", "nsupe;", "⊉", "nsupset;", "⊃⃒", "nsupseteq;", "⊉", "nsupseteqq;", "⫆̸", "ntgl;", "≹", "ntilde", "ñ", "ntilde;", "ñ", "ntlg;", "≸", "ntriangleleft;", "⋪", "ntrianglelefteq;", "⋬", "ntriangleright;", "⋫", "ntrianglerighteq;", "⋭", "nu;", "ν", "num;", "#", "numero;", "№", "numsp;", " ", "nvDash;", "⊭", "nvHarr;", "⤄", "nvap;", "≍⃒", "nvdash;", "⊬", "nvge;", "≥⃒", "nvgt;", "\u003e⃒", "nvinfin;", "⧞", "nvlArr;", "⤂", "nvle;", "≤⃒", "nvlt;", "\u003c⃒", "nvltrie;", "⊴⃒", "nvrArr;", "⤃", "nvrtrie;", "⊵⃒", "nvsim;", "∼⃒", "nwArr;", "⇖", "nwarhk;", "⤣", "nwarr;", "↖", "nwarrow;", "↖", "nwnear;", "⤧", "oS;", "Ⓢ", "oacute", "ó", "oacute;", "ó", "oast;", "⊛", "ocir;", "⊚", "ocirc", "ô", "ocirc;", "ô", "ocy;", "о", "odash;", "⊝", "odblac;", "ő", "odiv;", "⨸", "odot;", "⊙", "odsold;", "⦼", "oelig;", "œ", "ofcir;", "⦿", "ofr;", "\ud835\udd2c", "ogon;", "˛", "ograve", "ò", "ograve;", "ò", "ogt;", "⧁", "ohbar;", "⦵", "ohm;", "Ω", "oint;", "∮", "olarr;", "↺", "olcir;", "⦾", "olcross;", "⦻", "oline;", "‾", "olt;", "⧀", "omacr;", "ō", "omega;", "ω", "omicron;", "ο", "omid;", "⦶", "ominus;", "⊖", "oopf;", "\ud835\udd60", "opar;", "⦷", "operp;", "⦹", "oplus;", "⊕", "or;", "∨", "orarr;", "↻", "ord;", "⩝", "order;", "ℴ", "orderof;", "ℴ", "ordf", "ª", "ordf;", "ª", "ordm", "º", "ordm;", "º", "origof;", "⊶", "oror;", "⩖", "orslope;", "⩗", "orv;", "⩛", "oscr;", "ℴ", "oslash", "ø", "oslash;", "ø", "osol;", "⊘", "otilde", "õ", "otilde;", "õ", "otimes;", "⊗", "otimesas;", "⨶", "ouml", "ö", "ouml;", "ö", "ovbar;", "⌽", "par;", "∥", "para", "¶", "para;", "¶", "parallel;", "∥", "parsim;", "⫳", "parsl;", "⫽", "part;", "∂", "pcy;", "п", "percnt;", "%", "period;", ".", "permil;", "‰", "perp;", "⊥", "pertenk;", "‱", "pfr;", "\ud835\udd2d", "phi;", "φ", "phiv;", "ϕ", "phmmat;", "ℳ", "phone;", "☎", "pi;", "π", "pitchfork;", "⋔", "piv;", "ϖ", "planck;", "ℏ", "planckh;", "ℎ", "plankv;", "ℏ", "plus;", "+", "plusacir;", "⨣", "plusb;", "⊞", "pluscir;", "⨢", "plusdo;", "∔", "plusdu;", "⨥", "pluse;", "⩲", "plusmn", "±", "plusmn;", "±", "plussim;", "⨦", "plustwo;", "⨧", "pm;", "±", "pointint;", "⨕", "popf;", "\ud835\udd61", "pound", "£", "pound;", "£", "pr;", "≺", "prE;", "⪳", "prap;", "⪷", "prcue;", "≼", "pre;", "⪯", "prec;", "≺", "precapprox;", "⪷", "preccurlyeq;", "≼", "preceq;", "⪯", "precnapprox;", "⪹", "precneqq;", "⪵", "precnsim;", "⋨", "precsim;", "≾", "prime;", "′", "primes;", "ℙ", "prnE;", "⪵", "prnap;", "⪹", "prnsim;", "⋨", "prod;", "∏", "profalar;", "⌮", "profline;", "⌒", "profsurf;", "⌓", "prop;", "∝", "propto;", "∝", "prsim;", "≾", "prurel;", "⊰", "pscr;", "\ud835\udcc5", "psi;", "ψ", "puncsp;", " ", "qfr;", "\ud835\udd2e", "qint;", "⨌", "qopf;", "\ud835\udd62", "qprime;", "⁗", "qscr;", "\ud835\udcc6", "quaternions;", "ℍ", "quatint;", "⨖", "quest;", "?", "questeq;", "≟", "quot", "\u0022", "quot;", "\u0022", "rAarr;", "⇛", "rArr;", "⇒", "rAtail;", "⤜", "rBarr;", "⤏", "rHar;", "⥤", "race;", "∽̱", "racute;", "ŕ", "radic;", "√", "raemptyv;", "⦳", "rang;", "⟩", "rangd;", "⦒", "range;", "⦥", "rangle;", "⟩", "raquo", "»", "raquo;", "»", "rarr;", "→", "rarrap;", "⥵", "rarrb;", "⇥", "rarrbfs;", "⤠", "rarrc;", "⤳", "rarrfs;", "⤞", "rarrhk;", "↪", "rarrlp;", "↬", "rarrpl;", "⥅", "rarrsim;", "⥴", "rarrtl;", "↣", "rarrw;", "↝", "ratail;", "⤚", "ratio;", "∶", "rationals;", "ℚ", "rbarr;", "⤍", "rbbrk;", "❳", "rbrace;", "}", "rbrack;", "]", "rbrke;", "⦌", "rbrksld;", "⦎", "rbrkslu;", "⦐", "rcaron;", "ř", "rcedil;", "ŗ", "rceil;", "⌉", "rcub;", "}", "rcy;", "р", "rdca;", "⤷", "rdldhar;", "⥩", "rdquo;", "”", "rdquor;", "”", "rdsh;", "↳", "real;", "ℜ", "realine;", "ℛ", "realpart;", "ℜ", "reals;", "ℝ", "rect;", "▭", "reg", "®", "reg;", "®", "rfisht;", "⥽", "rfloor;", "⌋", "rfr;", "\ud835\udd2f", "rhard;", "⇁", "rharu;", "⇀", "rharul;", "⥬", "rho;", "ρ", "rhov;", "ϱ", "rightarrow;", "→", "rightarrowtail;", "↣", "rightharpoondown;", "⇁", "rightharpoonup;", "⇀", "rightleftarrows;", "⇄", "rightleftharpoons;", "⇌", "rightrightarrows;", "⇉", "rightsquigarrow;", "↝", "rightthreetimes;", "⋌", "ring;", "˚", "risingdotseq;", "≓", "rlarr;", "⇄", "rlhar;", "⇌", "rlm;", "‏", "rmoust;", "⎱", "rmoustache;", "⎱", "rnmid;", "⫮", "roang;", "⟭", "roarr;", "⇾", "robrk;", "⟧", "ropar;", "⦆", "ropf;", "\ud835\udd63", "roplus;", "⨮", "rotimes;", "⨵", "rpar;", ")", "rpargt;", "⦔", "rppolint;", "⨒", "rrarr;", "⇉", "rsaquo;", "›", "rscr;", "\ud835\udcc7", "rsh;", "↱", "rsqb;", "]", "rsquo;", "’", "rsquor;", "’", "rthree;", "⋌", "rtimes;", "⋊", "rtri;", "▹", "rtrie;", "⊵", "rtrif;", "▸", "rtriltri;", "⧎", "ruluhar;", "⥨", "rx;", "℞", "sacute;", "ś", "sbquo;", "‚", "sc;", "≻", "scE;", "⪴", "scap;", "⪸", "scaron;", "š", "sccue;", "≽", "sce;", "⪰", "scedil;", "ş", "scirc;", "ŝ", "scnE;", "⪶", "scnap;", "⪺", "scnsim;", "⋩", "scpolint;", "⨓", "scsim;", "≿", "scy;", "с", "sdot;", "⋅", "sdotb;", "⊡", "sdote;", "⩦", "seArr;", "⇘", "searhk;", "⤥", "searr;", "↘", "searrow;", "↘", "sect", "§", "sect;", "§", "semi;", ";", "seswar;", "⤩", "setminus;", "∖", "setmn;", "∖", "sext;", "✶", "sfr;", "\ud835\udd30", "sfrown;", "⌢", "sharp;", "♯", "shchcy;", "щ", "shcy;", "ш", "shortmid;", "∣", "shortparallel;", "∥", "shy", "­", "shy;", "­", "sigma;", "σ", "sigmaf;", "ς", "sigmav;", "ς", "sim;", "∼", "simdot;", "⩪", "sime;", "≃", "simeq;", "≃", "simg;", "⪞", "simgE;", "⪠", "siml;", "⪝", "simlE;", "⪟", "simne;", "≆", "simplus;", "⨤", "simrarr;", "⥲", "slarr;", "←", "smallsetminus;", "∖", "smashp;", "⨳", "smeparsl;", "⧤", "smid;", "∣", "smile;", "⌣", "smt;", "⪪", "smte;", "⪬", "smtes;", "⪬︀", "softcy;", "ь", "sol;", "/", "solb;", "⧄", "solbar;", "⌿", "sopf;", "\ud835\udd64", "spades;", "♠", "spadesuit;", "♠", "spar;", "∥", "sqcap;", "⊓", "sqcaps;", "⊓︀", "sqcup;", "⊔", "sqcups;", "⊔︀", "sqsub;", "⊏", "sqsube;", "⊑", "sqsubset;", "⊏", "sqsubseteq;", "⊑", "sqsup;", "⊐", "sqsupe;", "⊒", "sqsupset;", "⊐", "sqsupseteq;", "⊒", "squ;", "□", "square;", "□", "squarf;", "▪", "squf;", "▪", "srarr;", "→", "sscr;", "\ud835\udcc8", "ssetmn;", "∖", "ssmile;", "⌣", "sstarf;", "⋆", "star;", "☆", "starf;", "★", "straightepsilon;", "ϵ", "straightphi;", "ϕ", "strns;", "¯", "sub;", "⊂", "subE;", "⫅", "subdot;", "⪽", "sube;", "⊆", "subedot;", "⫃", "submult;", "⫁", "subnE;", "⫋", "subne;", "⊊", "subplus;", "⪿", "subrarr;", "⥹", "subset;", "⊂", "subseteq;", "⊆", "subseteqq;", "⫅", "subsetneq;", "⊊", "subsetneqq;", "⫋", "subsim;", "⫇", "subsub;", "⫕", "subsup;", "⫓", "succ;", "≻", "succapprox;", "⪸", "succcurlyeq;", "≽", "succeq;", "⪰", "succnapprox;", "⪺", "succneqq;", "⪶", "succnsim;", "⋩", "succsim;", "≿", "sum;", "∑", "sung;", "♪", "sup1", "¹", "sup1;", "¹", "sup2", "²", "sup2;", "²", "sup3", "³", "sup3;", "³", "sup;", "⊃", "supE;", "⫆", "supdot;", "⪾", "supdsub;", "⫘", "supe;", "⊇", "supedot;", "⫄", "suphsol;", "⟉", "suphsub;", "⫗", "suplarr;", "⥻", "supmult;", "⫂", "supnE;", "⫌", "supne;", "⊋", "supplus;", "⫀", "supset;", "⊃", "supseteq;", "⊇", "supseteqq;", "⫆", "supsetneq;", "⊋", "supsetneqq;", "⫌", "supsim;", "⫈", "supsub;", "⫔", "supsup;", "⫖", "swArr;", "⇙", "swarhk;", "⤦", "swarr;", "↙", "swarrow;", "↙", "swnwar;", "⤪", "szlig", "ß", "szlig;", "ß", "target;", "⌖", "tau;", "τ", "tbrk;", "⎴", "tcaron;", "ť", "tcedil;", "ţ", "tcy;", "т", "tdot;", "⃛", "telrec;", "⌕", "tfr;", "\ud835\udd31", "there4;", "∴", "therefore;", "∴", "theta;", "θ", "thetasym;", "ϑ", "thetav;", "ϑ", "thickapprox;", "≈", "thicksim;", "∼", "thinsp;", " ", "thkap;", "≈", "thksim;", "∼", "thorn", "þ", "thorn;", "þ", "tilde;", "˜", "times", "×", "times;", "×", "timesb;", "⊠", "timesbar;", "⨱", "timesd;", "⨰", "tint;", "∭", "toea;", "⤨", "top;", "⊤", "topbot;", "⌶", "topcir;", "⫱", "topf;", "\ud835\udd65", "topfork;", "⫚", "tosa;", "⤩", "tprime;", "‴", "trade;", "™", "triangle;", "▵", "triangledown;", "▿", "triangleleft;", "◃", "trianglelefteq;", "⊴", "triangleq;", "≜", "triangleright;", "▹", "trianglerighteq;", "⊵", "tridot;", "◬", "trie;", "≜", "triminus;", "⨺", "triplus;", "⨹", "trisb;", "⧍", "tritime;", "⨻", "trpezium;", "⏢", "tscr;", "\ud835\udcc9", "tscy;", "ц", "tshcy;", "ћ", "tstrok;", "ŧ", "twixt;", "≬", "twoheadleftarrow;", "↞", "twoheadrightarrow;", "↠", "uArr;", "⇑", "uHar;", "⥣", "uacute", "ú", "uacute;", "ú", "uarr;", "↑", "ubrcy;", "ў", "ubreve;", "ŭ", "ucirc", "û", "ucirc;", "û", "ucy;", "у", "udarr;", "⇅", "udblac;", "ű", "udhar;", "⥮", "ufisht;", "⥾", "ufr;", "\ud835\udd32", "ugrave", "ù", "ugrave;", "ù", "uharl;", "↿", "uharr;", "↾", "uhblk;", "▀", "ulcorn;", "⌜", "ulcorner;", "⌜", "ulcrop;", "⌏", "ultri;", "◸", "umacr;", "ū", "uml", "¨", "uml;", "¨", "uogon;", "ų", "uopf;", "\ud835\udd66", "uparrow;", "↑", "updownarrow;", "↕", "upharpoonleft;", "↿", "upharpoonright;", "↾", "uplus;", "⊎", "upsi;", "υ", "upsih;", "ϒ", "upsilon;", "υ", "upuparrows;", "⇈", "urcorn;", "⌝", "urcorner;", "⌝", "urcrop;", "⌎", "uring;", "ů", "urtri;", "◹", "uscr;", "\ud835\udcca", "utdot;", "⋰", "utilde;", "ũ", "utri;", "▵", "utrif;", "▴", "uuarr;", "⇈", "uuml", "ü", "uuml;", "ü", "uwangle;", "⦧", "vArr;", "⇕", "vBar;", "⫨", "vBarv;", "⫩", "vDash;", "⊨", "vangrt;", "⦜", "varepsilon;", "ϵ", "varkappa;", "ϰ", "varnothing;", "∅", "varphi;", "ϕ", "varpi;", "ϖ", "varpropto;", "∝", "varr;", "↕", "varrho;", "ϱ", "varsigma;", "ς", "varsubsetneq;", "⊊︀", "varsubsetneqq;", "⫋︀", "varsupsetneq;", "⊋︀", "varsupsetneqq;", "⫌︀", "vartheta;", "ϑ", "vartriangleleft;", "⊲", "vartriangleright;", "⊳", "vcy;", "в", "vdash;", "⊢", "vee;", "∨", "veebar;", "⊻", "veeeq;", "≚", "vellip;", "⋮", "verbar;", "|", "vert;", "|", "vfr;", "\ud835\udd33", "vltri;", "⊲", "vnsub;", "⊂⃒", "vnsup;", "⊃⃒", "vopf;", "\ud835\udd67", "vprop;", "∝", "vrtri;", "⊳", "vscr;", "\ud835\udccb", "vsubnE;", "⫋︀", "vsubne;", "⊊︀", "vsupnE;", "⫌︀", "vsupne;", "⊋︀", "vzigzag;", "⦚", "wcirc;", "ŵ", "wedbar;", "⩟", "wedge;", "∧", "wedgeq;", "≙", "weierp;", "℘", "wfr;", "\ud835\udd34", "wopf;", "\ud835\udd68", "wp;", "℘", "wr;", "≀", "wreath;", "≀", "wscr;", "\ud835\udccc", "xcap;", "⋂", "xcirc;", "◯", "xcup;", "⋃", "xdtri;", "▽", "xfr;", "\ud835\udd35", "xhArr;", "⟺", "xharr;", "⟷", "xi;", "ξ", "xlArr;", "⟸", "xlarr;", "⟵", "xmap;", "⟼", "xnis;", "⋻", "xodot;", "⨀", "xopf;", "\ud835\udd69", "xoplus;", "⨁", "xotime;", "⨂", "xrArr;", "⟹", "xrarr;", "⟶", "xscr;", "\ud835\udccd", "xsqcup;", "⨆", "xuplus;", "⨄", "xutri;", "△", "xvee;", "⋁", "xwedge;", "⋀", "yacute", "ý", "yacute;", "ý", "yacy;", "я", "ycirc;", "ŷ", "ycy;", "ы", "yen", "¥", "yen;", "¥", "yfr;", "\ud835\udd36", "yicy;", "ї", "yopf;", "\ud835\udd6a", "yscr;", "\ud835\udcce", "yucy;", "ю", "yuml", "ÿ", "yuml;", "ÿ", "zacute;", "ź", "zcaron;", "ž", "zcy;", "з", "zdot;", "ż", "zeetrf;", "ℨ", "zeta;", "ζ", "zfr;", "\ud835\udd37", "zhcy;", "ж", "zigrarr;", "⇝", "zopf;", "\ud835\udd6b", "zscr;", "\ud835\udccf", "zwj;", "‍", "zwnj;", "‌"]);
/** @type {Map<string, string>} */
const mb_702 = mapBuilderConstructor_703();
/** @type {number} */
let i_704 = 0;
/** @type {number} */
let n_705 = strs_701.length;
while (i_704 < n_705) {
  mapBuilderSet_706(mb_702, listedGet_673(strs_701, i_704), listedGet_673(strs_701, i_704 + 1 | 0));
  i_704 = i_704 + 2 | 0;
}
/** @type {Map<string, string>} */
const return_707 = mappedToMap_708(mb_702);
/** @type {Map<string, string>} */
const htmlNamedCharacters_436 = return_707;
/** @type {HtmlCodec} */
const return_709 = new HtmlCodec();
/** @type {Codec_437} */
export const htmlCodec = return_709;
/**
 * @param {number} x_758
 * @returns {string}
 */
function htmlStateStr_455(x_758) {
  let return_759;
  if (x_758 === 0) {
    return_759 = "Pcdata";
  } else if (x_758 === 1) {
    return_759 = "OName";
  } else if (x_758 === 2) {
    return_759 = "CName";
  } else if (x_758 === 3) {
    return_759 = "BeforeAttr";
  } else if (x_758 === 4) {
    return_759 = "BeforeEq";
  } else if (x_758 === 5) {
    return_759 = "BeforeValue";
  } else if (x_758 === 6) {
    return_759 = "Attr";
  } else if (x_758 === 7) {
    return_759 = "AfterAttr";
  } else if (x_758 === 8) {
    return_759 = "SpecialBody";
  } else {
    return_759 = x_758.toString();
  }
  return return_759;
}
/**
 * @param {number} x_760
 * @returns {string}
 */
function tagStateStr_456(x_760) {
  return x_760.toString();
}
/**
 * @param {number} x_761
 * @returns {string}
 */
function attribStateStr_457(x_761) {
  let return_762;
  if (x_761 === 0) {
    return_762 = "Generic";
  } else if (x_761 === 1) {
    return_762 = "Css";
  } else if (x_761 === 2) {
    return_762 = "Js";
  } else if (x_761 === 3) {
    return_762 = "Url";
  } else if (x_761 === 4) {
    return_762 = "Urls";
  } else {
    return_762 = x_761.toString();
  }
  return return_762;
}
/**
 * @param {number} x_763
 * @returns {string}
 */
function delimStateStr_458(x_763) {
  let return_764;
  if (x_763 === 0) {
    return_764 = "Uq";
  } else if (x_763 === 1) {
    return_764 = "Sq";
  } else if (x_763 === 2) {
    return_764 = "Dq";
  } else {
    return_764 = x_763.toString();
  }
  return return_764;
}
/**
 * @param {number} x_765
 * @returns {string}
 */
function urlStateStr_470(x_765) {
  let return_766;
  if (x_765 === 0) {
    return_766 = "Start";
  } else if (x_765 === 1) {
    return_766 = "BeforeQuery";
  } else if (x_765 === 2) {
    return_766 = "Query";
  } else if (x_765 === 3) {
    return_766 = "Fragment";
  } else {
    return_766 = x_765.toString();
  }
  return return_766;
}
/**
 * @param {AutoescState_443<UrlEscaperContext>} before_767
 * @param {string | null} literalPart_768
 * @returns {AfterPropagate_444<UrlEscaperContext>}
 */
function urlPropagateContext_449(before_767, literalPart_768) {
  let return_769;
  let t_770;
  let t_771;
  let t_772;
  let t_773;
  let t_774;
  let t_775;
  let t_776;
  let t_777;
  let t_778;
  let t_779;
  let t_780;
  let t_781;
  let t_782;
  let t_783;
  let t_784;
  let t_785;
  let t_786;
  let t_787;
  let t_788;
  let t_789;
  let t_790;
  let t_791;
  let t_792;
  let t_793;
  let t_794;
  let t_795;
  let t_796;
  let t_797;
  let t_798;
  let t_799;
  let t_800;
  let t_801;
  let t_802;
  let t_803;
  let t_804;
  fn_805: {
    const contextBefore_806 = before_767.context;
    t_770 = new CodeSet_807(Object.freeze([new CodePoints_808("#")]), false);
    const pattern0_809 = new Sequence_810(Object.freeze([Begin_363, t_770])).compiled();
    t_771 = new CodeSet_807(Object.freeze([new CodePoints_808("?")]), false);
    const pattern1_811 = new Sequence_810(Object.freeze([Begin_363, t_771])).compiled();
    t_772 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("#")]), true), 1, null, false);
    const pattern2_813 = new Sequence_810(Object.freeze([Begin_363, t_772])).compiled();
    t_773 = new Or_814(Object.freeze([new Sequence_810(Object.freeze([new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808(":"), new CodePoints_808("#"), new CodePoints_808("?")]), true), 0, null, false), new CodePoints_808(":")])), new CodeSet_807(Object.freeze([new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" "), new CodePoints_808(":"), new CodePoints_808("#"), new CodePoints_808("?")]), true)]));
    const pattern3_815 = new Sequence_810(Object.freeze([Begin_363, t_773])).compiled();
    t_774 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("?"), new CodePoints_808("#")]), true), 1, null, false);
    const pattern4_816 = new Sequence_810(Object.freeze([Begin_363, t_774])).compiled();
    if (!(literalPart_768 == null)) {
      const literalPart_817 = literalPart_768;
      if (contextBefore_806.urlState === 0) {
        let match_818;
        try {
          t_797 = pattern3_815.find(literalPart_817);
          match_818 = t_797;
        } catch {
          match_818 = null;
        }
        if (!(match_818 == null)) {
          const match_819 = match_818;
          t_775 = match_819.full.value;
          t_776 = match_819.full.end;
          t_777 = new AutoescState_443(new UrlEscaperContext(1), before_767.subsidiary);
          return_769 = new AfterPropagate_444(t_775, t_776, t_777);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 0) {
        let match_820;
        try {
          t_798 = pattern1_811.find(literalPart_817);
          match_820 = t_798;
        } catch {
          match_820 = null;
        }
        if (!(match_820 == null)) {
          const match_821 = match_820;
          t_778 = match_821.full.value;
          t_779 = match_821.full.end;
          t_780 = new AutoescState_443(new UrlEscaperContext(2), before_767.subsidiary);
          return_769 = new AfterPropagate_444(t_778, t_779, t_780);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 0) {
        let match_822;
        try {
          t_799 = pattern0_809.find(literalPart_817);
          match_822 = t_799;
        } catch {
          match_822 = null;
        }
        if (!(match_822 == null)) {
          const match_823 = match_822;
          t_781 = match_823.full.value;
          t_782 = match_823.full.end;
          t_783 = new AutoescState_443(new UrlEscaperContext(3), before_767.subsidiary);
          return_769 = new AfterPropagate_444(t_781, t_782, t_783);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 1) {
        let match_824;
        try {
          t_800 = pattern4_816.find(literalPart_817);
          match_824 = t_800;
        } catch {
          match_824 = null;
        }
        if (!(match_824 == null)) {
          const match_825 = match_824;
          t_784 = match_825.full.value;
          t_785 = match_825.full.end;
          return_769 = new AfterPropagate_444(t_784, t_785, before_767);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 1) {
        let match_826;
        try {
          t_801 = pattern1_811.find(literalPart_817);
          match_826 = t_801;
        } catch {
          match_826 = null;
        }
        if (!(match_826 == null)) {
          const match_827 = match_826;
          t_786 = match_827.full.value;
          t_787 = match_827.full.end;
          t_788 = new AutoescState_443(new UrlEscaperContext(2), before_767.subsidiary);
          return_769 = new AfterPropagate_444(t_786, t_787, t_788);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 1) {
        let match_828;
        try {
          t_802 = pattern0_809.find(literalPart_817);
          match_828 = t_802;
        } catch {
          match_828 = null;
        }
        if (!(match_828 == null)) {
          const match_829 = match_828;
          t_789 = match_829.full.value;
          t_790 = match_829.full.end;
          t_791 = new AutoescState_443(new UrlEscaperContext(3), before_767.subsidiary);
          return_769 = new AfterPropagate_444(t_789, t_790, t_791);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 2) {
        let match_830;
        try {
          t_803 = pattern2_813.find(literalPart_817);
          match_830 = t_803;
        } catch {
          match_830 = null;
        }
        if (!(match_830 == null)) {
          const match_831 = match_830;
          t_792 = match_831.full.value;
          t_793 = match_831.full.end;
          return_769 = new AfterPropagate_444(t_792, t_793, before_767);
          break fn_805;
        }
      }
      if (contextBefore_806.urlState === 2) {
        let match_832;
        try {
          t_804 = pattern0_809.find(literalPart_817);
          match_832 = t_804;
        } catch {
          match_832 = null;
        }
        if (!(match_832 == null)) {
          const match_833 = match_832;
          t_794 = match_833.full.value;
          t_795 = match_833.full.end;
          t_796 = new AutoescState_443(new UrlEscaperContext(3), before_767.subsidiary);
          return_769 = new AfterPropagate_444(t_794, t_795, t_796);
          break fn_805;
        }
      }
    }
    if (literalPart_768 == null) {
      return_769 = new AfterPropagate_444("", 0, before_767);
      break fn_805;
    }
    return_769 = panic_605();
  }
  return return_769;
}
/** @type {UrlContextPropagator} */
const urlContextPropagator_596 = new UrlContextPropagator();
/** @type {Regex_834} */
const protocolAllowList_654 = new Sequence_810(Object.freeze([Begin_363, new Or_814(Object.freeze([new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("H"), new CodePoints_808("h")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("T"), new CodePoints_808("t")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("T"), new CodePoints_808("t")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("P"), new CodePoints_808("p")]), false), new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("S"), new CodePoints_808("s")]), false), 0, 1, false)])), new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("M"), new CodePoints_808("m")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("A"), new CodePoints_808("a")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("I"), new CodePoints_808("i")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("L"), new CodePoints_808("l")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("T"), new CodePoints_808("t")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("O"), new CodePoints_808("o")]), false)]))])), End_365])).compiled();
/** @type {SafeUrl} */
const fallbackSafeUrl_655 = new SafeUrl("about:zz_Temper_zz#");
/** @type {Array<boolean>} */
const lb_835 = [];
/** @type {number} */
let i_836 = 0;
while (i_836 < 128) {
  if (i_836 === 47) {
    t_376 = true;
  } else {
    if (i_836 === 46) {
      t_375 = true;
    } else {
      if (i_836 === 45) {
        t_374 = true;
      } else {
        if (i_836 === 95) {
          t_373 = true;
        } else {
          if (48 <= i_836) {
            t_370 = i_836 <= 57;
          } else {
            t_370 = false;
          }
          if (t_370) {
            t_372 = true;
          } else {
            if (97 <=(i_836 | 32)) {
              t_371 = (i_836 | 32) <= 122;
            } else {
              t_371 = false;
            }
            t_372 = t_371;
          }
          t_373 = t_372;
        }
        t_374 = t_373;
      }
      t_375 = t_374;
    }
    t_376 = t_375;
  }
  listBuilderAdd_837(lb_835, t_376);
  i_836 = i_836 + 1 | 0;
}
/** @type {Array<boolean>} */
const urlQuerySafe_690 = listBuilderToList_838(lb_835);
/** @type {Array<boolean>} */
const lb_839 = [];
/** @type {number} */
let i_840 = 0;
while (i_840 < 128) {
  if (listedGet_673(urlQuerySafe_690, i_840)) {
    t_380 = true;
  } else {
    if (i_840 === 58) {
      t_379 = true;
    } else {
      if (i_840 === 63) {
        t_378 = true;
      } else {
        if (i_840 === 35) {
          t_377 = true;
        } else {
          t_377 = i_840 === 38;
        }
        t_378 = t_377;
      }
      t_379 = t_378;
    }
    t_380 = t_379;
  }
  listBuilderAdd_837(lb_839, t_380);
  i_840 = i_840 + 1 | 0;
}
/** @type {Array<boolean>} */
const urlSafe_672 = listBuilderToList_838(lb_839);
/** @type {HtmlUrlPartUrlEscaper} */
const return_841 = new HtmlUrlPartUrlEscaper();
/** @type {HtmlUrlPartUrlEscaper} */
const htmlUrlPartUrlEscaper_603 = return_841;
/** @type {HtmlProtocolFilteringUrlEscaper} */
const return_842 = new HtmlProtocolFilteringUrlEscaper();
/** @type {HtmlProtocolFilteringUrlEscaper} */
const htmlProtocolFilteringUrlEscaper_602 = return_842;
/** @type {HtmlAsIfQueryUrlEscaper} */
const return_843 = new HtmlAsIfQueryUrlEscaper();
/** @type {HtmlAsIfQueryUrlEscaper} */
const htmlAsIfQueryUrlEscaper_604 = return_843;
/**
 * @param {AutoescState_443<HtmlEscaperContext>} before_844
 * @param {string | null} literalPart_845
 * @returns {AfterPropagate_444<HtmlEscaperContext>}
 */
function htmlPropagateContext_442(before_844, literalPart_845) {
  let return_846;
  let t_847;
  let t_848;
  let t_849;
  let t_850;
  let t_851;
  let t_852;
  let t_853;
  let t_854;
  let t_855;
  let t_856;
  let t_857;
  let t_858;
  let t_859;
  let t_860;
  let t_861;
  let t_862;
  let t_863;
  let t_864;
  let t_865;
  let t_866;
  let t_867;
  let t_868;
  let t_869;
  let t_870;
  let t_871;
  let t_872;
  let t_873;
  let t_874;
  let t_875;
  let t_876;
  let t_877;
  let t_878;
  let t_879;
  let t_880;
  let t_881;
  let t_882;
  let t_883;
  let t_884;
  let t_885;
  let t_886;
  let t_887;
  let t_888;
  let t_889;
  let t_890;
  let t_891;
  let t_892;
  let t_893;
  let t_894;
  let t_895;
  let t_896;
  let t_897;
  let t_898;
  let t_899;
  let t_900;
  let t_901;
  let t_902;
  let t_903;
  let t_904;
  let t_905;
  let t_906;
  let t_907;
  let t_908;
  let t_909;
  let t_910;
  let t_911;
  let t_912;
  let t_913;
  let t_914;
  let t_915;
  let t_916;
  let t_917;
  let t_918;
  let t_919;
  let t_920;
  let t_921;
  let t_922;
  let t_923;
  let t_924;
  let t_925;
  let t_926;
  let t_927;
  let t_928;
  let t_929;
  let t_930;
  let t_931;
  let t_932;
  let t_933;
  let t_934;
  let t_935;
  let t_936;
  let t_937;
  let t_938;
  let t_939;
  let t_940;
  let t_941;
  let t_942;
  let t_943;
  let t_944;
  let t_945;
  let t_946;
  let t_947;
  let t_948;
  let t_949;
  let t_950;
  let t_951;
  let t_952;
  let t_953;
  let t_954;
  let t_955;
  let t_956;
  let t_957;
  let t_958;
  let t_959;
  let t_960;
  let t_961;
  let t_962;
  let t_963;
  let t_964;
  let t_965;
  let t_966;
  let t_967;
  let t_968;
  let t_969;
  let t_970;
  let t_971;
  let t_972;
  let t_973;
  let t_974;
  let t_975;
  let t_976;
  let t_977;
  let t_978;
  let t_979;
  let t_980;
  let t_981;
  let t_982;
  let t_983;
  let t_984;
  let t_985;
  let t_986;
  let t_987;
  let t_988;
  let t_989;
  let t_990;
  let t_991;
  let t_992;
  let t_993;
  let t_994;
  let t_995;
  let t_996;
  let t_997;
  let t_998;
  let t_999;
  let t_1000;
  let t_1001;
  let t_1002;
  let t_1003;
  let t_1004;
  let t_1005;
  let t_1006;
  let t_1007;
  let t_1008;
  let t_1009;
  let t_1010;
  let t_1011;
  let t_1012;
  let t_1013;
  let t_1014;
  let t_1015;
  let t_1016;
  let t_1017;
  let t_1018;
  let t_1019;
  let t_1020;
  let t_1021;
  let t_1022;
  let t_1023;
  let t_1024;
  fn_1025: {
    const contextBefore_1026 = before_844.context;
    t_847 = new CodePoints_808("\u0022");
    const pattern0_1027 = new Sequence_810(Object.freeze([Begin_363, t_847])).compiled();
    t_848 = new Sequence_810(Object.freeze([new CodePoints_808("\u0022"), new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("\u0022")]), true), 0, null, false), new Repeat_812(new CodePoints_808("\u0022"), 0, 1, false)]));
    const pattern1_1028 = new Sequence_810(Object.freeze([Begin_363, t_848])).compiled();
    t_849 = new CodePoints_808("'");
    const pattern2_1029 = new Sequence_810(Object.freeze([Begin_363, t_849])).compiled();
    t_850 = new Sequence_810(Object.freeze([new CodePoints_808("'"), new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("'")]), true), 0, null, false), new Repeat_812(new CodePoints_808("'"), 0, 1, false)]));
    const pattern3_1030 = new Sequence_810(Object.freeze([Begin_363, t_850])).compiled();
    t_851 = new CodePoints_808("\u003e");
    const pattern4_1031 = new Sequence_810(Object.freeze([Begin_363, t_851])).compiled();
    t_852 = new CodeSet_807(Object.freeze([new CodePoints_808("\u003e"), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" ")]), false);
    const pattern5_1032 = new Sequence_810(Object.freeze([Begin_363, t_852])).compiled();
    t_853 = new CodeSet_807(Object.freeze([new CodeRange_1033(65, 90), new CodeRange_1033(97, 122), new CodeRange_1033(48, 57), new CodeRange_1033(58, 58), new CodePoints_808("-")]), false);
    const pattern6_1034 = new Sequence_810(Object.freeze([Begin_363, t_853])).compiled();
    t_854 = new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("S"), new CodePoints_808("s")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("R"), new CodePoints_808("r")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("C"), new CodePoints_808("c")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("S"), new CodePoints_808("s")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("E"), new CodePoints_808("e")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("T"), new CodePoints_808("t")]), false)]));
    const pattern7_1035 = new Sequence_810(Object.freeze([Begin_363, t_854])).compiled();
    t_855 = new Or_814(Object.freeze([new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("S"), new CodePoints_808("s")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("R"), new CodePoints_808("r")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("C"), new CodePoints_808("c")]), false)])), new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("H"), new CodePoints_808("h")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("R"), new CodePoints_808("r")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("E"), new CodePoints_808("e")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("F"), new CodePoints_808("f")]), false)]))]));
    const pattern8_1036 = new Sequence_810(Object.freeze([Begin_363, t_855])).compiled();
    t_856 = new Sequence_810(Object.freeze([new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" ")]), false), 0, null, false), new Repeat_812(new CodePoints_808("/"), 0, 1, false), new CodePoints_808("\u003e")]));
    const pattern9_1037 = new Sequence_810(Object.freeze([Begin_363, t_856])).compiled();
    t_857 = new CodeSet_807(Object.freeze([new CodePoints_808("\u003e"), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" ")]), true);
    const pattern10_1038 = new Sequence_810(Object.freeze([Begin_363, t_857])).compiled();
    t_858 = new CodeSet_807(Object.freeze([new CodeRange_1033(97, 122), new CodeRange_1033(65, 90)]), false);
    const pattern11_1039 = new Sequence_810(Object.freeze([Begin_363, t_858])).compiled();
    t_859 = new CodePoints_808(",");
    const pattern12_1040 = new Sequence_810(Object.freeze([Begin_363, t_859])).compiled();
    t_860 = new CodePoints_808("\u003c");
    const pattern13_1041 = new Sequence_810(Object.freeze([Begin_363, t_860])).compiled();
    t_861 = new CodePoints_808("\u003c/");
    const pattern14_1042 = new Sequence_810(Object.freeze([Begin_363, t_861])).compiled();
    t_862 = new CodePoints_808("=");
    const pattern15_1043 = new Sequence_810(Object.freeze([Begin_363, t_862])).compiled();
    t_863 = new CodePoints_808("\u003e");
    const pattern16_1044 = new Sequence_810(Object.freeze([Begin_363, t_863])).compiled();
    t_864 = new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("D"), new CodePoints_808("d")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("A"), new CodePoints_808("a")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("T"), new CodePoints_808("t")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("A"), new CodePoints_808("a")]), false), new CodePoints_808("-"), new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("="), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" "), new CodePoints_808("\u003e")]), true), 0, null, false), new CodeSet_807(Object.freeze([new CodePoints_808("U"), new CodePoints_808("u")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("R"), new CodePoints_808("r")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("L"), new CodePoints_808("l"), new CodePoints_808("I"), new CodePoints_808("i")]), false), new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("="), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" "), new CodePoints_808("\u003e")]), true), 0, null, false)]));
    const pattern17_1045 = new Sequence_810(Object.freeze([Begin_363, t_864])).compiled();
    t_865 = new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("O"), new CodePoints_808("o")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("N"), new CodePoints_808("n")]), false), new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("="), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" "), new CodePoints_808("\u003e")]), true), 0, null, false)]));
    const pattern18_1046 = new Sequence_810(Object.freeze([Begin_363, t_865])).compiled();
    t_866 = new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodePoints_808("S"), new CodePoints_808("s")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("T"), new CodePoints_808("t")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("Y"), new CodePoints_808("y")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("L"), new CodePoints_808("l")]), false), new CodeSet_807(Object.freeze([new CodePoints_808("E"), new CodePoints_808("e")]), false)]));
    const pattern19_1047 = new Sequence_810(Object.freeze([Begin_363, t_866])).compiled();
    t_867 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" ")]), false), 1, null, false);
    const pattern20_1048 = new Sequence_810(Object.freeze([Begin_363, t_867])).compiled();
    t_868 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("\u0022")]), true), 1, null, false);
    const pattern21_1049 = new Sequence_810(Object.freeze([Begin_363, t_868])).compiled();
    t_869 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("'")]), true), 1, null, false);
    const pattern22_1050 = new Sequence_810(Object.freeze([Begin_363, t_869])).compiled();
    t_870 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("\u003c"), new CodePoints_808("\u003e")]), true), 1, null, false);
    const pattern23_1051 = new Sequence_810(Object.freeze([Begin_363, t_870])).compiled();
    t_871 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("="), new CodePoints_808("\u003e"), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" ")]), true), 1, null, false);
    const pattern24_1052 = new Sequence_810(Object.freeze([Begin_363, t_871])).compiled();
    t_872 = new Repeat_812(new CodeSet_807(Object.freeze([new CodePoints_808("\u003e"), new CodePoints_808("\t"), new CodePoints_808("\r"), new CodePoints_808("\n"), new CodePoints_808(" "), new CodePoints_808("\u0022")]), true), 1, null, false);
    const pattern25_1053 = new Sequence_810(Object.freeze([Begin_363, t_872])).compiled();
    t_873 = new CodeSet_807(Object.freeze([new CodePoints_808("\u003e")]), true);
    const pattern26_1054 = new Sequence_810(Object.freeze([Begin_363, t_873])).compiled();
    t_874 = new Sequence_810(Object.freeze([new Repeat_812(new CodeSet_807(Object.freeze([new CodeRange_1033(97, 122), new CodeRange_1033(65, 90), new CodeRange_1033(48, 57), new CodePoints_808("-")]), false), 1, null, false), new CodePoints_808(":")]));
    const pattern27_1055 = new Sequence_810(Object.freeze([Begin_363, t_874])).compiled();
    t_875 = new Sequence_810(Object.freeze([new CodeSet_807(Object.freeze([new CodeRange_1033(97, 122), new CodeRange_1033(65, 90)]), false), new Repeat_812(new CodeSet_807(Object.freeze([new CodeRange_1033(97, 122), new CodeRange_1033(65, 90), new CodeRange_1033(48, 57), new CodeRange_1033(58, 58), new CodePoints_808("-")]), false), 0, null, false)]));
    const pattern28_1056 = new Sequence_810(Object.freeze([Begin_363, t_875])).compiled();
    if (!(literalPart_845 == null)) {
      const literalPart_1057 = literalPart_845;
      if (contextBefore_1026.htmlState === 0) {
        let match_1058;
        try {
          t_963 = pattern14_1042.find(literalPart_1057);
          match_1058 = t_963;
        } catch {
          match_1058 = null;
        }
        if (!(match_1058 == null)) {
          const match_1059 = match_1058;
          try {
            t_964 = pattern11_1039.find(literalPart_1057.substring(match_1059.full.end, literalPart_1057.length));
            t_965 = t_964;
          } catch {
            t_965 = null;
          }
          if (!(t_965 == null)) {
            t_876 = match_1059.full.value;
            t_877 = match_1059.full.end;
            t_878 = new AutoescState_443(new HtmlEscaperContext(2, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
            return_846 = new AfterPropagate_444(t_876, t_877, t_878);
            break fn_1025;
          }
        }
      }
      if (contextBefore_1026.htmlState === 0) {
        let match_1060;
        try {
          t_966 = pattern13_1041.find(literalPart_1057);
          match_1060 = t_966;
        } catch {
          match_1060 = null;
        }
        if (!(match_1060 == null)) {
          const match_1061 = match_1060;
          try {
            t_967 = pattern11_1039.find(literalPart_1057.substring(match_1061.full.end, literalPart_1057.length));
            t_968 = t_967;
          } catch {
            t_968 = null;
          }
          if (!(t_968 == null)) {
            t_879 = match_1061.full.value;
            t_880 = match_1061.full.end;
            t_881 = new AutoescState_443(new HtmlEscaperContext(1, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
            return_846 = new AfterPropagate_444(t_879, t_880, t_881);
            break fn_1025;
          }
        }
      }
      if (contextBefore_1026.htmlState === 0) {
        let match_1062;
        try {
          t_969 = pattern13_1041.find(literalPart_1057);
          match_1062 = t_969;
        } catch {
          match_1062 = null;
        }
        if (!(match_1062 == null)) {
          t_882 = match_1062.full.end;
          return_846 = new AfterPropagate_444("\u0026lt;", t_882, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 0) {
        let match_1063;
        try {
          t_970 = pattern16_1044.find(literalPart_1057);
          match_1063 = t_970;
        } catch {
          match_1063 = null;
        }
        if (!(match_1063 == null)) {
          t_883 = match_1063.full.end;
          return_846 = new AfterPropagate_444("\u0026gt;", t_883, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 0) {
        let match_1064;
        try {
          t_971 = pattern23_1051.find(literalPart_1057);
          match_1064 = t_971;
        } catch {
          match_1064 = null;
        }
        if (!(match_1064 == null)) {
          const match_1065 = match_1064;
          t_884 = match_1065.full.value;
          t_885 = match_1065.full.end;
          return_846 = new AfterPropagate_444(t_884, t_885, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 2) {
        let match_1066;
        try {
          t_972 = pattern1_1028.find(literalPart_1057);
          match_1066 = t_972;
        } catch {
          match_1066 = null;
        }
        if (!(match_1066 == null)) {
          const match_1067 = match_1066;
          t_886 = match_1067.full.value;
          t_887 = match_1067.full.end;
          return_846 = new AfterPropagate_444(t_886, t_887, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 2) {
        let match_1068;
        try {
          t_973 = pattern3_1030.find(literalPart_1057);
          match_1068 = t_973;
        } catch {
          match_1068 = null;
        }
        if (!(match_1068 == null)) {
          const match_1069 = match_1068;
          t_888 = match_1069.full.value;
          t_889 = match_1069.full.end;
          return_846 = new AfterPropagate_444(t_888, t_889, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 2) {
        let match_1070;
        try {
          t_974 = pattern26_1054.find(literalPart_1057);
          match_1070 = t_974;
        } catch {
          match_1070 = null;
        }
        if (!(match_1070 == null)) {
          const match_1071 = match_1070;
          t_890 = match_1071.full.value;
          t_891 = match_1071.full.end;
          return_846 = new AfterPropagate_444(t_890, t_891, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 2) {
        let match_1072;
        try {
          t_975 = pattern16_1044.find(literalPart_1057);
          match_1072 = t_975;
        } catch {
          match_1072 = null;
        }
        if (!(match_1072 == null)) {
          const match_1073 = match_1072;
          t_892 = match_1073.full.value;
          t_893 = match_1073.full.end;
          t_894 = new AutoescState_443(new HtmlEscaperContext(0, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_892, t_893, t_894);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 1) {
        let match_1074;
        try {
          t_976 = pattern28_1056.find(literalPart_1057);
          match_1074 = t_976;
        } catch {
          match_1074 = null;
        }
        if (!(match_1074 == null)) {
          const match_1075 = match_1074;
          t_895 = match_1075.full.value;
          t_896 = match_1075.full.end;
          return_846 = new AfterPropagate_444(t_895, t_896, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 1) {
        try {
          t_977 = pattern4_1031.find(literalPart_1057);
          t_978 = t_977;
        } catch {
          t_978 = null;
        }
        if (!(t_978 == null)) {
          t_897 = new AutoescState_443(new HtmlEscaperContext(3, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444("", 0, t_897);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 1) {
        let match_1076;
        try {
          t_979 = pattern20_1048.find(literalPart_1057);
          match_1076 = t_979;
        } catch {
          match_1076 = null;
        }
        if (!(match_1076 == null)) {
          const match_1077 = match_1076;
          t_898 = match_1077.full.value;
          t_899 = match_1077.full.end;
          t_900 = new AutoescState_443(new HtmlEscaperContext(3, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_898, t_899, t_900);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1078;
        try {
          t_980 = pattern20_1048.find(literalPart_1057);
          match_1078 = t_980;
        } catch {
          match_1078 = null;
        }
        if (!(match_1078 == null)) {
          const match_1079 = match_1078;
          t_901 = match_1079.full.value;
          t_902 = match_1079.full.end;
          return_846 = new AfterPropagate_444(t_901, t_902, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1080;
        try {
          t_981 = pattern27_1055.find(literalPart_1057);
          match_1080 = t_981;
        } catch {
          match_1080 = null;
        }
        if (!(match_1080 == null)) {
          const match_1081 = match_1080;
          t_903 = match_1081.full.value;
          t_904 = match_1081.full.end;
          return_846 = new AfterPropagate_444(t_903, t_904, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1082;
        try {
          t_982 = pattern7_1035.find(literalPart_1057);
          match_1082 = t_982;
        } catch {
          match_1082 = null;
        }
        if (!(match_1082 == null)) {
          const match_1083 = match_1082;
          try {
            t_983 = pattern6_1034.find(literalPart_1057.substring(match_1083.full.end, literalPart_1057.length));
            t_984 = t_983;
          } catch {
            t_984 = null;
          }
          if (t_984 == null) {
            t_905 = new AfterPropagate_444(match_1083.full.value, match_1083.full.end, new AutoescState_443(new HtmlEscaperContext(4, contextBefore_1026.tagState, 4, contextBefore_1026.delimState), before_844.subsidiary));
            t_906 = new HtmlUrlDelegate();
            return_846 = t_905.push(t_906, htmlCodec);
            break fn_1025;
          }
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1084;
        try {
          t_985 = pattern8_1036.find(literalPart_1057);
          match_1084 = t_985;
        } catch {
          match_1084 = null;
        }
        if (!(match_1084 == null)) {
          const match_1085 = match_1084;
          try {
            t_986 = pattern6_1034.find(literalPart_1057.substring(match_1085.full.end, literalPart_1057.length));
            t_987 = t_986;
          } catch {
            t_987 = null;
          }
          if (t_987 == null) {
            t_907 = new AfterPropagate_444(match_1085.full.value, match_1085.full.end, new AutoescState_443(new HtmlEscaperContext(4, contextBefore_1026.tagState, 3, contextBefore_1026.delimState), before_844.subsidiary));
            t_908 = new HtmlUrlDelegate();
            return_846 = t_907.push(t_908, htmlCodec);
            break fn_1025;
          }
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1086;
        try {
          t_988 = pattern17_1045.find(literalPart_1057);
          match_1086 = t_988;
        } catch {
          match_1086 = null;
        }
        if (!(match_1086 == null)) {
          const match_1087 = match_1086;
          t_909 = new AfterPropagate_444(match_1087.full.value, match_1087.full.end, new AutoescState_443(new HtmlEscaperContext(4, contextBefore_1026.tagState, 3, contextBefore_1026.delimState), before_844.subsidiary));
          t_910 = new HtmlUrlDelegate();
          return_846 = t_909.push(t_910, htmlCodec);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1088;
        try {
          t_989 = pattern19_1047.find(literalPart_1057);
          match_1088 = t_989;
        } catch {
          match_1088 = null;
        }
        if (!(match_1088 == null)) {
          const match_1089 = match_1088;
          try {
            t_990 = pattern6_1034.find(literalPart_1057.substring(match_1089.full.end, literalPart_1057.length));
            t_991 = t_990;
          } catch {
            t_991 = null;
          }
          if (t_991 == null) {
            t_911 = new AfterPropagate_444(match_1089.full.value, match_1089.full.end, new AutoescState_443(new HtmlEscaperContext(4, contextBefore_1026.tagState, 1, contextBefore_1026.delimState), before_844.subsidiary));
            t_912 = new HtmlCssDelegate();
            return_846 = t_911.push(t_912, htmlCodec);
            break fn_1025;
          }
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1090;
        try {
          t_992 = pattern18_1046.find(literalPart_1057);
          match_1090 = t_992;
        } catch {
          match_1090 = null;
        }
        if (!(match_1090 == null)) {
          const match_1091 = match_1090;
          t_913 = new AfterPropagate_444(match_1091.full.value, match_1091.full.end, new AutoescState_443(new HtmlEscaperContext(4, contextBefore_1026.tagState, 2, contextBefore_1026.delimState), before_844.subsidiary));
          t_914 = new HtmlJsDelegate();
          return_846 = t_913.push(t_914, htmlCodec);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 3) {
        let match_1092;
        try {
          t_993 = pattern24_1052.find(literalPart_1057);
          match_1092 = t_993;
        } catch {
          match_1092 = null;
        }
        if (!(match_1092 == null)) {
          const match_1093 = match_1092;
          t_915 = match_1093.full.value;
          t_916 = match_1093.full.end;
          t_917 = new AutoescState_443(new HtmlEscaperContext(4, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_915, t_916, t_917);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 4) {
        let match_1094;
        try {
          t_994 = pattern20_1048.find(literalPart_1057);
          match_1094 = t_994;
        } catch {
          match_1094 = null;
        }
        if (!(match_1094 == null)) {
          const match_1095 = match_1094;
          t_918 = match_1095.full.value;
          t_919 = match_1095.full.end;
          return_846 = new AfterPropagate_444(t_918, t_919, before_844);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 4) {
        let match_1096;
        try {
          t_995 = pattern15_1043.find(literalPart_1057);
          match_1096 = t_995;
        } catch {
          match_1096 = null;
        }
        if (!(match_1096 == null)) {
          const match_1097 = match_1096;
          t_920 = match_1097.full.value;
          t_921 = match_1097.full.end;
          t_922 = new AutoescState_443(new HtmlEscaperContext(5, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_920, t_921, t_922);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 4) {
        try {
          t_996 = pattern9_1037.find(literalPart_1057);
          t_997 = t_996;
        } catch {
          t_997 = null;
        }
        if (!(t_997 == null)) {
          t_923 = new AutoescState_443(new HtmlEscaperContext(7, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444("", 0, t_923);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 5) {
        let match_1098;
        try {
          t_998 = pattern0_1027.find(literalPart_1057);
          match_1098 = t_998;
        } catch {
          match_1098 = null;
        }
        if (!(match_1098 == null)) {
          const match_1099 = match_1098;
          t_924 = match_1099.full.value;
          t_925 = match_1099.full.end;
          t_926 = new AutoescState_443(new HtmlEscaperContext(6, contextBefore_1026.tagState, contextBefore_1026.attribState, 2), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_924, t_925, t_926);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 5) {
        let match_1100;
        try {
          t_999 = pattern2_1029.find(literalPart_1057);
          match_1100 = t_999;
        } catch {
          match_1100 = null;
        }
        if (!(match_1100 == null)) {
          const match_1101 = match_1100;
          t_927 = match_1101.full.value;
          t_928 = match_1101.full.end;
          t_929 = new AutoescState_443(new HtmlEscaperContext(6, contextBefore_1026.tagState, contextBefore_1026.attribState, 1), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_927, t_928, t_929);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 5) {
        try {
          t_1000 = pattern10_1038.find(literalPart_1057);
          t_1001 = t_1000;
        } catch {
          t_1001 = null;
        }
        if (!(t_1001 == null)) {
          t_930 = new AutoescState_443(new HtmlEscaperContext(6, contextBefore_1026.tagState, contextBefore_1026.attribState, 0), before_844.subsidiary);
          return_846 = new AfterPropagate_444("\u0022", 0, t_930);
          break fn_1025;
        }
      }
    }
    if (literalPart_845 == null) {
      t_931 = contextBefore_1026.htmlState;
      t_1002 = t_931 === 5;
    } else {
      t_1002 = false;
    }
    if (t_1002) {
      t_932 = new AutoescState_443(new HtmlEscaperContext(6, contextBefore_1026.tagState, contextBefore_1026.attribState, 0), before_844.subsidiary);
      return_846 = new AfterPropagate_444("\u0022", 0, t_932);
      break fn_1025;
    }
    if (!(literalPart_845 == null)) {
      const literalPart_1102 = literalPart_845;
      if (contextBefore_1026.htmlState === 6) {
        t_933 = contextBefore_1026.delimState;
        t_1003 = t_933 === 0;
      } else {
        t_1003 = false;
      }
      if (t_1003) {
        try {
          t_1004 = pattern5_1032.find(literalPart_1102);
          t_1005 = t_1004;
        } catch {
          t_1005 = null;
        }
        if (!(t_1005 == null)) {
          t_934 = new AutoescState_443(new HtmlEscaperContext(7, contextBefore_1026.tagState, contextBefore_1026.attribState, 0), before_844.subsidiary);
          return_846 = new AfterPropagate_444("\u0022", 0, t_934);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_935 = contextBefore_1026.delimState;
        t_1006 = t_935 === 2;
      } else {
        t_1006 = false;
      }
      if (t_1006) {
        let match_1103;
        try {
          t_1007 = pattern0_1027.find(literalPart_1102);
          match_1103 = t_1007;
        } catch {
          match_1103 = null;
        }
        if (!(match_1103 == null)) {
          const match_1104 = match_1103;
          t_936 = match_1104.full.value;
          t_937 = match_1104.full.end;
          t_938 = new AutoescState_443(new HtmlEscaperContext(7, contextBefore_1026.tagState, contextBefore_1026.attribState, 0), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_936, t_937, t_938);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_939 = contextBefore_1026.delimState;
        t_1008 = t_939 === 1;
      } else {
        t_1008 = false;
      }
      if (t_1008) {
        let match_1105;
        try {
          t_1009 = pattern2_1029.find(literalPart_1102);
          match_1105 = t_1009;
        } catch {
          match_1105 = null;
        }
        if (!(match_1105 == null)) {
          const match_1106 = match_1105;
          t_940 = match_1106.full.value;
          t_941 = match_1106.full.end;
          t_942 = new AutoescState_443(new HtmlEscaperContext(7, contextBefore_1026.tagState, contextBefore_1026.attribState, 0), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_940, t_941, t_942);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_943 = contextBefore_1026.attribState;
        t_1010 = t_943 === 4;
      } else {
        t_1010 = false;
      }
      if (t_1010) {
        let match_1107;
        try {
          t_1011 = pattern12_1040.find(literalPart_1102);
          match_1107 = t_1011;
        } catch {
          match_1107 = null;
        }
        if (!(match_1107 == null)) {
          const match_1108 = match_1107;
          t_944 = new AfterPropagate_444(match_1108.full.value, match_1108.full.end, before_844).pop();
          t_945 = new HtmlUrlDelegate();
          return_846 = t_944.push(t_945, htmlCodec);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_946 = contextBefore_1026.delimState;
        t_1012 = t_946 === 0;
      } else {
        t_1012 = false;
      }
      if (t_1012) {
        let match_1109;
        try {
          t_1013 = pattern25_1053.find(literalPart_1102);
          match_1109 = t_1013;
        } catch {
          match_1109 = null;
        }
        if (!(match_1109 == null)) {
          const match_1110 = match_1109;
          t_947 = new AfterPropagate_444(match_1110.full.value, match_1110.full.end, before_844);
          return_846 = t_947.feed(false);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_948 = contextBefore_1026.delimState;
        t_1014 = t_948 === 0;
      } else {
        t_1014 = false;
      }
      if (t_1014) {
        let match_1111;
        try {
          t_1015 = pattern0_1027.find(literalPart_1102);
          match_1111 = t_1015;
        } catch {
          match_1111 = null;
        }
        if (!(match_1111 == null)) {
          t_949 = new AfterPropagate_444("\u0026#34;", match_1111.full.end, before_844);
          return_846 = t_949.feed(false);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_950 = contextBefore_1026.delimState;
        t_1016 = t_950 === 2;
      } else {
        t_1016 = false;
      }
      if (t_1016) {
        let match_1112;
        try {
          t_1017 = pattern21_1049.find(literalPart_1102);
          match_1112 = t_1017;
        } catch {
          match_1112 = null;
        }
        if (!(match_1112 == null)) {
          const match_1113 = match_1112;
          t_951 = new AfterPropagate_444(match_1113.full.value, match_1113.full.end, before_844);
          return_846 = t_951.feed(false);
          break fn_1025;
        }
      }
      if (contextBefore_1026.htmlState === 6) {
        t_952 = contextBefore_1026.delimState;
        t_1018 = t_952 === 1;
      } else {
        t_1018 = false;
      }
      if (t_1018) {
        let match_1114;
        try {
          t_1019 = pattern22_1050.find(literalPart_1102);
          match_1114 = t_1019;
        } catch {
          match_1114 = null;
        }
        if (!(match_1114 == null)) {
          const match_1115 = match_1114;
          t_953 = new AfterPropagate_444(match_1115.full.value, match_1115.full.end, before_844);
          return_846 = t_953.feed(false);
          break fn_1025;
        }
      }
    }
    if (literalPart_845 == null) {
      if (contextBefore_1026.htmlState === 6) {
        t_954 = contextBefore_1026.attribState;
        t_1020 = t_954 === 0;
      } else {
        t_1020 = false;
      }
      t_1021 = t_1020;
    } else {
      t_1021 = false;
    }
    if (t_1021) {
      return_846 = new AfterPropagate_444("", 0, before_844);
      break fn_1025;
    }
    if (literalPart_845 == null) {
      t_955 = contextBefore_1026.htmlState;
      t_1022 = t_955 === 6;
    } else {
      t_1022 = false;
    }
    if (t_1022) {
      t_956 = new AfterPropagate_444("", 0, before_844);
      return_846 = t_956.feed(true);
      break fn_1025;
    }
    if (contextBefore_1026.htmlState === 7) {
      t_957 = contextBefore_1026.attribState;
      t_1023 = t_957 === 0;
    } else {
      t_1023 = false;
    }
    if (t_1023) {
      t_958 = new AutoescState_443(new HtmlEscaperContext(3, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
      return_846 = new AfterPropagate_444("", 0, t_958);
      break fn_1025;
    }
    if (contextBefore_1026.htmlState === 7) {
      t_959 = new AfterPropagate_444("", 0, new AutoescState_443(new HtmlEscaperContext(3, contextBefore_1026.tagState, 0, contextBefore_1026.delimState), before_844.subsidiary));
      return_846 = t_959.pop();
      break fn_1025;
    }
    if (!(literalPart_845 == null)) {
      const literalPart_1116 = literalPart_845;
      if (contextBefore_1026.htmlState === 3) {
        let match_1117;
        try {
          t_1024 = pattern16_1044.find(literalPart_1116);
          match_1117 = t_1024;
        } catch {
          match_1117 = null;
        }
        if (!(match_1117 == null)) {
          const match_1118 = match_1117;
          t_960 = match_1118.full.value;
          t_961 = match_1118.full.end;
          t_962 = new AutoescState_443(new HtmlEscaperContext(0, contextBefore_1026.tagState, contextBefore_1026.attribState, contextBefore_1026.delimState), before_844.subsidiary);
          return_846 = new AfterPropagate_444(t_960, t_961, t_962);
          break fn_1025;
        }
      }
    }
    if (literalPart_845 == null) {
      return_846 = new AfterPropagate_444("", 0, before_844);
      break fn_1025;
    }
    return_846 = panic_605();
  }
  return return_846;
}
/** @type {HtmlPcdataEscaper} */
const return_1119 = new HtmlPcdataEscaper();
/** @type {HtmlPcdataEscaper} */
const htmlPcdataEscaper_1120 = return_1119;
/** @type {OutputHtmlSpaceEscaper} */
const return_1121 = new OutputHtmlSpaceEscaper();
/** @type {OutputHtmlSpaceEscaper} */
const outputHtmlSpaceEscaper_1122 = return_1121;
/** @type {HtmlAttributeEscaper} */
const return_1123 = new HtmlAttributeEscaper();
/** @type {HtmlAttributeEscaper} */
const htmlAttributeEscaper_1124 = return_1123;
/**
 * @param {AutoescState_443<HtmlEscaperContext>} stateBefore_1125
 * @returns {HtmlEscaper}
 */
export function pickHtmlEscaper(stateBefore_1125) {
  let return_1126;
  let t_1127;
  let t_1128;
  let t_1129;
  let t_1130;
  let t_1131;
  let escaper_1132;
  let t_1133 = stateBefore_1125.context.htmlState;
  if (t_1133 === 0) {
    escaper_1132 = htmlPcdataEscaper_1120;
  } else {
    if (t_1133 === 1) {
      t_1130 = true;
    } else {
      if (t_1133 === 2) {
        t_1129 = true;
      } else {
        if (t_1133 === 3) {
          t_1128 = true;
        } else {
          if (t_1133 === 4) {
            t_1127 = true;
          } else {
            t_1127 = t_1133 === 7;
          }
          t_1128 = t_1127;
        }
        t_1129 = t_1128;
      }
      t_1130 = t_1129;
    }
    if (t_1130) {
      escaper_1132 = outputHtmlSpaceEscaper_1122;
    } else if (t_1133 === 5) {
      escaper_1132 = panic_605();
    } else if (t_1133 === 6) {
      escaper_1132 = htmlAttributeEscaper_1124;
    } else if (t_1133 === 8) {
      escaper_1132 = outputHtmlSpaceEscaper_1122;
    } else {
      escaper_1132 = panic_605();
    }
  }
  const subsidiary_1134 = stateBefore_1125.subsidiary;
  if (!(subsidiary_1134 == null)) {
    const subsidiary_1135 = subsidiary_1134;
    let delegate_1136;
    try {
      t_1131 = requireInstanceOf__1137(subsidiary_1135.delegate, HtmlDelegate);
      delegate_1136 = t_1131;
    } catch {
      delegate_1136 = panic_605();
    }
    return_1126 = delegate_1136.escaper(escaper_1132);
  } else {
    return_1126 = escaper_1132;
  }
  return return_1126;
};
