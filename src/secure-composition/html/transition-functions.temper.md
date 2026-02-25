# Transition tables for Safe HTML

The transition tables are defined in `./safe-html.temper.md` and
similar files but the code generated from them lives here.

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
        htmlStateEnterAttr -> "EnterAttr";
        htmlStateAttr -> "Attr";
        htmlStateAfterAttr -> "AfterAttr";
        htmlStateSpecialBody -> "SpecialBody";
        htmlStateComment -> "Comment";
        htmlStateBogusComment -> "BogusComment";
        else -> x.toString();
      }
    }

    let tagStateStr(x: Int32): String {
      when (x) {
        tagStateGeneric -> "Generic";
        tagStateScript -> "Script";
        tagStateStyle -> "Style";
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

    let htmlStatesEqual(
      a: AutoescState<HtmlEscaperContext, HtmlEscaper>,
      b: AutoescState<HtmlEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: HtmlEscaperContext = a.context;
      let d: HtmlEscaperContext = b.context;
      if (c.htmlState != d.htmlState) { return false; }
      if (c.tagState != d.tagState) { return false; }
      if (c.attribState != d.attribState) { return false; }
      if (c.delimState != d.delimState) { return false; }
      true
    }

    let htmlPropagateContext(
      before: AutoescState<HtmlEscaperContext, HtmlEscaper>,
      literalPart: String?,
    ): AfterPropagate<HtmlEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx'^(?:"[^"]*"?)';
      let pattern2: Regex = rgx"^(?:')";
      let pattern3: Regex = rgx"^(?:'[^']*'?)";
      let pattern4: Regex = rgx'^(?:(?:"))';
      let pattern5: Regex = rgx"^(?:(?:'))";
      let pattern6: Regex = rgx"^(?:(?:--))";
      let pattern7: Regex = rgx"^(?:(?:</[Ss][Cc][Rr][Ii][Pp][Tt](?:$|[^-.0-9:A-Z_a-z])))";
      let pattern8: Regex = rgx"^(?:(?:</[Ss][Tt][Yy][Ll][Ee](?:$|[^-.0-9:A-Z_a-z])))";
      let pattern9: Regex = rgx"^(?:(?:>))";
      let pattern10: Regex = rgx"^(?:(?:[ \n\r\t]*/?>))";
      let pattern11: Regex = rgx"^(?:(?:[-.0-9:A-Z_a-z]))";
      let pattern12: Regex = rgx"^(?:(?:[->]))";
      let pattern13: Regex = rgx"^(?:(?:[>\t\r\n ]))";
      let pattern14: Regex = rgx"^(?:(?:[A-Za-z]))";
      let pattern15: Regex = rgx"^(?:(?:[Ss][Rr][Cc][Ss][Ee][Tt]))";
      let pattern16: Regex = rgx"^(?:(?:[Ss][Rr][Cc]|[Hh][Rr][Ee][Ff]))";
      let pattern17: Regex = rgx"^(?:(?:[^!-(>]|[!#$%&(])+)";
      let pattern18: Regex = rgx"^(?:(?:[^->]|-?>)+)";
      let pattern19: Regex = rgx"^(?:(?:[^<]|<+(?:[^/<]|$))+)";
      let pattern20: Regex = rgx"^(?:(?:[^>\t\r\n ]))";
      let pattern21: Regex = rgx"^(?:,[ \n\r\t]*)";
      let pattern22: Regex = rgx"^(?:--+)";
      let pattern23: Regex = rgx"^(?:--+>)";
      let pattern24: Regex = rgx"^(?:<!)";
      let pattern25: Regex = rgx"^(?:<)";
      let pattern26: Regex = rgx"^(?:</)";
      let pattern27: Regex = rgx"^(?:<[!?])";
      let pattern28: Regex = rgx"^(?:<[Ss][Cc][Rr][Ii][Pp][Tt])";
      let pattern29: Regex = rgx"^(?:<[Ss][Tt][Yy][Ll][Ee])";
      let pattern30: Regex = rgx"^(?:=)";
      let pattern31: Regex = rgx"^(?:>)";
      let pattern32: Regex = rgx"^(?:[ \n\r\t]+)";
      let pattern33: Regex = rgx"^(?:[-0-9A-Za-z]+:)";
      let pattern34: Regex = rgx"^(?:[A-Za-z][-.0-9:A-Z_a-z]*)";
      let pattern35: Regex = rgx"^(?:[Dd][Aa][Tt][Aa]-[^=>\t\r\n ]*[Uu][Rr][ILil][^=>\t\r\n ]*)";
      let pattern36: Regex = rgx"^(?:[Oo][Nn][^=>\t\r\n ]*)";
      let pattern37: Regex = rgx"^(?:[Ss][Tt][Yy][Ll][Ee])";
      let pattern38: Regex = rgx'^(?:[^">\t\r\n ]+)';
      let pattern39: Regex = rgx'^(?:[^"]+)';
      let pattern40: Regex = rgx"^(?:[^']+)";
      let pattern41: Regex = rgx"^(?:[^<>]+)";
      let pattern42: Regex = rgx"^(?:[^=>\t\r\n ]+)";
      let pattern43: Regex = rgx"^(?:[^>]+)";
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern26.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern14.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
          let match: Match? = pattern28.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateBeforeAttr,
                    tagStateScript,
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
          let match: Match? = pattern29.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateBeforeAttr,
                    tagStateStyle,
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
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern14.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern6.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateComment,
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
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateBogusComment,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "&lt;",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "&gt;",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStatePcdata) {
          let match: Match? = pattern41.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateCName) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
          let match: Match? = pattern34.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateOName) {
          do {
            if ((pattern9.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
          let match: Match? = pattern32.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
        if (contextBefore.htmlState == htmlStateComment) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
        if (contextBefore.htmlState == htmlStateComment) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateComment) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern12.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                before,
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBogusComment) {
          let match: Match? = pattern43.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBogusComment) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
        if (contextBefore.htmlState == htmlStateBeforeAttr && contextBefore.tagState == tagStateStyle) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateSpecialBody,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            ).push(new HtmlCssDelegate(), noTagEndCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr && contextBefore.tagState == tagStateScript) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateSpecialBody,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            ).push(new HtmlJsDelegate(),  noTagEndCodec);
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr && contextBefore.tagState == tagStateGeneric) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern32.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern33.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateBeforeEq,
                    contextBefore.tagState,
                    attribStateUrls,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateBeforeEq,
                    contextBefore.tagState,
                    attribStateUrl,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern35.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateUrl,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern37.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern11.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateBeforeEq,
                    contextBefore.tagState,
                    attribStateCss,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern36.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateBeforeEq,
                  contextBefore.tagState,
                  attribStateJs,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeAttr) {
          let match: Match? = pattern42.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
          let match: Match? = pattern32.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateBeforeEq) {
          let match: Match? = pattern30.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
            if ((pattern10.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
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
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateEnterAttr,
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
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateEnterAttr,
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
            if ((pattern20.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateEnterAttr,
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
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "\"",
            String.begin,
            new AutoescState<HtmlEscaperContext, HtmlEscaper>(
              new HtmlEscaperContext(
                htmlStateEnterAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                delimStateUq,
              ),
              before.subsidiary,
            ),
          );
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateUrls) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext, HtmlEscaper>(
              new HtmlEscaperContext(
                htmlStateAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          ).push(new HtmlUrlDelegate(), htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateUrl) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext, HtmlEscaper>(
              new HtmlEscaperContext(
                htmlStateAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          ).push(new HtmlUrlDelegate(), htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateCss) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext, HtmlEscaper>(
              new HtmlEscaperContext(
                htmlStateAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          ).push(new HtmlCssDelegate(), htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateJs) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext, HtmlEscaper>(
              new HtmlEscaperContext(
                htmlStateAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          ).push(new HtmlJsDelegate(),  htmlCodec);
        }
      }
      if (contextBefore.htmlState == htmlStateEnterAttr && contextBefore.attribState == attribStateGeneric) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new AutoescState<HtmlEscaperContext, HtmlEscaper>(
              new HtmlEscaperContext(
                htmlStateAttr,
                contextBefore.tagState,
                contextBefore.attribState,
                contextBefore.delimState,
              ),
              before.subsidiary,
            ),
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          do {
            if ((pattern13.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateAfterAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateDq) {
          do {
            if ((pattern4.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateAfterAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateSq) {
          do {
            if ((pattern5.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStateAfterAttr,
                    contextBefore.tagState,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.attribState == attribStateUrls) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateEnterAttr,
                  contextBefore.tagState,
                  contextBefore.attribState,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            ).pop();
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          let match: Match? = pattern38.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateUq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "&#34;",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateDq) {
          let match: Match? = pattern39.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAttr && contextBefore.delimState == delimStateSq) {
          let match: Match? = pattern40.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.delimState == delimStateDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  contextBefore.tagState,
                  attribStateGeneric,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.delimState == delimStateSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  contextBefore.tagState,
                  attribStateGeneric,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateAfterAttr && contextBefore.delimState == delimStateUq) {
          do {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              "\"",
              String.begin,
              new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                new HtmlEscaperContext(
                  htmlStateBeforeAttr,
                  contextBefore.tagState,
                  attribStateGeneric,
                  contextBefore.delimState,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody && contextBefore.tagState == tagStateScript) {
          do {
            if ((pattern7.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStatePcdata,
                    tagStateGeneric,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody && contextBefore.tagState == tagStateStyle) {
          do {
            if ((pattern8.find(literalPart) orelse null) != null) {
              return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<HtmlEscaperContext, HtmlEscaper>(
                  new HtmlEscaperContext(
                    htmlStatePcdata,
                    tagStateGeneric,
                    contextBefore.attribState,
                    contextBefore.delimState,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.htmlState == htmlStateSpecialBody) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<HtmlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            before,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class HtmlContextPropagator extends ContextPropagator<HtmlEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<HtmlEscaperContext, HtmlEscaper>,
        literalPart: String?,
      ): AfterPropagate<HtmlEscaperContext, HtmlEscaper> {
        htmlPropagateContext(before, literalPart)
      }
    }

<!-- GENERATED_PROPAGATION_FN: Url esc=HtmlEscaper -->

    let urlStateStr(x: Int32): String {
      when (x) {
        urlStateStart -> "Start";
        urlStateBeforeQuery -> "BeforeQuery";
        urlStateQuery -> "Query";
        urlStateFragment -> "Fragment";
        else -> x.toString();
      }
    }

    let urlStatesEqual(
      a: AutoescState<UrlEscaperContext, HtmlEscaper>,
      b: AutoescState<UrlEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: UrlEscaperContext = a.context;
      let d: UrlEscaperContext = b.context;
      if (c.urlState != d.urlState) { return false; }
      true
    }

    let urlPropagateContext(
      before: AutoescState<UrlEscaperContext, HtmlEscaper>,
      literalPart: String?,
    ): AfterPropagate<UrlEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx"^(?:(?:[^#]|#)+)";
      let pattern1: Regex = rgx"^(?:[#])";
      let pattern2: Regex = rgx"^(?:[?])";
      let pattern3: Regex = rgx"^(?:[^#/:?]*[/:])";
      let pattern4: Regex = rgx"^(?:[^#/:?]+)";
      let pattern5: Regex = rgx"^(?:[^#?]+)";
      let pattern6: Regex = rgx"^(?:[^#]+)";
      if (literalPart != null) {
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext, HtmlEscaper>(
                new UrlEscaperContext(
                  urlStateBeforeQuery,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.urlState == urlStateStart) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext, HtmlEscaper>(
                new UrlEscaperContext(
                  urlStateQuery,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        do {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext, HtmlEscaper>(
                new UrlEscaperContext(
                  urlStateFragment,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern5.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext, HtmlEscaper>(
                new UrlEscaperContext(
                  urlStateQuery,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateBeforeQuery) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext, HtmlEscaper>(
                new UrlEscaperContext(
                  urlStateFragment,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateQuery) {
          let match: Match? = pattern6.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.urlState == urlStateQuery) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<UrlEscaperContext, HtmlEscaper>(
                new UrlEscaperContext(
                  urlStateFragment,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.urlState == urlStateFragment) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<UrlEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            before,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class UrlContextPropagator extends ContextPropagator<UrlEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<UrlEscaperContext, HtmlEscaper>,
        literalPart: String?,
      ): AfterPropagate<UrlEscaperContext, HtmlEscaper> {
        urlPropagateContext(before, literalPart)
      }
    }

    let urlContextPropagator = doPure { new UrlContextPropagator() };

<!-- GENERATED_PROPAGATION_FN: Css esc=HtmlEscaper -->

    let cssStateStr(x: Int32): String {
      when (x) {
        cssStateTop -> "Top";
        cssStateQuoted -> "Quoted";
        cssStateComment -> "Comment";
        cssStateHash -> "Hash";
        cssStateBeforeUrl -> "BeforeUrl";
        cssStateUrl -> "Url";
        cssStateAfterUrl -> "AfterUrl";
        else -> x.toString();
      }
    }

    let cssDelimStr(x: Int32): String {
      when (x) {
        cssDelimUq -> "Uq";
        cssDelimDq -> "Dq";
        cssDelimSq -> "Sq";
        else -> x.toString();
      }
    }

    let cssStatesEqual(
      a: AutoescState<CssEscaperContext, HtmlEscaper>,
      b: AutoescState<CssEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: CssEscaperContext = a.context;
      let d: CssEscaperContext = b.context;
      if (c.cssState != d.cssState) { return false; }
      if (c.cssDelim != d.cssDelim) { return false; }
      true
    }

    let cssPropagateContext(
      before: AutoescState<CssEscaperContext, HtmlEscaper>,
      literalPart: String?,
    ): AfterPropagate<CssEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx"^(?:#)";
      let pattern2: Regex = rgx"^(?:')";
      let pattern3: Regex = rgx"^(?:((?:[^!-(-0-9<A-Z\\_a-z-􏿿]|[!#$%&(])|\\[^<]?)+)";
      let pattern4: Regex = rgx"^(?:(?:))";
      let pattern5: Regex = rgx"^(?:(?:[!#$%&*-~]|[-􏿿]|\\[0-9A-Fa-f]{1,6}[ \n\r\t]?|\\.?)+)";
      let pattern6: Regex = rgx'^(?:(?:["\n\r]))';
      let pattern7: Regex = rgx"^(?:(?:['\n\r]))";
      let pattern8: Regex = rgx"^(?:(?:[*/]))";
      let pattern9: Regex = rgx"^(?:(?:[-0-9A-Z_a-z]|\\[0-9A-Fa-f]{1,6}[ \n\r\t]?|\\.?|[-􏿿])+)";
      let pattern10: Regex = rgx"^(?:(?:[-0-9A-Z_a-z-􏿿]|\\[^<]?)+)";
      let pattern11: Regex = rgx"^(?:(?:[\n\r]))";
      let pattern12: Regex = rgx'^(?:(?:[^"\\\n\r]|\\[^<]?)+)';
      let pattern13: Regex = rgx"^(?:(?:[^'\\\n\r]|\\[^<]?)+)";
      let pattern14: Regex = rgx"^(?:(?:[^|]))";
      let pattern15: Regex = rgx"^(?:(?:\\.|[^\\\n\r])+)";
      let pattern16: Regex = rgx"^(?:/)";
      let pattern17: Regex = rgx"^(?:/\*)";
      let pattern18: Regex = rgx"^(?:<)";
      let pattern19: Regex = rgx"^(?:[ \n\r\t]+)";
      let pattern20: Regex = rgx"^(?:[*]+)";
      let pattern21: Regex = rgx"^(?:[*]+/)";
      let pattern22: Regex = rgx"^(?:[Uu][Rr][Ll][ \n\r\t]*\()";
      let pattern23: Regex = rgx"^(?:[^*/<]+)";
      let pattern24: Regex = rgx"^(?:\\<)";
      let pattern25: Regex = rgx"^(?:\\?<)";
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateBeforeUrl,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateQuoted,
                  cssDelimDq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateQuoted,
                  cssDelimSq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "< ",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\\3c ",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateComment,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateHash,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern3.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateTop) {
          let match: Match? = pattern10.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimDq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new AutoescState<CssEscaperContext, HtmlEscaper>(
                  new CssEscaperContext(
                    cssStateTop,
                    cssDelimUq,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimSq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "'",
                String.begin,
                new AutoescState<CssEscaperContext, HtmlEscaper>(
                  new CssEscaperContext(
                    cssStateTop,
                    cssDelimUq,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateQuoted) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\\3c ",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern12.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateQuoted && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern13.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern14.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                before,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "< ",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              " /",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern8.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                before,
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateComment) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "* ",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateHash) {
          let match: Match? = pattern9.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.cssState == cssStateHash) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            before,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateHash) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateUrl,
                  cssDelimDq,
                ),
                before.subsidiary,
              ),
            ).push(new CssUrlDelegate(), cssCodec);
          }
        }
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateUrl,
                  cssDelimSq,
                ),
                before.subsidiary,
              ),
            ).push(new CssUrlDelegate(), cssCodec);
          }
        }
      }
      if (literalPart == null && contextBefore.cssState == cssStateBeforeUrl) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "\"",
            String.begin,
            new AutoescState<CssEscaperContext, HtmlEscaper>(
              new CssEscaperContext(
                cssStateUrl,
                cssDelimUq,
              ),
              before.subsidiary,
            ),
          ).push(new CssUrlDelegate(), cssCodec);
        }
      }
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateBeforeUrl) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\"",
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateUrl,
                  cssDelimUq,
                ),
                before.subsidiary,
              ),
            ).push(new CssUrlDelegate(), cssCodec);
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimDq) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<CssEscaperContext, HtmlEscaper>(
                  new CssEscaperContext(
                    cssStateAfterUrl,
                    contextBefore.cssDelim,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimSq) {
          do {
            if ((pattern7.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "",
                String.begin,
                new AutoescState<CssEscaperContext, HtmlEscaper>(
                  new CssEscaperContext(
                    cssStateAfterUrl,
                    contextBefore.cssDelim,
                  ),
                  before.subsidiary,
                ),
              ).pop();
            }
          }
        }
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "%27",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "%22",
              match.full.end,
              before,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.cssState == cssStateUrl) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            before,
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "%3c",
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimUq) {
          let match: Match? = pattern5.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.cssState == cssStateUrl) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateAfterUrl,
                  contextBefore.cssDelim,
                ),
                before.subsidiary,
              ),
            ).pop();
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimDq) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimSq) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimDq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new AutoescState<CssEscaperContext, HtmlEscaper>(
                  new CssEscaperContext(
                    cssStateTop,
                    cssDelimUq,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimSq) {
          do {
            if ((pattern11.find(literalPart) orelse null) != null) {
              return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
                "'",
                String.begin,
                new AutoescState<CssEscaperContext, HtmlEscaper>(
                  new CssEscaperContext(
                    cssStateTop,
                    cssDelimUq,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.cssState == cssStateAfterUrl && contextBefore.cssDelim == cssDelimUq) {
          do {
            return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
              "\"",
              String.begin,
              new AutoescState<CssEscaperContext, HtmlEscaper>(
                new CssEscaperContext(
                  cssStateTop,
                  cssDelimUq,
                ),
                before.subsidiary,
              ),
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<CssEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            before,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class CssContextPropagator extends ContextPropagator<CssEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<CssEscaperContext, HtmlEscaper>,
        literalPart: String?,
      ): AfterPropagate<CssEscaperContext, HtmlEscaper> {
        cssPropagateContext(before, literalPart)
      }
    }

    let cssContextPropagator = doPure { new CssContextPropagator() };

<!-- GENERATED_PROPAGATION_FN: Js esc=HtmlEscaper -->

    let jsStateStr(x: Int32): String {
      when (x) {
        jsStateTop -> "Top";
        jsStateBCmt -> "BCmt";
        jsStateLCmt -> "LCmt";
        jsStateId -> "Id";
        jsStateDStr -> "DStr";
        jsStateSStr -> "SStr";
        jsStateBStr -> "BStr";
        jsStateRegx -> "Regx";
        jsStateChSet -> "ChSet";
        else -> x.toString();
      }
    }

    let jsAllowStr(x: Int32): String {
      when (x) {
        jsAllowRe -> "Re";
        jsAllowDiv -> "Div";
        else -> x.toString();
      }
    }

    let jsStackStr(x: Int32): String {
      when (x) {
        else -> x.toString();
      }
    }

    let jsDepthStr(x: Int32): String {
      when (x) {
        jsDepthZero -> "Zero";
        else -> x.toString();
      }
    }

    let jsStatesEqual(
      a: AutoescState<JsEscaperContext, HtmlEscaper>,
      b: AutoescState<JsEscaperContext, HtmlEscaper>,
    ): Boolean {
      if (a.subsidiary != b.subsidiary) { return false; }
      let c: JsEscaperContext = a.context;
      let d: JsEscaperContext = b.context;
      if (c.jsState != d.jsState) { return false; }
      if (c.jsAllow != d.jsAllow) { return false; }
      if (c.jsStack != d.jsStack) { return false; }
      if (c.jsDepth != d.jsDepth) { return false; }
      true
    }

    let jsPropagateContext(
      before: AutoescState<JsEscaperContext, HtmlEscaper>,
      literalPart: String?,
    ): AfterPropagate<JsEscaperContext, HtmlEscaper> {
      let contextBefore = before.context;
      let pattern0: Regex = rgx'^(?:")';
      let pattern1: Regex = rgx"^(?:')";
      let pattern2: Regex = rgx"^(?:(?:!={0,2}|#|%=?|&&?=?|\(|\*\*?=?|\+=?|,|-=?|=>|[.](?:[.][.])?|:|;|<<?=?|={1,3}|>{1,3}=?|\?(?:\?=?|[.]?)|@|\[|^=?|[|]{1,2}=?|~))";
      let pattern3: Regex = rgx"^(?:(?:(?:[$0-9A-Z_a-z-􏿿]|\\u[0-9A-Fa-f]{4})))";
      let pattern4: Regex = rgx"^(?:(?:0[Xx][0-9A-Fa-f]+|(?:(?:(?:0|[1-9][0-9]*)(?:[.][0-9]+?)?|[.][0-9]+)(?:[Ee][+-]?[0-9]+)?))n?)";
      let pattern5: Regex = rgx"^(?:(?:[$A-Z_a-z-􏿿]|\\u[0-9A-Fa-f]{4})(?:[$0-9A-Z_a-z-􏿿]|\\u[0-9A-Fa-f]{4})*)";
      let pattern6: Regex = rgx"^(?:(?:[\n\r  ]))";
      let pattern7: Regex = rgx'^(?:(?:[^"\\\n\r  ]|\\.?)+)';
      let pattern8: Regex = rgx"^(?:(?:[^$\\`]|\\.?)+)";
      let pattern9: Regex = rgx"^(?:(?:[^'\\\n\r  ]|\\.?)+)";
      let pattern10: Regex = rgx"^(?:(?:[^*]|\*+[^*/])+)";
      let pattern11: Regex = rgx"^(?:(?:abstract|await|break|case|catch|const|continue|debugger|default|delete|do|else|enum|export|extends|final|finally|for|function|goto|if|implements|import|in|instanceof|interface|is|namespace|native|new|package|return|static|switch|synchronized|throw|throws|transient|try|typeof|use|var|volatile|while|with|yield))";
      let pattern12: Regex = rgx"^(?:.)";
      let pattern13: Regex = rgx"^(?:/)";
      let pattern14: Regex = rgx"^(?://)";
      let pattern15: Regex = rgx"^(?:/=?)";
      let pattern16: Regex = rgx"^(?:/[dgimsuvy]*)";
      let pattern17: Regex = rgx"^(?:/\*)";
      let pattern18: Regex = rgx"^(?:[	 \n\r  -​  　﻿]+)";
      let pattern19: Regex = rgx"^(?:[$])";
      let pattern20: Regex = rgx"^(?:[$]\{)";
      let pattern21: Regex = rgx"^(?:[\n\r  ])";
      let pattern22: Regex = rgx"^(?:[^/\[\\\n\r  ]|\\.?])";
      let pattern23: Regex = rgx"^(?:[^\\\]\n\r  ]|\\.?])";
      let pattern24: Regex = rgx"^(?:[^\n\r  ]+)";
      let pattern25: Regex = rgx"^(?:\)|\]|\+\+|--)";
      let pattern26: Regex = rgx"^(?:\*+/)";
      let pattern27: Regex = rgx"^(?:\[)";
      let pattern28: Regex = rgx"^(?:\{)";
      let pattern29: Regex = rgx"^(?:\})";
      let pattern30: Regex = rgx"^(?:])";
      let pattern31: Regex = rgx"^(?:`)";
      if (literalPart != null) {
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern17.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateBCmt,
                  contextBefore.jsAllow,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern14.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateLCmt,
                  contextBefore.jsAllow,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern18.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateDStr,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateSStr,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateBStr,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern4.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop && contextBefore.jsAllow == jsAllowRe) {
          let match: Match? = pattern13.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateRegx,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern15.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowRe,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern11.find(literalPart) orelse null;
          if (match != null) {
            if ((pattern3.find(literalPart.slice(match.full.end, literalPart.end)) orelse null) == null) {
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                match.full.value,
                match.full.end,
                new AutoescState<JsEscaperContext, HtmlEscaper>(
                  new JsEscaperContext(
                    contextBefore.jsState,
                    jsAllowRe,
                    contextBefore.jsStack,
                    contextBefore.jsDepth,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern5.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern25.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern2.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowRe,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern28.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowRe,
                  computeJsShl(contextBefore.jsStack),
                  computeJsIncr(contextBefore.jsDepth),
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop && computeJsIsProg(contextBefore.jsStack)) {
          let match: Match? = pattern29.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  contextBefore.jsState,
                  jsAllowRe,
                  computeJsShr(contextBefore.jsStack),
                  computeJsDecr(contextBefore.jsDepth),
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop) {
          let match: Match? = pattern12.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
      }
      if (literalPart == null && contextBefore.jsState == jsStateTop) {
        do {
          return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            new AutoescState<JsEscaperContext, HtmlEscaper>(
              new JsEscaperContext(
                contextBefore.jsState,
                jsAllowDiv,
                contextBefore.jsStack,
                contextBefore.jsDepth,
              ),
              before.subsidiary,
            ),
          );
        }
      }
      if (literalPart != null) {
        if (contextBefore.jsState == jsStateBCmt) {
          let match: Match? = pattern26.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  contextBefore.jsAllow,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateBCmt) {
          let match: Match? = pattern10.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateLCmt) {
          let match: Match? = pattern21.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  contextBefore.jsAllow,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateLCmt) {
          let match: Match? = pattern24.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateDStr) {
          let match: Match? = pattern0.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateDStr) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                "\"",
                String.begin,
                new AutoescState<JsEscaperContext, HtmlEscaper>(
                  new JsEscaperContext(
                    jsStateTop,
                    jsAllowDiv,
                    contextBefore.jsStack,
                    contextBefore.jsDepth,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateDStr) {
          let match: Match? = pattern7.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateSStr) {
          let match: Match? = pattern1.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateSStr) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                "'",
                String.begin,
                new AutoescState<JsEscaperContext, HtmlEscaper>(
                  new JsEscaperContext(
                    jsStateTop,
                    jsAllowDiv,
                    contextBefore.jsStack,
                    contextBefore.jsDepth,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateSStr) {
          let match: Match? = pattern9.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern31.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern20.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowRe,
                  computeJsShlp(contextBefore.jsStack),
                  computeJsIncr(contextBefore.jsDepth),
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateTop && computeJsIsBStr(contextBefore.jsStack)) {
          let match: Match? = pattern29.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateBStr,
                  jsAllowRe,
                  computeJsShr(contextBefore.jsStack),
                  computeJsDecr(contextBefore.jsDepth),
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern8.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateBStr) {
          let match: Match? = pattern19.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          let match: Match? = pattern16.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateTop,
                  jsAllowDiv,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          do {
            if ((pattern6.find(literalPart) orelse null) != null) {
              return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
                "/",
                String.begin,
                new AutoescState<JsEscaperContext, HtmlEscaper>(
                  new JsEscaperContext(
                    jsStateTop,
                    jsAllowDiv,
                    contextBefore.jsStack,
                    contextBefore.jsDepth,
                  ),
                  before.subsidiary,
                ),
              );
            }
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          let match: Match? = pattern27.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateChSet,
                  contextBefore.jsAllow,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateRegx) {
          let match: Match? = pattern22.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
        if (contextBefore.jsState == jsStateChSet) {
          let match: Match? = pattern30.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              new AutoescState<JsEscaperContext, HtmlEscaper>(
                new JsEscaperContext(
                  jsStateRegx,
                  contextBefore.jsAllow,
                  contextBefore.jsStack,
                  contextBefore.jsDepth,
                ),
                before.subsidiary,
              ),
            );
          }
        }
        if (contextBefore.jsState == jsStateChSet) {
          let match: Match? = pattern23.find(literalPart) orelse null;
          if (match != null) {
            return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
              match.full.value,
              match.full.end,
              before,
            );
          }
        }
      }
      if (literalPart == null) {
        do {
          return new AfterPropagate<JsEscaperContext, HtmlEscaper>(
            "",
            String.begin,
            before,
          );
        }
      }
      panic()
    }

<!-- /GENERATED_PROPAGATION_FN -->

    export class JsContextPropagator extends ContextPropagator<JsEscaperContext, HtmlEscaper> {
      public after(
        before: AutoescState<JsEscaperContext, HtmlEscaper>,
        literalPart: String?,
      ): AfterPropagate<JsEscaperContext, HtmlEscaper> {
        jsPropagateContext(before, literalPart)
      }
    }

    let jsContextPropagator = doPure { new JsContextPropagator() };

</details>
