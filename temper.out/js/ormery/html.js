import {
  percentEscapeOctetTo as percentEscapeOctetTo_360
} from "./url.js";
import {
  Codec as Codec_439, AutoescState as AutoescState_445, AfterPropagate as AfterPropagate_446, ContextPropagator as ContextPropagator_447, Context as Context_469, Escaper as Escaper_495, EscaperPicker as EscaperPicker_547, ContextualAutoescapingAccumulator as ContextualAutoescapingAccumulator_587, Delegate as Delegate_590, Subsidiary as Subsidiary_593, ContextDelegate as ContextDelegate_609
} from "./core.js";
import {
  Begin as Begin_365, End as End_367, CodeSet as CodeSet_809, CodePoints as CodePoints_810, Sequence as Sequence_812, Repeat as Repeat_814, Or as Or_816, Regex as Regex_836, CodeRange as CodeRange_1035
} from "@temperlang/std/regex";
import {
  type as type__440, requireInstanceOf as requireInstanceOf__1139, stringGet as stringGet_395, stringNext as stringNext_397, stringToInt32 as stringToInt32_430, stringBuilderAppendCodePoint as stringBuilderAppendCodePoint_432, mappedGetOr as mappedGetOr_437, float64ToString as float64ToString_524, panic as panic_607, listedGet as listedGet_675, mapBuilderConstructor as mapBuilderConstructor_705, mapBuilderSet as mapBuilderSet_708, mappedToMap as mappedToMap_710, listBuilderAdd as listBuilderAdd_839, listBuilderToList as listBuilderToList_840
} from "@temperlang/core";
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
/** @type {boolean} */
let t_381;
/** @type {boolean} */
let t_382;
export class HtmlCodec extends type__440(Codec_439) {
  /**
   * @param {string} s_384
   * @returns {string}
   */
  encode(s_384) {
    let return_385;
    let t_386;
    let t_387;
    let t_388;
    let t_389;
    const sb_390 = [""];
    const end_391 = s_384.length;
    let encodedTo_392 = 0;
    let i_393 = 0;
    while (i_393 < end_391) {
      continue_394: {
        t_386 = stringGet_395(s_384, i_393);
        if (t_386 === 38) {
          t_389 = "\u0026amp;";
        } else if (t_386 === 60) {
          t_389 = "\u0026lt;";
        } else if (t_386 === 62) {
          t_389 = "\u0026gt;";
        } else if (t_386 === 39) {
          t_389 = "\u0026#39;";
        } else if (t_386 === 34) {
          t_389 = "\u0026#34;";
        } else if (t_386 === 0) {
          t_389 = "\u0026#0;";
        } else {
          break continue_394;
        }
        const replacement_396 = t_389;
        sb_390[0] += s_384.substring(encodedTo_392, i_393);
        sb_390[0] += replacement_396;
        t_387 = stringNext_397(s_384, i_393);
        encodedTo_392 = t_387;
      }
      t_388 = stringNext_397(s_384, i_393);
      i_393 = t_388;
    }
    if (encodedTo_392 > 0) {
      sb_390[0] += s_384.substring(encodedTo_392, end_391);
      return_385 = sb_390[0];
    } else {
      return_385 = s_384;
    }
    return return_385;
  }
  /**
   * @param {string} s_399
   * @returns {string}
   */
  decode(s_399) {
    let return_400;
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
    let t_415;
    let t_416;
    const sb_417 = [""];
    const end_418 = s_399.length;
    let decodedTo_419 = 0;
    let i_420 = 0;
    while (i_420 < end_418) {
      continue_421: {
        if (stringGet_395(s_399, i_420) === 38) {
          const startOfEntity_422 = stringNext_397(s_399, i_420);
          let endOfEntity_423 = startOfEntity_422;
          if (startOfEntity_422 < end_418) {
            t_401 = stringGet_395(s_399, startOfEntity_422);
            t_409 = Object.is(t_401, "#");
          } else {
            t_409 = false;
          }
          if (t_409) {
            t_402 = stringNext_397(s_399, startOfEntity_422);
            endOfEntity_423 = t_402;
            if (endOfEntity_423 >= end_418) {
              break continue_421;
            }
            let base_424 = 10;
            if ((stringGet_395(s_399, endOfEntity_423) | 32) === 120) {
              t_403 = stringNext_397(s_399, endOfEntity_423);
              endOfEntity_423 = t_403;
              base_424 = 16;
            }
            let digitQuota_425 = 7;
            const startOfDigits_426 = endOfEntity_423;
            while (true) {
              if (!(endOfEntity_423 < end_418)) {
                break;
              }
              const cp_427 = stringGet_395(s_399, endOfEntity_423);
              if (48 <= cp_427) {
                t_410 = cp_427 <= 57;
              } else {
                t_410 = false;
              }
              if (! t_410) {
                if (base_424 === 16) {
                  const lcp_428 = cp_427 | 32;
                  if (97 <= lcp_428) {
                    t_411 = lcp_428 <= 102;
                  } else {
                    t_411 = false;
                  }
                  if (! t_411) {
                    break;
                  }
                } else {
                  break;
                }
              }
              t_404 = stringNext_397(s_399, endOfEntity_423);
              endOfEntity_423 = t_404;
            }
            const endOfDigits_429 = endOfEntity_423;
            if (endOfDigits_429 === startOfDigits_426) {
              break continue_421;
            }
            if (endOfEntity_423 < end_418) {
              t_405 = stringGet_395(s_399, endOfEntity_423);
              t_412 = t_405 === 59;
            } else {
              t_412 = false;
            }
            if (t_412) {
              t_406 = stringNext_397(s_399, endOfEntity_423);
              endOfEntity_423 = t_406;
            }
            try {
              t_413 = stringToInt32_430(s_399.substring(startOfDigits_426, endOfDigits_429), base_424);
            } catch {
              break continue_421;
            }
            const decodedCp_431 = t_413;
            if (0 <= decodedCp_431) {
              t_414 = decodedCp_431 <= 1114111;
            } else {
              t_414 = false;
            }
            if (t_414) {
              sb_417[0] += s_399.substring(decodedTo_419, i_420);
              if (55296 <= decodedCp_431) {
                t_415 = decodedCp_431 <= 57343;
              } else {
                t_415 = false;
              }
              if (t_415) {
                sb_417[0] += "�";
              } else {
                try {
                  stringBuilderAppendCodePoint_432(sb_417, decodedCp_431);
                } catch {
                  break continue_421;
                }
              }
              decodedTo_419 = endOfEntity_423;
            }
          } else {
            while (endOfEntity_423 < end_418) {
              const cp_433 = stringGet_395(s_399, endOfEntity_423);
              t_407 = stringNext_397(s_399, endOfEntity_423);
              endOfEntity_423 = t_407;
              if (cp_433 === 59) {
                break;
              }
              const lcp_434 = cp_433 | 32;
              if (97 <= lcp_434) {
                t_416 = lcp_434 <= 122;
              } else {
                t_416 = false;
              }
              if (! t_416) {
                break;
              }
            }
            if (startOfEntity_422 < endOfEntity_423) {
              const entityName_435 = s_399.substring(startOfEntity_422, endOfEntity_423);
              const entityValue_436 = mappedGetOr_437(htmlNamedCharacters_438, entityName_435, "");
              if (! ! entityValue_436) {
                sb_417[0] += s_399.substring(decodedTo_419, i_420);
                sb_417[0] += entityValue_436;
                decodedTo_419 = endOfEntity_423;
              }
            }
          }
        }
      }
      t_408 = stringNext_397(s_399, i_420);
      i_420 = t_408;
    }
    if (decodedTo_419 > 0) {
      sb_417[0] += s_399.substring(decodedTo_419, end_418);
      return_400 = sb_417[0];
    } else {
      return_400 = s_399;
    }
    return return_400;
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlContextPropagator extends type__440(ContextPropagator_447) {
  /**
   * @param {AutoescState_445<HtmlEscaperContext>} before_442
   * @param {string | null} literalPart_443
   * @returns {AfterPropagate_446<HtmlEscaperContext>}
   */
  after(before_442, literalPart_443) {
    return htmlPropagateContext_444(before_442, literalPart_443);
  }
  constructor() {
    super ();
    return;
  }
};
export class UrlContextPropagator extends type__440(ContextPropagator_447) {
  /**
   * @param {AutoescState_445<UrlEscaperContext>} before_449
   * @param {string | null} literalPart_450
   * @returns {AfterPropagate_446<UrlEscaperContext>}
   */
  after(before_449, literalPart_450) {
    return urlPropagateContext_451(before_449, literalPart_450);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlEscaperContext extends type__440(Context_469) {
  /** @type {number} */
  #htmlState_452;
  /** @type {number} */
  #tagState_453;
  /** @type {number} */
  #attribState_454;
  /** @type {number} */
  #delimState_455;
  /** @returns {string} */
  toString() {
    return "HtmlEscaperContext(" + htmlStateStr_457(this.#htmlState_452) + ", " + tagStateStr_458(this.#tagState_453) + ", " + attribStateStr_459(this.#attribState_454) + ", " + delimStateStr_460(this.#delimState_455) + ")";
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
   * @param {number} htmlState_461
   * @param {number} tagState_462
   * @param {number} attribState_463
   * @param {number} delimState_464
   */
  constructor(htmlState_461, tagState_462, attribState_463, delimState_464) {
    super ();
    this.#htmlState_452 = htmlState_461;
    this.#tagState_453 = tagState_462;
    this.#attribState_454 = attribState_463;
    this.#delimState_455 = delimState_464;
    return;
  }
  /** @returns {number} */
  get htmlState() {
    return this.#htmlState_452;
  }
  /** @returns {number} */
  get tagState() {
    return this.#tagState_453;
  }
  /** @returns {number} */
  get attribState() {
    return this.#attribState_454;
  }
  /** @returns {number} */
  get delimState() {
    return this.#delimState_455;
  }
};
export class UrlEscaperContext extends type__440(Context_469) {
  /** @type {number} */
  #urlState_470;
  /** @returns {string} */
  toString() {
    return "UrlEscaperContext(" + urlStateStr_472(this.#urlState_470) + ")";
  }
  /** @param {number} urlState_473 */
  constructor(urlState_473) {
    super ();
    this.#urlState_470 = urlState_473;
    return;
  }
  /** @returns {number} */
  get urlState() {
    return this.#urlState_470;
  }
};
export class SafeHtml extends type__440() {
  /** @type {string} */
  #text_475;
  /** @returns {string} */
  toString() {
    return this.#text_475;
  }
  /** @param {string} text_477 */
  constructor(text_477) {
    super ();
    this.#text_475 = text_477;
    return;
  }
  /** @returns {string} */
  get text() {
    return this.#text_475;
  }
};
export class SafeUrl extends type__440() {
  /** @type {string} */
  #text_479;
  /** @returns {string} */
  toString() {
    return this.#text_479;
  }
  /** @param {string} text_481 */
  constructor(text_481) {
    super ();
    this.#text_479 = text_481;
    return;
  }
  /** @returns {string} */
  get text() {
    return this.#text_479;
  }
};
export class HtmlEscaper extends type__440(Escaper_495) {
  /**
   * @param {SafeHtml} x_484
   * @returns {string}
   */
  applySafeHtml(x_484) {
    null;
  }
  /**
   * @param {SafeUrl} x_486
   * @returns {string}
   */
  applySafeUrl(x_486) {
    null;
  }
  /**
   * @param {number} x_488
   * @returns {string}
   */
  applyInt32(x_488) {
    null;
  }
  /**
   * @param {bigint} x_490
   * @returns {string}
   */
  applyInt64(x_490) {
    null;
  }
  /**
   * @param {number} x_492
   * @returns {string}
   */
  applyFloat64(x_492) {
    null;
  }
  /**
   * @param {string} x_494
   * @returns {string}
   */
  applyString(x_494) {
    null;
  }
};
export class OutputHtmlSpaceEscaper extends type__440(HtmlEscaper) {
  /** @type {OutputHtmlSpaceEscaper} */
  static #instance_496 = new OutputHtmlSpaceEscaper();
  /** @returns {OutputHtmlSpaceEscaper} */
  static get instance() {
    return this.#instance_496;
  }
  /**
   * @param {SafeHtml} x_498
   * @returns {string}
   */
  applySafeHtml(x_498) {
    return " ";
  }
  /**
   * @param {SafeUrl} x_500
   * @returns {string}
   */
  applySafeUrl(x_500) {
    return " ";
  }
  /**
   * @param {number} x_502
   * @returns {string}
   */
  applyInt32(x_502) {
    return " ";
  }
  /**
   * @param {bigint} x_504
   * @returns {string}
   */
  applyInt64(x_504) {
    return " ";
  }
  /**
   * @param {number} x_506
   * @returns {string}
   */
  applyFloat64(x_506) {
    return " ";
  }
  /**
   * @param {string} x_508
   * @returns {string}
   */
  applyString(x_508) {
    return " ";
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlPcdataEscaper extends type__440(HtmlEscaper) {
  /** @type {HtmlPcdataEscaper} */
  static #instance_509 = new HtmlPcdataEscaper();
  /** @returns {HtmlPcdataEscaper} */
  static get instance() {
    return this.#instance_509;
  }
  /**
   * @param {SafeHtml} x_511
   * @returns {string}
   */
  applySafeHtml(x_511) {
    return x_511.toString();
  }
  /**
   * @param {SafeUrl} x_513
   * @returns {string}
   */
  applySafeUrl(x_513) {
    let t_514 = x_513.text;
    return this.applyString(t_514);
  }
  /**
   * @param {number} x_516
   * @returns {string}
   */
  applyInt32(x_516) {
    let t_517 = x_516.toString();
    return this.applyString(t_517);
  }
  /**
   * @param {bigint} x_519
   * @returns {string}
   */
  applyInt64(x_519) {
    let t_520 = x_519.toString();
    return this.applyString(t_520);
  }
  /**
   * @param {number} x_522
   * @returns {string}
   */
  applyFloat64(x_522) {
    let t_523 = float64ToString_524(x_522);
    return this.applyString(t_523);
  }
  /**
   * @param {string} x_526
   * @returns {string}
   */
  applyString(x_526) {
    return htmlCodec.encode(x_526);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlAttributeEscaper extends type__440(HtmlEscaper) {
  /** @type {HtmlAttributeEscaper} */
  static #instance_527 = new HtmlAttributeEscaper();
  /** @returns {HtmlAttributeEscaper} */
  static get instance() {
    return this.#instance_527;
  }
  /**
   * @param {SafeHtml} x_529
   * @returns {string}
   */
  applySafeHtml(x_529) {
    let t_530 = htmlCodec.decode(x_529.text);
    return this.applyString(t_530);
  }
  /**
   * @param {SafeUrl} x_532
   * @returns {string}
   */
  applySafeUrl(x_532) {
    let t_533 = x_532.text;
    return this.applyString(t_533);
  }
  /**
   * @param {number} x_535
   * @returns {string}
   */
  applyInt32(x_535) {
    let t_536 = x_535.toString();
    return this.applyString(t_536);
  }
  /**
   * @param {bigint} x_538
   * @returns {string}
   */
  applyInt64(x_538) {
    let t_539 = x_538.toString();
    return this.applyString(t_539);
  }
  /**
   * @param {number} x_541
   * @returns {string}
   */
  applyFloat64(x_541) {
    let t_542 = float64ToString_524(x_541);
    return this.applyString(t_542);
  }
  /**
   * @param {string} x_544
   * @returns {string}
   */
  applyString(x_544) {
    return htmlCodec.encode(x_544);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlEscaperPicker extends type__440(EscaperPicker_547) {
  /**
   * @param {AutoescState_445<HtmlEscaperContext>} stateBefore_546
   * @returns {HtmlEscaper}
   */
  escaperFor(stateBefore_546) {
    return pickHtmlEscaper(stateBefore_546);
  }
  constructor() {
    super ();
    return;
  }
};
export class SafeHtmlBuilder extends type__440(ContextualAutoescapingAccumulator_587) {
  /** @returns {globalThis.Array<string>} */
  static newCollector() {
    return[""];
  }
  /** @returns {AutoescState_445<HtmlEscaperContext>} */
  static initialState() {
    return new AutoescState_445(new HtmlEscaperContext(0, 0, 0, 0), null);
  }
  /** @returns {HtmlContextPropagator} */
  static propagator() {
    return new HtmlContextPropagator();
  }
  /** @returns {EscaperPicker_547<HtmlEscaperContext, HtmlEscaper>} */
  static picker() {
    return new HtmlEscaperPicker();
  }
  /**
   * @param {globalThis.Array<string>} collector_553
   * @returns {SafeHtml}
   */
  static fromCollector(collector_553) {
    return new SafeHtml(collector_553[0]);
  }
  /**
   * @param {AutoescState_445<HtmlEscaperContext>} a_555
   * @param {AutoescState_445<HtmlEscaperContext>} b_556
   * @returns {AutoescState_445<HtmlEscaperContext>}
   */
  static mergeStates(a_555, b_556) {
    return a_555;
  }
  /** @type {AutoescState_445<HtmlEscaperContext>} */
  #_state_557;
  /** @type {globalThis.Array<string>} */
  #collector_558;
  /** @returns {AutoescState_445<HtmlEscaperContext>} */
  get state() {
    return this.#_state_557;
  }
  /** @param {AutoescState_445<HtmlEscaperContext>} x_561 */
  set state(x_561) {
    this.#_state_557 = x_561;
    return;
  }
  /** @returns {EscaperPicker_547<HtmlEscaperContext, HtmlEscaper>} */
  get escaperPicker() {
    return SafeHtmlBuilder.picker();
  }
  /** @returns {ContextPropagator_447<HtmlEscaperContext>} */
  get contextPropagator() {
    return SafeHtmlBuilder.propagator();
  }
  /** @returns {SafeHtml} */
  get accumulated() {
    return SafeHtmlBuilder.fromCollector(this.#collector_558);
  }
  /** @param {string} fixed_566 */
  collectFixed(fixed_566) {
    this.#collector_558[0] += fixed_566;
    return;
  }
  /** @param {SafeHtml} x_568 */
  appendSafeHtml(x_568) {
    let t_569 = this.prepareForAppend().applySafeHtml(x_568);
    this.#collector_558[0] += t_569;
    return;
  }
  /** @param {SafeUrl} x_571 */
  appendSafeUrl(x_571) {
    let t_572 = this.prepareForAppend().applySafeUrl(x_571);
    this.#collector_558[0] += t_572;
    return;
  }
  /** @param {number} x_574 */
  appendInt32(x_574) {
    let t_575 = this.prepareForAppend().applyInt32(x_574);
    this.#collector_558[0] += t_575;
    return;
  }
  /** @param {bigint} x_577 */
  appendInt64(x_577) {
    let t_578 = this.prepareForAppend().applyInt64(x_577);
    this.#collector_558[0] += t_578;
    return;
  }
  /** @param {number} x_580 */
  appendFloat64(x_580) {
    let t_581 = this.prepareForAppend().applyFloat64(x_580);
    this.#collector_558[0] += t_581;
    return;
  }
  /** @param {string} x_583 */
  appendString(x_583) {
    let t_584 = this.prepareForAppend().applyString(x_583);
    this.#collector_558[0] += t_584;
    return;
  }
  constructor() {
    super ();
    let t_585 = SafeHtmlBuilder.initialState();
    this.#_state_557 = t_585;
    let t_586 = SafeHtmlBuilder.newCollector();
    this.#collector_558 = t_586;
    return;
  }
};
export class HtmlDelegate extends type__440(Delegate_590) {
  /**
   * @param {HtmlEscaper} outer_589
   * @returns {HtmlEscaper}
   */
  escaper(outer_589) {
    null;
  }
};
export class HtmlUrlDelegate extends type__440(ContextDelegate_609, HtmlDelegate) {
  /** @type {AutoescState_445<UrlEscaperContext>} */
  #_state_591;
  /** @type {Subsidiary_593 | null} */
  #_subsidiary_592;
  /** @returns {AutoescState_445<UrlEscaperContext>} */
  get state() {
    return this.#_state_591;
  }
  /** @param {AutoescState_445<UrlEscaperContext>} x_596 */
  set state(x_596) {
    this.#_state_591 = x_596;
    return;
  }
  /** @returns {ContextPropagator_447<UrlEscaperContext>} */
  get contextPropagator() {
    return urlContextPropagator_598;
  }
  /**
   * @param {HtmlEscaper} outer_600
   * @returns {HtmlEscaper}
   */
  escaper(outer_600) {
    let return_601;
    let t_602;
    let t_603 = this.state.context.urlState;
    if (t_603 === 0) {
      return_601 = new HtmlUrlEscaperAdapter(htmlProtocolFilteringUrlEscaper_604, outer_600);
    } else if (t_603 === 1) {
      return_601 = new HtmlUrlEscaperAdapter(htmlUrlPartUrlEscaper_605, outer_600);
    } else {
      if (t_603 === 2) {
        t_602 = true;
      } else {
        t_602 = t_603 === 3;
      }
      if (t_602) {
        return_601 = new HtmlUrlEscaperAdapter(htmlAsIfQueryUrlEscaper_606, outer_600);
      } else {
        return_601 = panic_607();
      }
    }
    return return_601;
  }
  constructor() {
    super ();
    let t_608 = new AutoescState_445(new UrlEscaperContext(0), null);
    this.#_state_591 = t_608;
    this.#_subsidiary_592 = null;
    return;
  }
};
export class HtmlUrlEscaperAdapter extends type__440(HtmlEscaper) {
  /** @type {UrlEscaper} */
  #first_610;
  /** @type {HtmlEscaper} */
  #second_611;
  /**
   * @param {SafeHtml} x_613
   * @returns {string}
   */
  applySafeHtml(x_613) {
    let t_614 = x_613.text;
    let t_615 = this.#first_610.applyString(t_614);
    return this.#second_611.applySafeUrl(t_615);
  }
  /**
   * @param {SafeUrl} x_617
   * @returns {string}
   */
  applySafeUrl(x_617) {
    let t_618 = this.#first_610.applySafeUrl(x_617);
    return this.#second_611.applySafeUrl(t_618);
  }
  /**
   * @param {number} x_620
   * @returns {string}
   */
  applyInt32(x_620) {
    let t_621 = x_620.toString();
    let t_622 = this.#first_610.applyString(t_621);
    return this.#second_611.applySafeUrl(t_622);
  }
  /**
   * @param {bigint} x_624
   * @returns {string}
   */
  applyInt64(x_624) {
    let t_625 = x_624.toString();
    let t_626 = this.#first_610.applyString(t_625);
    return this.#second_611.applySafeUrl(t_626);
  }
  /**
   * @param {number} x_628
   * @returns {string}
   */
  applyFloat64(x_628) {
    let t_629 = float64ToString_524(x_628);
    let t_630 = this.#first_610.applyString(t_629);
    return this.#second_611.applySafeUrl(t_630);
  }
  /**
   * @param {string} x_632
   * @returns {string}
   */
  applyString(x_632) {
    let t_633 = this.#first_610.applyString(x_632);
    return this.#second_611.applySafeUrl(t_633);
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
   * @param {UrlEscaper} first_634
   * @param {HtmlEscaper} second_635
   */
  constructor(first_634, second_635) {
    super ();
    this.#first_610 = first_634;
    this.#second_611 = second_635;
    return;
  }
  /** @returns {UrlEscaper} */
  get first() {
    return this.#first_610;
  }
  /** @returns {HtmlEscaper} */
  get second() {
    return this.#second_611;
  }
};
export class UrlEscaper extends type__440(Escaper_495) {
  /**
   * @param {SafeUrl} x_639
   * @returns {SafeUrl}
   */
  applySafeUrl(x_639) {
    null;
  }
  /**
   * @param {string} x_641
   * @returns {SafeUrl}
   */
  applyString(x_641) {
    null;
  }
};
export class HtmlProtocolFilteringUrlEscaper extends type__440(UrlEscaper) {
  /** @type {HtmlProtocolFilteringUrlEscaper} */
  static #instance_642 = new HtmlProtocolFilteringUrlEscaper();
  /** @returns {HtmlProtocolFilteringUrlEscaper} */
  static get instance() {
    return this.#instance_642;
  }
  /**
   * @param {SafeUrl} x_644
   * @returns {SafeUrl}
   */
  applySafeUrl(x_644) {
    return x_644;
  }
  /**
   * @param {string} x_646
   * @returns {SafeUrl}
   */
  applyString(x_646) {
    let return_647;
    let t_648;
    let t_649;
    let t_650;
    fn_651: {
      let protocolEnd_652 = 0;
      const end_653 = x_646.length;
      while (protocolEnd_652 < end_653) {
        const cp_654 = stringGet_395(x_646, protocolEnd_652);
        if (cp_654 === 58) {
          const protocol_655 = x_646.substring(0, protocolEnd_652);
          try {
            t_649 = protocolAllowList_656.find(protocol_655);
            t_650 = t_649;
          } catch {
            t_650 = null;
          }
          if (!(t_650 == null)) {
            return_647 = new SafeUrl(x_646);
          } else {
            return_647 = fallbackSafeUrl_657;
          }
          break fn_651;
        }
        t_648 = stringNext_397(x_646, protocolEnd_652);
        protocolEnd_652 = t_648;
      }
      return_647 = htmlUrlPartUrlEscaper_605.applyString(x_646);
    }
    return return_647;
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlUrlPartUrlEscaper extends type__440(UrlEscaper) {
  /** @type {HtmlUrlPartUrlEscaper} */
  static #instance_658 = new HtmlUrlPartUrlEscaper();
  /** @returns {HtmlUrlPartUrlEscaper} */
  static get instance() {
    return this.#instance_658;
  }
  /**
   * @param {SafeUrl} x_660
   * @returns {SafeUrl}
   */
  applySafeUrl(x_660) {
    return x_660;
  }
  /**
   * @param {string} x_662
   * @returns {SafeUrl}
   */
  applyString(x_662) {
    let t_663;
    let t_664;
    let t_665;
    let t_666;
    let t_667;
    let t_668;
    let i_669 = 0;
    const end_670 = x_662.length;
    let emitted_671 = 0;
    const sb_672 = [""];
    while (i_669 < end_670) {
      const cp_673 = stringGet_395(x_662, i_669);
      if (cp_673 < urlSafe_674.length) {
        t_663 = listedGet_675(urlSafe_674, cp_673);
        t_667 = ! t_663;
      } else {
        t_667 = false;
      }
      if (t_667) {
        sb_672[0] += x_662.substring(emitted_671, i_669);
        percentEscapeOctetTo_360(cp_673, sb_672);
        t_664 = stringNext_397(x_662, i_669);
        emitted_671 = t_664;
      }
      t_665 = stringNext_397(x_662, i_669);
      i_669 = t_665;
    }
    if (emitted_671 > 0) {
      sb_672[0] += x_662.substring(emitted_671, end_670);
      t_666 = sb_672[0];
      t_668 = t_666;
    } else {
      t_668 = x_662;
    }
    return new SafeUrl(t_668);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlAsIfQueryUrlEscaper extends type__440(UrlEscaper) {
  /** @type {HtmlAsIfQueryUrlEscaper} */
  static #instance_676 = new HtmlAsIfQueryUrlEscaper();
  /** @returns {HtmlAsIfQueryUrlEscaper} */
  static get instance() {
    return this.#instance_676;
  }
  /**
   * @param {SafeUrl} x_678
   * @returns {SafeUrl}
   */
  applySafeUrl(x_678) {
    return x_678;
  }
  /**
   * @param {string} x_680
   * @returns {SafeUrl}
   */
  applyString(x_680) {
    let t_681;
    let t_682;
    let t_683;
    let t_684;
    let t_685;
    let t_686;
    let i_687 = 0;
    const end_688 = x_680.length;
    let emitted_689 = 0;
    const sb_690 = [""];
    while (i_687 < end_688) {
      const cp_691 = stringGet_395(x_680, i_687);
      if (cp_691 < urlQuerySafe_692.length) {
        t_681 = listedGet_675(urlQuerySafe_692, cp_691);
        t_685 = ! t_681;
      } else {
        t_685 = false;
      }
      if (t_685) {
        sb_690[0] += x_680.substring(emitted_689, i_687);
        percentEscapeOctetTo_360(cp_691, sb_690);
        t_682 = stringNext_397(x_680, i_687);
        emitted_689 = t_682;
      }
      t_683 = stringNext_397(x_680, i_687);
      i_687 = t_683;
    }
    if (emitted_689 > 0) {
      sb_690[0] += x_680.substring(emitted_689, end_688);
      t_684 = sb_690[0];
      t_686 = t_684;
    } else {
      t_686 = x_680;
    }
    return new SafeUrl(t_686);
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlCssDelegate extends type__440(HtmlDelegate) {
  /**
   * @param {string | null} s_694
   * @returns {string}
   */
  process(s_694) {
    let return_695;
    if (!(s_694 == null)) {
      return_695 = s_694;
    } else {
      return_695 = "";
    }
    return return_695;
  }
  /**
   * @param {HtmlEscaper} outer_697
   * @returns {HtmlEscaper | null}
   */
  escaper(outer_697) {
    return outer_697;
  }
  constructor() {
    super ();
    return;
  }
};
export class HtmlJsDelegate extends type__440(HtmlDelegate) {
  /**
   * @param {string | null} s_699
   * @returns {string}
   */
  process(s_699) {
    let return_700;
    if (!(s_699 == null)) {
      return_700 = s_699;
    } else {
      return_700 = "";
    }
    return return_700;
  }
  /**
   * @param {HtmlEscaper} outer_702
   * @returns {HtmlEscaper | null}
   */
  escaper(outer_702) {
    return outer_702;
  }
  constructor() {
    super ();
    return;
  }
};
/** @type {Array<string>} */
const strs_703 = Object.freeze(["AElig", "Æ", "AElig;", "Æ", "AMP", "\u0026", "AMP;", "\u0026", "Aacute", "Á", "Aacute;", "Á", "Abreve;", "Ă", "Acirc", "Â", "Acirc;", "Â", "Acy;", "А", "Afr;", "\ud835\udd04", "Agrave", "À", "Agrave;", "À", "Alpha;", "Α", "Amacr;", "Ā", "And;", "⩓", "Aogon;", "Ą", "Aopf;", "\ud835\udd38", "ApplyFunction;", "⁡", "Aring", "Å", "Aring;", "Å", "Ascr;", "\ud835\udc9c", "Assign;", "≔", "Atilde", "Ã", "Atilde;", "Ã", "Auml", "Ä", "Auml;", "Ä", "Backslash;", "∖", "Barv;", "⫧", "Barwed;", "⌆", "Bcy;", "Б", "Because;", "∵", "Bernoullis;", "ℬ", "Beta;", "Β", "Bfr;", "\ud835\udd05", "Bopf;", "\ud835\udd39", "Breve;", "˘", "Bscr;", "ℬ", "Bumpeq;", "≎", "CHcy;", "Ч", "COPY", "©", "COPY;", "©", "Cacute;", "Ć", "Cap;", "⋒", "CapitalDifferentialD;", "ⅅ", "Cayleys;", "ℭ", "Ccaron;", "Č", "Ccedil", "Ç", "Ccedil;", "Ç", "Ccirc;", "Ĉ", "Cconint;", "∰", "Cdot;", "Ċ", "Cedilla;", "¸", "CenterDot;", "·", "Cfr;", "ℭ", "Chi;", "Χ", "CircleDot;", "⊙", "CircleMinus;", "⊖", "CirclePlus;", "⊕", "CircleTimes;", "⊗", "ClockwiseContourIntegral;", "∲", "CloseCurlyDoubleQuote;", "”", "CloseCurlyQuote;", "’", "Colon;", "∷", "Colone;", "⩴", "Congruent;", "≡", "Conint;", "∯", "ContourIntegral;", "∮", "Copf;", "ℂ", "Coproduct;", "∐", "CounterClockwiseContourIntegral;", "∳", "Cross;", "⨯", "Cscr;", "\ud835\udc9e", "Cup;", "⋓", "CupCap;", "≍", "DD;", "ⅅ", "DDotrahd;", "⤑", "DJcy;", "Ђ", "DScy;", "Ѕ", "DZcy;", "Џ", "Dagger;", "‡", "Darr;", "↡", "Dashv;", "⫤", "Dcaron;", "Ď", "Dcy;", "Д", "Del;", "∇", "Delta;", "Δ", "Dfr;", "\ud835\udd07", "DiacriticalAcute;", "´", "DiacriticalDot;", "˙", "DiacriticalDoubleAcute;", "˝", "DiacriticalGrave;", "`", "DiacriticalTilde;", "˜", "Diamond;", "⋄", "DifferentialD;", "ⅆ", "Dopf;", "\ud835\udd3b", "Dot;", "¨", "DotDot;", "⃜", "DotEqual;", "≐", "DoubleContourIntegral;", "∯", "DoubleDot;", "¨", "DoubleDownArrow;", "⇓", "DoubleLeftArrow;", "⇐", "DoubleLeftRightArrow;", "⇔", "DoubleLeftTee;", "⫤", "DoubleLongLeftArrow;", "⟸", "DoubleLongLeftRightArrow;", "⟺", "DoubleLongRightArrow;", "⟹", "DoubleRightArrow;", "⇒", "DoubleRightTee;", "⊨", "DoubleUpArrow;", "⇑", "DoubleUpDownArrow;", "⇕", "DoubleVerticalBar;", "∥", "DownArrow;", "↓", "DownArrowBar;", "⤓", "DownArrowUpArrow;", "⇵", "DownBreve;", "̑", "DownLeftRightVector;", "⥐", "DownLeftTeeVector;", "⥞", "DownLeftVector;", "↽", "DownLeftVectorBar;", "⥖", "DownRightTeeVector;", "⥟", "DownRightVector;", "⇁", "DownRightVectorBar;", "⥗", "DownTee;", "⊤", "DownTeeArrow;", "↧", "Downarrow;", "⇓", "Dscr;", "\ud835\udc9f", "Dstrok;", "Đ", "ENG;", "Ŋ", "ETH", "Ð", "ETH;", "Ð", "Eacute", "É", "Eacute;", "É", "Ecaron;", "Ě", "Ecirc", "Ê", "Ecirc;", "Ê", "Ecy;", "Э", "Edot;", "Ė", "Efr;", "\ud835\udd08", "Egrave", "È", "Egrave;", "È", "Element;", "∈", "Emacr;", "Ē", "EmptySmallSquare;", "◻", "EmptyVerySmallSquare;", "▫", "Eogon;", "Ę", "Eopf;", "\ud835\udd3c", "Epsilon;", "Ε", "Equal;", "⩵", "EqualTilde;", "≂", "Equilibrium;", "⇌", "Escr;", "ℰ", "Esim;", "⩳", "Eta;", "Η", "Euml", "Ë", "Euml;", "Ë", "Exists;", "∃", "ExponentialE;", "ⅇ", "Fcy;", "Ф", "Ffr;", "\ud835\udd09", "FilledSmallSquare;", "◼", "FilledVerySmallSquare;", "▪", "Fopf;", "\ud835\udd3d", "ForAll;", "∀", "Fouriertrf;", "ℱ", "Fscr;", "ℱ", "GJcy;", "Ѓ", "GT", "\u003e", "GT;", "\u003e", "Gamma;", "Γ", "Gammad;", "Ϝ", "Gbreve;", "Ğ", "Gcedil;", "Ģ", "Gcirc;", "Ĝ", "Gcy;", "Г", "Gdot;", "Ġ", "Gfr;", "\ud835\udd0a", "Gg;", "⋙", "Gopf;", "\ud835\udd3e", "GreaterEqual;", "≥", "GreaterEqualLess;", "⋛", "GreaterFullEqual;", "≧", "GreaterGreater;", "⪢", "GreaterLess;", "≷", "GreaterSlantEqual;", "⩾", "GreaterTilde;", "≳", "Gscr;", "\ud835\udca2", "Gt;", "≫", "HARDcy;", "Ъ", "Hacek;", "ˇ", "Hat;", "^", "Hcirc;", "Ĥ", "Hfr;", "ℌ", "HilbertSpace;", "ℋ", "Hopf;", "ℍ", "HorizontalLine;", "─", "Hscr;", "ℋ", "Hstrok;", "Ħ", "HumpDownHump;", "≎", "HumpEqual;", "≏", "IEcy;", "Е", "IJlig;", "Ĳ", "IOcy;", "Ё", "Iacute", "Í", "Iacute;", "Í", "Icirc", "Î", "Icirc;", "Î", "Icy;", "И", "Idot;", "İ", "Ifr;", "ℑ", "Igrave", "Ì", "Igrave;", "Ì", "Im;", "ℑ", "Imacr;", "Ī", "ImaginaryI;", "ⅈ", "Implies;", "⇒", "Int;", "∬", "Integral;", "∫", "Intersection;", "⋂", "InvisibleComma;", "⁣", "InvisibleTimes;", "⁢", "Iogon;", "Į", "Iopf;", "\ud835\udd40", "Iota;", "Ι", "Iscr;", "ℐ", "Itilde;", "Ĩ", "Iukcy;", "І", "Iuml", "Ï", "Iuml;", "Ï", "Jcirc;", "Ĵ", "Jcy;", "Й", "Jfr;", "\ud835\udd0d", "Jopf;", "\ud835\udd41", "Jscr;", "\ud835\udca5", "Jsercy;", "Ј", "Jukcy;", "Є", "KHcy;", "Х", "KJcy;", "Ќ", "Kappa;", "Κ", "Kcedil;", "Ķ", "Kcy;", "К", "Kfr;", "\ud835\udd0e", "Kopf;", "\ud835\udd42", "Kscr;", "\ud835\udca6", "LJcy;", "Љ", "LT", "\u003c", "LT;", "\u003c", "Lacute;", "Ĺ", "Lambda;", "Λ", "Lang;", "⟪", "Laplacetrf;", "ℒ", "Larr;", "↞", "Lcaron;", "Ľ", "Lcedil;", "Ļ", "Lcy;", "Л", "LeftAngleBracket;", "⟨", "LeftArrow;", "←", "LeftArrowBar;", "⇤", "LeftArrowRightArrow;", "⇆", "LeftCeiling;", "⌈", "LeftDoubleBracket;", "⟦", "LeftDownTeeVector;", "⥡", "LeftDownVector;", "⇃", "LeftDownVectorBar;", "⥙", "LeftFloor;", "⌊", "LeftRightArrow;", "↔", "LeftRightVector;", "⥎", "LeftTee;", "⊣", "LeftTeeArrow;", "↤", "LeftTeeVector;", "⥚", "LeftTriangle;", "⊲", "LeftTriangleBar;", "⧏", "LeftTriangleEqual;", "⊴", "LeftUpDownVector;", "⥑", "LeftUpTeeVector;", "⥠", "LeftUpVector;", "↿", "LeftUpVectorBar;", "⥘", "LeftVector;", "↼", "LeftVectorBar;", "⥒", "Leftarrow;", "⇐", "Leftrightarrow;", "⇔", "LessEqualGreater;", "⋚", "LessFullEqual;", "≦", "LessGreater;", "≶", "LessLess;", "⪡", "LessSlantEqual;", "⩽", "LessTilde;", "≲", "Lfr;", "\ud835\udd0f", "Ll;", "⋘", "Lleftarrow;", "⇚", "Lmidot;", "Ŀ", "LongLeftArrow;", "⟵", "LongLeftRightArrow;", "⟷", "LongRightArrow;", "⟶", "Longleftarrow;", "⟸", "Longleftrightarrow;", "⟺", "Longrightarrow;", "⟹", "Lopf;", "\ud835\udd43", "LowerLeftArrow;", "↙", "LowerRightArrow;", "↘", "Lscr;", "ℒ", "Lsh;", "↰", "Lstrok;", "Ł", "Lt;", "≪", "Map;", "⤅", "Mcy;", "М", "MediumSpace;", " ", "Mellintrf;", "ℳ", "Mfr;", "\ud835\udd10", "MinusPlus;", "∓", "Mopf;", "\ud835\udd44", "Mscr;", "ℳ", "Mu;", "Μ", "NJcy;", "Њ", "Nacute;", "Ń", "Ncaron;", "Ň", "Ncedil;", "Ņ", "Ncy;", "Н", "NegativeMediumSpace;", "​", "NegativeThickSpace;", "​", "NegativeThinSpace;", "​", "NegativeVeryThinSpace;", "​", "NestedGreaterGreater;", "≫", "NestedLessLess;", "≪", "NewLine;", "\n", "Nfr;", "\ud835\udd11", "NoBreak;", "⁠", "NonBreakingSpace;", " ", "Nopf;", "ℕ", "Not;", "⫬", "NotCongruent;", "≢", "NotCupCap;", "≭", "NotDoubleVerticalBar;", "∦", "NotElement;", "∉", "NotEqual;", "≠", "NotEqualTilde;", "≂̸", "NotExists;", "∄", "NotGreater;", "≯", "NotGreaterEqual;", "≱", "NotGreaterFullEqual;", "≧̸", "NotGreaterGreater;", "≫̸", "NotGreaterLess;", "≹", "NotGreaterSlantEqual;", "⩾̸", "NotGreaterTilde;", "≵", "NotHumpDownHump;", "≎̸", "NotHumpEqual;", "≏̸", "NotLeftTriangle;", "⋪", "NotLeftTriangleBar;", "⧏̸", "NotLeftTriangleEqual;", "⋬", "NotLess;", "≮", "NotLessEqual;", "≰", "NotLessGreater;", "≸", "NotLessLess;", "≪̸", "NotLessSlantEqual;", "⩽̸", "NotLessTilde;", "≴", "NotNestedGreaterGreater;", "⪢̸", "NotNestedLessLess;", "⪡̸", "NotPrecedes;", "⊀", "NotPrecedesEqual;", "⪯̸", "NotPrecedesSlantEqual;", "⋠", "NotReverseElement;", "∌", "NotRightTriangle;", "⋫", "NotRightTriangleBar;", "⧐̸", "NotRightTriangleEqual;", "⋭", "NotSquareSubset;", "⊏̸", "NotSquareSubsetEqual;", "⋢", "NotSquareSuperset;", "⊐̸", "NotSquareSupersetEqual;", "⋣", "NotSubset;", "⊂⃒", "NotSubsetEqual;", "⊈", "NotSucceeds;", "⊁", "NotSucceedsEqual;", "⪰̸", "NotSucceedsSlantEqual;", "⋡", "NotSucceedsTilde;", "≿̸", "NotSuperset;", "⊃⃒", "NotSupersetEqual;", "⊉", "NotTilde;", "≁", "NotTildeEqual;", "≄", "NotTildeFullEqual;", "≇", "NotTildeTilde;", "≉", "NotVerticalBar;", "∤", "Nscr;", "\ud835\udca9", "Ntilde", "Ñ", "Ntilde;", "Ñ", "Nu;", "Ν", "OElig;", "Œ", "Oacute", "Ó", "Oacute;", "Ó", "Ocirc", "Ô", "Ocirc;", "Ô", "Ocy;", "О", "Odblac;", "Ő", "Ofr;", "\ud835\udd12", "Ograve", "Ò", "Ograve;", "Ò", "Omacr;", "Ō", "Omega;", "Ω", "Omicron;", "Ο", "Oopf;", "\ud835\udd46", "OpenCurlyDoubleQuote;", "“", "OpenCurlyQuote;", "‘", "Or;", "⩔", "Oscr;", "\ud835\udcaa", "Oslash", "Ø", "Oslash;", "Ø", "Otilde", "Õ", "Otilde;", "Õ", "Otimes;", "⨷", "Ouml", "Ö", "Ouml;", "Ö", "OverBar;", "‾", "OverBrace;", "⏞", "OverBracket;", "⎴", "OverParenthesis;", "⏜", "PartialD;", "∂", "Pcy;", "П", "Pfr;", "\ud835\udd13", "Phi;", "Φ", "Pi;", "Π", "PlusMinus;", "±", "Poincareplane;", "ℌ", "Popf;", "ℙ", "Pr;", "⪻", "Precedes;", "≺", "PrecedesEqual;", "⪯", "PrecedesSlantEqual;", "≼", "PrecedesTilde;", "≾", "Prime;", "″", "Product;", "∏", "Proportion;", "∷", "Proportional;", "∝", "Pscr;", "\ud835\udcab", "Psi;", "Ψ", "QUOT", "\u0022", "QUOT;", "\u0022", "Qfr;", "\ud835\udd14", "Qopf;", "ℚ", "Qscr;", "\ud835\udcac", "RBarr;", "⤐", "REG", "®", "REG;", "®", "Racute;", "Ŕ", "Rang;", "⟫", "Rarr;", "↠", "Rarrtl;", "⤖", "Rcaron;", "Ř", "Rcedil;", "Ŗ", "Rcy;", "Р", "Re;", "ℜ", "ReverseElement;", "∋", "ReverseEquilibrium;", "⇋", "ReverseUpEquilibrium;", "⥯", "Rfr;", "ℜ", "Rho;", "Ρ", "RightAngleBracket;", "⟩", "RightArrow;", "→", "RightArrowBar;", "⇥", "RightArrowLeftArrow;", "⇄", "RightCeiling;", "⌉", "RightDoubleBracket;", "⟧", "RightDownTeeVector;", "⥝", "RightDownVector;", "⇂", "RightDownVectorBar;", "⥕", "RightFloor;", "⌋", "RightTee;", "⊢", "RightTeeArrow;", "↦", "RightTeeVector;", "⥛", "RightTriangle;", "⊳", "RightTriangleBar;", "⧐", "RightTriangleEqual;", "⊵", "RightUpDownVector;", "⥏", "RightUpTeeVector;", "⥜", "RightUpVector;", "↾", "RightUpVectorBar;", "⥔", "RightVector;", "⇀", "RightVectorBar;", "⥓", "Rightarrow;", "⇒", "Ropf;", "ℝ", "RoundImplies;", "⥰", "Rrightarrow;", "⇛", "Rscr;", "ℛ", "Rsh;", "↱", "RuleDelayed;", "⧴", "SHCHcy;", "Щ", "SHcy;", "Ш", "SOFTcy;", "Ь", "Sacute;", "Ś", "Sc;", "⪼", "Scaron;", "Š", "Scedil;", "Ş", "Scirc;", "Ŝ", "Scy;", "С", "Sfr;", "\ud835\udd16", "ShortDownArrow;", "↓", "ShortLeftArrow;", "←", "ShortRightArrow;", "→", "ShortUpArrow;", "↑", "Sigma;", "Σ", "SmallCircle;", "∘", "Sopf;", "\ud835\udd4a", "Sqrt;", "√", "Square;", "□", "SquareIntersection;", "⊓", "SquareSubset;", "⊏", "SquareSubsetEqual;", "⊑", "SquareSuperset;", "⊐", "SquareSupersetEqual;", "⊒", "SquareUnion;", "⊔", "Sscr;", "\ud835\udcae", "Star;", "⋆", "Sub;", "⋐", "Subset;", "⋐", "SubsetEqual;", "⊆", "Succeeds;", "≻", "SucceedsEqual;", "⪰", "SucceedsSlantEqual;", "≽", "SucceedsTilde;", "≿", "SuchThat;", "∋", "Sum;", "∑", "Sup;", "⋑", "Superset;", "⊃", "SupersetEqual;", "⊇", "Supset;", "⋑", "THORN", "Þ", "THORN;", "Þ", "TRADE;", "™", "TSHcy;", "Ћ", "TScy;", "Ц", "Tab;", "\t", "Tau;", "Τ", "Tcaron;", "Ť", "Tcedil;", "Ţ", "Tcy;", "Т", "Tfr;", "\ud835\udd17", "Therefore;", "∴", "Theta;", "Θ", "ThickSpace;", "  ", "ThinSpace;", " ", "Tilde;", "∼", "TildeEqual;", "≃", "TildeFullEqual;", "≅", "TildeTilde;", "≈", "Topf;", "\ud835\udd4b", "TripleDot;", "⃛", "Tscr;", "\ud835\udcaf", "Tstrok;", "Ŧ", "Uacute", "Ú", "Uacute;", "Ú", "Uarr;", "↟", "Uarrocir;", "⥉", "Ubrcy;", "Ў", "Ubreve;", "Ŭ", "Ucirc", "Û", "Ucirc;", "Û", "Ucy;", "У", "Udblac;", "Ű", "Ufr;", "\ud835\udd18", "Ugrave", "Ù", "Ugrave;", "Ù", "Umacr;", "Ū", "UnderBar;", "_", "UnderBrace;", "⏟", "UnderBracket;", "⎵", "UnderParenthesis;", "⏝", "Union;", "⋃", "UnionPlus;", "⊎", "Uogon;", "Ų", "Uopf;", "\ud835\udd4c", "UpArrow;", "↑", "UpArrowBar;", "⤒", "UpArrowDownArrow;", "⇅", "UpDownArrow;", "↕", "UpEquilibrium;", "⥮", "UpTee;", "⊥", "UpTeeArrow;", "↥", "Uparrow;", "⇑", "Updownarrow;", "⇕", "UpperLeftArrow;", "↖", "UpperRightArrow;", "↗", "Upsi;", "ϒ", "Upsilon;", "Υ", "Uring;", "Ů", "Uscr;", "\ud835\udcb0", "Utilde;", "Ũ", "Uuml", "Ü", "Uuml;", "Ü", "VDash;", "⊫", "Vbar;", "⫫", "Vcy;", "В", "Vdash;", "⊩", "Vdashl;", "⫦", "Vee;", "⋁", "Verbar;", "‖", "Vert;", "‖", "VerticalBar;", "∣", "VerticalLine;", "|", "VerticalSeparator;", "❘", "VerticalTilde;", "≀", "VeryThinSpace;", " ", "Vfr;", "\ud835\udd19", "Vopf;", "\ud835\udd4d", "Vscr;", "\ud835\udcb1", "Vvdash;", "⊪", "Wcirc;", "Ŵ", "Wedge;", "⋀", "Wfr;", "\ud835\udd1a", "Wopf;", "\ud835\udd4e", "Wscr;", "\ud835\udcb2", "Xfr;", "\ud835\udd1b", "Xi;", "Ξ", "Xopf;", "\ud835\udd4f", "Xscr;", "\ud835\udcb3", "YAcy;", "Я", "YIcy;", "Ї", "YUcy;", "Ю", "Yacute", "Ý", "Yacute;", "Ý", "Ycirc;", "Ŷ", "Ycy;", "Ы", "Yfr;", "\ud835\udd1c", "Yopf;", "\ud835\udd50", "Yscr;", "\ud835\udcb4", "Yuml;", "Ÿ", "ZHcy;", "Ж", "Zacute;", "Ź", "Zcaron;", "Ž", "Zcy;", "З", "Zdot;", "Ż", "ZeroWidthSpace;", "​", "Zeta;", "Ζ", "Zfr;", "ℨ", "Zopf;", "ℤ", "Zscr;", "\ud835\udcb5", "aacute", "á", "aacute;", "á", "abreve;", "ă", "ac;", "∾", "acE;", "∾̳", "acd;", "∿", "acirc", "â", "acirc;", "â", "acute", "´", "acute;", "´", "acy;", "а", "aelig", "æ", "aelig;", "æ", "af;", "⁡", "afr;", "\ud835\udd1e", "agrave", "à", "agrave;", "à", "alefsym;", "ℵ", "aleph;", "ℵ", "alpha;", "α", "amacr;", "ā", "amalg;", "⨿", "amp", "\u0026", "amp;", "\u0026", "and;", "∧", "andand;", "⩕", "andd;", "⩜", "andslope;", "⩘", "andv;", "⩚", "ang;", "∠", "ange;", "⦤", "angle;", "∠", "angmsd;", "∡", "angmsdaa;", "⦨", "angmsdab;", "⦩", "angmsdac;", "⦪", "angmsdad;", "⦫", "angmsdae;", "⦬", "angmsdaf;", "⦭", "angmsdag;", "⦮", "angmsdah;", "⦯", "angrt;", "∟", "angrtvb;", "⊾", "angrtvbd;", "⦝", "angsph;", "∢", "angst;", "Å", "angzarr;", "⍼", "aogon;", "ą", "aopf;", "\ud835\udd52", "ap;", "≈", "apE;", "⩰", "apacir;", "⩯", "ape;", "≊", "apid;", "≋", "apos;", "'", "approx;", "≈", "approxeq;", "≊", "aring", "å", "aring;", "å", "ascr;", "\ud835\udcb6", "ast;", "*", "asymp;", "≈", "asympeq;", "≍", "atilde", "ã", "atilde;", "ã", "auml", "ä", "auml;", "ä", "awconint;", "∳", "awint;", "⨑", "bNot;", "⫭", "backcong;", "≌", "backepsilon;", "϶", "backprime;", "‵", "backsim;", "∽", "backsimeq;", "⋍", "barvee;", "⊽", "barwed;", "⌅", "barwedge;", "⌅", "bbrk;", "⎵", "bbrktbrk;", "⎶", "bcong;", "≌", "bcy;", "б", "bdquo;", "„", "becaus;", "∵", "because;", "∵", "bemptyv;", "⦰", "bepsi;", "϶", "bernou;", "ℬ", "beta;", "β", "beth;", "ℶ", "between;", "≬", "bfr;", "\ud835\udd1f", "bigcap;", "⋂", "bigcirc;", "◯", "bigcup;", "⋃", "bigodot;", "⨀", "bigoplus;", "⨁", "bigotimes;", "⨂", "bigsqcup;", "⨆", "bigstar;", "★", "bigtriangledown;", "▽", "bigtriangleup;", "△", "biguplus;", "⨄", "bigvee;", "⋁", "bigwedge;", "⋀", "bkarow;", "⤍", "blacklozenge;", "⧫", "blacksquare;", "▪", "blacktriangle;", "▴", "blacktriangledown;", "▾", "blacktriangleleft;", "◂", "blacktriangleright;", "▸", "blank;", "␣", "blk12;", "▒", "blk14;", "░", "blk34;", "▓", "block;", "█", "bne;", "=⃥", "bnequiv;", "≡⃥", "bnot;", "⌐", "bopf;", "\ud835\udd53", "bot;", "⊥", "bottom;", "⊥", "bowtie;", "⋈", "boxDL;", "╗", "boxDR;", "╔", "boxDl;", "╖", "boxDr;", "╓", "boxH;", "═", "boxHD;", "╦", "boxHU;", "╩", "boxHd;", "╤", "boxHu;", "╧", "boxUL;", "╝", "boxUR;", "╚", "boxUl;", "╜", "boxUr;", "╙", "boxV;", "║", "boxVH;", "╬", "boxVL;", "╣", "boxVR;", "╠", "boxVh;", "╫", "boxVl;", "╢", "boxVr;", "╟", "boxbox;", "⧉", "boxdL;", "╕", "boxdR;", "╒", "boxdl;", "┐", "boxdr;", "┌", "boxh;", "─", "boxhD;", "╥", "boxhU;", "╨", "boxhd;", "┬", "boxhu;", "┴", "boxminus;", "⊟", "boxplus;", "⊞", "boxtimes;", "⊠", "boxuL;", "╛", "boxuR;", "╘", "boxul;", "┘", "boxur;", "└", "boxv;", "│", "boxvH;", "╪", "boxvL;", "╡", "boxvR;", "╞", "boxvh;", "┼", "boxvl;", "┤", "boxvr;", "├", "bprime;", "‵", "breve;", "˘", "brvbar", "¦", "brvbar;", "¦", "bscr;", "\ud835\udcb7", "bsemi;", "⁏", "bsim;", "∽", "bsime;", "⋍", "bsol;", "\\", "bsolb;", "⧅", "bsolhsub;", "⟈", "bull;", "•", "bullet;", "•", "bump;", "≎", "bumpE;", "⪮", "bumpe;", "≏", "bumpeq;", "≏", "cacute;", "ć", "cap;", "∩", "capand;", "⩄", "capbrcup;", "⩉", "capcap;", "⩋", "capcup;", "⩇", "capdot;", "⩀", "caps;", "∩︀", "caret;", "⁁", "caron;", "ˇ", "ccaps;", "⩍", "ccaron;", "č", "ccedil", "ç", "ccedil;", "ç", "ccirc;", "ĉ", "ccups;", "⩌", "ccupssm;", "⩐", "cdot;", "ċ", "cedil", "¸", "cedil;", "¸", "cemptyv;", "⦲", "cent", "¢", "cent;", "¢", "centerdot;", "·", "cfr;", "\ud835\udd20", "chcy;", "ч", "check;", "✓", "checkmark;", "✓", "chi;", "χ", "cir;", "○", "cirE;", "⧃", "circ;", "ˆ", "circeq;", "≗", "circlearrowleft;", "↺", "circlearrowright;", "↻", "circledR;", "®", "circledS;", "Ⓢ", "circledast;", "⊛", "circledcirc;", "⊚", "circleddash;", "⊝", "cire;", "≗", "cirfnint;", "⨐", "cirmid;", "⫯", "cirscir;", "⧂", "clubs;", "♣", "clubsuit;", "♣", "colon;", ":", "colone;", "≔", "coloneq;", "≔", "comma;", ",", "commat;", "@", "comp;", "∁", "compfn;", "∘", "complement;", "∁", "complexes;", "ℂ", "cong;", "≅", "congdot;", "⩭", "conint;", "∮", "copf;", "\ud835\udd54", "coprod;", "∐", "copy", "©", "copy;", "©", "copysr;", "℗", "crarr;", "↵", "cross;", "✗", "cscr;", "\ud835\udcb8", "csub;", "⫏", "csube;", "⫑", "csup;", "⫐", "csupe;", "⫒", "ctdot;", "⋯", "cudarrl;", "⤸", "cudarrr;", "⤵", "cuepr;", "⋞", "cuesc;", "⋟", "cularr;", "↶", "cularrp;", "⤽", "cup;", "∪", "cupbrcap;", "⩈", "cupcap;", "⩆", "cupcup;", "⩊", "cupdot;", "⊍", "cupor;", "⩅", "cups;", "∪︀", "curarr;", "↷", "curarrm;", "⤼", "curlyeqprec;", "⋞", "curlyeqsucc;", "⋟", "curlyvee;", "⋎", "curlywedge;", "⋏", "curren", "¤", "curren;", "¤", "curvearrowleft;", "↶", "curvearrowright;", "↷", "cuvee;", "⋎", "cuwed;", "⋏", "cwconint;", "∲", "cwint;", "∱", "cylcty;", "⌭", "dArr;", "⇓", "dHar;", "⥥", "dagger;", "†", "daleth;", "ℸ", "darr;", "↓", "dash;", "‐", "dashv;", "⊣", "dbkarow;", "⤏", "dblac;", "˝", "dcaron;", "ď", "dcy;", "д", "dd;", "ⅆ", "ddagger;", "‡", "ddarr;", "⇊", "ddotseq;", "⩷", "deg", "°", "deg;", "°", "delta;", "δ", "demptyv;", "⦱", "dfisht;", "⥿", "dfr;", "\ud835\udd21", "dharl;", "⇃", "dharr;", "⇂", "diam;", "⋄", "diamond;", "⋄", "diamondsuit;", "♦", "diams;", "♦", "die;", "¨", "digamma;", "ϝ", "disin;", "⋲", "div;", "÷", "divide", "÷", "divide;", "÷", "divideontimes;", "⋇", "divonx;", "⋇", "djcy;", "ђ", "dlcorn;", "⌞", "dlcrop;", "⌍", "dollar;", "\u0024", "dopf;", "\ud835\udd55", "dot;", "˙", "doteq;", "≐", "doteqdot;", "≑", "dotminus;", "∸", "dotplus;", "∔", "dotsquare;", "⊡", "doublebarwedge;", "⌆", "downarrow;", "↓", "downdownarrows;", "⇊", "downharpoonleft;", "⇃", "downharpoonright;", "⇂", "drbkarow;", "⤐", "drcorn;", "⌟", "drcrop;", "⌌", "dscr;", "\ud835\udcb9", "dscy;", "ѕ", "dsol;", "⧶", "dstrok;", "đ", "dtdot;", "⋱", "dtri;", "▿", "dtrif;", "▾", "duarr;", "⇵", "duhar;", "⥯", "dwangle;", "⦦", "dzcy;", "џ", "dzigrarr;", "⟿", "eDDot;", "⩷", "eDot;", "≑", "eacute", "é", "eacute;", "é", "easter;", "⩮", "ecaron;", "ě", "ecir;", "≖", "ecirc", "ê", "ecirc;", "ê", "ecolon;", "≕", "ecy;", "э", "edot;", "ė", "ee;", "ⅇ", "efDot;", "≒", "efr;", "\ud835\udd22", "eg;", "⪚", "egrave", "è", "egrave;", "è", "egs;", "⪖", "egsdot;", "⪘", "el;", "⪙", "elinters;", "⏧", "ell;", "ℓ", "els;", "⪕", "elsdot;", "⪗", "emacr;", "ē", "empty;", "∅", "emptyset;", "∅", "emptyv;", "∅", "emsp13;", " ", "emsp14;", " ", "emsp;", " ", "eng;", "ŋ", "ensp;", " ", "eogon;", "ę", "eopf;", "\ud835\udd56", "epar;", "⋕", "eparsl;", "⧣", "eplus;", "⩱", "epsi;", "ε", "epsilon;", "ε", "epsiv;", "ϵ", "eqcirc;", "≖", "eqcolon;", "≕", "eqsim;", "≂", "eqslantgtr;", "⪖", "eqslantless;", "⪕", "equals;", "=", "equest;", "≟", "equiv;", "≡", "equivDD;", "⩸", "eqvparsl;", "⧥", "erDot;", "≓", "erarr;", "⥱", "escr;", "ℯ", "esdot;", "≐", "esim;", "≂", "eta;", "η", "eth", "ð", "eth;", "ð", "euml", "ë", "euml;", "ë", "euro;", "€", "excl;", "!", "exist;", "∃", "expectation;", "ℰ", "exponentiale;", "ⅇ", "fallingdotseq;", "≒", "fcy;", "ф", "female;", "♀", "ffilig;", "ﬃ", "fflig;", "ﬀ", "ffllig;", "ﬄ", "ffr;", "\ud835\udd23", "filig;", "ﬁ", "fjlig;", "fj", "flat;", "♭", "fllig;", "ﬂ", "fltns;", "▱", "fnof;", "ƒ", "fopf;", "\ud835\udd57", "forall;", "∀", "fork;", "⋔", "forkv;", "⫙", "fpartint;", "⨍", "frac12", "½", "frac12;", "½", "frac13;", "⅓", "frac14", "¼", "frac14;", "¼", "frac15;", "⅕", "frac16;", "⅙", "frac18;", "⅛", "frac23;", "⅔", "frac25;", "⅖", "frac34", "¾", "frac34;", "¾", "frac35;", "⅗", "frac38;", "⅜", "frac45;", "⅘", "frac56;", "⅚", "frac58;", "⅝", "frac78;", "⅞", "frasl;", "⁄", "frown;", "⌢", "fscr;", "\ud835\udcbb", "gE;", "≧", "gEl;", "⪌", "gacute;", "ǵ", "gamma;", "γ", "gammad;", "ϝ", "gap;", "⪆", "gbreve;", "ğ", "gcirc;", "ĝ", "gcy;", "г", "gdot;", "ġ", "ge;", "≥", "gel;", "⋛", "geq;", "≥", "geqq;", "≧", "geqslant;", "⩾", "ges;", "⩾", "gescc;", "⪩", "gesdot;", "⪀", "gesdoto;", "⪂", "gesdotol;", "⪄", "gesl;", "⋛︀", "gesles;", "⪔", "gfr;", "\ud835\udd24", "gg;", "≫", "ggg;", "⋙", "gimel;", "ℷ", "gjcy;", "ѓ", "gl;", "≷", "glE;", "⪒", "gla;", "⪥", "glj;", "⪤", "gnE;", "≩", "gnap;", "⪊", "gnapprox;", "⪊", "gne;", "⪈", "gneq;", "⪈", "gneqq;", "≩", "gnsim;", "⋧", "gopf;", "\ud835\udd58", "grave;", "`", "gscr;", "ℊ", "gsim;", "≳", "gsime;", "⪎", "gsiml;", "⪐", "gt", "\u003e", "gt;", "\u003e", "gtcc;", "⪧", "gtcir;", "⩺", "gtdot;", "⋗", "gtlPar;", "⦕", "gtquest;", "⩼", "gtrapprox;", "⪆", "gtrarr;", "⥸", "gtrdot;", "⋗", "gtreqless;", "⋛", "gtreqqless;", "⪌", "gtrless;", "≷", "gtrsim;", "≳", "gvertneqq;", "≩︀", "gvnE;", "≩︀", "hArr;", "⇔", "hairsp;", " ", "half;", "½", "hamilt;", "ℋ", "hardcy;", "ъ", "harr;", "↔", "harrcir;", "⥈", "harrw;", "↭", "hbar;", "ℏ", "hcirc;", "ĥ", "hearts;", "♥", "heartsuit;", "♥", "hellip;", "…", "hercon;", "⊹", "hfr;", "\ud835\udd25", "hksearow;", "⤥", "hkswarow;", "⤦", "hoarr;", "⇿", "homtht;", "∻", "hookleftarrow;", "↩", "hookrightarrow;", "↪", "hopf;", "\ud835\udd59", "horbar;", "―", "hscr;", "\ud835\udcbd", "hslash;", "ℏ", "hstrok;", "ħ", "hybull;", "⁃", "hyphen;", "‐", "iacute", "í", "iacute;", "í", "ic;", "⁣", "icirc", "î", "icirc;", "î", "icy;", "и", "iecy;", "е", "iexcl", "¡", "iexcl;", "¡", "iff;", "⇔", "ifr;", "\ud835\udd26", "igrave", "ì", "igrave;", "ì", "ii;", "ⅈ", "iiiint;", "⨌", "iiint;", "∭", "iinfin;", "⧜", "iiota;", "℩", "ijlig;", "ĳ", "imacr;", "ī", "image;", "ℑ", "imagline;", "ℐ", "imagpart;", "ℑ", "imath;", "ı", "imof;", "⊷", "imped;", "Ƶ", "in;", "∈", "incare;", "℅", "infin;", "∞", "infintie;", "⧝", "inodot;", "ı", "int;", "∫", "intcal;", "⊺", "integers;", "ℤ", "intercal;", "⊺", "intlarhk;", "⨗", "intprod;", "⨼", "iocy;", "ё", "iogon;", "į", "iopf;", "\ud835\udd5a", "iota;", "ι", "iprod;", "⨼", "iquest", "¿", "iquest;", "¿", "iscr;", "\ud835\udcbe", "isin;", "∈", "isinE;", "⋹", "isindot;", "⋵", "isins;", "⋴", "isinsv;", "⋳", "isinv;", "∈", "it;", "⁢", "itilde;", "ĩ", "iukcy;", "і", "iuml", "ï", "iuml;", "ï", "jcirc;", "ĵ", "jcy;", "й", "jfr;", "\ud835\udd27", "jmath;", "ȷ", "jopf;", "\ud835\udd5b", "jscr;", "\ud835\udcbf", "jsercy;", "ј", "jukcy;", "є", "kappa;", "κ", "kappav;", "ϰ", "kcedil;", "ķ", "kcy;", "к", "kfr;", "\ud835\udd28", "kgreen;", "ĸ", "khcy;", "х", "kjcy;", "ќ", "kopf;", "\ud835\udd5c", "kscr;", "\ud835\udcc0", "lAarr;", "⇚", "lArr;", "⇐", "lAtail;", "⤛", "lBarr;", "⤎", "lE;", "≦", "lEg;", "⪋", "lHar;", "⥢", "lacute;", "ĺ", "laemptyv;", "⦴", "lagran;", "ℒ", "lambda;", "λ", "lang;", "⟨", "langd;", "⦑", "langle;", "⟨", "lap;", "⪅", "laquo", "«", "laquo;", "«", "larr;", "←", "larrb;", "⇤", "larrbfs;", "⤟", "larrfs;", "⤝", "larrhk;", "↩", "larrlp;", "↫", "larrpl;", "⤹", "larrsim;", "⥳", "larrtl;", "↢", "lat;", "⪫", "latail;", "⤙", "late;", "⪭", "lates;", "⪭︀", "lbarr;", "⤌", "lbbrk;", "❲", "lbrace;", "{", "lbrack;", "[", "lbrke;", "⦋", "lbrksld;", "⦏", "lbrkslu;", "⦍", "lcaron;", "ľ", "lcedil;", "ļ", "lceil;", "⌈", "lcub;", "{", "lcy;", "л", "ldca;", "⤶", "ldquo;", "“", "ldquor;", "„", "ldrdhar;", "⥧", "ldrushar;", "⥋", "ldsh;", "↲", "le;", "≤", "leftarrow;", "←", "leftarrowtail;", "↢", "leftharpoondown;", "↽", "leftharpoonup;", "↼", "leftleftarrows;", "⇇", "leftrightarrow;", "↔", "leftrightarrows;", "⇆", "leftrightharpoons;", "⇋", "leftrightsquigarrow;", "↭", "leftthreetimes;", "⋋", "leg;", "⋚", "leq;", "≤", "leqq;", "≦", "leqslant;", "⩽", "les;", "⩽", "lescc;", "⪨", "lesdot;", "⩿", "lesdoto;", "⪁", "lesdotor;", "⪃", "lesg;", "⋚︀", "lesges;", "⪓", "lessapprox;", "⪅", "lessdot;", "⋖", "lesseqgtr;", "⋚", "lesseqqgtr;", "⪋", "lessgtr;", "≶", "lesssim;", "≲", "lfisht;", "⥼", "lfloor;", "⌊", "lfr;", "\ud835\udd29", "lg;", "≶", "lgE;", "⪑", "lhard;", "↽", "lharu;", "↼", "lharul;", "⥪", "lhblk;", "▄", "ljcy;", "љ", "ll;", "≪", "llarr;", "⇇", "llcorner;", "⌞", "llhard;", "⥫", "lltri;", "◺", "lmidot;", "ŀ", "lmoust;", "⎰", "lmoustache;", "⎰", "lnE;", "≨", "lnap;", "⪉", "lnapprox;", "⪉", "lne;", "⪇", "lneq;", "⪇", "lneqq;", "≨", "lnsim;", "⋦", "loang;", "⟬", "loarr;", "⇽", "lobrk;", "⟦", "longleftarrow;", "⟵", "longleftrightarrow;", "⟷", "longmapsto;", "⟼", "longrightarrow;", "⟶", "looparrowleft;", "↫", "looparrowright;", "↬", "lopar;", "⦅", "lopf;", "\ud835\udd5d", "loplus;", "⨭", "lotimes;", "⨴", "lowast;", "∗", "lowbar;", "_", "loz;", "◊", "lozenge;", "◊", "lozf;", "⧫", "lpar;", "(", "lparlt;", "⦓", "lrarr;", "⇆", "lrcorner;", "⌟", "lrhar;", "⇋", "lrhard;", "⥭", "lrm;", "‎", "lrtri;", "⊿", "lsaquo;", "‹", "lscr;", "\ud835\udcc1", "lsh;", "↰", "lsim;", "≲", "lsime;", "⪍", "lsimg;", "⪏", "lsqb;", "[", "lsquo;", "‘", "lsquor;", "‚", "lstrok;", "ł", "lt", "\u003c", "lt;", "\u003c", "ltcc;", "⪦", "ltcir;", "⩹", "ltdot;", "⋖", "lthree;", "⋋", "ltimes;", "⋉", "ltlarr;", "⥶", "ltquest;", "⩻", "ltrPar;", "⦖", "ltri;", "◃", "ltrie;", "⊴", "ltrif;", "◂", "lurdshar;", "⥊", "luruhar;", "⥦", "lvertneqq;", "≨︀", "lvnE;", "≨︀", "mDDot;", "∺", "macr", "¯", "macr;", "¯", "male;", "♂", "malt;", "✠", "maltese;", "✠", "map;", "↦", "mapsto;", "↦", "mapstodown;", "↧", "mapstoleft;", "↤", "mapstoup;", "↥", "marker;", "▮", "mcomma;", "⨩", "mcy;", "м", "mdash;", "—", "measuredangle;", "∡", "mfr;", "\ud835\udd2a", "mho;", "℧", "micro", "µ", "micro;", "µ", "mid;", "∣", "midast;", "*", "midcir;", "⫰", "middot", "·", "middot;", "·", "minus;", "−", "minusb;", "⊟", "minusd;", "∸", "minusdu;", "⨪", "mlcp;", "⫛", "mldr;", "…", "mnplus;", "∓", "models;", "⊧", "mopf;", "\ud835\udd5e", "mp;", "∓", "mscr;", "\ud835\udcc2", "mstpos;", "∾", "mu;", "μ", "multimap;", "⊸", "mumap;", "⊸", "nGg;", "⋙̸", "nGt;", "≫⃒", "nGtv;", "≫̸", "nLeftarrow;", "⇍", "nLeftrightarrow;", "⇎", "nLl;", "⋘̸", "nLt;", "≪⃒", "nLtv;", "≪̸", "nRightarrow;", "⇏", "nVDash;", "⊯", "nVdash;", "⊮", "nabla;", "∇", "nacute;", "ń", "nang;", "∠⃒", "nap;", "≉", "napE;", "⩰̸", "napid;", "≋̸", "napos;", "ŉ", "napprox;", "≉", "natur;", "♮", "natural;", "♮", "naturals;", "ℕ", "nbsp", " ", "nbsp;", " ", "nbump;", "≎̸", "nbumpe;", "≏̸", "ncap;", "⩃", "ncaron;", "ň", "ncedil;", "ņ", "ncong;", "≇", "ncongdot;", "⩭̸", "ncup;", "⩂", "ncy;", "н", "ndash;", "–", "ne;", "≠", "neArr;", "⇗", "nearhk;", "⤤", "nearr;", "↗", "nearrow;", "↗", "nedot;", "≐̸", "nequiv;", "≢", "nesear;", "⤨", "nesim;", "≂̸", "nexist;", "∄", "nexists;", "∄", "nfr;", "\ud835\udd2b", "ngE;", "≧̸", "nge;", "≱", "ngeq;", "≱", "ngeqq;", "≧̸", "ngeqslant;", "⩾̸", "nges;", "⩾̸", "ngsim;", "≵", "ngt;", "≯", "ngtr;", "≯", "nhArr;", "⇎", "nharr;", "↮", "nhpar;", "⫲", "ni;", "∋", "nis;", "⋼", "nisd;", "⋺", "niv;", "∋", "njcy;", "њ", "nlArr;", "⇍", "nlE;", "≦̸", "nlarr;", "↚", "nldr;", "‥", "nle;", "≰", "nleftarrow;", "↚", "nleftrightarrow;", "↮", "nleq;", "≰", "nleqq;", "≦̸", "nleqslant;", "⩽̸", "nles;", "⩽̸", "nless;", "≮", "nlsim;", "≴", "nlt;", "≮", "nltri;", "⋪", "nltrie;", "⋬", "nmid;", "∤", "nopf;", "\ud835\udd5f", "not", "¬", "not;", "¬", "notin;", "∉", "notinE;", "⋹̸", "notindot;", "⋵̸", "notinva;", "∉", "notinvb;", "⋷", "notinvc;", "⋶", "notni;", "∌", "notniva;", "∌", "notnivb;", "⋾", "notnivc;", "⋽", "npar;", "∦", "nparallel;", "∦", "nparsl;", "⫽⃥", "npart;", "∂̸", "npolint;", "⨔", "npr;", "⊀", "nprcue;", "⋠", "npre;", "⪯̸", "nprec;", "⊀", "npreceq;", "⪯̸", "nrArr;", "⇏", "nrarr;", "↛", "nrarrc;", "⤳̸", "nrarrw;", "↝̸", "nrightarrow;", "↛", "nrtri;", "⋫", "nrtrie;", "⋭", "nsc;", "⊁", "nsccue;", "⋡", "nsce;", "⪰̸", "nscr;", "\ud835\udcc3", "nshortmid;", "∤", "nshortparallel;", "∦", "nsim;", "≁", "nsime;", "≄", "nsimeq;", "≄", "nsmid;", "∤", "nspar;", "∦", "nsqsube;", "⋢", "nsqsupe;", "⋣", "nsub;", "⊄", "nsubE;", "⫅̸", "nsube;", "⊈", "nsubset;", "⊂⃒", "nsubseteq;", "⊈", "nsubseteqq;", "⫅̸", "nsucc;", "⊁", "nsucceq;", "⪰̸", "nsup;", "⊅", "nsupE;", "⫆̸", "nsupe;", "⊉", "nsupset;", "⊃⃒", "nsupseteq;", "⊉", "nsupseteqq;", "⫆̸", "ntgl;", "≹", "ntilde", "ñ", "ntilde;", "ñ", "ntlg;", "≸", "ntriangleleft;", "⋪", "ntrianglelefteq;", "⋬", "ntriangleright;", "⋫", "ntrianglerighteq;", "⋭", "nu;", "ν", "num;", "#", "numero;", "№", "numsp;", " ", "nvDash;", "⊭", "nvHarr;", "⤄", "nvap;", "≍⃒", "nvdash;", "⊬", "nvge;", "≥⃒", "nvgt;", "\u003e⃒", "nvinfin;", "⧞", "nvlArr;", "⤂", "nvle;", "≤⃒", "nvlt;", "\u003c⃒", "nvltrie;", "⊴⃒", "nvrArr;", "⤃", "nvrtrie;", "⊵⃒", "nvsim;", "∼⃒", "nwArr;", "⇖", "nwarhk;", "⤣", "nwarr;", "↖", "nwarrow;", "↖", "nwnear;", "⤧", "oS;", "Ⓢ", "oacute", "ó", "oacute;", "ó", "oast;", "⊛", "ocir;", "⊚", "ocirc", "ô", "ocirc;", "ô", "ocy;", "о", "odash;", "⊝", "odblac;", "ő", "odiv;", "⨸", "odot;", "⊙", "odsold;", "⦼", "oelig;", "œ", "ofcir;", "⦿", "ofr;", "\ud835\udd2c", "ogon;", "˛", "ograve", "ò", "ograve;", "ò", "ogt;", "⧁", "ohbar;", "⦵", "ohm;", "Ω", "oint;", "∮", "olarr;", "↺", "olcir;", "⦾", "olcross;", "⦻", "oline;", "‾", "olt;", "⧀", "omacr;", "ō", "omega;", "ω", "omicron;", "ο", "omid;", "⦶", "ominus;", "⊖", "oopf;", "\ud835\udd60", "opar;", "⦷", "operp;", "⦹", "oplus;", "⊕", "or;", "∨", "orarr;", "↻", "ord;", "⩝", "order;", "ℴ", "orderof;", "ℴ", "ordf", "ª", "ordf;", "ª", "ordm", "º", "ordm;", "º", "origof;", "⊶", "oror;", "⩖", "orslope;", "⩗", "orv;", "⩛", "oscr;", "ℴ", "oslash", "ø", "oslash;", "ø", "osol;", "⊘", "otilde", "õ", "otilde;", "õ", "otimes;", "⊗", "otimesas;", "⨶", "ouml", "ö", "ouml;", "ö", "ovbar;", "⌽", "par;", "∥", "para", "¶", "para;", "¶", "parallel;", "∥", "parsim;", "⫳", "parsl;", "⫽", "part;", "∂", "pcy;", "п", "percnt;", "%", "period;", ".", "permil;", "‰", "perp;", "⊥", "pertenk;", "‱", "pfr;", "\ud835\udd2d", "phi;", "φ", "phiv;", "ϕ", "phmmat;", "ℳ", "phone;", "☎", "pi;", "π", "pitchfork;", "⋔", "piv;", "ϖ", "planck;", "ℏ", "planckh;", "ℎ", "plankv;", "ℏ", "plus;", "+", "plusacir;", "⨣", "plusb;", "⊞", "pluscir;", "⨢", "plusdo;", "∔", "plusdu;", "⨥", "pluse;", "⩲", "plusmn", "±", "plusmn;", "±", "plussim;", "⨦", "plustwo;", "⨧", "pm;", "±", "pointint;", "⨕", "popf;", "\ud835\udd61", "pound", "£", "pound;", "£", "pr;", "≺", "prE;", "⪳", "prap;", "⪷", "prcue;", "≼", "pre;", "⪯", "prec;", "≺", "precapprox;", "⪷", "preccurlyeq;", "≼", "preceq;", "⪯", "precnapprox;", "⪹", "precneqq;", "⪵", "precnsim;", "⋨", "precsim;", "≾", "prime;", "′", "primes;", "ℙ", "prnE;", "⪵", "prnap;", "⪹", "prnsim;", "⋨", "prod;", "∏", "profalar;", "⌮", "profline;", "⌒", "profsurf;", "⌓", "prop;", "∝", "propto;", "∝", "prsim;", "≾", "prurel;", "⊰", "pscr;", "\ud835\udcc5", "psi;", "ψ", "puncsp;", " ", "qfr;", "\ud835\udd2e", "qint;", "⨌", "qopf;", "\ud835\udd62", "qprime;", "⁗", "qscr;", "\ud835\udcc6", "quaternions;", "ℍ", "quatint;", "⨖", "quest;", "?", "questeq;", "≟", "quot", "\u0022", "quot;", "\u0022", "rAarr;", "⇛", "rArr;", "⇒", "rAtail;", "⤜", "rBarr;", "⤏", "rHar;", "⥤", "race;", "∽̱", "racute;", "ŕ", "radic;", "√", "raemptyv;", "⦳", "rang;", "⟩", "rangd;", "⦒", "range;", "⦥", "rangle;", "⟩", "raquo", "»", "raquo;", "»", "rarr;", "→", "rarrap;", "⥵", "rarrb;", "⇥", "rarrbfs;", "⤠", "rarrc;", "⤳", "rarrfs;", "⤞", "rarrhk;", "↪", "rarrlp;", "↬", "rarrpl;", "⥅", "rarrsim;", "⥴", "rarrtl;", "↣", "rarrw;", "↝", "ratail;", "⤚", "ratio;", "∶", "rationals;", "ℚ", "rbarr;", "⤍", "rbbrk;", "❳", "rbrace;", "}", "rbrack;", "]", "rbrke;", "⦌", "rbrksld;", "⦎", "rbrkslu;", "⦐", "rcaron;", "ř", "rcedil;", "ŗ", "rceil;", "⌉", "rcub;", "}", "rcy;", "р", "rdca;", "⤷", "rdldhar;", "⥩", "rdquo;", "”", "rdquor;", "”", "rdsh;", "↳", "real;", "ℜ", "realine;", "ℛ", "realpart;", "ℜ", "reals;", "ℝ", "rect;", "▭", "reg", "®", "reg;", "®", "rfisht;", "⥽", "rfloor;", "⌋", "rfr;", "\ud835\udd2f", "rhard;", "⇁", "rharu;", "⇀", "rharul;", "⥬", "rho;", "ρ", "rhov;", "ϱ", "rightarrow;", "→", "rightarrowtail;", "↣", "rightharpoondown;", "⇁", "rightharpoonup;", "⇀", "rightleftarrows;", "⇄", "rightleftharpoons;", "⇌", "rightrightarrows;", "⇉", "rightsquigarrow;", "↝", "rightthreetimes;", "⋌", "ring;", "˚", "risingdotseq;", "≓", "rlarr;", "⇄", "rlhar;", "⇌", "rlm;", "‏", "rmoust;", "⎱", "rmoustache;", "⎱", "rnmid;", "⫮", "roang;", "⟭", "roarr;", "⇾", "robrk;", "⟧", "ropar;", "⦆", "ropf;", "\ud835\udd63", "roplus;", "⨮", "rotimes;", "⨵", "rpar;", ")", "rpargt;", "⦔", "rppolint;", "⨒", "rrarr;", "⇉", "rsaquo;", "›", "rscr;", "\ud835\udcc7", "rsh;", "↱", "rsqb;", "]", "rsquo;", "’", "rsquor;", "’", "rthree;", "⋌", "rtimes;", "⋊", "rtri;", "▹", "rtrie;", "⊵", "rtrif;", "▸", "rtriltri;", "⧎", "ruluhar;", "⥨", "rx;", "℞", "sacute;", "ś", "sbquo;", "‚", "sc;", "≻", "scE;", "⪴", "scap;", "⪸", "scaron;", "š", "sccue;", "≽", "sce;", "⪰", "scedil;", "ş", "scirc;", "ŝ", "scnE;", "⪶", "scnap;", "⪺", "scnsim;", "⋩", "scpolint;", "⨓", "scsim;", "≿", "scy;", "с", "sdot;", "⋅", "sdotb;", "⊡", "sdote;", "⩦", "seArr;", "⇘", "searhk;", "⤥", "searr;", "↘", "searrow;", "↘", "sect", "§", "sect;", "§", "semi;", ";", "seswar;", "⤩", "setminus;", "∖", "setmn;", "∖", "sext;", "✶", "sfr;", "\ud835\udd30", "sfrown;", "⌢", "sharp;", "♯", "shchcy;", "щ", "shcy;", "ш", "shortmid;", "∣", "shortparallel;", "∥", "shy", "­", "shy;", "­", "sigma;", "σ", "sigmaf;", "ς", "sigmav;", "ς", "sim;", "∼", "simdot;", "⩪", "sime;", "≃", "simeq;", "≃", "simg;", "⪞", "simgE;", "⪠", "siml;", "⪝", "simlE;", "⪟", "simne;", "≆", "simplus;", "⨤", "simrarr;", "⥲", "slarr;", "←", "smallsetminus;", "∖", "smashp;", "⨳", "smeparsl;", "⧤", "smid;", "∣", "smile;", "⌣", "smt;", "⪪", "smte;", "⪬", "smtes;", "⪬︀", "softcy;", "ь", "sol;", "/", "solb;", "⧄", "solbar;", "⌿", "sopf;", "\ud835\udd64", "spades;", "♠", "spadesuit;", "♠", "spar;", "∥", "sqcap;", "⊓", "sqcaps;", "⊓︀", "sqcup;", "⊔", "sqcups;", "⊔︀", "sqsub;", "⊏", "sqsube;", "⊑", "sqsubset;", "⊏", "sqsubseteq;", "⊑", "sqsup;", "⊐", "sqsupe;", "⊒", "sqsupset;", "⊐", "sqsupseteq;", "⊒", "squ;", "□", "square;", "□", "squarf;", "▪", "squf;", "▪", "srarr;", "→", "sscr;", "\ud835\udcc8", "ssetmn;", "∖", "ssmile;", "⌣", "sstarf;", "⋆", "star;", "☆", "starf;", "★", "straightepsilon;", "ϵ", "straightphi;", "ϕ", "strns;", "¯", "sub;", "⊂", "subE;", "⫅", "subdot;", "⪽", "sube;", "⊆", "subedot;", "⫃", "submult;", "⫁", "subnE;", "⫋", "subne;", "⊊", "subplus;", "⪿", "subrarr;", "⥹", "subset;", "⊂", "subseteq;", "⊆", "subseteqq;", "⫅", "subsetneq;", "⊊", "subsetneqq;", "⫋", "subsim;", "⫇", "subsub;", "⫕", "subsup;", "⫓", "succ;", "≻", "succapprox;", "⪸", "succcurlyeq;", "≽", "succeq;", "⪰", "succnapprox;", "⪺", "succneqq;", "⪶", "succnsim;", "⋩", "succsim;", "≿", "sum;", "∑", "sung;", "♪", "sup1", "¹", "sup1;", "¹", "sup2", "²", "sup2;", "²", "sup3", "³", "sup3;", "³", "sup;", "⊃", "supE;", "⫆", "supdot;", "⪾", "supdsub;", "⫘", "supe;", "⊇", "supedot;", "⫄", "suphsol;", "⟉", "suphsub;", "⫗", "suplarr;", "⥻", "supmult;", "⫂", "supnE;", "⫌", "supne;", "⊋", "supplus;", "⫀", "supset;", "⊃", "supseteq;", "⊇", "supseteqq;", "⫆", "supsetneq;", "⊋", "supsetneqq;", "⫌", "supsim;", "⫈", "supsub;", "⫔", "supsup;", "⫖", "swArr;", "⇙", "swarhk;", "⤦", "swarr;", "↙", "swarrow;", "↙", "swnwar;", "⤪", "szlig", "ß", "szlig;", "ß", "target;", "⌖", "tau;", "τ", "tbrk;", "⎴", "tcaron;", "ť", "tcedil;", "ţ", "tcy;", "т", "tdot;", "⃛", "telrec;", "⌕", "tfr;", "\ud835\udd31", "there4;", "∴", "therefore;", "∴", "theta;", "θ", "thetasym;", "ϑ", "thetav;", "ϑ", "thickapprox;", "≈", "thicksim;", "∼", "thinsp;", " ", "thkap;", "≈", "thksim;", "∼", "thorn", "þ", "thorn;", "þ", "tilde;", "˜", "times", "×", "times;", "×", "timesb;", "⊠", "timesbar;", "⨱", "timesd;", "⨰", "tint;", "∭", "toea;", "⤨", "top;", "⊤", "topbot;", "⌶", "topcir;", "⫱", "topf;", "\ud835\udd65", "topfork;", "⫚", "tosa;", "⤩", "tprime;", "‴", "trade;", "™", "triangle;", "▵", "triangledown;", "▿", "triangleleft;", "◃", "trianglelefteq;", "⊴", "triangleq;", "≜", "triangleright;", "▹", "trianglerighteq;", "⊵", "tridot;", "◬", "trie;", "≜", "triminus;", "⨺", "triplus;", "⨹", "trisb;", "⧍", "tritime;", "⨻", "trpezium;", "⏢", "tscr;", "\ud835\udcc9", "tscy;", "ц", "tshcy;", "ћ", "tstrok;", "ŧ", "twixt;", "≬", "twoheadleftarrow;", "↞", "twoheadrightarrow;", "↠", "uArr;", "⇑", "uHar;", "⥣", "uacute", "ú", "uacute;", "ú", "uarr;", "↑", "ubrcy;", "ў", "ubreve;", "ŭ", "ucirc", "û", "ucirc;", "û", "ucy;", "у", "udarr;", "⇅", "udblac;", "ű", "udhar;", "⥮", "ufisht;", "⥾", "ufr;", "\ud835\udd32", "ugrave", "ù", "ugrave;", "ù", "uharl;", "↿", "uharr;", "↾", "uhblk;", "▀", "ulcorn;", "⌜", "ulcorner;", "⌜", "ulcrop;", "⌏", "ultri;", "◸", "umacr;", "ū", "uml", "¨", "uml;", "¨", "uogon;", "ų", "uopf;", "\ud835\udd66", "uparrow;", "↑", "updownarrow;", "↕", "upharpoonleft;", "↿", "upharpoonright;", "↾", "uplus;", "⊎", "upsi;", "υ", "upsih;", "ϒ", "upsilon;", "υ", "upuparrows;", "⇈", "urcorn;", "⌝", "urcorner;", "⌝", "urcrop;", "⌎", "uring;", "ů", "urtri;", "◹", "uscr;", "\ud835\udcca", "utdot;", "⋰", "utilde;", "ũ", "utri;", "▵", "utrif;", "▴", "uuarr;", "⇈", "uuml", "ü", "uuml;", "ü", "uwangle;", "⦧", "vArr;", "⇕", "vBar;", "⫨", "vBarv;", "⫩", "vDash;", "⊨", "vangrt;", "⦜", "varepsilon;", "ϵ", "varkappa;", "ϰ", "varnothing;", "∅", "varphi;", "ϕ", "varpi;", "ϖ", "varpropto;", "∝", "varr;", "↕", "varrho;", "ϱ", "varsigma;", "ς", "varsubsetneq;", "⊊︀", "varsubsetneqq;", "⫋︀", "varsupsetneq;", "⊋︀", "varsupsetneqq;", "⫌︀", "vartheta;", "ϑ", "vartriangleleft;", "⊲", "vartriangleright;", "⊳", "vcy;", "в", "vdash;", "⊢", "vee;", "∨", "veebar;", "⊻", "veeeq;", "≚", "vellip;", "⋮", "verbar;", "|", "vert;", "|", "vfr;", "\ud835\udd33", "vltri;", "⊲", "vnsub;", "⊂⃒", "vnsup;", "⊃⃒", "vopf;", "\ud835\udd67", "vprop;", "∝", "vrtri;", "⊳", "vscr;", "\ud835\udccb", "vsubnE;", "⫋︀", "vsubne;", "⊊︀", "vsupnE;", "⫌︀", "vsupne;", "⊋︀", "vzigzag;", "⦚", "wcirc;", "ŵ", "wedbar;", "⩟", "wedge;", "∧", "wedgeq;", "≙", "weierp;", "℘", "wfr;", "\ud835\udd34", "wopf;", "\ud835\udd68", "wp;", "℘", "wr;", "≀", "wreath;", "≀", "wscr;", "\ud835\udccc", "xcap;", "⋂", "xcirc;", "◯", "xcup;", "⋃", "xdtri;", "▽", "xfr;", "\ud835\udd35", "xhArr;", "⟺", "xharr;", "⟷", "xi;", "ξ", "xlArr;", "⟸", "xlarr;", "⟵", "xmap;", "⟼", "xnis;", "⋻", "xodot;", "⨀", "xopf;", "\ud835\udd69", "xoplus;", "⨁", "xotime;", "⨂", "xrArr;", "⟹", "xrarr;", "⟶", "xscr;", "\ud835\udccd", "xsqcup;", "⨆", "xuplus;", "⨄", "xutri;", "△", "xvee;", "⋁", "xwedge;", "⋀", "yacute", "ý", "yacute;", "ý", "yacy;", "я", "ycirc;", "ŷ", "ycy;", "ы", "yen", "¥", "yen;", "¥", "yfr;", "\ud835\udd36", "yicy;", "ї", "yopf;", "\ud835\udd6a", "yscr;", "\ud835\udcce", "yucy;", "ю", "yuml", "ÿ", "yuml;", "ÿ", "zacute;", "ź", "zcaron;", "ž", "zcy;", "з", "zdot;", "ż", "zeetrf;", "ℨ", "zeta;", "ζ", "zfr;", "\ud835\udd37", "zhcy;", "ж", "zigrarr;", "⇝", "zopf;", "\ud835\udd6b", "zscr;", "\ud835\udccf", "zwj;", "‍", "zwnj;", "‌"]);
/** @type {Map<string, string>} */
const mb_704 = mapBuilderConstructor_705();
/** @type {number} */
let i_706 = 0;
/** @type {number} */
let n_707 = strs_703.length;
while (i_706 < n_707) {
  mapBuilderSet_708(mb_704, listedGet_675(strs_703, i_706), listedGet_675(strs_703, i_706 + 1 | 0));
  i_706 = i_706 + 2 | 0;
}
/** @type {Map<string, string>} */
const return_709 = mappedToMap_710(mb_704);
/** @type {Map<string, string>} */
const htmlNamedCharacters_438 = return_709;
/** @type {HtmlCodec} */
const return_711 = new HtmlCodec();
/** @type {Codec_439} */
export const htmlCodec = return_711;
/**
 * @param {number} x_760
 * @returns {string}
 */
function htmlStateStr_457(x_760) {
  let return_761;
  if (x_760 === 0) {
    return_761 = "Pcdata";
  } else if (x_760 === 1) {
    return_761 = "OName";
  } else if (x_760 === 2) {
    return_761 = "CName";
  } else if (x_760 === 3) {
    return_761 = "BeforeAttr";
  } else if (x_760 === 4) {
    return_761 = "BeforeEq";
  } else if (x_760 === 5) {
    return_761 = "BeforeValue";
  } else if (x_760 === 6) {
    return_761 = "Attr";
  } else if (x_760 === 7) {
    return_761 = "AfterAttr";
  } else if (x_760 === 8) {
    return_761 = "SpecialBody";
  } else {
    return_761 = x_760.toString();
  }
  return return_761;
}
/**
 * @param {number} x_762
 * @returns {string}
 */
function tagStateStr_458(x_762) {
  return x_762.toString();
}
/**
 * @param {number} x_763
 * @returns {string}
 */
function attribStateStr_459(x_763) {
  let return_764;
  if (x_763 === 0) {
    return_764 = "Generic";
  } else if (x_763 === 1) {
    return_764 = "Css";
  } else if (x_763 === 2) {
    return_764 = "Js";
  } else if (x_763 === 3) {
    return_764 = "Url";
  } else if (x_763 === 4) {
    return_764 = "Urls";
  } else {
    return_764 = x_763.toString();
  }
  return return_764;
}
/**
 * @param {number} x_765
 * @returns {string}
 */
function delimStateStr_460(x_765) {
  let return_766;
  if (x_765 === 0) {
    return_766 = "Uq";
  } else if (x_765 === 1) {
    return_766 = "Sq";
  } else if (x_765 === 2) {
    return_766 = "Dq";
  } else {
    return_766 = x_765.toString();
  }
  return return_766;
}
/**
 * @param {number} x_767
 * @returns {string}
 */
function urlStateStr_472(x_767) {
  let return_768;
  if (x_767 === 0) {
    return_768 = "Start";
  } else if (x_767 === 1) {
    return_768 = "BeforeQuery";
  } else if (x_767 === 2) {
    return_768 = "Query";
  } else if (x_767 === 3) {
    return_768 = "Fragment";
  } else {
    return_768 = x_767.toString();
  }
  return return_768;
}
/**
 * @param {AutoescState_445<UrlEscaperContext>} before_769
 * @param {string | null} literalPart_770
 * @returns {AfterPropagate_446<UrlEscaperContext>}
 */
function urlPropagateContext_451(before_769, literalPart_770) {
  let return_771;
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
  let t_805;
  let t_806;
  fn_807: {
    const contextBefore_808 = before_769.context;
    t_772 = new CodeSet_809(Object.freeze([new CodePoints_810("#")]), false);
    const pattern0_811 = new Sequence_812(Object.freeze([Begin_365, t_772])).compiled();
    t_773 = new CodeSet_809(Object.freeze([new CodePoints_810("?")]), false);
    const pattern1_813 = new Sequence_812(Object.freeze([Begin_365, t_773])).compiled();
    t_774 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("#")]), true), 1, null, false);
    const pattern2_815 = new Sequence_812(Object.freeze([Begin_365, t_774])).compiled();
    t_775 = new Or_816(Object.freeze([new Sequence_812(Object.freeze([new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810(":"), new CodePoints_810("#"), new CodePoints_810("?")]), true), 0, null, false), new CodePoints_810(":")])), new CodeSet_809(Object.freeze([new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" "), new CodePoints_810(":"), new CodePoints_810("#"), new CodePoints_810("?")]), true)]));
    const pattern3_817 = new Sequence_812(Object.freeze([Begin_365, t_775])).compiled();
    t_776 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("?"), new CodePoints_810("#")]), true), 1, null, false);
    const pattern4_818 = new Sequence_812(Object.freeze([Begin_365, t_776])).compiled();
    if (!(literalPart_770 == null)) {
      const literalPart_819 = literalPart_770;
      if (contextBefore_808.urlState === 0) {
        let match_820;
        try {
          t_799 = pattern3_817.find(literalPart_819);
          match_820 = t_799;
        } catch {
          match_820 = null;
        }
        if (!(match_820 == null)) {
          const match_821 = match_820;
          t_777 = match_821.full.value;
          t_778 = match_821.full.end;
          t_779 = new AutoescState_445(new UrlEscaperContext(1), before_769.subsidiary);
          return_771 = new AfterPropagate_446(t_777, t_778, t_779);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 0) {
        let match_822;
        try {
          t_800 = pattern1_813.find(literalPart_819);
          match_822 = t_800;
        } catch {
          match_822 = null;
        }
        if (!(match_822 == null)) {
          const match_823 = match_822;
          t_780 = match_823.full.value;
          t_781 = match_823.full.end;
          t_782 = new AutoescState_445(new UrlEscaperContext(2), before_769.subsidiary);
          return_771 = new AfterPropagate_446(t_780, t_781, t_782);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 0) {
        let match_824;
        try {
          t_801 = pattern0_811.find(literalPart_819);
          match_824 = t_801;
        } catch {
          match_824 = null;
        }
        if (!(match_824 == null)) {
          const match_825 = match_824;
          t_783 = match_825.full.value;
          t_784 = match_825.full.end;
          t_785 = new AutoescState_445(new UrlEscaperContext(3), before_769.subsidiary);
          return_771 = new AfterPropagate_446(t_783, t_784, t_785);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 1) {
        let match_826;
        try {
          t_802 = pattern4_818.find(literalPart_819);
          match_826 = t_802;
        } catch {
          match_826 = null;
        }
        if (!(match_826 == null)) {
          const match_827 = match_826;
          t_786 = match_827.full.value;
          t_787 = match_827.full.end;
          return_771 = new AfterPropagate_446(t_786, t_787, before_769);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 1) {
        let match_828;
        try {
          t_803 = pattern1_813.find(literalPart_819);
          match_828 = t_803;
        } catch {
          match_828 = null;
        }
        if (!(match_828 == null)) {
          const match_829 = match_828;
          t_788 = match_829.full.value;
          t_789 = match_829.full.end;
          t_790 = new AutoescState_445(new UrlEscaperContext(2), before_769.subsidiary);
          return_771 = new AfterPropagate_446(t_788, t_789, t_790);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 1) {
        let match_830;
        try {
          t_804 = pattern0_811.find(literalPart_819);
          match_830 = t_804;
        } catch {
          match_830 = null;
        }
        if (!(match_830 == null)) {
          const match_831 = match_830;
          t_791 = match_831.full.value;
          t_792 = match_831.full.end;
          t_793 = new AutoescState_445(new UrlEscaperContext(3), before_769.subsidiary);
          return_771 = new AfterPropagate_446(t_791, t_792, t_793);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 2) {
        let match_832;
        try {
          t_805 = pattern2_815.find(literalPart_819);
          match_832 = t_805;
        } catch {
          match_832 = null;
        }
        if (!(match_832 == null)) {
          const match_833 = match_832;
          t_794 = match_833.full.value;
          t_795 = match_833.full.end;
          return_771 = new AfterPropagate_446(t_794, t_795, before_769);
          break fn_807;
        }
      }
      if (contextBefore_808.urlState === 2) {
        let match_834;
        try {
          t_806 = pattern0_811.find(literalPart_819);
          match_834 = t_806;
        } catch {
          match_834 = null;
        }
        if (!(match_834 == null)) {
          const match_835 = match_834;
          t_796 = match_835.full.value;
          t_797 = match_835.full.end;
          t_798 = new AutoescState_445(new UrlEscaperContext(3), before_769.subsidiary);
          return_771 = new AfterPropagate_446(t_796, t_797, t_798);
          break fn_807;
        }
      }
    }
    if (literalPart_770 == null) {
      return_771 = new AfterPropagate_446("", 0, before_769);
      break fn_807;
    }
    return_771 = panic_607();
  }
  return return_771;
}
/** @type {UrlContextPropagator} */
const urlContextPropagator_598 = new UrlContextPropagator();
/** @type {Regex_836} */
const protocolAllowList_656 = new Sequence_812(Object.freeze([Begin_365, new Or_816(Object.freeze([new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("H"), new CodePoints_810("h")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("T"), new CodePoints_810("t")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("T"), new CodePoints_810("t")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("P"), new CodePoints_810("p")]), false), new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("S"), new CodePoints_810("s")]), false), 0, 1, false)])), new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("M"), new CodePoints_810("m")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("A"), new CodePoints_810("a")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("I"), new CodePoints_810("i")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("L"), new CodePoints_810("l")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("T"), new CodePoints_810("t")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("O"), new CodePoints_810("o")]), false)]))])), End_367])).compiled();
/** @type {SafeUrl} */
const fallbackSafeUrl_657 = new SafeUrl("about:zz_Temper_zz#");
/** @type {Array<boolean>} */
const lb_837 = [];
/** @type {number} */
let i_838 = 0;
while (i_838 < 128) {
  if (i_838 === 47) {
    t_378 = true;
  } else {
    if (i_838 === 46) {
      t_377 = true;
    } else {
      if (i_838 === 45) {
        t_376 = true;
      } else {
        if (i_838 === 95) {
          t_375 = true;
        } else {
          if (48 <= i_838) {
            t_372 = i_838 <= 57;
          } else {
            t_372 = false;
          }
          if (t_372) {
            t_374 = true;
          } else {
            if (97 <=(i_838 | 32)) {
              t_373 = (i_838 | 32) <= 122;
            } else {
              t_373 = false;
            }
            t_374 = t_373;
          }
          t_375 = t_374;
        }
        t_376 = t_375;
      }
      t_377 = t_376;
    }
    t_378 = t_377;
  }
  listBuilderAdd_839(lb_837, t_378);
  i_838 = i_838 + 1 | 0;
}
/** @type {Array<boolean>} */
const urlQuerySafe_692 = listBuilderToList_840(lb_837);
/** @type {Array<boolean>} */
const lb_841 = [];
/** @type {number} */
let i_842 = 0;
while (i_842 < 128) {
  if (listedGet_675(urlQuerySafe_692, i_842)) {
    t_382 = true;
  } else {
    if (i_842 === 58) {
      t_381 = true;
    } else {
      if (i_842 === 63) {
        t_380 = true;
      } else {
        if (i_842 === 35) {
          t_379 = true;
        } else {
          t_379 = i_842 === 38;
        }
        t_380 = t_379;
      }
      t_381 = t_380;
    }
    t_382 = t_381;
  }
  listBuilderAdd_839(lb_841, t_382);
  i_842 = i_842 + 1 | 0;
}
/** @type {Array<boolean>} */
const urlSafe_674 = listBuilderToList_840(lb_841);
/** @type {HtmlUrlPartUrlEscaper} */
const return_843 = new HtmlUrlPartUrlEscaper();
/** @type {HtmlUrlPartUrlEscaper} */
const htmlUrlPartUrlEscaper_605 = return_843;
/** @type {HtmlProtocolFilteringUrlEscaper} */
const return_844 = new HtmlProtocolFilteringUrlEscaper();
/** @type {HtmlProtocolFilteringUrlEscaper} */
const htmlProtocolFilteringUrlEscaper_604 = return_844;
/** @type {HtmlAsIfQueryUrlEscaper} */
const return_845 = new HtmlAsIfQueryUrlEscaper();
/** @type {HtmlAsIfQueryUrlEscaper} */
const htmlAsIfQueryUrlEscaper_606 = return_845;
/**
 * @param {AutoescState_445<HtmlEscaperContext>} before_846
 * @param {string | null} literalPart_847
 * @returns {AfterPropagate_446<HtmlEscaperContext>}
 */
