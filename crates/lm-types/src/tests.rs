//! Tests for the LM type system.

use crate::TypeChecker;
use lm_lexer::Lexer;
use lm_parser::Parser;

/// Helper: parse source and type-check, returning diagnostics.
fn check(source: &str) -> Vec<lm_diagnostics::Diagnostic> {
    let (tokens, _) = Lexer::new(source, 0).tokenize();
    let (program, parse_diags) = Parser::new(tokens).parse();
    // If there are parse errors, return them (don't type-check broken ASTs)
    if parse_diags
        .iter()
        .any(|d| d.severity == lm_diagnostics::Severity::Error)
    {
        return parse_diags;
    }
    let checker = TypeChecker::new();
    checker.check(&program)
}

/// Helper: assert no errors in diagnostics.
fn assert_no_errors(source: &str) {
    let diags = check(source);
    let errors: Vec<_> = diags
        .iter()
        .filter(|d| d.severity == lm_diagnostics::Severity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "expected no errors, got:\n{}",
        errors
            .iter()
            .map(|d| format!("  [{}] {}", d.code, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Helper: assert there's at least one error with the given code.
fn assert_has_error(source: &str, code: &str) {
    let diags = check(source);
    let has_it = diags
        .iter()
        .any(|d| d.code.0 == code && d.severity == lm_diagnostics::Severity::Error);
    assert!(
        has_it,
        "expected error {}, got:\n{}",
        code,
        diags
            .iter()
            .map(|d| format!("  [{}] {} {}", d.code, d.severity, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Helper: assert a warning with the given code exists.
fn assert_has_warning(source: &str, code: &str) {
    let diags = check(source);
    let has_it = diags
        .iter()
        .any(|d| d.code.0 == code && d.severity == lm_diagnostics::Severity::Warning);
    assert!(
        has_it,
        "expected warning {}, got:\n{}",
        code,
        diags
            .iter()
            .map(|d| format!("  [{}] {} {}", d.code, d.severity, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

// ---------------------------------------------------------------
// Type inference: positive cases
// ---------------------------------------------------------------

#[test]
fn test_literal_int() {
    assert_no_errors("let x = 42;");
}

#[test]
fn test_literal_float() {
    assert_no_errors("let x = 3.14;");
}

#[test]
fn test_literal_string() {
    assert_no_errors(r#"let x = "hello";"#);
}

#[test]
fn test_literal_bool() {
    assert_no_errors("let x = true;");
}

#[test]
fn test_binary_add_int() {
    assert_no_errors("let x = 1 + 2;");
}

#[test]
fn test_binary_add_float() {
    assert_no_errors("let x = 1.0 + 2.0;");
}

#[test]
fn test_binary_sub() {
    assert_no_errors("let x = 10 - 3;");
}

#[test]
fn test_binary_mul() {
    assert_no_errors("let x = 2 * 3;");
}

#[test]
fn test_binary_div() {
    assert_no_errors("let x = 10 / 2;");
}

#[test]
fn test_string_concat() {
    assert_no_errors(r#"let x = "hello" ++ " world";"#);
}

#[test]
fn test_comparison_eq() {
    assert_no_errors("let x = 1 == 2;");
}

#[test]
fn test_comparison_lt() {
    assert_no_errors("let x = 1 < 2;");
}

#[test]
fn test_logical_and() {
    assert_no_errors("let x = true && false;");
}

#[test]
fn test_logical_or() {
    assert_no_errors("let x = true || false;");
}

#[test]
fn test_unary_not() {
    assert_no_errors("let x = !true;");
}

#[test]
fn test_unary_neg() {
    assert_no_errors("let x = -42;");
}

#[test]
fn test_fn_def_and_call() {
    assert_no_errors(
        "fn add(a: Int, b: Int) -> Int { a + b }
         let x = add(1, 2);",
    );
}

#[test]
fn test_fn_no_return_annotation() {
    assert_no_errors(
        "fn double(x: Int) -> Int { x * 2 }
         let y = double(5);",
    );
}

#[test]
fn test_if_else_type_inference() {
    assert_no_errors("let x = if true { 1 } else { 2 };");
}

#[test]
fn test_let_binding_in_expr() {
    assert_no_errors(
        "fn f() -> Int {
            let x = 10;
            x + 5
         }",
    );
}

#[test]
fn test_block_type_is_last_expr() {
    assert_no_errors(
        "fn f() -> Int {
            let a = 1;
            let b = 2;
            a + b
         }",
    );
}

#[test]
fn test_match_on_int_with_wildcard() {
    assert_no_errors(
        "fn f(x: Int) -> String {
            match x {
                0 -> \"zero\",
                _ -> \"other\",
            }
         }",
    );
}

#[test]
fn test_adt_construction_and_matching() {
    assert_no_errors(
        "type Shape = | Circle(Float) | Rect(Float, Float)

         fn area(s: Shape) -> Float {
            match s {
                Circle(r) -> 3.14 * r * r,
                Rect(w, h) -> w * h,
            }
         }

         let c = Circle(5.0);
         let a = area(c);",
    );
}

#[test]
fn test_option_some_none() {
    assert_no_errors(
        "let x = Some(42);
         let y = None;",
    );
}

#[test]
fn test_result_ok_err() {
    assert_no_errors(
        "let x = Ok(42);
         let y = Err(\"oops\");",
    );
}

#[test]
fn test_nested_expressions() {
    assert_no_errors("let x = (1 + 2) * (3 + 4);");
}

#[test]
fn test_hello_example() {
    assert_no_errors(
        "fn add(a: Int, b: Int) -> Int {
            a + b
         }

         io fn greet(name: String) -> Unit {
            print(\"Hello, \" ++ name ++ \"!\")
         }

         let x = 10;
         let y = add(x, 5);",
    );
}

#[test]
fn test_calculator_example() {
    assert_no_errors(
        "type Shape =
            | Circle(Float)
            | Rect(Float, Float)

         fn area(s: Shape) -> Float {
            match s {
                Circle(r) -> 3.14159 * r * r,
                Rect(w, h) -> w * h,
            }
         }

         fn describe(s: Shape) -> String {
            match s {
                Circle(r) -> \"circle with radius \" ++ to_string(r),
                Rect(w, h) -> \"rectangle \" ++ to_string(w) ++ \" x \" ++ to_string(h),
            }
         }

         fn safe_div(a: Int, b: Int) -> Result<Int, String> {
            match b {
                0 -> Err(\"division by zero\"),
                _ -> Ok(a / b),
            }
         }

         let c = Circle(5.0);
         let r = Rect(3.0, 4.0);
         let a1 = area(c);
         let a2 = area(r);
         let result = safe_div(10, 0);",
    );
}

// ---------------------------------------------------------------
// Type errors: negative cases
// ---------------------------------------------------------------

#[test]
fn test_error_add_int_string() {
    assert_has_error(r#"let x = 1 + "hello";"#, "E0206");
}

#[test]
fn test_error_undefined_variable() {
    assert_has_error("let x = y;", "E0201");
}

#[test]
fn test_error_wrong_arg_count() {
    assert_has_error(
        "fn add(a: Int, b: Int) -> Int { a + b }
         let x = add(1);",
        "E0204",
    );
}

#[test]
fn test_error_string_plus_int() {
    assert_has_error(r#"let x = "hi" + 1;"#, "E0206");
}

#[test]
fn test_error_if_branches_different_types() {
    assert_has_error(r#"let x = if true { 1 } else { "hi" };"#, "E0200");
}

#[test]
fn test_error_undefined_type() {
    assert_has_error("fn f(x: Foo) -> Int { 42 }", "E0202");
}

#[test]
fn test_error_undefined_variant() {
    assert_has_error("let x = Banana(42);", "E0203");
}

#[test]
fn test_error_not_on_int() {
    assert_has_error("let x = !42;", "E0206");
}

#[test]
fn test_error_concat_int() {
    assert_has_error("let x = 1 ++ 2;", "E0206");
}

#[test]
fn test_error_and_on_int() {
    assert_has_error("let x = 1 && 2;", "E0206");
}

#[test]
fn test_error_return_type_mismatch() {
    assert_has_error(
        r#"fn f() -> Int { "hello" }"#,
        "E0200",
    );
}

// ---------------------------------------------------------------
// Effect checking
// ---------------------------------------------------------------

#[test]
fn test_effect_pure_calling_pure_ok() {
    assert_no_errors(
        "fn add(a: Int, b: Int) -> Int { a + b }
         fn double(x: Int) -> Int { add(x, x) }",
    );
}

#[test]
fn test_effect_io_calling_pure_ok() {
    assert_no_errors(
        "fn add(a: Int, b: Int) -> Int { a + b }
         io fn show(x: Int) -> Unit { print(to_string(add(x, 0))) }",
    );
}

#[test]
fn test_effect_io_calling_io_ok() {
    assert_no_errors(
        "io fn greet(name: String) -> Unit { print(name) }
         io fn greet2(name: String) -> Unit { greet(name) }",
    );
}

#[test]
fn test_effect_pure_calling_io_error() {
    assert_has_error(
        "fn bad() -> Unit { print(\"oops\") }",
        "E0300",
    );
}

#[test]
fn test_effect_transitive_io() {
    // greet is io, so if double_greet is pure and calls greet, it's an error
    assert_has_error(
        "io fn greet(name: String) -> Unit { print(name) }
         fn double_greet(name: String) -> Unit { greet(name) }",
        "E0300",
    );
}

// ---------------------------------------------------------------
// Exhaustiveness checking
// ---------------------------------------------------------------

#[test]
fn test_exhaustive_adt_all_variants() {
    assert_no_errors(
        "type Color = | Red | Green | Blue

         fn name(c: Color) -> String {
            match c {
                Red -> \"red\",
                Green -> \"green\",
                Blue -> \"blue\",
            }
         }",
    );
}

#[test]
fn test_nonexhaustive_adt_missing_variant() {
    assert_has_error(
        "type Color = | Red | Green | Blue

         fn name(c: Color) -> String {
            match c {
                Red -> \"red\",
                Green -> \"green\",
            }
         }",
        "E0400",
    );
}

#[test]
fn test_exhaustive_with_wildcard() {
    assert_no_errors(
        "type Color = | Red | Green | Blue

         fn name(c: Color) -> String {
            match c {
                Red -> \"red\",
                _ -> \"other\",
            }
         }",
    );
}

#[test]
fn test_nonexhaustive_bool_missing() {
    assert_has_error(
        "fn f(b: Bool) -> Int {
            match b {
                true -> 1,
            }
         }",
        "E0400",
    );
}

#[test]
fn test_exhaustive_bool_both() {
    assert_no_errors(
        "fn f(b: Bool) -> Int {
            match b {
                true -> 1,
                false -> 0,
            }
         }",
    );
}

#[test]
fn test_unreachable_pattern_after_wildcard() {
    assert_has_warning(
        "fn f(x: Int) -> Int {
            match x {
                _ -> 0,
                1 -> 1,
            }
         }",
        "E0401",
    );
}

#[test]
fn test_nonexhaustive_int_no_wildcard() {
    assert_has_error(
        "fn f(x: Int) -> Int {
            match x {
                0 -> 0,
                1 -> 1,
            }
         }",
        "E0400",
    );
}

#[test]
fn test_nonexhaustive_option_missing_none() {
    assert_has_error(
        "fn f(x: Int) -> Int {
            let opt = Some(x);
            match opt {
                Some(v) -> v,
            }
         }",
        "E0400",
    );
}

#[test]
fn test_exhaustive_result() {
    assert_no_errors(
        "fn safe_div(a: Int, b: Int) -> Result<Int, String> {
            match b {
                0 -> Err(\"division by zero\"),
                _ -> Ok(a / b),
            }
         }

         fn handle(r: Result<Int, String>) -> Int {
            match r {
                Ok(v) -> v,
                Err(e) -> 0,
            }
         }",
    );
}
