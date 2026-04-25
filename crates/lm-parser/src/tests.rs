//! Snapshot tests for the LM parser.

use super::*;
use lm_lexer::Lexer;

/// Helper: parse source and format AST + diagnostics for snapshot comparison.
fn parse_snapshot(source: &str) -> String {
    let (tokens, _lex_diags) = Lexer::new(source, 0).tokenize();
    let (program, diagnostics) = Parser::new(tokens).parse();

    let mut out = String::new();

    out.push_str("=== AST ===\n");
    out.push_str(&serde_json::to_string_pretty(&program).unwrap());
    out.push('\n');

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

// ---------------------------------------------------------------
// Expression parsing tests
// ---------------------------------------------------------------

#[test]
fn test_integer_literal() {
    let result = parse_snapshot("let x = 42;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_float_literal() {
    let result = parse_snapshot("let x = 3.14;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_string_literal() {
    let result = parse_snapshot(r#"let x = "hello";"#);
    insta::assert_snapshot!(result);
}

#[test]
fn test_bool_literal() {
    let result = parse_snapshot("let x = true;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_binary_ops() {
    let result = parse_snapshot("let x = 1 + 2 * 3;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_precedence_add_mul() {
    // 1 + 2 * 3 should be 1 + (2 * 3)
    let result = parse_snapshot("let x = 1 + 2 * 3;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_precedence_comparison() {
    // a + b < c * d should be (a + b) < (c * d)
    let result = parse_snapshot("let x = a + b < c * d;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_precedence_logical() {
    // a && b || c should be (a && b) || c
    let result = parse_snapshot("let x = a && b || c;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_unary_not() {
    let result = parse_snapshot("let x = !true;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_unary_neg() {
    let result = parse_snapshot("let x = -42;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_function_call() {
    let result = parse_snapshot("let x = add(1, 2);");
    insta::assert_snapshot!(result);
}

#[test]
fn test_nested_function_call() {
    let result = parse_snapshot("let x = f(g(1));");
    insta::assert_snapshot!(result);
}

#[test]
fn test_string_concat() {
    let result = parse_snapshot(r#"let x = "hello" ++ " " ++ "world";"#);
    insta::assert_snapshot!(result);
}

#[test]
fn test_if_else() {
    let result = parse_snapshot("fn f(x: Int) -> Int { if x > 0 { x } else { 0 } }");
    insta::assert_snapshot!(result);
}

#[test]
fn test_match_expr() {
    let result = parse_snapshot(
        "fn f(x: Int) -> String {
    match x {
        0 -> \"zero\",
        1 -> \"one\",
        _ -> \"other\",
    }
}",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn test_match_variant_pattern() {
    let result = parse_snapshot(
        "fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14 * r * r,
        Rect(w, h) -> w * h,
    }
}",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn test_block_expr() {
    let result = parse_snapshot("fn f() -> Int { let x = 1; let y = 2; x + y }");
    insta::assert_snapshot!(result);
}

// ---------------------------------------------------------------
// Function definition tests
// ---------------------------------------------------------------

#[test]
fn test_fn_no_annotations() {
    let result = parse_snapshot("fn add(a, b) { a + b }");
    insta::assert_snapshot!(result);
}

#[test]
fn test_fn_with_annotations() {
    let result = parse_snapshot("fn add(a: Int, b: Int) -> Int { a + b }");
    insta::assert_snapshot!(result);
}

#[test]
fn test_io_fn() {
    let result = parse_snapshot("io fn greet(name: String) -> Unit { print(name) }");
    insta::assert_snapshot!(result);
}

// ---------------------------------------------------------------
// Type definition tests
// ---------------------------------------------------------------

#[test]
fn test_type_def() {
    let result = parse_snapshot(
        "type Shape =
    | Circle(Float)
    | Rect(Float, Float)",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn test_type_def_with_type_params() {
    let result = parse_snapshot(
        "type Result<T, E> =
    | Ok(T)
    | Err(E)",
    );
    insta::assert_snapshot!(result);
}

// ---------------------------------------------------------------
// Error recovery tests
// ---------------------------------------------------------------

#[test]
fn test_error_missing_semicolon() {
    let result = parse_snapshot("let x = 42\nlet y = 10;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_error_unclosed_paren() {
    let result = parse_snapshot("let x = f(1, 2;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_error_unexpected_token() {
    let result = parse_snapshot("+ 42;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_error_missing_else() {
    let result = parse_snapshot("fn f(x: Int) -> Int { if x > 0 { x } }");
    insta::assert_snapshot!(result);
}

#[test]
fn test_multi_error_recovery() {
    let result = parse_snapshot("let x = ;\nlet y = ;\nlet z = 42;");
    insta::assert_snapshot!(result);
}

// ---------------------------------------------------------------
// Full program tests
// ---------------------------------------------------------------

#[test]
fn test_hello_example() {
    let source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/hello.lm"
    ))
    .unwrap();
    let result = parse_snapshot(&source);
    insta::assert_snapshot!(result);
}

#[test]
fn test_calculator_example() {
    let source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/calculator.lm"
    ))
    .unwrap();
    let result = parse_snapshot(&source);
    insta::assert_snapshot!(result);
}

#[test]
fn test_variant_construct() {
    let result = parse_snapshot("let x = Circle(5.0);");
    insta::assert_snapshot!(result);
}

#[test]
fn test_variant_construct_unit() {
    let result = parse_snapshot("let x = None;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_type_annotation_generic() {
    let result = parse_snapshot("fn safe_div(a: Int, b: Int) -> Result<Int, String> { a / b }");
    insta::assert_snapshot!(result);
}

#[test]
fn test_parenthesized_expr() {
    let result = parse_snapshot("let x = (1 + 2) * 3;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_left_associativity() {
    // 1 - 2 - 3 should be (1 - 2) - 3
    let result = parse_snapshot("let x = 1 - 2 - 3;");
    insta::assert_snapshot!(result);
}

#[test]
fn test_error_missing_arrow_in_match() {
    let result = parse_snapshot("fn f(x: Int) -> Int { match x { 0 \"zero\" } }");
    insta::assert_snapshot!(result);
}

#[test]
fn test_error_missing_eq_in_let() {
    let result = parse_snapshot("let x 42;");
    insta::assert_snapshot!(result);
}
