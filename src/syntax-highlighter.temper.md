# Temper Syntax Highlighter

A syntax highlighter for Temper code, written in Temper. Uses
secure-composition's `html"..."` tagged strings for contextual
auto-escaping — values interpolated into element content are
HTML-entity-encoded automatically.

    let { html, SafeHtml } = import("./html");

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

      public toHtml(): SafeHtml {
        let cls = cssClass();
        html"<span class='${cls}'>${value}</span>"
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

    export let highlightWord(word: String): SafeHtml {
      if (word == "") {
        return html"";
      }
      let tokenType = classifyWord(word);
      let token = new Token(tokenType, word);
      token.toHtml()
    }

    export let highlightLine(line: String): SafeHtml {
      let words = line.split(" ");
      if (words.length == 0) {
        return html"";
      }
      var result = highlightWord(words[0]);
      for (var i = 1; i < words.length; i = i + 1) {
        let word = highlightWord(words[i]);
        result = html"${result} ${word}";
      }
      result
    }

    export let highlightSource(source: String): SafeHtml {
      let lines = source.split("\n");
      if (lines.length == 0) {
        return html"";
      }
      var result = highlightLine(lines[0]);
      for (var i = 1; i < lines.length; i = i + 1) {
        let line = highlightLine(lines[i]);
        result = html"${result}\n${line}";
      }
      result
    }

    export let highlightBlock(source: String): SafeHtml {
      let highlighted = highlightSource(source);
      html"<pre class='temper-code'><code>${highlighted}</code></pre>"
    }
