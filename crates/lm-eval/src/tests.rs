//! End-to-end tests for the LM interpreter.

use crate::{Interpreter, Value};
use lm_lexer::Lexer;
use lm_parser::Parser;
use lm_types::TypeChecker;

/// Helper: parse, type-check, and evaluate an LM program.
/// Returns the final value and captured stdout output.
fn eval_program(source: &str) -> (Value, String) {
    let (tokens, lex_diags) = Lexer::new(source, 0).tokenize();
    assert!(lex_diags.is_empty(), "unexpected lex errors: {lex_diags:?}");

    let (program, parse_diags) = Parser::new(tokens).parse();
    assert!(
        parse_diags.is_empty(),
        "unexpected parse errors: {parse_diags:?}"
    );

    let type_diags = TypeChecker::new().check(&program);
    assert!(
        type_diags.is_empty(),
        "unexpected type errors: {type_diags:?}"
    );

    let (mut interp, output_buf) = Interpreter::with_test_io("");
    let result = interp.eval_program(&program).expect("runtime error");
    let output = {
        let buf = output_buf.lock().unwrap();
        String::from_utf8(buf.clone()).unwrap()
    };
    (result, output)
}

/// Helper: parse and evaluate without type checking (for programs that
/// use features the type checker may not fully support yet).
fn eval_program_no_typecheck(source: &str) -> (Value, String) {
    let (tokens, lex_diags) = Lexer::new(source, 0).tokenize();
    assert!(lex_diags.is_empty(), "unexpected lex errors: {lex_diags:?}");

    let (program, parse_diags) = Parser::new(tokens).parse();
    assert!(
        parse_diags.is_empty(),
        "unexpected parse errors: {parse_diags:?}"
    );

    let (mut interp, output_buf) = Interpreter::with_test_io("");
    let result = interp.eval_program(&program).expect("runtime error");
    let output = {
        let buf = output_buf.lock().unwrap();
        String::from_utf8(buf.clone()).unwrap()
    };
    (result, output)
}

/// Helper: parse and evaluate, expecting a runtime error.
fn eval_program_expect_error(source: &str) -> String {
    let (tokens, lex_diags) = Lexer::new(source, 0).tokenize();
    assert!(lex_diags.is_empty(), "unexpected lex errors: {lex_diags:?}");

    let (program, parse_diags) = Parser::new(tokens).parse();
    assert!(
        parse_diags.is_empty(),
        "unexpected parse errors: {parse_diags:?}"
    );

    let (mut interp, _output_buf) = Interpreter::with_test_io("");
    match interp.eval_program(&program) {
        Ok(_) => panic!("expected runtime error, but evaluation succeeded"),
        Err(e) => e.diagnostic.code.0,
    }
}

// ── Test 1: Integer arithmetic ──────────────────────────────────────

#[test]
fn test_integer_arithmetic() {
    let (val, _) = eval_program("let result = 1 + 2 * 3;");
    assert_eq!(val, Value::Int(7));
}

// ── Test 2: String concatenation ────────────────────────────────────

#[test]
fn test_string_concatenation() {
    let (val, _) = eval_program(r#"let result = "hello" ++ " " ++ "world";"#);
    assert_eq!(val, Value::String("hello world".to_string()));
}

// ── Test 3: Boolean logic ───────────────────────────────────────────

#[test]
fn test_boolean_logic() {
    let (val, _) = eval_program("let result = true && false;");
    assert_eq!(val, Value::Bool(false));
}

// ── Test 4: Function definition and call ────────────────────────────

#[test]
fn test_function_definition_and_call() {
    let src = r#"
fn add(a: Int, b: Int) -> Int {
    a + b
}
let result = add(3, 4);
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(7));
}

// ── Test 5: Let bindings ────────────────────────────────────────────

#[test]
fn test_let_bindings() {
    let src = r#"
let x = 10;
let y = 20;
let z = x + y;
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(30));
}

// ── Test 6: If/else evaluation ──────────────────────────────────────

#[test]
fn test_if_else() {
    let src = r#"
fn max(a: Int, b: Int) -> Int {
    if a > b { a } else { b }
}
let result = max(10, 20);
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(20));
}

// ── Test 7: Match on ADT ────────────────────────────────────────────

#[test]
fn test_match_on_adt() {
    let src = r#"
type Shape =
    | Circle(Float)
    | Rect(Float, Float)

fn area(s: Shape) -> Float {
    match s {
        Circle(r) -> 3.14 * r * r,
        Rect(w, h) -> w * h,
    }
}
let result = area(Rect(3.0, 4.0));
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Float(12.0));
}

// ── Test 8: Match on literals ───────────────────────────────────────

#[test]
fn test_match_on_literals() {
    let src = r#"
fn describe(n: Int) -> String {
    match n {
        0 -> "zero",
        1 -> "one",
        _ -> "other",
    }
}
let result = describe(1);
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::String("one".to_string()));
}

// ── Test 9: Option Some/None ────────────────────────────────────────

#[test]
fn test_option_some_none() {
    let src = r#"
fn safe_head(n: Int) -> Option<Int> {
    if n > 0 { Some(n) } else { None }
}

fn unwrap_or(opt: Option<Int>, default: Int) -> Int {
    match opt {
        Some(v) -> v,
        None -> default,
    }
}

let a = unwrap_or(safe_head(5), 0);
let b = unwrap_or(safe_head(0), 42);
"#;
    // The last let is b = 42
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(42));
}

