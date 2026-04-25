//! Token kinds for the LM lexer.

use serde::{Deserialize, Serialize};

/// All possible token kinds in the LM language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    // === Literals ===
    /// Integer literal, e.g. `42`
    IntLit,
    /// Floating-point literal, e.g. `3.14`
    FloatLit,
    /// String literal, e.g. `"hello"`
    StringLit,
    /// Boolean literal: `true` or `false`
    BoolLit,

    // === Identifiers ===
    /// An identifier, e.g. `foo`, `my_var`, `Shape`
    Ident,

    // === Keywords ===
    /// `let`
    Let,
    /// `fn`
    Fn,
    /// `io`
    Io,
    /// `pure`
    Pure,
    /// `type`
    Type,
    /// `match`
    Match,
    /// `if`
    If,
    /// `else`
    Else,

    // === Operators ===
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `++` (string concatenation)
    PlusPlus,
    /// `==`
    EqEq,
    /// `!=`
    BangEq,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `&&`
    AmpAmp,
    /// `||`
    PipePipe,
    /// `!`
    Bang,
    /// `=`
    Eq,
    /// `->`
    Arrow,
    /// `|` (used in type variant declarations and match arms)
    Pipe,

    // === Delimiters ===
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `_` (wildcard pattern — only when standalone, not part of an identifier)
    Underscore,

    // === Special ===
    /// End of file.
    Eof,
}

impl TokenKind {
    /// A short human-readable name for the token kind.
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::IntLit => "integer",
            TokenKind::FloatLit => "float",
            TokenKind::StringLit => "string",
            TokenKind::BoolLit => "boolean",
            TokenKind::Ident => "identifier",
            TokenKind::Let => "let",
            TokenKind::Fn => "fn",
            TokenKind::Io => "io",
            TokenKind::Pure => "pure",
            TokenKind::Type => "type",
            TokenKind::Match => "match",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::PlusPlus => "++",
            TokenKind::EqEq => "==",
            TokenKind::BangEq => "!=",
            TokenKind::Lt => "<",
            TokenKind::LtEq => "<=",
            TokenKind::Gt => ">",
            TokenKind::GtEq => ">=",
            TokenKind::AmpAmp => "&&",
            TokenKind::PipePipe => "||",
            TokenKind::Bang => "!",
            TokenKind::Eq => "=",
            TokenKind::Arrow => "->",
            TokenKind::Pipe => "|",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Semi => ";",
            TokenKind::Colon => ":",
            TokenKind::Underscore => "_",
            TokenKind::Eof => "EOF",
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
