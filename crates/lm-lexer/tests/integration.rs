//! Integration tests verifying the lexer works end-to-end with diagnostics.

use lm_diagnostics::{Severity, Span};
use lm_lexer::{Lexer, TokenKind};

#[test]
fn lex_hello_example() {
    let source = include_str!("../../../examples/hello.lm");
    let (tokens, diagnostics) = Lexer::new(source, 0).tokenize();

    assert!(diagnostics.is_empty(), "hello.lm should have no errors");

    // Check that we get the expected keywords
    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    assert!(kinds.contains(&TokenKind::Fn));
    assert!(kinds.contains(&TokenKind::Let));
    assert!(kinds.contains(&TokenKind::Io));
    assert!(kinds.contains(&TokenKind::PlusPlus));
    assert!(kinds.contains(&TokenKind::Eof));
}

#[test]
fn lex_calculator_example() {
    let source = include_str!("../../../examples/calculator.lm");
    let (tokens, diagnostics) = Lexer::new(source, 0).tokenize();

    assert!(diagnostics.is_empty(), "calculator.lm should have no errors");

    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    assert!(kinds.contains(&TokenKind::Type));
    assert!(kinds.contains(&TokenKind::Pipe));
    assert!(kinds.contains(&TokenKind::Match));
    assert!(kinds.contains(&TokenKind::Arrow));
    assert!(kinds.contains(&TokenKind::FloatLit));
}

#[test]
fn lex_errors_example_produces_diagnostics() {
    let source = include_str!("../../../examples/errors.lm");
    let (_, diagnostics) = Lexer::new(source, 0).tokenize();

    assert!(!diagnostics.is_empty(), "errors.lm should produce diagnostics");

    // Should have at least one of each error type we planted
    let codes: Vec<&str> = diagnostics.iter().map(|d| d.code.0.as_str()).collect();
    assert!(codes.contains(&"E0003"), "expected E0003 for `42abc`");
    assert!(codes.contains(&"E0002"), "expected E0002 for unterminated strings");
    assert!(codes.contains(&"E0001"), "expected E0001 for unrecognized `@`");
}

#[test]
fn diagnostic_json_roundtrip() {
    let source = "let @x = 1;";
    let (_, diagnostics) = Lexer::new(source, 0).tokenize();

    assert_eq!(diagnostics.len(), 1);
    let diag = &diagnostics[0];

    // Serialize to JSON
    let json = diag.to_json();

    // Deserialize back
    let parsed: lm_diagnostics::Diagnostic = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.code, diag.code);
    assert_eq!(parsed.severity, Severity::Error);
    assert_eq!(parsed.message, diag.message);
}

#[test]
fn diagnostic_human_render_contains_source() {
    let source = "let @x = 1;";
    let (_, diagnostics) = Lexer::new(source, 0).tokenize();

    let rendered = diagnostics[0].render_plain(source, "test.lm");
    assert!(rendered.contains("test.lm:1:5"));
    assert!(rendered.contains("let @x = 1;"));
    assert!(rendered.contains("E0001"));
}

#[test]
fn empty_source_produces_only_eof() {
    let (tokens, diagnostics) = Lexer::new("", 0).tokenize();
    assert!(diagnostics.is_empty());
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

#[test]
fn whitespace_only_source() {
    let (tokens, diagnostics) = Lexer::new("   \n\t\n  ", 0).tokenize();
    assert!(diagnostics.is_empty());
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

#[test]
fn span_positions_are_correct() {
    let source = "let x = 42;";
    let (tokens, _) = Lexer::new(source, 0).tokenize();

    // "let" at 0..3
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[0].span, Span::new(0, 0, 3));
    assert_eq!(tokens[0].text, "let");

    // "x" at 4..5
    assert_eq!(tokens[1].kind, TokenKind::Ident);
    assert_eq!(tokens[1].span, Span::new(0, 4, 5));

    // "=" at 6..7
    assert_eq!(tokens[2].kind, TokenKind::Eq);
    assert_eq!(tokens[2].span, Span::new(0, 6, 7));

    // "42" at 8..10
    assert_eq!(tokens[3].kind, TokenKind::IntLit);
    assert_eq!(tokens[3].span, Span::new(0, 8, 10));

    // ";" at 10..11
    assert_eq!(tokens[4].kind, TokenKind::Semi);
    assert_eq!(tokens[4].span, Span::new(0, 10, 11));
}
