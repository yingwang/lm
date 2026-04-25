//! Snapshot tests for the LM lexer.

use super::*;

/// Helper: tokenize source and format tokens + diagnostics for snapshot comparison.
fn lex_snapshot(source: &str) -> String {
    let (tokens, diagnostics) = Lexer::new(source, 0).tokenize();

    let mut out = String::new();

    out.push_str("=== Tokens ===\n");
    for tok in &tokens {
        out.push_str(&format!(
            "{:<14} {:>4}..{:<4} {:?}\n",
            format!("{:?}", tok.kind),
            tok.span.start,
            tok.span.end,
            tok.text,
        ));
    }

    if !diagnostics.is_empty() {
        out.push_str("\n=== Diagnostics ===\n");
        for diag in &diagnostics {
            out.push_str(&format!(
                "[{}] {}: {}\n",
                diag.code, diag.severity, diag.message
            ));
        }
    }

    out
}

#[test]
fn test_basic_numbers() {
    let result = lex_snapshot("42 3.14 0 100");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    IntLit            0..2    "42"
    FloatLit          3..7    "3.14"
    IntLit            8..9    "0"
    IntLit           10..13   "100"
    Eof              13..13   ""
    "#);
}

#[test]
fn test_basic_strings() {
    let result = lex_snapshot(r#""hello" "world" """#);
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    StringLit         0..7    "\"hello\""
    StringLit         8..15   "\"world\""
    StringLit        16..18   "\"\""
    Eof              18..18   ""
    "#);
}

#[test]
fn test_identifiers() {
    let result = lex_snapshot("foo bar _test myVar Shape");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Ident             0..3    "foo"
    Ident             4..7    "bar"
    Ident             8..13   "_test"
    Ident            14..19   "myVar"
    Ident            20..25   "Shape"
    Eof              25..25   ""
    "#);
}

#[test]
fn test_keywords_vs_identifiers() {
    let result = lex_snapshot("let fn io pure type match if else true false letter iffoo");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Let               0..3    "let"
    Fn                4..6    "fn"
    Io                7..9    "io"
    Pure             10..14   "pure"
    Type             15..19   "type"
    Match            20..25   "match"
    If               26..28   "if"
    Else             29..33   "else"
    BoolLit          34..38   "true"
    BoolLit          39..44   "false"
    Ident            45..51   "letter"
    Ident            52..57   "iffoo"
    Eof              57..57   ""
    "#);
}

#[test]
fn test_all_operators() {
    let result = lex_snapshot("+ - * / ++ == != < <= > >= && || ! = -> |");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Plus              0..1    "+"
    Minus             2..3    "-"
    Star              4..5    "*"
    Slash             6..7    "/"
    PlusPlus          8..10   "++"
    EqEq             11..13   "=="
    BangEq           14..16   "!="
    Lt               17..18   "<"
    LtEq             19..21   "<="
    Gt               22..23   ">"
    GtEq             24..26   ">="
    AmpAmp           27..29   "&&"
    PipePipe         30..32   "||"
    Bang             33..34   "!"
    Eq               35..36   "="
    Arrow            37..39   "->"
    Pipe             40..41   "|"
    Eof              41..41   ""
    "#);
}

#[test]
fn test_string_with_escapes() {
    let result = lex_snapshot(r#""hello\nworld" "tab\there" "quote\"end" "back\\slash""#);
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    StringLit         0..14   "\"hello\\nworld\""
    StringLit        15..26   "\"tab\\there\""
    StringLit        27..39   "\"quote\\\"end\""
    StringLit        40..53   "\"back\\\\slash\""
    Eof              53..53   ""
    "#);
}

#[test]
fn test_unterminated_string() {
    let result = lex_snapshot("\"hello\nworld");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    StringLit         0..6    "\"hello"
    Ident             7..12   "world"
    Eof              12..12   ""

    === Diagnostics ===
    [E0002] error: unterminated string literal
    "#);
}

#[test]
fn test_unrecognized_character() {
    let result = lex_snapshot("let x = @;");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Let               0..3    "let"
    Ident             4..5    "x"
    Eq                6..7    "="
    Semi              9..10   ";"
    Eof              10..10   ""

    === Diagnostics ===
    [E0001] error: unrecognized character `@`
    "#);
}

#[test]
fn test_multiline_input() {
    let result = lex_snapshot("let x = 1;\nlet y = 2;\nlet z = x + y;");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Let               0..3    "let"
    Ident             4..5    "x"
    Eq                6..7    "="
    IntLit            8..9    "1"
    Semi              9..10   ";"
    Let              11..14   "let"
    Ident            15..16   "y"
    Eq               17..18   "="
    IntLit           19..20   "2"
    Semi             20..21   ";"
    Let              22..25   "let"
    Ident            26..27   "z"
    Eq               28..29   "="
    Ident            30..31   "x"
    Plus             32..33   "+"
    Ident            34..35   "y"
    Semi             35..36   ";"
    Eof              36..36   ""
    "#);
}

#[test]
fn test_comments() {
    let result = lex_snapshot("let x = 1; // this is a comment\nlet y = 2;");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Let               0..3    "let"
    Ident             4..5    "x"
    Eq                6..7    "="
    IntLit            8..9    "1"
    Semi              9..10   ";"
    Let              32..35   "let"
    Ident            36..37   "y"
    Eq               38..39   "="
    IntLit           40..41   "2"
    Semi             41..42   ";"
    Eof              42..42   ""
    "#);
}

#[test]
fn test_empty_input() {
    let result = lex_snapshot("");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Eof               0..0    ""
    "#);
}

#[test]
fn test_complete_function() {
    let result = lex_snapshot("fn add(a: Int, b: Int) -> Int {\n    a + b\n}");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Fn                0..2    "fn"
    Ident             3..6    "add"
    LParen            6..7    "("
    Ident             7..8    "a"
    Colon             8..9    ":"
    Ident            10..13   "Int"
    Comma            13..14   ","
    Ident            15..16   "b"
    Colon            16..17   ":"
    Ident            18..21   "Int"
    RParen           21..22   ")"
    Arrow            23..25   "->"
    Ident            26..29   "Int"
    LBrace           30..31   "{"
    Ident            36..37   "a"
    Plus             38..39   "+"
    Ident            40..41   "b"
    RBrace           42..43   "}"
    Eof              43..43   ""
    "#);
}

#[test]
fn test_consecutive_operators() {
    let result = lex_snapshot("!true==false!=true&&false||true");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Bang              0..1    "!"
    BoolLit           1..5    "true"
    EqEq              5..7    "=="
    BoolLit           7..12   "false"
    BangEq           12..14   "!="
    BoolLit          14..18   "true"
    AmpAmp           18..20   "&&"
    BoolLit          20..25   "false"
    PipePipe         25..27   "||"
    BoolLit          27..31   "true"
    Eof              31..31   ""
    "#);
}

#[test]
fn test_delimiters() {
    let result = lex_snapshot("(){}[],;:");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    LParen            0..1    "("
    RParen            1..2    ")"
    LBrace            2..3    "{"
    RBrace            3..4    "}"
    LBracket          4..5    "["
    RBracket          5..6    "]"
    Comma             6..7    ","
    Semi              7..8    ";"
    Colon             8..9    ":"
    Eof               9..9    ""
    "#);
}

#[test]
fn test_invalid_number_suffix() {
    let result = lex_snapshot("42abc");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    IntLit            0..5    "42abc"
    Eof               5..5    ""

    === Diagnostics ===
    [E0003] error: invalid number literal `42abc`
    "#);
}

#[test]
fn test_match_expression() {
    let result = lex_snapshot("match s {\n    Circle(r) -> 3.14 * r * r,\n    Rect(w, h) -> w * h,\n}");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Match             0..5    "match"
    Ident             6..7    "s"
    LBrace            8..9    "{"
    Ident            14..20   "Circle"
    LParen           20..21   "("
    Ident            21..22   "r"
    RParen           22..23   ")"
    Arrow            24..26   "->"
    FloatLit         27..31   "3.14"
    Star             32..33   "*"
    Ident            34..35   "r"
    Star             36..37   "*"
    Ident            38..39   "r"
    Comma            39..40   ","
    Ident            45..49   "Rect"
    LParen           49..50   "("
    Ident            50..51   "w"
    Comma            51..52   ","
    Ident            53..54   "h"
    RParen           54..55   ")"
    Arrow            56..58   "->"
    Ident            59..60   "w"
    Star             61..62   "*"
    Ident            63..64   "h"
    Comma            64..65   ","
    RBrace           66..67   "}"
    Eof              67..67   ""
    "#);
}

#[test]
fn test_type_declaration() {
    let result = lex_snapshot("type Shape =\n    | Circle(Float)\n    | Rect(Float, Float)");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Type              0..4    "type"
    Ident             5..10   "Shape"
    Eq               11..12   "="
    Pipe             17..18   "|"
    Ident            19..25   "Circle"
    LParen           25..26   "("
    Ident            26..31   "Float"
    RParen           31..32   ")"
    Pipe             37..38   "|"
    Ident            39..43   "Rect"
    LParen           43..44   "("
    Ident            44..49   "Float"
    Comma            49..50   ","
    Ident            51..56   "Float"
    RParen           56..57   ")"
    Eof              57..57   ""
    "#);
}

#[test]
fn test_string_concat_operator() {
    let result = lex_snapshot(r#""Hello, " ++ name"#);
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    StringLit         0..9    "\"Hello, \""
    PlusPlus         10..12   "++"
    Ident            13..17   "name"
    Eof              17..17   ""
    "#);
}

#[test]
fn test_io_function() {
    let result = lex_snapshot("io fn greet(name: String) -> Unit {}");
    insta::assert_snapshot!(result, @r#"
    === Tokens ===
    Io                0..2    "io"
    Fn                3..5    "fn"
    Ident             6..11   "greet"
    LParen           11..12   "("
    Ident            12..16   "name"
    Colon            16..17   ":"
    Ident            18..24   "String"
    RParen           24..25   ")"
    Arrow            26..28   "->"
    Ident            29..33   "Unit"
    LBrace           34..35   "{"
    RBrace           35..36   "}"
    Eof              36..36   ""
    "#);
}
