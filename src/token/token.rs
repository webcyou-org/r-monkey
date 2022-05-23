struct TokenType string;

struct Token {
    Type: TokenType,
	Literal: string,
}

let ILLEGAL = "ILLEGAL";
let EOF = "EOF";

// 識別子 + リテラル
let IDENT = "IDENT"; // add, foobar, x, y,...
let INT = "INT"; // 123456

// 演算子
let ASSIGN = "=";
let PLUS = "+";

// デリミタ
let COMMA = ",";
let SEMICOLON = ";";

let LPAREN = "(";
let RPAREN = ")";
let LBRACE = "{";
let RBRACE = "}";

// キーワード
let FUNCTION = "FUNCTION";
let LET = "LET";
