# Temper Syntax Highlighter

A syntax highlighter for Temper code, written in Temper. Uses Temper's
available String and List primitives: split, comparison, template strings,
and list operations.

## Token Types

    export class TokenType(public name: String) {
      public get isKeyword(): Boolean { name == "keyword" }
      public get isType(): Boolean { name == "type" }
      public get isString(): Boolean { name == "string" }
      public get isNumber(): Boolean { name == "number" }
      public get isComment(): Boolean { name == "comment" }
      public get isOperator(): Boolean { name == "operator" }
      public get isIdentifier(): Boolean { name == "identifier" }
    }

## Token

    export class Token(
      public tokenType: TokenType,
      public value: String,
    ) {
      public cssClass(): String {
        let name = tokenType.name;
        when (name) {
          "keyword" -> "kw";
          "type" -> "typ";
          "string" -> "str";
          "number" -> "num";
          "comment" -> "cmt";
          "operator" -> "op";
          else -> "id";
        }
      }

      public toHtml(): String {
        let cls = cssClass();
        "<span class=\"${cls}\">${value}</span>"
      }
    }

## Keyword and Type Lists

    let temperKeywords: List<String> = [
      "if", "else", "for", "while", "do", "when", "break", "continue", "return",
      "let", "var", "class", "export", "import", "public", "private", "protected",
      "throws", "new", "this", "get", "set", "static", "extends", "implements",
      "true", "false", "null", "bubble", "orelse", "of",
    ];

    let temperTypes: List<String> = [
      "String", "Int", "Boolean", "List", "Map", "Bubble", "Pair",
      "Float", "Double", "Byte", "Short", "Long", "Char", "Void",
      "Record", "Schema", "Field", "Query", "InMemoryStore",
      "ListBuilder", "MapBuilder", "WhereClause", "OrderClause",
    ];

## Classifier

Classifies a word token based on known keywords and types.

    export let classifyWord(word: String): TokenType {
      for (let kw of temperKeywords) {
        if (kw == word) {
          return new TokenType("keyword");
        }
      }
      for (let tp of temperTypes) {
        if (tp == word) {
          return new TokenType("type");
        }
      }
      new TokenType("identifier")
    }

## Simple Line Highlighter

Since Temper's String type works with code points and StringIndex rather than
integer-indexed substring operations, we use a word-level approach: split each
line by spaces and classify each token.

    export let highlightWord(word: String): String {
      if (word == "") {
        return "";
      }
      let tokenType = classifyWord(word);
      let token = new Token(tokenType, word);
      token.toHtml()
    }

    export let highlightLine(line: String): String {
      let words = line.split(" ");
      let highlighted = words.map { (w: String): String => highlightWord(w) };
      highlighted.join(" ") { (s: String): String => s }
    }

    export let highlightSource(source: String): String {
      let lines = source.split("\n");
      let highlighted = lines.map { (line: String): String => highlightLine(line) };
      highlighted.join("\n") { (s: String): String => s }
    }

    export let highlightBlock(source: String): String {
      let highlighted = highlightSource(source);
      "<pre class=\"temper-code\"><code>${highlighted}</code></pre>"
    }
