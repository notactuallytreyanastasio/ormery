# Transition tables for Safe HTML

The transition tables are defined in `./safe-html.temper.md` but the code generated from them lives here.

<details><summary>Generated propagation functions</summary>

<!-- GENERATED_PROPAGATION_FN: Html -->

    let htmlStateStr(x: Int32): String {
      when (x) {
        htmlStatePcdata -> "Pcdata";
        htmlStateOName -> "OName";
        htmlStateCName -> "CName";
        htmlStateBeforeAttr -> "BeforeAttr";
        htmlStateBeforeEq -> "BeforeEq";
        htmlStateBeforeValue -> "BeforeValue";
        htmlStateAttr -> "Attr";
        htmlStateAfterAttr -> "AfterAttr";
        htmlStateSpecialBody -> "SpecialBody";
        else -> x.toString();
      }
    }
    let tagStateStr(x: Int32): String {
      when (x) {
        else -> x.toString();
      }
    }
    let attribStateStr(x: Int32): String {
      when (x) {
        attribStateGeneric -> "Generic";
        attribStateCss -> "Css";
        attribStateJs -> "Js";
        attribStateUrl -> "Url";
        attribStateUrls -> "Urls";
        else -> x.toString();
      }
    }
    let delimStateStr(x: Int32): String {
      when (x) {
        delimStateUq -> "Uq";
        delimStateSq -> "Sq";
        delimStateDq -> "Dq";
        else -> x.toString();
      }
    }
    let htmlPropagateContext(
      before: AutoescState<HtmlEscaperContext>,
      literalPart: String?,
    ): AfterPropagate<HtmlEscaperContext> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx'^(?:"[^"]*"?)';
      let pattern2: Regex = rgx"^(?:')";
      let pattern3: Regex = rgx"^(?:'[^']*'?)";
      let pattern4: Regex = rgx"^(?:(?:>))";
      let pattern5: Regex = rgx"^(?:(?:[>\t\r\n ]))";
      let pattern6: Regex = rgx"^(?:(?:[A-Za-z0-9:-:-]))";
      let pattern7: Regex = rgx"^(?:(?:[Ss][Rr][Cc][Ss][Ee][Tt]))";
      let pattern8: Regex = rgx"^(?:(?:[Ss][Rr][Cc]|[Hh][Rr][Ee][Ff]))";
      let pattern9: Regex = rgx"^(?:(?:[\t\r\n ]*/?>))";
      let pattern10: Regex = rgx"^(?:(?:[^>\t\r\n ]))";
      let pattern11: Regex = rgx"^(?:(?:[a-zA-Z]))";
      let pattern12: Regex = rgx"^(?:,)";
      let pattern13: Regex = rgx"^(?:<)";
      let pattern14: Regex = rgx"^(?:</)";
      let pattern15: Regex = rgx"^(?:=)";
      let pattern16: Regex = rgx"^(?:>)";
      let pattern17: Regex = rgx"^(?:[Dd][Aa][Tt][Aa]-[^=\t\r\n >]*[Uu][Rr][LlIi][^=\t\r\n >]*)";
      let pattern18: Regex = rgx"^(?:[Oo][Nn][^=\t\r\n >]*)";
      let pattern19: Regex = rgx"^(?:[Ss][Tt][Yy][Ll][Ee])";
      let pattern20: Regex = rgx"^(?:[\t\r\n ]+)";
      let pattern21: Regex = rgx'^(?:[^"]+)';
      let pattern22: Regex = rgx"^(?:[^']+)";
      let pattern23: Regex = rgx"^(?:[^<>]+)";
      let pattern24: Regex = rgx"^(?:[^=>\t\r\n ]+)";
      let pattern25: Regex = rgx'^(?:[^>\t\r\n "]+)';
      let pattern26: Regex = rgx"^(?:[^>])";
      let pattern27: Regex = rgx"^(?:[a-zA-Z0-9-]+:)";
      let pattern28: Regex = rgx"^(?:[a-zA-Z][a-zA-Z0-9:-:-]*)";
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern14.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateCName,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern13.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateOName,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern13.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              "&lt;",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              "&gt;",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern26.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStatePcdata,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          let match: Match? = pattern28.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          do {
            if ((pattern4.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateBeforeAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern7.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern6.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateBeforeEq,
                    contextBefore.tagState,
                    attribStateUrls,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).push(new HtmlUrlDelegate(), htmlCodec);
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern8.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern6.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateBeforeEq,
                    contextBefore.tagState,
                    attribStateUrl,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).push(new HtmlUrlDelegate(), htmlCodec);
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateUrl,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            ).push(new HtmlUrlDelegate(), htmlCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern6.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateBeforeEq,
                    contextBefore.tagState,
                    attribStateCss,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).push(new HtmlCssDelegate(), htmlCodec);
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateJs,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            ).push(new HtmlJsDelegate(),  htmlCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateBeforeValue,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          do {
            if ((pattern9.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateAfterAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  delimStateDq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  delimStateSq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeValue) {
          do {
            if ((pattern10.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext>(
                "\"",
                String.begin,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    delimStateUq,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
      }
      if (literalPart == null && contextBefore.htmlState == htmlStateBeforeValue) {
        do {
          return new AfterPropagate<HtmlEscaperContext>(
            "\"",
            String.begin,
            new AutoescState<HtmlEscaperContext>(
              new HtmlEscaperContext(
                htmlStateAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                delimStateUq,
              ),
              before.subsidiary,
            ),
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          do {
            if ((pattern5.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext>(
                "\"",
                String.begin,
                new AutoescState<HtmlEscaperContext>(
                  new HtmlEscaperContext(
                    htmlStateAfterAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    delimStateUq,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateAfterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  delimStateUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStateAfterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  delimStateUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.attribState == attribStateUrls) {
          let match: Match? = pattern12.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            ).pop().push(new HtmlUrlDelegate(), htmlCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            ).feed(false);
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              "&#34;",
              match.full.end,
              before,
            ).feed(false);
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateDq) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            ).feed(false);
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateSq) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            ).feed(false);
          }
        }
      }
      if (literalPart == null && contextBefore.htmlState == htmlStateAttr && contextBefore.attribState == attribStateGeneric) {
        do {
          return new AfterPropagate<HtmlEscaperContext>(
            "",
            String.begin,
            before,
          );
        }
      }
      if (literalPart == null && contextBefore.htmlState == htmlStateAttr) {
        do {
          return new AfterPropagate<HtmlEscaperContext>(
            "",
            String.begin,
            before,
          ).feed(true);
        }
      }
      if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.attribState == attribStateGeneric) {
        do {
          return new AfterPropagate<HtmlEscaperContext>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext>(
              new HtmlEscaperContext(
                htmlStateBeforeAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          );
        }
      }
      if (contextBefore.htmlState == htmlStateAfterAttr) {
        do {
          return new AfterPropagate<HtmlEscaperContext>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext>(
              new HtmlEscaperContext(
                htmlStateBeforeAttr,
                contextBefore.tagState,
                attribStateGeneric,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          ).pop();
        }
      }
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext>(
                new HtmlEscaperContext(
                  htmlStatePcdata,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<HtmlEscaperContext>(
            "",
            String.begin,
            before,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class HtmlContextPropagator extends ContextPropagator<HtmlEscaperContext> {
      public after(
        before: AutoescState<HtmlEscaperContext>,
        literalPart: String?,
      ): AfterPropagate<HtmlEscaperContext> {
        htmlPropagateContext(before, literalPart)
      }
    }

<!-- GENERATED_PROPAGATION_FN: Url -->

    let urlStateStr(x: Int32): String {
      when (x) {
        urlStateStart -> "Start";
        urlStateBeforeQuery -> "BeforeQuery";
        urlStateQuery -> "Query";
        urlStateFragment -> "Fragment";
        else -> x.toString();
      }
    }
    let urlPropagateContext(
      before: AutoescState<UrlEscaperContext>,
      literalPart: String?,
    ): AfterPropagate<UrlEscaperContext> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx"^(?:[#])";
      let pattern1: Regex = rgx"^(?:[?])";
      let pattern2: Regex = rgx"^(?:[^#]+)";
      let pattern3: Regex = rgx"^(?:[^:#?]*:|[^\t\r\n :#?])";
      let pattern4: Regex = rgx"^(?:[^?#]+)";
      if (literalPart != null) {
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext>(
                new UrlEscaperContext(
                  urlStateBeforeQuery,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext>(
                new UrlEscaperContext(
                  urlStateQuery,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext>(
                new UrlEscaperContext(
                  urlStateFragment,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext>(
                new UrlEscaperContext(
                  urlStateQuery,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext>(
                new UrlEscaperContext(
                  urlStateFragment,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateQuery) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.urlState == urlStateQuery) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext>(
                new UrlEscaperContext(
                  urlStateFragment,
                ),
                before.subsidiary,
              ),
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<UrlEscaperContext>(
            "",
            String.begin,
            before,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class UrlContextPropagator extends ContextPropagator<UrlEscaperContext> {
      public after(
        before: AutoescState<UrlEscaperContext>,
        literalPart: String?,
      ): AfterPropagate<UrlEscaperContext> {
        urlPropagateContext(before, literalPart)
      }
    }

    let urlContextPropagator = new UrlContextPropagator();

</details>