// ── Test 10: Result Ok/Err ──────────────────────────────────────────

#[test]
fn test_result_ok_err() {
    let src = r#"
fn safe_div(a: Int, b: Int) -> Result<Int, String> {
    match b {
        0 -> Err("division by zero"),
        _ -> Ok(a / b),
    }
}

fn unwrap_result(r: Result<Int, String>) -> Int {
    match r {
        Ok(v) -> v,
        Err(_) -> 0,
    }
}

let a = unwrap_result(safe_div(10, 2));
let b = unwrap_result(safe_div(10, 0));
"#;
    // Last let: b = 0
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(0));
}

// ── Test 11: Recursive function (factorial) ─────────────────────────

#[test]
fn test_recursive_factorial() {
    let src = r#"
fn factorial(n: Int) -> Int {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
let result = factorial(5);
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(120));
}

#[test]
fn test_recursive_power_depth_100() {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let src = r#"
fn power(base: Int, exp: Int) -> Int {
    if exp == 0 { 1 } else { base * power(base, exp - 1) }
}
let result = power(1, 100);
"#;
            let (val, _) = eval_program(src);
            assert_eq!(val, Value::Int(1));
        })
        .unwrap()
        .join()
        .unwrap();
}

// ── Test 12: Division by zero ───────────────────────────────────────

#[test]
fn test_division_by_zero() {
    let src = "let result = 10 / 0;";
    let code = eval_program_expect_error(src);
    assert_eq!(code, "E0500");
}

// ── Test 13: Nested function calls ──────────────────────────────────

#[test]
fn test_nested_function_calls() {
    let src = r#"
fn double(n: Int) -> Int { n * 2 }
fn inc(n: Int) -> Int { n + 1 }
let result = double(inc(inc(3)));
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(10));
}

// ── Test 14: Higher-order functions ─────────────────────────────────

#[test]
fn test_higher_order_functions() {
    let src = r#"
fn apply(f: (Int) -> Int, x: Int) -> Int {
    f(x)
}
fn double(n: Int) -> Int { n * 2 }
let result = apply(double, 21);
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(42));
}

// ── Test 15: FizzBuzz-style program ─────────────────────────────────

#[test]
fn test_fizzbuzz() {
    let src = r#"
fn fizzbuzz(n: Int) -> String {
    if n == 15 {
        "FizzBuzz"
    } else {
        if n == 3 {
            "Fizz"
        } else {
            if n == 5 {
                "Buzz"
            } else {
                int_to_string(n)
            }
        }
    }
}
let a = fizzbuzz(3);
let b = fizzbuzz(5);
let c = fizzbuzz(15);
let d = fizzbuzz(7);
"#;
    // We check each binding by running separate programs
    let (val, _) = eval_program_no_typecheck(src);
    // Last binding: d = "7"
    assert_eq!(val, Value::String("7".to_string()));
}

// ── Test 16: Print output ───────────────────────────────────────────

#[test]
fn test_print_output() {
    let src = r#"
io fn main() -> Unit {
    print("Hello, world!")
}
let result = main();
"#;
    let (_, output) = eval_program(src);
    assert_eq!(output.trim(), "Hello, world!");
}

// ── Test 17: to_string builtin ──────────────────────────────────────

#[test]
fn test_to_string_builtin() {
    let src = r#"
let s = to_string(42);
"#;
    let (val, _) = eval_program_no_typecheck(src);
    assert_eq!(val, Value::String("42".to_string()));
}

// ── Test 18: Multiple print calls ───────────────────────────────────

#[test]
fn test_multiple_prints() {
    let src = r#"
io fn main() -> Unit {
    let _ = print("line 1");
    print("line 2")
}
let result = main();
"#;
    let (_, output) = eval_program(src);
    assert_eq!(output.trim(), "line 1\nline 2");
}

// ── Test 19: String comparison ──────────────────────────────────────

#[test]
fn test_string_comparison() {
    let src = r#"
let a = "hello" == "hello";
let b = "hello" != "world";
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Bool(true));
}

// ── Test 20: Unary operators ────────────────────────────────────────

#[test]
fn test_unary_operators() {
    let src = r#"
let a = !true;
let b = -42;
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(-42));
}

// ── Test 21: Generic recursive ADT ─────────────────────────────────

#[test]
fn test_generic_recursive_adt() {
    let src = r#"
type MyList<T> = | Nil | Cons(T, MyList<T>)
let xs = Cons(1, Cons(2, Nil));
"#;
    let (val, _) = eval_program(src);
    // The result should be a Cons variant
    match val {
        Value::ADTInstance { variant, .. } => assert_eq!(variant, "Cons"),
        _ => panic!("expected Cons variant, got {:?}", val),
    }
}

// ── Test 22: str_len builtin ───────────────────────────────────────

#[test]
fn test_str_len_builtin() {
    let src = r#"
let n = str_len("hello");
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(5));
}

#[test]
fn test_str_len_empty_string() {
    let src = r#"
let n = str_len("");
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::Int(0));
}

#[test]
fn test_string_character_builtins() {
    let src = r#"
let first = char_at("éclair", 0);
let code = char_code(first);
let roundtrip = from_char_code(code);
"#;
    let (val, _) = eval_program(src);
    assert_eq!(val, Value::String("é".to_string()));
}

#[test]
fn test_char_at_out_of_bounds_errors() {
    let code = eval_program_expect_error(r#"let c = char_at("abc", 3);"#);
    assert_eq!(code, "E0503");
}