function htmlPropagateContext_444(before_846, literalPart_847) {
  let return_848;
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
  let t_1025;
  let t_1026;
  fn_1027: {
    const contextBefore_1028 = before_846.context;
    t_849 = new CodePoints_810("\u0022");
    const pattern0_1029 = new Sequence_812(Object.freeze([Begin_365, t_849])).compiled();
    t_850 = new Sequence_812(Object.freeze([new CodePoints_810("\u0022"), new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("\u0022")]), true), 0, null, false), new Repeat_814(new CodePoints_810("\u0022"), 0, 1, false)]));
    const pattern1_1030 = new Sequence_812(Object.freeze([Begin_365, t_850])).compiled();
    t_851 = new CodePoints_810("'");
    const pattern2_1031 = new Sequence_812(Object.freeze([Begin_365, t_851])).compiled();
    t_852 = new Sequence_812(Object.freeze([new CodePoints_810("'"), new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("'")]), true), 0, null, false), new Repeat_814(new CodePoints_810("'"), 0, 1, false)]));
    const pattern3_1032 = new Sequence_812(Object.freeze([Begin_365, t_852])).compiled();
    t_853 = new CodePoints_810("\u003e");
    const pattern4_1033 = new Sequence_812(Object.freeze([Begin_365, t_853])).compiled();
    t_854 = new CodeSet_809(Object.freeze([new CodePoints_810("\u003e"), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" ")]), false);
    const pattern5_1034 = new Sequence_812(Object.freeze([Begin_365, t_854])).compiled();
    t_855 = new CodeSet_809(Object.freeze([new CodeRange_1035(65, 90), new CodeRange_1035(97, 122), new CodeRange_1035(48, 57), new CodeRange_1035(58, 58), new CodePoints_810("-")]), false);
    const pattern6_1036 = new Sequence_812(Object.freeze([Begin_365, t_855])).compiled();
    t_856 = new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("S"), new CodePoints_810("s")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("R"), new CodePoints_810("r")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("C"), new CodePoints_810("c")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("S"), new CodePoints_810("s")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("E"), new CodePoints_810("e")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("T"), new CodePoints_810("t")]), false)]));
    const pattern7_1037 = new Sequence_812(Object.freeze([Begin_365, t_856])).compiled();
    t_857 = new Or_816(Object.freeze([new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("S"), new CodePoints_810("s")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("R"), new CodePoints_810("r")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("C"), new CodePoints_810("c")]), false)])), new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("H"), new CodePoints_810("h")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("R"), new CodePoints_810("r")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("E"), new CodePoints_810("e")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("F"), new CodePoints_810("f")]), false)]))]));
    const pattern8_1038 = new Sequence_812(Object.freeze([Begin_365, t_857])).compiled();
    t_858 = new Sequence_812(Object.freeze([new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" ")]), false), 0, null, false), new Repeat_814(new CodePoints_810("/"), 0, 1, false), new CodePoints_810("\u003e")]));
    const pattern9_1039 = new Sequence_812(Object.freeze([Begin_365, t_858])).compiled();
    t_859 = new CodeSet_809(Object.freeze([new CodePoints_810("\u003e"), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" ")]), true);
    const pattern10_1040 = new Sequence_812(Object.freeze([Begin_365, t_859])).compiled();
    t_860 = new CodeSet_809(Object.freeze([new CodeRange_1035(97, 122), new CodeRange_1035(65, 90)]), false);
    const pattern11_1041 = new Sequence_812(Object.freeze([Begin_365, t_860])).compiled();
    t_861 = new CodePoints_810(",");
    const pattern12_1042 = new Sequence_812(Object.freeze([Begin_365, t_861])).compiled();
    t_862 = new CodePoints_810("\u003c");
    const pattern13_1043 = new Sequence_812(Object.freeze([Begin_365, t_862])).compiled();
    t_863 = new CodePoints_810("\u003c/");
    const pattern14_1044 = new Sequence_812(Object.freeze([Begin_365, t_863])).compiled();
    t_864 = new CodePoints_810("=");
    const pattern15_1045 = new Sequence_812(Object.freeze([Begin_365, t_864])).compiled();
    t_865 = new CodePoints_810("\u003e");
    const pattern16_1046 = new Sequence_812(Object.freeze([Begin_365, t_865])).compiled();
    t_866 = new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("D"), new CodePoints_810("d")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("A"), new CodePoints_810("a")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("T"), new CodePoints_810("t")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("A"), new CodePoints_810("a")]), false), new CodePoints_810("-"), new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("="), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" "), new CodePoints_810("\u003e")]), true), 0, null, false), new CodeSet_809(Object.freeze([new CodePoints_810("U"), new CodePoints_810("u")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("R"), new CodePoints_810("r")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("L"), new CodePoints_810("l"), new CodePoints_810("I"), new CodePoints_810("i")]), false), new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("="), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" "), new CodePoints_810("\u003e")]), true), 0, null, false)]));
    const pattern17_1047 = new Sequence_812(Object.freeze([Begin_365, t_866])).compiled();
    t_867 = new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("O"), new CodePoints_810("o")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("N"), new CodePoints_810("n")]), false), new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("="), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" "), new CodePoints_810("\u003e")]), true), 0, null, false)]));
    const pattern18_1048 = new Sequence_812(Object.freeze([Begin_365, t_867])).compiled();
    t_868 = new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodePoints_810("S"), new CodePoints_810("s")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("T"), new CodePoints_810("t")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("Y"), new CodePoints_810("y")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("L"), new CodePoints_810("l")]), false), new CodeSet_809(Object.freeze([new CodePoints_810("E"), new CodePoints_810("e")]), false)]));
    const pattern19_1049 = new Sequence_812(Object.freeze([Begin_365, t_868])).compiled();
    t_869 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" ")]), false), 1, null, false);
    const pattern20_1050 = new Sequence_812(Object.freeze([Begin_365, t_869])).compiled();
    t_870 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("\u0022")]), true), 1, null, false);
    const pattern21_1051 = new Sequence_812(Object.freeze([Begin_365, t_870])).compiled();
    t_871 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("'")]), true), 1, null, false);
    const pattern22_1052 = new Sequence_812(Object.freeze([Begin_365, t_871])).compiled();
    t_872 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("\u003c"), new CodePoints_810("\u003e")]), true), 1, null, false);
    const pattern23_1053 = new Sequence_812(Object.freeze([Begin_365, t_872])).compiled();
    t_873 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("="), new CodePoints_810("\u003e"), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" ")]), true), 1, null, false);
    const pattern24_1054 = new Sequence_812(Object.freeze([Begin_365, t_873])).compiled();
    t_874 = new Repeat_814(new CodeSet_809(Object.freeze([new CodePoints_810("\u003e"), new CodePoints_810("\t"), new CodePoints_810("\r"), new CodePoints_810("\n"), new CodePoints_810(" "), new CodePoints_810("\u0022")]), true), 1, null, false);
    const pattern25_1055 = new Sequence_812(Object.freeze([Begin_365, t_874])).compiled();
    t_875 = new CodeSet_809(Object.freeze([new CodePoints_810("\u003e")]), true);
    const pattern26_1056 = new Sequence_812(Object.freeze([Begin_365, t_875])).compiled();
    t_876 = new Sequence_812(Object.freeze([new Repeat_814(new CodeSet_809(Object.freeze([new CodeRange_1035(97, 122), new CodeRange_1035(65, 90), new CodeRange_1035(48, 57), new CodePoints_810("-")]), false), 1, null, false), new CodePoints_810(":")]));
    const pattern27_1057 = new Sequence_812(Object.freeze([Begin_365, t_876])).compiled();
    t_877 = new Sequence_812(Object.freeze([new CodeSet_809(Object.freeze([new CodeRange_1035(97, 122), new CodeRange_1035(65, 90)]), false), new Repeat_814(new CodeSet_809(Object.freeze([new CodeRange_1035(97, 122), new CodeRange_1035(65, 90), new CodeRange_1035(48, 57), new CodeRange_1035(58, 58), new CodePoints_810("-")]), false), 0, null, false)]));
    const pattern28_1058 = new Sequence_812(Object.freeze([Begin_365, t_877])).compiled();
    if (!(literalPart_847 == null)) {
      const literalPart_1059 = literalPart_847;
      if (contextBefore_1028.htmlState === 0) {
        let match_1060;
        try {
          t_965 = pattern14_1044.find(literalPart_1059);
          match_1060 = t_965;
        } catch {
          match_1060 = null;
        }
        if (!(match_1060 == null)) {
          const match_1061 = match_1060;
          try {
            t_966 = pattern11_1041.find(literalPart_1059.substring(match_1061.full.end, literalPart_1059.length));
            t_967 = t_966;
          } catch {
            t_967 = null;
          }
          if (!(t_967 == null)) {
            t_878 = match_1061.full.value;
            t_879 = match_1061.full.end;
            t_880 = new AutoescState_445(new HtmlEscaperContext(2, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
            return_848 = new AfterPropagate_446(t_878, t_879, t_880);
            break fn_1027;
          }
        }
      }
      if (contextBefore_1028.htmlState === 0) {
        let match_1062;
        try {
          t_968 = pattern13_1043.find(literalPart_1059);
          match_1062 = t_968;
        } catch {
          match_1062 = null;
        }
        if (!(match_1062 == null)) {
          const match_1063 = match_1062;
          try {
            t_969 = pattern11_1041.find(literalPart_1059.substring(match_1063.full.end, literalPart_1059.length));
            t_970 = t_969;
          } catch {
            t_970 = null;
          }
          if (!(t_970 == null)) {
            t_881 = match_1063.full.value;
            t_882 = match_1063.full.end;
            t_883 = new AutoescState_445(new HtmlEscaperContext(1, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
            return_848 = new AfterPropagate_446(t_881, t_882, t_883);
            break fn_1027;
          }
        }
      }
      if (contextBefore_1028.htmlState === 0) {
        let match_1064;
        try {
          t_971 = pattern13_1043.find(literalPart_1059);
          match_1064 = t_971;
        } catch {
          match_1064 = null;
        }
        if (!(match_1064 == null)) {
          t_884 = match_1064.full.end;
          return_848 = new AfterPropagate_446("\u0026lt;", t_884, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 0) {
        let match_1065;
        try {
          t_972 = pattern16_1046.find(literalPart_1059);
          match_1065 = t_972;
        } catch {
          match_1065 = null;
        }
        if (!(match_1065 == null)) {
          t_885 = match_1065.full.end;
          return_848 = new AfterPropagate_446("\u0026gt;", t_885, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 0) {
        let match_1066;
        try {
          t_973 = pattern23_1053.find(literalPart_1059);
          match_1066 = t_973;
        } catch {
          match_1066 = null;
        }
        if (!(match_1066 == null)) {
          const match_1067 = match_1066;
          t_886 = match_1067.full.value;
          t_887 = match_1067.full.end;
          return_848 = new AfterPropagate_446(t_886, t_887, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 2) {
        let match_1068;
        try {
          t_974 = pattern1_1030.find(literalPart_1059);
          match_1068 = t_974;
        } catch {
          match_1068 = null;
        }
        if (!(match_1068 == null)) {
          const match_1069 = match_1068;
          t_888 = match_1069.full.value;
          t_889 = match_1069.full.end;
          return_848 = new AfterPropagate_446(t_888, t_889, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 2) {
        let match_1070;
        try {
          t_975 = pattern3_1032.find(literalPart_1059);
          match_1070 = t_975;
        } catch {
          match_1070 = null;
        }
        if (!(match_1070 == null)) {
          const match_1071 = match_1070;
          t_890 = match_1071.full.value;
          t_891 = match_1071.full.end;
          return_848 = new AfterPropagate_446(t_890, t_891, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 2) {
        let match_1072;
        try {
          t_976 = pattern26_1056.find(literalPart_1059);
          match_1072 = t_976;
        } catch {
          match_1072 = null;
        }
        if (!(match_1072 == null)) {
          const match_1073 = match_1072;
          t_892 = match_1073.full.value;
          t_893 = match_1073.full.end;
          return_848 = new AfterPropagate_446(t_892, t_893, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 2) {
        let match_1074;
        try {
          t_977 = pattern16_1046.find(literalPart_1059);
          match_1074 = t_977;
        } catch {
          match_1074 = null;
        }
        if (!(match_1074 == null)) {
          const match_1075 = match_1074;
          t_894 = match_1075.full.value;
          t_895 = match_1075.full.end;
          t_896 = new AutoescState_445(new HtmlEscaperContext(0, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_894, t_895, t_896);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 1) {
        let match_1076;
        try {
          t_978 = pattern28_1058.find(literalPart_1059);
          match_1076 = t_978;
        } catch {
          match_1076 = null;
        }
        if (!(match_1076 == null)) {
          const match_1077 = match_1076;
          t_897 = match_1077.full.value;
          t_898 = match_1077.full.end;
          return_848 = new AfterPropagate_446(t_897, t_898, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 1) {
        try {
          t_979 = pattern4_1033.find(literalPart_1059);
          t_980 = t_979;
        } catch {
          t_980 = null;
        }
        if (!(t_980 == null)) {
          t_899 = new AutoescState_445(new HtmlEscaperContext(3, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446("", 0, t_899);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 1) {
        let match_1078;
        try {
          t_981 = pattern20_1050.find(literalPart_1059);
          match_1078 = t_981;
        } catch {
          match_1078 = null;
        }
        if (!(match_1078 == null)) {
          const match_1079 = match_1078;
          t_900 = match_1079.full.value;
          t_901 = match_1079.full.end;
          t_902 = new AutoescState_445(new HtmlEscaperContext(3, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_900, t_901, t_902);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1080;
        try {
          t_982 = pattern20_1050.find(literalPart_1059);
          match_1080 = t_982;
        } catch {
          match_1080 = null;
        }
        if (!(match_1080 == null)) {
          const match_1081 = match_1080;
          t_903 = match_1081.full.value;
          t_904 = match_1081.full.end;
          return_848 = new AfterPropagate_446(t_903, t_904, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1082;
        try {
          t_983 = pattern27_1057.find(literalPart_1059);
          match_1082 = t_983;
        } catch {
          match_1082 = null;
        }
        if (!(match_1082 == null)) {
          const match_1083 = match_1082;
          t_905 = match_1083.full.value;
          t_906 = match_1083.full.end;
          return_848 = new AfterPropagate_446(t_905, t_906, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1084;
        try {
          t_984 = pattern7_1037.find(literalPart_1059);
          match_1084 = t_984;
        } catch {
          match_1084 = null;
        }
        if (!(match_1084 == null)) {
          const match_1085 = match_1084;
          try {
            t_985 = pattern6_1036.find(literalPart_1059.substring(match_1085.full.end, literalPart_1059.length));
            t_986 = t_985;
          } catch {
            t_986 = null;
          }
          if (t_986 == null) {
            t_907 = new AfterPropagate_446(match_1085.full.value, match_1085.full.end, new AutoescState_445(new HtmlEscaperContext(4, contextBefore_1028.tagState, 4, contextBefore_1028.delimState), before_846.subsidiary));
            t_908 = new HtmlUrlDelegate();
            return_848 = t_907.push(t_908, htmlCodec);
            break fn_1027;
          }
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1086;
        try {
          t_987 = pattern8_1038.find(literalPart_1059);
          match_1086 = t_987;
        } catch {
          match_1086 = null;
        }
        if (!(match_1086 == null)) {
          const match_1087 = match_1086;
          try {
            t_988 = pattern6_1036.find(literalPart_1059.substring(match_1087.full.end, literalPart_1059.length));
            t_989 = t_988;
          } catch {
            t_989 = null;
          }
          if (t_989 == null) {
            t_909 = new AfterPropagate_446(match_1087.full.value, match_1087.full.end, new AutoescState_445(new HtmlEscaperContext(4, contextBefore_1028.tagState, 3, contextBefore_1028.delimState), before_846.subsidiary));
            t_910 = new HtmlUrlDelegate();
            return_848 = t_909.push(t_910, htmlCodec);
            break fn_1027;
          }
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1088;
        try {
          t_990 = pattern17_1047.find(literalPart_1059);
          match_1088 = t_990;
        } catch {
          match_1088 = null;
        }
        if (!(match_1088 == null)) {
          const match_1089 = match_1088;
          t_911 = new AfterPropagate_446(match_1089.full.value, match_1089.full.end, new AutoescState_445(new HtmlEscaperContext(4, contextBefore_1028.tagState, 3, contextBefore_1028.delimState), before_846.subsidiary));
          t_912 = new HtmlUrlDelegate();
          return_848 = t_911.push(t_912, htmlCodec);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1090;
        try {
          t_991 = pattern19_1049.find(literalPart_1059);
          match_1090 = t_991;
        } catch {
          match_1090 = null;
        }
        if (!(match_1090 == null)) {
          const match_1091 = match_1090;
          try {
            t_992 = pattern6_1036.find(literalPart_1059.substring(match_1091.full.end, literalPart_1059.length));
            t_993 = t_992;
          } catch {
            t_993 = null;
          }
          if (t_993 == null) {
            t_913 = new AfterPropagate_446(match_1091.full.value, match_1091.full.end, new AutoescState_445(new HtmlEscaperContext(4, contextBefore_1028.tagState, 1, contextBefore_1028.delimState), before_846.subsidiary));
            t_914 = new HtmlCssDelegate();
            return_848 = t_913.push(t_914, htmlCodec);
            break fn_1027;
          }
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1092;
        try {
          t_994 = pattern18_1048.find(literalPart_1059);
          match_1092 = t_994;
        } catch {
          match_1092 = null;
        }
        if (!(match_1092 == null)) {
          const match_1093 = match_1092;
          t_915 = new AfterPropagate_446(match_1093.full.value, match_1093.full.end, new AutoescState_445(new HtmlEscaperContext(4, contextBefore_1028.tagState, 2, contextBefore_1028.delimState), before_846.subsidiary));
          t_916 = new HtmlJsDelegate();
          return_848 = t_915.push(t_916, htmlCodec);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 3) {
        let match_1094;
        try {
          t_995 = pattern24_1054.find(literalPart_1059);
          match_1094 = t_995;
        } catch {
          match_1094 = null;
        }
        if (!(match_1094 == null)) {
          const match_1095 = match_1094;
          t_917 = match_1095.full.value;
          t_918 = match_1095.full.end;
          t_919 = new AutoescState_445(new HtmlEscaperContext(4, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_917, t_918, t_919);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 4) {
        let match_1096;
        try {
          t_996 = pattern20_1050.find(literalPart_1059);
          match_1096 = t_996;
        } catch {
          match_1096 = null;
        }
        if (!(match_1096 == null)) {
          const match_1097 = match_1096;
          t_920 = match_1097.full.value;
          t_921 = match_1097.full.end;
          return_848 = new AfterPropagate_446(t_920, t_921, before_846);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 4) {
        let match_1098;
        try {
          t_997 = pattern15_1045.find(literalPart_1059);
          match_1098 = t_997;
        } catch {
          match_1098 = null;
        }
        if (!(match_1098 == null)) {
          const match_1099 = match_1098;
          t_922 = match_1099.full.value;
          t_923 = match_1099.full.end;
          t_924 = new AutoescState_445(new HtmlEscaperContext(5, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_922, t_923, t_924);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 4) {
        try {
          t_998 = pattern9_1039.find(literalPart_1059);
          t_999 = t_998;
        } catch {
          t_999 = null;
        }
        if (!(t_999 == null)) {
          t_925 = new AutoescState_445(new HtmlEscaperContext(7, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446("", 0, t_925);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 5) {
        let match_1100;
        try {
          t_1000 = pattern0_1029.find(literalPart_1059);
          match_1100 = t_1000;
        } catch {
          match_1100 = null;
        }
        if (!(match_1100 == null)) {
          const match_1101 = match_1100;
          t_926 = match_1101.full.value;
          t_927 = match_1101.full.end;
          t_928 = new AutoescState_445(new HtmlEscaperContext(6, contextBefore_1028.tagState, contextBefore_1028.attribState, 2), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_926, t_927, t_928);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 5) {
        let match_1102;
        try {
          t_1001 = pattern2_1031.find(literalPart_1059);
          match_1102 = t_1001;
        } catch {
          match_1102 = null;
        }
        if (!(match_1102 == null)) {
          const match_1103 = match_1102;
          t_929 = match_1103.full.value;
          t_930 = match_1103.full.end;
          t_931 = new AutoescState_445(new HtmlEscaperContext(6, contextBefore_1028.tagState, contextBefore_1028.attribState, 1), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_929, t_930, t_931);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 5) {
        try {
          t_1002 = pattern10_1040.find(literalPart_1059);
          t_1003 = t_1002;
        } catch {
          t_1003 = null;
        }
        if (!(t_1003 == null)) {
          t_932 = new AutoescState_445(new HtmlEscaperContext(6, contextBefore_1028.tagState, contextBefore_1028.attribState, 0), before_846.subsidiary);
          return_848 = new AfterPropagate_446("\u0022", 0, t_932);
          break fn_1027;
        }
      }
    }
    if (literalPart_847 == null) {
      t_933 = contextBefore_1028.htmlState;
      t_1004 = t_933 === 5;
    } else {
      t_1004 = false;
    }
    if (t_1004) {
      t_934 = new AutoescState_445(new HtmlEscaperContext(6, contextBefore_1028.tagState, contextBefore_1028.attribState, 0), before_846.subsidiary);
      return_848 = new AfterPropagate_446("\u0022", 0, t_934);
      break fn_1027;
    }
    if (!(literalPart_847 == null)) {
      const literalPart_1104 = literalPart_847;
      if (contextBefore_1028.htmlState === 6) {
        t_935 = contextBefore_1028.delimState;
        t_1005 = t_935 === 0;
      } else {
        t_1005 = false;
      }
      if (t_1005) {
        try {
          t_1006 = pattern5_1034.find(literalPart_1104);
          t_1007 = t_1006;
        } catch {
          t_1007 = null;
        }
        if (!(t_1007 == null)) {
          t_936 = new AutoescState_445(new HtmlEscaperContext(7, contextBefore_1028.tagState, contextBefore_1028.attribState, 0), before_846.subsidiary);
          return_848 = new AfterPropagate_446("\u0022", 0, t_936);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_937 = contextBefore_1028.delimState;
        t_1008 = t_937 === 2;
      } else {
        t_1008 = false;
      }
      if (t_1008) {
        let match_1105;
        try {
          t_1009 = pattern0_1029.find(literalPart_1104);
          match_1105 = t_1009;
        } catch {
          match_1105 = null;
        }
        if (!(match_1105 == null)) {
          const match_1106 = match_1105;
          t_938 = match_1106.full.value;
          t_939 = match_1106.full.end;
          t_940 = new AutoescState_445(new HtmlEscaperContext(7, contextBefore_1028.tagState, contextBefore_1028.attribState, 0), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_938, t_939, t_940);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_941 = contextBefore_1028.delimState;
        t_1010 = t_941 === 1;
      } else {
        t_1010 = false;
      }
      if (t_1010) {
        let match_1107;
        try {
          t_1011 = pattern2_1031.find(literalPart_1104);
          match_1107 = t_1011;
        } catch {
          match_1107 = null;
        }
        if (!(match_1107 == null)) {
          const match_1108 = match_1107;
          t_942 = match_1108.full.value;
          t_943 = match_1108.full.end;
          t_944 = new AutoescState_445(new HtmlEscaperContext(7, contextBefore_1028.tagState, contextBefore_1028.attribState, 0), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_942, t_943, t_944);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_945 = contextBefore_1028.attribState;
        t_1012 = t_945 === 4;
      } else {
        t_1012 = false;
      }
      if (t_1012) {
        let match_1109;
        try {
          t_1013 = pattern12_1042.find(literalPart_1104);
          match_1109 = t_1013;
        } catch {
          match_1109 = null;
        }
        if (!(match_1109 == null)) {
          const match_1110 = match_1109;
          t_946 = new AfterPropagate_446(match_1110.full.value, match_1110.full.end, before_846).pop();
          t_947 = new HtmlUrlDelegate();
          return_848 = t_946.push(t_947, htmlCodec);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_948 = contextBefore_1028.delimState;
        t_1014 = t_948 === 0;
      } else {
        t_1014 = false;
      }
      if (t_1014) {
        let match_1111;
        try {
          t_1015 = pattern25_1055.find(literalPart_1104);
          match_1111 = t_1015;
        } catch {
          match_1111 = null;
        }
        if (!(match_1111 == null)) {
          const match_1112 = match_1111;
          t_949 = new AfterPropagate_446(match_1112.full.value, match_1112.full.end, before_846);
          return_848 = t_949.feed(false);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_950 = contextBefore_1028.delimState;
        t_1016 = t_950 === 0;
      } else {
        t_1016 = false;
      }
      if (t_1016) {
        let match_1113;
        try {
          t_1017 = pattern0_1029.find(literalPart_1104);
          match_1113 = t_1017;
        } catch {
          match_1113 = null;
        }
        if (!(match_1113 == null)) {
          t_951 = new AfterPropagate_446("\u0026#34;", match_1113.full.end, before_846);
          return_848 = t_951.feed(false);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_952 = contextBefore_1028.delimState;
        t_1018 = t_952 === 2;
      } else {
        t_1018 = false;
      }
      if (t_1018) {
        let match_1114;
        try {
          t_1019 = pattern21_1051.find(literalPart_1104);
          match_1114 = t_1019;
        } catch {
          match_1114 = null;
        }
        if (!(match_1114 == null)) {
          const match_1115 = match_1114;
          t_953 = new AfterPropagate_446(match_1115.full.value, match_1115.full.end, before_846);
          return_848 = t_953.feed(false);
          break fn_1027;
        }
      }
      if (contextBefore_1028.htmlState === 6) {
        t_954 = contextBefore_1028.delimState;
        t_1020 = t_954 === 1;
      } else {
        t_1020 = false;
      }
      if (t_1020) {
        let match_1116;
        try {
          t_1021 = pattern22_1052.find(literalPart_1104);
          match_1116 = t_1021;
        } catch {
          match_1116 = null;
        }
        if (!(match_1116 == null)) {
          const match_1117 = match_1116;
          t_955 = new AfterPropagate_446(match_1117.full.value, match_1117.full.end, before_846);
          return_848 = t_955.feed(false);
          break fn_1027;
        }
      }
    }
    if (literalPart_847 == null) {
      if (contextBefore_1028.htmlState === 6) {
        t_956 = contextBefore_1028.attribState;
        t_1022 = t_956 === 0;
      } else {
        t_1022 = false;
      }
      t_1023 = t_1022;
    } else {
      t_1023 = false;
    }
    if (t_1023) {
      return_848 = new AfterPropagate_446("", 0, before_846);
      break fn_1027;
    }
    if (literalPart_847 == null) {
      t_957 = contextBefore_1028.htmlState;
      t_1024 = t_957 === 6;
    } else {
      t_1024 = false;
    }
    if (t_1024) {
      t_958 = new AfterPropagate_446("", 0, before_846);
      return_848 = t_958.feed(true);
      break fn_1027;
    }
    if (contextBefore_1028.htmlState === 7) {
      t_959 = contextBefore_1028.attribState;
      t_1025 = t_959 === 0;
    } else {
      t_1025 = false;
    }
    if (t_1025) {
      t_960 = new AutoescState_445(new HtmlEscaperContext(3, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
      return_848 = new AfterPropagate_446("", 0, t_960);
      break fn_1027;
    }
    if (contextBefore_1028.htmlState === 7) {
      t_961 = new AfterPropagate_446("", 0, new AutoescState_445(new HtmlEscaperContext(3, contextBefore_1028.tagState, 0, contextBefore_1028.delimState), before_846.subsidiary));
      return_848 = t_961.pop();
      break fn_1027;
    }
    if (!(literalPart_847 == null)) {
      const literalPart_1118 = literalPart_847;
      if (contextBefore_1028.htmlState === 3) {
        let match_1119;
        try {
          t_1026 = pattern16_1046.find(literalPart_1118);
          match_1119 = t_1026;
        } catch {
          match_1119 = null;
        }
        if (!(match_1119 == null)) {
          const match_1120 = match_1119;
          t_962 = match_1120.full.value;
          t_963 = match_1120.full.end;
          t_964 = new AutoescState_445(new HtmlEscaperContext(0, contextBefore_1028.tagState, contextBefore_1028.attribState, contextBefore_1028.delimState), before_846.subsidiary);
          return_848 = new AfterPropagate_446(t_962, t_963, t_964);
          break fn_1027;
        }
      }
    }
    if (literalPart_847 == null) {
      return_848 = new AfterPropagate_446("", 0, before_846);
      break fn_1027;
    }
    return_848 = panic_607();
  }
  return return_848;
}
/** @type {HtmlPcdataEscaper} */
const return_1121 = new HtmlPcdataEscaper();
/** @type {HtmlPcdataEscaper} */
const htmlPcdataEscaper_1122 = return_1121;
/** @type {OutputHtmlSpaceEscaper} */
const return_1123 = new OutputHtmlSpaceEscaper();
/** @type {OutputHtmlSpaceEscaper} */
const outputHtmlSpaceEscaper_1124 = return_1123;
/** @type {HtmlAttributeEscaper} */
const return_1125 = new HtmlAttributeEscaper();
/** @type {HtmlAttributeEscaper} */
const htmlAttributeEscaper_1126 = return_1125;
/**
 * @param {AutoescState_445<HtmlEscaperContext>} stateBefore_1127
 * @returns {HtmlEscaper}
 */
export function pickHtmlEscaper(stateBefore_1127) {
  let return_1128;
  let t_1129;
  let t_1130;
  let t_1131;
  let t_1132;
  let t_1133;
  let escaper_1134;
  let t_1135 = stateBefore_1127.context.htmlState;
  if (t_1135 === 0) {
    escaper_1134 = htmlPcdataEscaper_1122;
  } else {
    if (t_1135 === 1) {
      t_1132 = true;
    } else {
      if (t_1135 === 2) {
        t_1131 = true;
      } else {
        if (t_1135 === 3) {
          t_1130 = true;
        } else {
          if (t_1135 === 4) {
            t_1129 = true;
          } else {
            t_1129 = t_1135 === 7;
          }
          t_1130 = t_1129;
        }
        t_1131 = t_1130;
      }
      t_1132 = t_1131;
    }
    if (t_1132) {
      escaper_1134 = outputHtmlSpaceEscaper_1124;
    } else if (t_1135 === 5) {
      escaper_1134 = panic_607();
    } else if (t_1135 === 6) {
      escaper_1134 = htmlAttributeEscaper_1126;
    } else if (t_1135 === 8) {
      escaper_1134 = outputHtmlSpaceEscaper_1124;
    } else {
      escaper_1134 = panic_607();
    }
  }
  const subsidiary_1136 = stateBefore_1127.subsidiary;
  if (!(subsidiary_1136 == null)) {
    const subsidiary_1137 = subsidiary_1136;
    let delegate_1138;
    try {
      t_1133 = requireInstanceOf__1139(subsidiary_1137.delegate, HtmlDelegate);
      delegate_1138 = t_1133;
    } catch {
      delegate_1138 = panic_607();
    }
    return_1128 = delegate_1138.escaper(escaper_1134);
  } else {
    return_1128 = escaper_1134;
  }
  return return_1128;
};
