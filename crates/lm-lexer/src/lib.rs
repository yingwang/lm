//! Hand-written lexer for the LM programming language.
//!
//! The lexer transforms source text into a stream of [`Token`]s, producing
//! [`Diagnostic`]s for any lexical errors encountered.
//!
//! # Usage
//!
//! ```
//! use lm_lexer::Lexer;
//!
//! let source = "let x = 42;";
//! let (tokens, diagnostics) = Lexer::new(source, 0).tokenize();
//! assert!(diagnostics.is_empty());
//! ```

mod token;

pub use token::TokenKind;

use lm_diagnostics::{Diagnostic, Label, QuickFix, Span};
use serde::{Deserialize, Serialize};

/// A single token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,
    /// The byte span in the source file.
    pub span: Span,
    /// The literal text of the token as it appeared in source.
    pub text: String,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, span: Span, text: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            text: text.into(),
        }
    }
}

/// The LM lexer.
///
/// Transforms source text into a sequence of tokens, skipping whitespace
/// and comments, and collecting diagnostics for lexical errors.
pub struct Lexer<'src> {
    /// The full source text.
    source: &'src str,
    /// Source bytes for fast indexing.
    bytes: &'src [u8],
    /// File identifier for span construction.
    file_id: u32,
    /// Current byte offset.
    pos: usize,
    /// Collected diagnostics.
    diagnostics: Vec<Diagnostic>,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer for the given source text and file identifier.
    pub fn new(source: &'src str, file_id: u32) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            file_id,
            pos: 0,
            diagnostics: Vec::new(),
        }
    }

    /// Consume the lexer and return all tokens and diagnostics.
    ///
    /// Whitespace and comments are skipped. An `Eof` token is always
    /// appended at the end.
    pub fn tokenize(mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                tokens.push(Token::new(
                    TokenKind::Eof,
                    Span::point(self.file_id, self.pos),
                    "",
                ));
                break;
            }

            match self.next_token() {
                Some(tok) => tokens.push(tok),
                None => {
                    // Error already reported, advance past the bad character
                }
            }
        }

        (tokens, self.diagnostics)
    }

    /// Are we at the end of input?
    fn is_at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    /// Peek at the current byte without consuming.
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Peek at the byte after the current one.
    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.pos + 1).copied()
    }

    /// Advance one byte and return it.
    fn advance(&mut self) -> Option<u8> {
        let b = self.bytes.get(self.pos).copied();
        if b.is_some() {
            self.pos += 1;
        }
        b
    }

    /// Skip whitespace and line comments.
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while let Some(b) = self.peek() {
                if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                    self.advance();
                } else {
                    break;
                }
            }

            // Skip line comments
            if self.peek() == Some(b'/') && self.peek_next() == Some(b'/') {
                while let Some(b) = self.peek() {
                    if b == b'\n' {
                        break;
                    }
                    self.advance();
                }
                // Continue to skip more whitespace/comments
                continue;
            }

            break;
        }
    }

    /// Try to lex the next token. Returns `None` if an error was reported
    /// (the bad character is consumed and a diagnostic is emitted).
    fn next_token(&mut self) -> Option<Token> {
        let start = self.pos;
        let b = self.peek()?;

        match b {
            // String literal
            b'"' => Some(self.lex_string()),

            // Number literal (digit or leading dot would be handled differently,
            // but LM does not support `.5` style floats)
            b'0'..=b'9' => Some(self.lex_number()),

            // Identifiers and keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Some(self.lex_identifier_or_keyword()),

            // Two-character operators first, then single-character
            b'+' => {
                self.advance();
                if self.peek() == Some(b'+') {
                    self.advance();
                    Some(self.make_token(TokenKind::PlusPlus, start))
                } else {
                    Some(self.make_token(TokenKind::Plus, start))
                }
            }
            b'-' => {
                self.advance();
                if self.peek() == Some(b'>') {
                    self.advance();
                    Some(self.make_token(TokenKind::Arrow, start))
                } else {
                    Some(self.make_token(TokenKind::Minus, start))
                }
            }
            b'*' => {
                self.advance();
                Some(self.make_token(TokenKind::Star, start))
            }
            b'/' => {
                // Comments already handled in skip_whitespace_and_comments
                self.advance();
                Some(self.make_token(TokenKind::Slash, start))
            }
            b'%' => {
                self.advance();
                Some(self.make_token(TokenKind::Percent, start))
            }
            b'=' => {
                self.advance();
                if self.peek() == Some(b'=') {
                    self.advance();
                    Some(self.make_token(TokenKind::EqEq, start))
                } else {
                    Some(self.make_token(TokenKind::Eq, start))
                }
            }
            b'!' => {
                self.advance();
                if self.peek() == Some(b'=') {
                    self.advance();
                    Some(self.make_token(TokenKind::BangEq, start))
                } else {
                    Some(self.make_token(TokenKind::Bang, start))
                }
            }
            b'<' => {
                self.advance();
                if self.peek() == Some(b'=') {
                    self.advance();
                    Some(self.make_token(TokenKind::LtEq, start))
                } else {
                    Some(self.make_token(TokenKind::Lt, start))
                }
            }
            b'>' => {
                self.advance();
                if self.peek() == Some(b'=') {
                    self.advance();
                    Some(self.make_token(TokenKind::GtEq, start))
                } else {
                    Some(self.make_token(TokenKind::Gt, start))
                }
            }
            b'&' => {
                self.advance();
                if self.peek() == Some(b'&') {
                    self.advance();
                    Some(self.make_token(TokenKind::AmpAmp, start))
                } else {
                    // Single `&` is not a valid token in LM
                    let span = Span::new(self.file_id, start, self.pos);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0001",
                            "unrecognized character `&`",
                            span,
                        )
                        .with_label(Label::new(span, "expected `&&` for logical AND"))
                        .with_help("use `&&` for logical AND; single `&` is not supported in LM"),
                    );
                    None
                }
            }
            b'|' => {
                self.advance();
                if self.peek() == Some(b'|') {
                    self.advance();
                    Some(self.make_token(TokenKind::PipePipe, start))
                } else {
                    Some(self.make_token(TokenKind::Pipe, start))
                }
            }

            // Delimiters
            b'(' => { self.advance(); Some(self.make_token(TokenKind::LParen, start)) }
            b')' => { self.advance(); Some(self.make_token(TokenKind::RParen, start)) }
            b'{' => { self.advance(); Some(self.make_token(TokenKind::LBrace, start)) }
            b'}' => { self.advance(); Some(self.make_token(TokenKind::RBrace, start)) }
            b'[' => { self.advance(); Some(self.make_token(TokenKind::LBracket, start)) }
            b']' => { self.advance(); Some(self.make_token(TokenKind::RBracket, start)) }
            b',' => { self.advance(); Some(self.make_token(TokenKind::Comma, start)) }
            b';' => { self.advance(); Some(self.make_token(TokenKind::Semi, start)) }
            b':' => { self.advance(); Some(self.make_token(TokenKind::Colon, start)) }

            // Unrecognized character
            _ => {
                self.advance();
                let span = Span::new(self.file_id, start, self.pos);
                let ch = self.source[start..self.pos].chars().next().unwrap_or('?');
                self.diagnostics.push(
                    Diagnostic::error(
                        "E0001",
                        format!("unrecognized character `{}`", ch),
                        span,
                    )
                    .with_label(Label::new(span, "not a valid LM token"))
                    .with_help("LM uses ASCII operators and identifiers")
                    .with_quickfix(QuickFix::new(span, "", format!("remove `{}`", ch))),
                );
                None
            }
        }
    }

    /// Lex a string literal, handling escape sequences.
    ///
    /// Produces E0002 for unterminated strings and E0001 for invalid escapes.
    fn lex_string(&mut self) -> Token {
        let start = self.pos;
        self.advance(); // consume opening `"`

        let mut value = String::new();
        loop {
            match self.peek() {
                None | Some(b'\n') => {
                    // Unterminated string
                    let span = Span::new(self.file_id, start, self.pos);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0002",
                            "unterminated string literal",
                            span,
                        )
                        .with_label(Label::new(
                            Span::new(self.file_id, start, start + 1),
                            "string starts here",
                        ))
                        .with_help("add a closing `\"` to terminate the string"),
                    );
                    return Token::new(
                        TokenKind::StringLit,
                        Span::new(self.file_id, start, self.pos),
                        &self.source[start..self.pos],
                    );
                }
                Some(b'\\') => {
                    self.advance(); // consume backslash
                    match self.peek() {
                        Some(b'\\') => { self.advance(); value.push('\\'); }
                        Some(b'"') => { self.advance(); value.push('"'); }
                        Some(b'n') => { self.advance(); value.push('\n'); }
                        Some(b't') => { self.advance(); value.push('\t'); }
                        Some(other) => {
                            let esc_start = self.pos - 1;
                            self.advance();
                            let span = Span::new(self.file_id, esc_start, self.pos);
                            let ch = other as char;
                            self.diagnostics.push(
                                Diagnostic::error(
                                    "E0001",
                                    format!("invalid escape sequence `\\{}`", ch),
                                    span,
                                )
                                .with_label(Label::new(span, "unknown escape"))
                                .with_help("valid escapes are: \\\\, \\\", \\n, \\t"),
                            );
                            value.push('\\');
                            value.push(ch);
                        }
                        None => {
                            // Backslash at end of file — unterminated
                            let span = Span::new(self.file_id, start, self.pos);
                            self.diagnostics.push(
                                Diagnostic::error(
                                    "E0002",
                                    "unterminated string literal",
                                    span,
                                )
                                .with_label(Label::new(
                                    Span::new(self.file_id, start, start + 1),
                                    "string starts here",
                                ))
                                .with_help("add a closing `\"` to terminate the string"),
                            );
                            return Token::new(
                                TokenKind::StringLit,
                                Span::new(self.file_id, start, self.pos),
                                &self.source[start..self.pos],
                            );
                        }
                    }
                }
                Some(b'"') => {
                    self.advance(); // consume closing `"`
                    break;
                }
                Some(_) => {
                    let ch = self.source[self.pos..].chars().next().unwrap();
                    self.pos += ch.len_utf8();
                    value.push(ch);
                }
            }
        }

        let _ = value; // We store the raw text, not the processed value
        Token::new(
            TokenKind::StringLit,
            Span::new(self.file_id, start, self.pos),
            &self.source[start..self.pos],
        )
    }

    /// Lex a number literal (integer or float).
    ///
    /// Produces E0003 for invalid number literals.
    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        let mut has_dot = false;

        // Consume digits
        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' => { self.advance(); }
                b'.' => {
                    // Check that the next char after dot is a digit (not `..` range or method call)
                    if has_dot {
                        break; // second dot — stop
                    }
                    match self.peek_next() {
                        Some(b'0'..=b'9') => {
                            has_dot = true;
                            self.advance(); // consume '.'
                        }
                        _ => break, // dot not followed by digit — not part of this number
                    }
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    // Invalid suffix on number literal
                    let suffix_start = self.pos;
                    while let Some(b) = self.peek() {
                        if b.is_ascii_alphanumeric() || b == b'_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let span = Span::new(self.file_id, start, self.pos);
                    let text = &self.source[start..self.pos];
                    let suffix = &self.source[suffix_start..self.pos];
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0003",
                            format!("invalid number literal `{}`", text),
                            span,
                        )
                        .with_label(Label::new(
                            Span::new(self.file_id, suffix_start, self.pos),
                            format!("unexpected suffix `{}`", suffix),
                        ))
                        .with_help("number literals must not have alphabetic suffixes"),
                    );
                    return Token::new(
                        if has_dot { TokenKind::FloatLit } else { TokenKind::IntLit },
                        span,
                        text,
                    );
                }
                _ => break,
            }
        }

        let text = &self.source[start..self.pos];
        let kind = if has_dot {
            TokenKind::FloatLit
        } else {
            TokenKind::IntLit
        };

        Token::new(kind, Span::new(self.file_id, start, self.pos), text)
    }

    /// Lex an identifier or keyword.
    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.pos;

        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.pos];
        let kind = match text {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "io" => TokenKind::Io,
            "pure" => TokenKind::Pure,
            "type" => TokenKind::Type,
            "match" => TokenKind::Match,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "true" => TokenKind::BoolLit,
            "false" => TokenKind::BoolLit,
            _ => TokenKind::Ident,
        };

        Token::new(kind, Span::new(self.file_id, start, self.pos), text)
    }

    /// Build a token from a start position to the current position.
    fn make_token(&self, kind: TokenKind, start: usize) -> Token {
        Token::new(
            kind,
            Span::new(self.file_id, start, self.pos),
            &self.source[start..self.pos],
        )
    }
}

#[cfg(test)]
mod tests;
