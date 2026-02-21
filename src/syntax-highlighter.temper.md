# Temper Syntax Highlighter

A syntax highlighter for Temper code, written in Temper, compiled to TypeScript.

```temper
// Token types for syntax highlighting
export class TokenType(public name: String) {
  public get isKeyword(): Boolean { name == "keyword" }
  public get isType(): Boolean { name == "type" }
  public get isString(): Boolean { name == "string" }
  public get isNumber(): Boolean { name == "number" }
  public get isComment(): Boolean { name == "comment" }
  public get isOperator(): Boolean { name == "operator" }
  public get isIdentifier(): Boolean { name == "identifier" }
}

// A highlighted token
export class Token(
  public tokenType: TokenType,
  public value: String,
) {
  public toHtml(): String {
    let className = if (tokenType.name == "keyword") {
      "kw"
    } else if (tokenType.name == "type") {
      "typ"
    } else if (tokenType.name == "string") {
      "str"
    } else if (tokenType.name == "number") {
      "num"
    } else if (tokenType.name == "comment") {
      "cmt"
    } else if (tokenType.name == "operator") {
      "op"
    } else {
      "id"
    };
    "<span class=\"${className}\">${escapeHtml(value)}</span>"
  }

  private escapeHtml(text: String): String {
    text
      .replace("&", "&amp;")
      .replace("<", "&lt;")
      .replace(">", "&gt;")
      .replace("\"", "&quot;")
  }
}

// Syntax highlighter for Temper code
export class TemperHighlighter() {
  private keywords: List<String> = [
    "if", "else", "for", "while", "do", "when", "break", "continue", "return",
    "let", "var", "class", "export", "import", "public", "private", "protected",
    "throws", "new", "this", "get", "set", "static", "extends", "implements",
    "true", "false", "null", "bubble", "orelse", "in"
  ];

  private types: List<String> = [
    "String", "Int", "Boolean", "List", "Map", "Bubble", "Pair",
    "Float", "Double", "Byte", "Short", "Long", "Char", "Void",
    "Record", "Schema", "Field", "Query", "InMemoryStore"
  ];

  private operators: List<String> = [
    "=>", "->", "==", "!=", "<=", ">=", "<=>", "&&", "||",
    "+", "-", "*", "/", "%", "<", ">", "=", "!", "&", "|"
  ];

  // Check if a word is a keyword
  private isKeyword(word: String): Boolean {
    keywords.contains(word)
  }

  // Check if a word is a type (starts with uppercase)
  private isType(word: String): Boolean {
    if (word.length() == 0) {
      false
    } else {
      let firstChar = word.substring(0, 1);
      let isUpper = firstChar == firstChar.toUpperCase() && firstChar != firstChar.toLowerCase();
      isUpper || types.contains(word)
    }
  }

  // Check if a character is alphanumeric or underscore
  private isIdentChar(ch: String): Boolean {
    if (ch.length() != 1) {
      false
    } else {
      let isLetter = (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z");
      let isDigit = ch >= "0" && ch <= "9";
      let isUnderscore = ch == "_";
      isLetter || isDigit || isUnderscore
    }
  }

  // Tokenize Temper source code
  public tokenize(source: String): List<Token> {
    let tokens: ListBuilder<Token> = [];
    var i = 0;
    let len = source.length();

    while (i < len) {
      let ch = source.substring(i, i + 1);

      // Skip whitespace (but preserve it in output)
      if (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
        i = i + 1;
        continue;
      }

      // Line comment: //
      if (i + 1 < len && source.substring(i, i + 2) == "//") {
        let start = i;
        while (i < len && source.substring(i, i + 1) != "\n") {
          i = i + 1;
        }
        let comment = source.substring(start, i);
        tokens.add(new Token(new TokenType("comment"), comment));
        continue;
      }

      // Block comment: /* */
      if (i + 1 < len && source.substring(i, i + 2) == "/*") {
        let start = i;
        i = i + 2;
        while (i + 1 < len && source.substring(i, i + 2) != "*/") {
          i = i + 1;
        }
        i = i + 2; // skip */
        let comment = source.substring(start, i);
        tokens.add(new Token(new TokenType("comment"), comment));
        continue;
      }

      // String literal: "..."
      if (ch == "\"") {
        let start = i;
        i = i + 1;
        while (i < len) {
          let current = source.substring(i, i + 1);
          if (current == "\\") {
            i = i + 2; // skip escaped character
          } else if (current == "\"") {
            i = i + 1;
            break;
          } else {
            i = i + 1;
          }
        }
        let str = source.substring(start, i);
        tokens.add(new Token(new TokenType("string"), str));
        continue;
      }

      // Number literal
      if (ch >= "0" && ch <= "9") {
        let start = i;
        while (i < len) {
          let current = source.substring(i, i + 1);
          let isDigit = current >= "0" && current <= "9";
          let isDot = current == ".";
          if (isDigit || isDot) {
            i = i + 1;
          } else {
            break;
          }
        }
        let num = source.substring(start, i);
        tokens.add(new Token(new TokenType("number"), num));
        continue;
      }

      // Identifier or keyword
      if (isIdentChar(ch)) {
        let start = i;
        while (i < len && isIdentChar(source.substring(i, i + 1))) {
          i = i + 1;
        }
        let word = source.substring(start, i);

        let tokenType = if (isKeyword(word)) {
          new TokenType("keyword")
        } else if (isType(word)) {
          new TokenType("type")
        } else {
          new TokenType("identifier")
        };

        tokens.add(new Token(tokenType, word));
        continue;
      }

      // Check for multi-character operators
      let twoChar = if (i + 1 < len) {
        source.substring(i, i + 2)
      } else {
        ""
      };

      let threeChar = if (i + 2 < len) {
        source.substring(i, i + 3)
      } else {
        ""
      };

      if (threeChar == "<=>" || threeChar == "...") {
        tokens.add(new Token(new TokenType("operator"), threeChar));
        i = i + 3;
        continue;
      }

      if (twoChar == "=>" || twoChar == "->" || twoChar == "==" ||
          twoChar == "!=" || twoChar == "<=" || twoChar == ">=" ||
          twoChar == "&&" || twoChar == "||") {
        tokens.add(new Token(new TokenType("operator"), twoChar));
        i = i + 2;
        continue;
      }

      // Single character operator or punctuation
      if (ch == "=" || ch == "+" || ch == "-" || ch == "*" || ch == "/" ||
          ch == "%" || ch == "<" || ch == ">" || ch == "!" || ch == "&" ||
          ch == "|" || ch == "(" || ch == ")" || ch == "{" || ch == "}" ||
          ch == "[" || ch == "]" || ch == "," || ch == ";" || ch == ":" ||
          ch == ".") {
        tokens.add(new Token(new TokenType("operator"), ch));
        i = i + 1;
        continue;
      }

      // Unknown character - skip it
      i = i + 1;
    }

    tokens.toList()
  }

  // Highlight Temper source code and return HTML
  public highlight(source: String): String {
    let tokens = tokenize(source);
    var html = "";

    // Reconstruct source with highlighting
    var pos = 0;
    for (token in tokens) {
      // Find token in source starting from pos
      let tokenStart = source.indexOf(token.value, pos);

      // Add any whitespace before token
      if (tokenStart > pos) {
        let whitespace = source.substring(pos, tokenStart);
        html = html + whitespace;
      }

      // Add highlighted token
      html = html + token.toHtml();
      pos = tokenStart + token.value.length();
    }

    // Add any remaining content
    if (pos < source.length()) {
      html = html + source.substring(pos, source.length());
    }

    html
  }

  // Convenience function: highlight and wrap in <pre><code>
  public highlightBlock(source: String): String {
    let highlighted = highlight(source);
    "<pre class=\"temper-code\"><code>${highlighted}</code></pre>"
  }
}
```
