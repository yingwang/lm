//! Parser for the LM programming language.
//!
//! This crate implements a hand-written recursive descent parser with Pratt
//! parsing for operator precedence. It transforms a token stream from
//! [`lm_lexer`] into an abstract syntax tree ([`ast::Program`]).
//!
//! # Error recovery
//!
//! When a parse error occurs, the parser tries to synchronize to the next
//! statement boundary (`;`, `}`, `fn`, `type`, `let` at top level) and
//! continues parsing to collect multiple diagnostics.
//!
//! # Error codes
//!
//! | Code  | Description                                  |
//! |-------|----------------------------------------------|
//! | E0100 | Unexpected token                             |
//! | E0101 | Expected expression                          |
//! | E0102 | Expected type annotation                     |
//! | E0103 | Expected pattern                             |
//! | E0104 | Unclosed delimiter (paren, brace, bracket)   |
//! | E0105 | Missing semicolon                            |
//! | E0106 | Expected `->` in match arm                   |
//! | E0107 | Expected `=` in let binding                  |
//! | E0108 | Missing else branch                          |
//!
//! # Usage
//!
//! ```
//! use lm_lexer::Lexer;
//! use lm_parser::Parser;
//!
//! let source = "let x = 42;";
//! let (tokens, lex_diags) = Lexer::new(source, 0).tokenize();
//! let (program, parse_diags) = Parser::new(tokens).parse();
//! ```

pub mod ast;

#[cfg(test)]
mod tests;

use ast::*;
use lm_diagnostics::{Diagnostic, Label, Span};
use lm_lexer::{Token, TokenKind};

/// Binding power for Pratt parsing (left and right).
#[derive(Debug, Clone, Copy)]
struct Bp {
    left: u8,
    right: u8,
}

/// Left-associative binding power.
const fn left_assoc(p: u8) -> Bp {
    Bp {
        left: p,
        right: p + 1,
    }
}

/// Prefix binding power (no left side).
const fn prefix(p: u8) -> Bp {
    Bp { left: 0, right: p }
}

/// Get the infix binding power for a token kind, if it is an infix operator.
fn infix_bp(kind: TokenKind) -> Option<Bp> {
    match kind {
        TokenKind::PipePipe => Some(left_assoc(2)),
        TokenKind::AmpAmp => Some(left_assoc(4)),
        TokenKind::EqEq | TokenKind::BangEq => Some(left_assoc(6)),
        TokenKind::Lt | TokenKind::LtEq | TokenKind::Gt | TokenKind::GtEq => Some(left_assoc(8)),
        TokenKind::PlusPlus => Some(left_assoc(10)),
        TokenKind::Plus | TokenKind::Minus => Some(left_assoc(12)),
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(left_assoc(14)),
        _ => None,
    }
}

/// Get the prefix binding power for a token kind.
fn prefix_bp(kind: TokenKind) -> Option<Bp> {
    match kind {
        TokenKind::Bang | TokenKind::Minus => Some(prefix(16)),
        _ => None,
    }
}

/// Map a token kind to its binary operator.
fn token_to_binop(kind: TokenKind) -> Option<BinOp> {
    match kind {
        TokenKind::Plus => Some(BinOp::Add),
        TokenKind::Minus => Some(BinOp::Sub),
        TokenKind::Star => Some(BinOp::Mul),
        TokenKind::Slash => Some(BinOp::Div),
        TokenKind::Percent => Some(BinOp::Mod),
        TokenKind::PlusPlus => Some(BinOp::Concat),
        TokenKind::EqEq => Some(BinOp::Eq),
        TokenKind::BangEq => Some(BinOp::Ne),
        TokenKind::Lt => Some(BinOp::Lt),
        TokenKind::LtEq => Some(BinOp::Le),
        TokenKind::Gt => Some(BinOp::Gt),
        TokenKind::GtEq => Some(BinOp::Ge),
        TokenKind::AmpAmp => Some(BinOp::And),
        TokenKind::PipePipe => Some(BinOp::Or),
        _ => None,
    }
}

/// Map a token kind to its unary operator.
fn token_to_unop(kind: TokenKind) -> Option<UnOp> {
    match kind {
        TokenKind::Bang => Some(UnOp::Not),
        TokenKind::Minus => Some(UnOp::Neg),
        _ => None,
    }
}

/// The LM parser.
///
/// Transforms a token stream into an abstract syntax tree, collecting
/// diagnostics for any parse errors encountered.
pub struct Parser {
    /// The token stream.
    tokens: Vec<Token>,
    /// Current position in the token stream.
    pos: usize,
    /// Collected diagnostics.
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    /// Create a new parser from a token stream.
    ///
    /// The token stream should end with an `Eof` token (as produced by the lexer).
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            diagnostics: Vec::new(),
        }
    }

    /// Parse the token stream into a [`Program`].
    ///
    /// Returns the (potentially partial) AST and any diagnostics encountered.
    pub fn parse(mut self) -> (Program, Vec<Diagnostic>) {
        let start_span = self.current_span();
        let mut decls = Vec::new();

        while !self.at_eof() {
            match self.parse_decl() {
                Some(decl) => decls.push(decl),
                None => {
                    // Error recovery: skip to next declaration boundary
                    self.synchronize_top_level();
                }
            }
        }

        let end_span = self.current_span();
        let span = if decls.is_empty() {
            start_span
        } else {
            start_span.merge(end_span)
        };

        (Program { decls, span }, self.diagnostics)
    }

    // ---------------------------------------------------------------
    // Token stream helpers
    // ---------------------------------------------------------------

    /// Get the current token.
    fn current(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or_else(|| self.tokens.last().expect("token stream must have Eof"))
    }

    /// Get the span of the current token.
    fn current_span(&self) -> Span {
        self.current().span
    }

    /// Check if we are at the end of the token stream.
    fn at_eof(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    /// Check if the current token is of the given kind.
    fn check(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    /// Peek ahead at the next token's kind.
    #[allow(dead_code)]
    fn peek_kind(&self) -> TokenKind {
        self.tokens
            .get(self.pos + 1)
            .map(|t| t.kind)
            .unwrap_or(TokenKind::Eof)
    }

    /// Advance and return the consumed token.
    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if tok.kind != TokenKind::Eof {
            self.pos += 1;
        }
        tok
    }

    /// If the current token matches `kind`, advance and return `true`.
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume a token of the expected kind, or emit a diagnostic.
    ///
    /// Returns `true` if the token was consumed, `false` if not.
    fn expect(&mut self, kind: TokenKind, error_code: &str, message: &str) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error(error_code, message, span)
                    .with_label(Label::new(span, format!("expected `{}`", kind.name()))),
            );
            false
        }
    }

    /// Get the previous token (the one just consumed).
    fn previous(&self) -> &Token {
        &self.tokens[self.pos.saturating_sub(1)]
    }

    // ---------------------------------------------------------------
    // Error recovery
    // ---------------------------------------------------------------

    /// Skip tokens until we reach a likely top-level declaration boundary.
    fn synchronize_top_level(&mut self) {
        while !self.at_eof() {
            // If we just passed a semicolon or closing brace, stop.
            if self.previous().kind == TokenKind::Semi || self.previous().kind == TokenKind::RBrace
            {
                return;
            }

            // If the current token starts a new declaration, stop.
            match self.current().kind {
                TokenKind::Fn | TokenKind::Type | TokenKind::Let | TokenKind::Io => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    // ---------------------------------------------------------------
    // Top-level declarations
    // ---------------------------------------------------------------

    /// Parse a top-level declaration.
    fn parse_decl(&mut self) -> Option<Decl> {
        match self.current().kind {
            TokenKind::Fn => self.parse_fn_def(Effect::Pure),
            TokenKind::Io => {
                let start = self.current_span();
                self.advance(); // eat `io`
                if self.check(TokenKind::Fn) {
                    self.parse_fn_def_with_effect(Effect::Io, start)
                } else {
                    let span = self.current_span();
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0100",
                            "expected `fn` after `io`",
                            span,
                        )
                        .with_label(Label::new(span, "expected `fn`"))
                        .with_help("`io` can only appear before `fn` to mark a function as IO-performing"),
                    );
                    None
                }
            }
            TokenKind::Pure => {
                let start = self.current_span();
                self.advance(); // eat `pure`
                if self.check(TokenKind::Fn) {
                    self.parse_fn_def_with_effect(Effect::Pure, start)
                } else {
                    let span = self.current_span();
                    self.diagnostics.push(
                        Diagnostic::error("E0100", "expected `fn` after `pure`", span)
                            .with_label(Label::new(span, "expected `fn`")),
                    );
                    None
                }
            }
            TokenKind::Type => self.parse_type_def(),
            TokenKind::Let => self.parse_let_def(),
            _ => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.diagnostics.push(
                    Diagnostic::error(
                        "E0100",
                        format!(
                            "unexpected token `{}`, expected a declaration (`fn`, `type`, or `let`)",
                            text
                        ),
                        span,
                    )
                    .with_label(Label::new(span, "unexpected token")),
                );
                self.advance();
                None
            }
        }
    }

    /// Parse a function definition (already determined to be pure).
    fn parse_fn_def(&mut self, effect: Effect) -> Option<Decl> {
        let start = self.current_span();
        self.parse_fn_def_with_effect(effect, start)
    }

    /// Parse a function definition with an explicit effect and start span.
    fn parse_fn_def_with_effect(&mut self, effect: Effect, start: Span) -> Option<Decl> {
        // Eat `fn`
        if !self.expect(TokenKind::Fn, "E0100", "expected `fn` keyword") {
            return None;
        }

        // Function name
        let name = if self.check(TokenKind::Ident) {
            let n = self.current().text.clone();
            self.advance();
            n
        } else {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0100", "expected function name", span)
                    .with_label(Label::new(span, "expected an identifier")),
            );
            return None;
        };

        // Parameter list
        if !self.expect(
            TokenKind::LParen,
            "E0104",
            "expected `(` after function name",
        ) {
            return None;
        }

        let params = self.parse_param_list();

        if !self.expect(
            TokenKind::RParen,
            "E0104",
            "unclosed `(` in function parameters",
        ) {
            // Try to recover
        }

        // Optional return type: `-> Type`
        let return_type = if self.eat(TokenKind::Arrow) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        // Body: block expression
        let body = self.parse_block_expr()?;
        let span = start.merge(body.span);

        Some(Decl {
            kind: DeclKind::FnDef {
                name,
                effect,
                params,
                return_type,
                body,
            },
            span,
        })
    }

    /// Parse a comma-separated parameter list (without the enclosing parens).
    fn parse_param_list(&mut self) -> Vec<Param> {
        let mut params = Vec::new();

        while !self.check(TokenKind::RParen) && !self.at_eof() {
            if let Some(param) = self.parse_param() {
                params.push(param);
            }

            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        params
    }

    /// Parse a single parameter: `name` or `name: Type`.
    fn parse_param(&mut self) -> Option<Param> {
        let start = self.current_span();

        if !self.check(TokenKind::Ident) {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0100", "expected parameter name", span)
                    .with_label(Label::new(span, "expected an identifier")),
            );
            return None;
        }

        let name = self.current().text.clone();
        self.advance();

        let type_annotation = if self.eat(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let span = start.merge(self.previous().span);
        Some(Param {
            name,
            type_annotation,
            span,
        })
    }

    /// Parse a type definition: `type Name = | Variant1(...) | Variant2(...)`.
    fn parse_type_def(&mut self) -> Option<Decl> {
        let start = self.current_span();
        self.advance(); // eat `type`

        // Type name
        let name = if self.check(TokenKind::Ident) {
            let n = self.current().text.clone();
            self.advance();
            n
        } else {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0100", "expected type name", span)
                    .with_label(Label::new(span, "expected an identifier")),
            );
            return None;
        };

        // Optional type parameters: `<T, U>`
        let type_params = if self.check(TokenKind::Lt) {
            self.parse_type_params()
        } else {
            Vec::new()
        };

        // `=`
        if !self.expect(TokenKind::Eq, "E0107", "expected `=` after type name") {
            return None;
        }

        // Variants: `| Variant1(...) | Variant2(...)`
        let mut variants = Vec::new();

        // The first `|` is optional
        self.eat(TokenKind::Pipe);

        loop {
            if self.at_eof() || self.is_decl_start() {
                break;
            }

            if let Some(variant) = self.parse_variant() {
                variants.push(variant);
            }

            if !self.eat(TokenKind::Pipe) {
                break;
            }
        }

        let span = if let Some(last) = variants.last() {
            start.merge(last.span)
        } else {
            start.merge(self.previous().span)
        };

        Some(Decl {
            kind: DeclKind::TypeDef {
                name,
                type_params,
                variants,
            },
            span,
        })
    }

    /// Parse type parameters: `<T, U>`.
    fn parse_type_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        self.advance(); // eat `<`

        loop {
            if self.check(TokenKind::Gt) || self.at_eof() {
                break;
            }

            if self.check(TokenKind::Ident) {
                params.push(self.current().text.clone());
                self.advance();
            }

            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::Gt, "E0104", "unclosed `<` in type parameters");
        params
    }

    /// Parse a single variant: `VariantName` or `VariantName(Type1, Type2)`.
    fn parse_variant(&mut self) -> Option<Variant> {
        let start = self.current_span();

        if !self.check(TokenKind::Ident) {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0100", "expected variant name", span)
                    .with_label(Label::new(span, "expected an identifier")),
            );
            return None;
        }

        let name = self.current().text.clone();
        self.advance();

        let fields = if self.eat(TokenKind::LParen) {
            let mut fields = Vec::new();
            while !self.check(TokenKind::RParen) && !self.at_eof() {
                if let Some(ty) = self.parse_type_annotation() {
                    fields.push(ty);
                }
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen, "E0104", "unclosed `(` in variant fields");
            fields
        } else {
            Vec::new()
        };

        let span = start.merge(self.previous().span);
        Some(Variant { name, fields, span })
    }

    /// Parse a top-level let binding: `let name = expr;` or `let name: Type = expr;`.
    fn parse_let_def(&mut self) -> Option<Decl> {
        let start = self.current_span();
        self.advance(); // eat `let`

        // Binding name
        let name = if self.check(TokenKind::Ident) {
            let n = self.current().text.clone();
            self.advance();
            n
        } else {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0100", "expected variable name after `let`", span)
                    .with_label(Label::new(span, "expected an identifier")),
            );
            return None;
        };

        // Optional type annotation
        let type_annotation = if self.eat(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        // `=`
        if !self.expect(TokenKind::Eq, "E0107", "expected `=` in let binding") {
            return None;
        }

        // Value expression
        let value = self.parse_expr()?;

        // `;`
        self.expect(TokenKind::Semi, "E0105", "expected `;` after let binding");

        let span = start.merge(self.previous().span);
        Some(Decl {
            kind: DeclKind::LetDef {
                name,
                type_annotation,
                value,
            },
            span,
        })
    }

    /// Check if the current token starts a top-level declaration.
    fn is_decl_start(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Fn | TokenKind::Type | TokenKind::Let | TokenKind::Io | TokenKind::Pure
        )
    }

    // ---------------------------------------------------------------
    // Type annotations
    // ---------------------------------------------------------------

    /// Parse a type annotation.
    fn parse_type_annotation(&mut self) -> Option<TypeAnnotation> {
        // Function type: `(T1, T2) -> T3`
        if self.check(TokenKind::LParen) {
            return self.parse_fn_type();
        }

        let start = self.current_span();

        if !self.check(TokenKind::Ident) {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0102", "expected type annotation", span)
                    .with_label(Label::new(span, "expected a type name")),
            );
            return None;
        }

        let name = self.current().text.clone();
        self.advance();

        // Check for type application: `Name<T1, T2>`
        if self.check(TokenKind::Lt) {
            self.advance(); // eat `<`
            let mut args = Vec::new();
            loop {
                if self.check(TokenKind::Gt) || self.at_eof() {
                    break;
                }
                if let Some(ty) = self.parse_type_annotation() {
                    args.push(ty);
                }
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::Gt, "E0104", "unclosed `<` in type application");
            let span = start.merge(self.previous().span);
            Some(TypeAnnotation {
                kind: TypeKind::App { name, args },
                span,
            })
        } else {
            let span = start.merge(self.previous().span);
            Some(TypeAnnotation {
                kind: TypeKind::Name { name },
                span,
            })
        }
    }

    /// Parse a function type: `(T1, T2) -> T3`.
    fn parse_fn_type(&mut self) -> Option<TypeAnnotation> {
        let start = self.current_span();
        self.advance(); // eat `(`

        let mut params = Vec::new();
        while !self.check(TokenKind::RParen) && !self.at_eof() {
            if let Some(ty) = self.parse_type_annotation() {
                params.push(ty);
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::RParen, "E0104", "unclosed `(` in function type");

        if !self.expect(TokenKind::Arrow, "E0102", "expected `->` in function type") {
            return None;
        }

        let ret = self.parse_type_annotation()?;
        let span = start.merge(ret.span);

        Some(TypeAnnotation {
            kind: TypeKind::Fn {
                params,
                ret: Box::new(ret),
            },
            span,
        })
    }

    // ---------------------------------------------------------------
    // Expressions (Pratt parsing)
    // ---------------------------------------------------------------

    /// Parse an expression.
    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_expr_bp(0)
    }

    /// Parse an expression with a minimum binding power.
    fn parse_expr_bp(&mut self, min_bp: u8) -> Option<Expr> {
        // Parse the left-hand side (prefix / atom)
        let mut lhs = self.parse_prefix()?;

        loop {
            // Check for postfix: function call
            if self.check(TokenKind::LParen) {
                // Function call has the highest binding power (18)
                if min_bp > 18 {
                    break;
                }
                lhs = self.parse_fn_call(lhs)?;
                continue;
            }

            // Check for infix operators
            let kind = self.current().kind;
            if let Some(bp) = infix_bp(kind) {
                if bp.left < min_bp {
                    break;
                }

                let op = token_to_binop(kind).unwrap();
                let op_span = self.current_span();
                self.advance(); // eat operator

                let rhs = match self.parse_expr_bp(bp.right) {
                    Some(e) => e,
                    None => {
                        // Error: expected expression after operator
                        let span = self.current_span();
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0101",
                                format!("expected expression after `{}`", kind.name()),
                                span,
                            )
                            .with_label(Label::new(op_span, "operator here"))
                            .with_label(Label::new(span, "expected an expression")),
                        );
                        return Some(lhs);
                    }
                };

                let span = lhs.span.merge(rhs.span);
                lhs = Expr {
                    kind: ExprKind::BinaryOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    span,
                };
                continue;
            }

            break;
        }

        Some(lhs)
    }

    /// Parse a prefix expression or atom.
    fn parse_prefix(&mut self) -> Option<Expr> {
        let kind = self.current().kind;

        // Unary prefix operators
        if let Some(bp) = prefix_bp(kind) {
            let op = token_to_unop(kind).unwrap();
            let start = self.current_span();
            self.advance(); // eat operator

            let operand = self.parse_expr_bp(bp.right)?;
            let span = start.merge(operand.span);

            return Some(Expr {
                kind: ExprKind::UnaryOp {
                    op,
                    operand: Box::new(operand),
                },
                span,
            });
        }

        self.parse_atom()
    }

    /// Parse an atomic expression (literals, identifiers, if, match, block, parenthesized).
    fn parse_atom(&mut self) -> Option<Expr> {
        match self.current().kind {
            // Integer literal
            TokenKind::IntLit => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.advance();
                let kind = match self.parse_i64_literal(&text, span) {
                    Some(value) => ExprKind::Literal {
                        value: LitValue::Int(value),
                    },
                    None => ExprKind::Error,
                };
                Some(Expr { kind, span })
            }

            // Float literal
            TokenKind::FloatLit => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.advance();
                let value = text.parse::<f64>().unwrap_or(0.0);
                Some(Expr {
                    kind: ExprKind::Literal {
                        value: LitValue::Float(value),
                    },
                    span,
                })
            }

            // String literal
            TokenKind::StringLit => {
                let span = self.current_span();
                let raw = self.current().text.clone();
                self.advance();
                // Strip surrounding quotes and process escapes
                let content = unescape_string(&raw);
                Some(Expr {
                    kind: ExprKind::Literal {
                        value: LitValue::String(content),
                    },
                    span,
                })
            }

            // Bool literal
            TokenKind::BoolLit => {
                let span = self.current_span();
                let value = self.current().text == "true";
                self.advance();
                Some(Expr {
                    kind: ExprKind::Literal {
                        value: LitValue::Bool(value),
                    },
                    span,
                })
            }

            // Identifier (variable reference, or variant constructor, or let expression keyword)
            TokenKind::Ident => self.parse_ident_or_variant(),

            // Let expression: `let x = value; body`
            TokenKind::Let => self.parse_let_expr(),

            // If-else expression
            TokenKind::If => self.parse_if_else(),

            // Match expression
            TokenKind::Match => self.parse_match(),

            // List literal: `[1, 2, 3]` or `[]`
            TokenKind::LBracket => self.parse_list_literal(),

            // Block expression: `{ ... }`
            TokenKind::LBrace => self.parse_block_expr(),

            // Parenthesized expression: `(expr)` or Unit literal `()`
            TokenKind::LParen => {
                let start = self.current_span();
                self.advance(); // eat `(`
                // Check for `()` — Unit literal
                if self.check(TokenKind::RParen) {
                    self.advance(); // eat `)`
                    let span = start.merge(self.previous().span);
                    Some(Expr {
                        kind: ExprKind::Literal { value: LitValue::Unit },
                        span,
                    })
                } else {
                    let inner = self.parse_expr()?;
                    if !self.expect(TokenKind::RParen, "E0104", "unclosed `(`") {
                        // Try to continue
                    }
                    let span = start.merge(self.previous().span);
                    Some(Expr {
                        kind: inner.kind,
                        span,
                    })
                }
            }

            _ => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.diagnostics.push(
                    Diagnostic::error(
                        "E0101",
                        format!("expected expression, found `{}`", text),
                        span,
                    )
                    .with_label(Label::new(span, "expected an expression")),
                );
                None
            }
        }
    }

    /// Parse an identifier, which might be a variant constructor.
    ///
    /// Heuristic: An identifier starting with an uppercase letter followed by `(`
    /// is parsed as a variant constructor.
    fn parse_ident_or_variant(&mut self) -> Option<Expr> {
        let span = self.current_span();
        let name = self.current().text.clone();
        self.advance();

        // Check if this looks like a variant constructor: uppercase name followed by `(`
        let is_upper = name
            .chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false);

        if is_upper && self.check(TokenKind::LParen) {
            // Variant construction: `Circle(5.0)`, `Ok(42)`
            self.advance(); // eat `(`
            let mut args = Vec::new();
            while !self.check(TokenKind::RParen) && !self.at_eof() {
                if let Some(arg) = self.parse_expr() {
                    args.push(arg);
                }
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(
                TokenKind::RParen,
                "E0104",
                "unclosed `(` in variant constructor",
            );
            let end = self.previous().span;
            Some(Expr {
                kind: ExprKind::VariantConstruct { name, args },
                span: span.merge(end),
            })
        } else if is_upper && !self.check(TokenKind::LParen) {
            // Might be a unit variant like `None`, or just a type name used as expression.
            // We treat it as a variant construct with no args.
            Some(Expr {
                kind: ExprKind::VariantConstruct {
                    name,
                    args: Vec::new(),
                },
                span,
            })
        } else {
            // Regular identifier
            Some(Expr {
                kind: ExprKind::Ident { name },
                span,
            })
        }
    }

    /// Parse a function call: `callee(arg1, arg2, ...)`.
    fn parse_fn_call(&mut self, callee: Expr) -> Option<Expr> {
        self.advance(); // eat `(`
        let mut args = Vec::new();

        while !self.check(TokenKind::RParen) && !self.at_eof() {
            if let Some(arg) = self.parse_expr() {
                args.push(arg);
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::RParen, "E0104", "unclosed `(` in function call");
        let span = callee.span.merge(self.previous().span);

        Some(Expr {
            kind: ExprKind::FnCall {
                callee: Box::new(callee),
                args,
            },
            span,
        })
    }

    /// Parse a list literal: `[expr, expr, ...]` or `[]`.
    fn parse_list_literal(&mut self) -> Option<Expr> {
        let start = self.current_span();
        self.advance(); // eat `[`

        let mut elements = Vec::new();
        while !self.check(TokenKind::RBracket) && !self.at_eof() {
            if let Some(elem) = self.parse_expr() {
                elements.push(elem);
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::RBracket, "E0104", "unclosed `[` in list literal");
        let span = start.merge(self.previous().span);

        Some(Expr {
            kind: ExprKind::ListLiteral { elements },
            span,
        })
    }

    /// Parse a let expression: `let name = value; body`.
    fn parse_let_expr(&mut self) -> Option<Expr> {
        let start = self.current_span();
        self.advance(); // eat `let`

        let name = if self.check(TokenKind::Ident) {
            let n = self.current().text.clone();
            self.advance();
            n
        } else {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error("E0100", "expected variable name after `let`", span)
                    .with_label(Label::new(span, "expected an identifier")),
            );
            return None;
        };

        // Optional type annotation
        let type_annotation = if self.eat(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        if !self.expect(TokenKind::Eq, "E0107", "expected `=` in let expression") {
            return None;
        }

        let value = self.parse_expr()?;

        if !self.expect(
            TokenKind::Semi,
            "E0105",
            "expected `;` after let expression value",
        ) {
            // Try to continue
        }

        let body = self.parse_expr()?;
        let span = start.merge(body.span);

        Some(Expr {
            kind: ExprKind::LetExpr {
                name,
                type_annotation,
                value: Box::new(value),
                body: Box::new(body),
            },
            span,
        })
    }

    /// Parse an if-else expression: `if cond { then } else { else_ }`.
    fn parse_if_else(&mut self) -> Option<Expr> {
        let start = self.current_span();
        self.advance(); // eat `if`

        let condition = self.parse_expr()?;
        let then_branch = self.parse_block_expr()?;

        if !self.eat(TokenKind::Else) {
            let span = self.current_span();
            self.diagnostics.push(
                Diagnostic::error(
                    "E0108",
                    "missing `else` branch in if expression",
                    span,
                )
                .with_label(Label::new(span, "expected `else` here"))
                .with_help(
                    "in LM, `if` is an expression and must have both `then` and `else` branches",
                ),
            );
            return None;
        }

        // else branch can be another if-else or a block
        let else_branch = if self.check(TokenKind::If) {
            self.parse_if_else()?
        } else {
            self.parse_block_expr()?
        };

        let span = start.merge(else_branch.span);

        Some(Expr {
            kind: ExprKind::IfElse {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            },
            span,
        })
    }

    /// Parse a match expression: `match scrutinee { arm1, arm2, ... }`.
    fn parse_match(&mut self) -> Option<Expr> {
        let start = self.current_span();
        self.advance(); // eat `match`

        let scrutinee = self.parse_expr()?;

        if !self.expect(
            TokenKind::LBrace,
            "E0104",
            "expected `{` after match scrutinee",
        ) {
            return None;
        }

        let mut arms = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.at_eof() {
            if let Some(arm) = self.parse_match_arm() {
                arms.push(arm);
            } else {
                // Recovery: skip to next comma or closing brace
                while !self.check(TokenKind::Comma)
                    && !self.check(TokenKind::RBrace)
                    && !self.at_eof()
                {
                    self.advance();
                }
            }
            // Optional trailing comma
            self.eat(TokenKind::Comma);
        }

        if !self.expect(
            TokenKind::RBrace,
            "E0104",
            "unclosed `{` in match expression",
        ) {
            // Try to continue
        }

        let span = start.merge(self.previous().span);
        Some(Expr {
            kind: ExprKind::Match {
                scrutinee: Box::new(scrutinee),
                arms,
            },
            span,
        })
    }

    /// Parse a match arm: `pattern -> expression`.
    fn parse_match_arm(&mut self) -> Option<MatchArm> {
        let start = self.current_span();
        let pattern = self.parse_pattern()?;

        if !self.expect(TokenKind::Arrow, "E0106", "expected `->` in match arm") {
            return None;
        }

        let body = self.parse_expr()?;
        let span = start.merge(body.span);

        Some(MatchArm {
            pattern,
            body,
            span,
        })
    }

    // ---------------------------------------------------------------
    // Patterns
    // ---------------------------------------------------------------

    /// Parse a pattern.
    fn parse_pattern(&mut self) -> Option<Pattern> {
        match self.current().kind {
            // Integer literal pattern
            TokenKind::IntLit => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.advance();
                let value = self.parse_i64_literal(&text, span)?;
                Some(Pattern {
                    kind: PatternKind::Literal {
                        value: LitValue::Int(value),
                    },
                    span,
                })
            }

            // Float literal pattern
            TokenKind::FloatLit => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.advance();
                let value = text.parse::<f64>().unwrap_or(0.0);
                Some(Pattern {
                    kind: PatternKind::Literal {
                        value: LitValue::Float(value),
                    },
                    span,
                })
            }

            // String literal pattern
            TokenKind::StringLit => {
                let span = self.current_span();
                let raw = self.current().text.clone();
                self.advance();
                let content = unescape_string(&raw);
                Some(Pattern {
                    kind: PatternKind::Literal {
                        value: LitValue::String(content),
                    },
                    span,
                })
            }

            // Bool literal pattern
            TokenKind::BoolLit => {
                let span = self.current_span();
                let value = self.current().text == "true";
                self.advance();
                Some(Pattern {
                    kind: PatternKind::Literal {
                        value: LitValue::Bool(value),
                    },
                    span,
                })
            }

            // Identifier: could be wildcard `_`, variant `Circle(r)`, or binding `x`
            TokenKind::Ident => {
                let span = self.current_span();
                let name = self.current().text.clone();
                self.advance();

                if name == "_" {
                    // Wildcard pattern
                    Some(Pattern {
                        kind: PatternKind::Wildcard,
                        span,
                    })
                } else if name
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
                {
                    // Variant pattern: `Circle(r)` or bare `None`
                    if self.eat(TokenKind::LParen) {
                        let mut fields = Vec::new();
                        while !self.check(TokenKind::RParen) && !self.at_eof() {
                            if let Some(p) = self.parse_pattern() {
                                fields.push(p);
                            }
                            if !self.eat(TokenKind::Comma) {
                                break;
                            }
                        }
                        self.expect(
                            TokenKind::RParen,
                            "E0104",
                            "unclosed `(` in variant pattern",
                        );
                        let end = self.previous().span;
                        Some(Pattern {
                            kind: PatternKind::Variant { name, fields },
                            span: span.merge(end),
                        })
                    } else {
                        // Bare variant (unit variant): `None`
                        Some(Pattern {
                            kind: PatternKind::Variant {
                                name,
                                fields: Vec::new(),
                            },
                            span,
                        })
                    }
                } else {
                    // Binding pattern: `x`
                    Some(Pattern {
                        kind: PatternKind::Ident { name },
                        span,
                    })
                }
            }

            // Negated integer literal in pattern: `-1`
            TokenKind::Minus => {
                let start = self.current_span();
                self.advance(); // eat `-`
                if self.check(TokenKind::IntLit) {
                    let text = self.current().text.clone();
                    let end = self.current_span();
                    self.advance();
                    let value = self.parse_negative_i64_literal(&text, start.merge(end))?;
                    Some(Pattern {
                        kind: PatternKind::Literal {
                            value: LitValue::Int(value),
                        },
                        span: start.merge(end),
                    })
                } else if self.check(TokenKind::FloatLit) {
                    let text = self.current().text.clone();
                    let end = self.current_span();
                    self.advance();
                    let value = -(text.parse::<f64>().unwrap_or(0.0));
                    Some(Pattern {
                        kind: PatternKind::Literal {
                            value: LitValue::Float(value),
                        },
                        span: start.merge(end),
                    })
                } else {
                    let span = self.current_span();
                    self.diagnostics.push(
                        Diagnostic::error("E0103", "expected a number after `-` in pattern", span)
                            .with_label(Label::new(span, "expected a number literal")),
                    );
                    None
                }
            }

            _ => {
                let span = self.current_span();
                let text = self.current().text.clone();
                self.diagnostics.push(
                    Diagnostic::error("E0103", format!("expected pattern, found `{}`", text), span)
                        .with_label(Label::new(span, "expected a pattern")),
                );
                None
            }
        }
    }

    // ---------------------------------------------------------------
    // Block expression
    // ---------------------------------------------------------------

    /// Parse a block expression: `{ expr1; expr2; ... exprN }`.
    ///
    /// The last expression in the block is its value. Intermediate
    /// expressions are separated by `;`.
    fn parse_block_expr(&mut self) -> Option<Expr> {
        let start = self.current_span();

        if !self.expect(TokenKind::LBrace, "E0104", "expected `{`") {
            return None;
        }

        let mut exprs = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.at_eof() {
            if let Some(expr) = self.parse_expr() {
                exprs.push(expr);
            } else {
                // Recovery
                while !self.check(TokenKind::Semi)
                    && !self.check(TokenKind::RBrace)
                    && !self.at_eof()
                {
                    self.advance();
                }
            }

            // If the next token is `;`, eat it and continue.
            // If it's `}`, we'll exit the loop.
            if self.check(TokenKind::Semi) {
                self.advance();
            } else if !self.check(TokenKind::RBrace) {
                // If there's no `;` and no `}`, that might be an error
                // but let's be lenient inside blocks and just continue
                break;
            }
        }

        if !self.expect(TokenKind::RBrace, "E0104", "unclosed `{`") {
            // Try to continue
        }

        let span = start.merge(self.previous().span);

        // If the block has exactly one expression, we might flatten it,
        // but for AST fidelity we keep the Block node.
        Some(Expr {
            kind: ExprKind::Block { exprs },
            span,
        })
    }

    /// Parse an integer literal and report range errors instead of silently
    /// changing the program value.
    fn parse_i64_literal(&mut self, text: &str, span: Span) -> Option<i64> {
        match text.parse::<i64>() {
            Ok(value) => Some(value),
            Err(_) => {
                self.diagnostics.push(
                    Diagnostic::error(
                        "E0100",
                        format!("integer literal `{text}` is out of range for Int"),
                        span,
                    )
                    .with_label(Label::new(span, "out of range"))
                    .with_help("Int values must fit in a signed 64-bit integer"),
                );
                None
            }
        }
    }

    /// Parse a negated integer literal, allowing `-9223372036854775808`.
    fn parse_negative_i64_literal(&mut self, text: &str, span: Span) -> Option<i64> {
        let signed = format!("-{text}");
        match signed.parse::<i64>() {
            Ok(value) => Some(value),
            Err(_) => {
                self.diagnostics.push(
                    Diagnostic::error(
                        "E0100",
                        format!("integer literal `{signed}` is out of range for Int"),
                        span,
                    )
                    .with_label(Label::new(span, "out of range"))
                    .with_help("Int values must fit in a signed 64-bit integer"),
                );
                None
            }
        }
    }
}

/// Unescape a string literal, stripping surrounding quotes.
fn unescape_string(raw: &str) -> String {
    // Strip surrounding quotes
    let inner = if raw.len() >= 2 && raw.starts_with('"') {
        let end = if raw.ends_with('"') {
            raw.len() - 1
        } else {
            raw.len()
        };
        &raw[1..end]
    } else {
        raw
    };

    let mut result = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}
