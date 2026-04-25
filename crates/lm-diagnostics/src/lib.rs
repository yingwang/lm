//! Diagnostic system for the LM programming language.
//!
//! This crate provides structured diagnostics that can be rendered as
//! human-readable terminal output (with ANSI colors) or serialized to JSON
//! for tooling consumption.
//!
//! # Error code ranges
//!
//! | Range         | Category                        |
//! |---------------|---------------------------------|
//! | E0001–E0099   | Lexer errors                    |
//! | E0100–E0199   | Parser errors                   |
//! | E0200–E0299   | Type checking errors            |
//! | E0300–E0399   | Effect checking errors          |
//! | E0400–E0499   | Pattern exhaustiveness errors   |
//! | E0500–E0599   | Runtime errors                  |

mod render;

use serde::{Deserialize, Serialize};

/// A byte-offset span in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Opaque identifier for the source file.
    pub file_id: u32,
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

impl Span {
    /// Create a new span.
    pub fn new(file_id: u32, start: usize, end: usize) -> Self {
        Self { file_id, start, end }
    }

    /// Create a zero-width span at a single position.
    pub fn point(file_id: u32, offset: usize) -> Self {
        Self {
            file_id,
            start: offset,
            end: offset,
        }
    }

    /// Merge two spans into one that covers both.
    pub fn merge(self, other: Span) -> Span {
        debug_assert_eq!(self.file_id, other.file_id);
        Span {
            file_id: self.file_id,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// A fatal error that prevents compilation.
    Error,
    /// A potential issue that does not prevent compilation.
    Warning,
    /// An informational note.
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

/// A structured error code such as `E0001`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorCode(pub String);

impl ErrorCode {
    /// Create a new error code.
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An annotated source label pointing at a specific span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// The source span this label points to.
    pub span: Span,
    /// A short message describing this location.
    pub message: String,
}

impl Label {
    /// Create a new label.
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

/// A structured code edit that tooling (LSP, Claude) can apply automatically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickFix {
    /// The span of text to replace.
    pub span: Span,
    /// The replacement text.
    pub replacement: String,
    /// A human-readable description of what this fix does.
    pub description: String,
}

impl QuickFix {
    /// Create a new quick-fix.
    pub fn new(span: Span, replacement: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            span,
            replacement: replacement.into(),
            description: description.into(),
        }
    }
}

/// A single diagnostic message with full context.
///
/// Diagnostics carry everything needed for both human-readable rendering
/// and structured tooling output (LSP, JSON).
///
/// # JSON schema
///
/// When serialized to JSON the structure is:
///
/// ```json
/// {
///   "code": "E0001",
///   "severity": "error",
///   "message": "unrecognized character `@`",
///   "span": { "file_id": 0, "start": 12, "end": 13 },
///   "labels": [
///     { "span": { "file_id": 0, "start": 12, "end": 13 }, "message": "unexpected here" }
///   ],
///   "notes": ["LM only supports ASCII operator characters"],
///   "help": "remove this character",
///   "quickfixes": [
///     { "span": { "file_id": 0, "start": 12, "end": 13 }, "replacement": "", "description": "remove the character" }
///   ]
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Error code identifying the class of diagnostic.
    pub code: ErrorCode,
    /// Severity level.
    pub severity: Severity,
    /// One-line main message.
    pub message: String,
    /// Primary source location.
    pub span: Span,
    /// Additional annotated source locations.
    pub labels: Vec<Label>,
    /// Extra explanatory notes.
    pub notes: Vec<String>,
    /// A suggested fix (human-readable).
    pub help: Option<String>,
    /// Structured code edits for automated tooling.
    pub quickfixes: Vec<QuickFix>,
}

impl Diagnostic {
    /// Create a new error diagnostic with the given code, message, and span.
    pub fn error(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: ErrorCode::new(code),
            severity: Severity::Error,
            message: message.into(),
            span,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            quickfixes: Vec::new(),
        }
    }

    /// Create a new warning diagnostic.
    pub fn warning(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: ErrorCode::new(code),
            severity: Severity::Warning,
            message: message.into(),
            span,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            quickfixes: Vec::new(),
        }
    }

    /// Create a new info diagnostic.
    pub fn info(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: ErrorCode::new(code),
            severity: Severity::Info,
            message: message.into(),
            span,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            quickfixes: Vec::new(),
        }
    }

    /// Add an annotated label.
    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    /// Add a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Set the help message.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Add a quick-fix.
    pub fn with_quickfix(mut self, fix: QuickFix) -> Self {
        self.quickfixes.push(fix);
        self
    }

    /// Serialize this diagnostic to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("diagnostic serialization should not fail")
    }

    /// Serialize this diagnostic to a pretty-printed JSON string.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("diagnostic serialization should not fail")
    }

    /// Render this diagnostic as human-readable text with ANSI colors.
    ///
    /// `source` is the full source text of the file, and `file_name` is
    /// the display path shown in the output.
    pub fn render(&self, source: &str, file_name: &str) -> String {
        render::render_diagnostic(self, source, file_name)
    }

    /// Render this diagnostic as human-readable text without ANSI colors.
    pub fn render_plain(&self, source: &str, file_name: &str) -> String {
        render::render_diagnostic_plain(self, source, file_name)
    }
}

/// A bag that collects multiple diagnostics during a compilation phase.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    /// Create an empty diagnostic bag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a diagnostic to the bag.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Return true if any diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    /// Return the number of collected diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Return true if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Iterate over all diagnostics.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    /// Consume the bag and return all diagnostics.
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Merge another bag into this one.
    pub fn merge(&mut self, other: DiagnosticBag) {
        self.diagnostics.extend(other.diagnostics);
    }

    /// Render all diagnostics as human-readable text.
    pub fn render_all(&self, source: &str, file_name: &str) -> String {
        self.diagnostics
            .iter()
            .map(|d| d.render(source, file_name))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Serialize all diagnostics to a JSON array string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.diagnostics)
            .expect("diagnostic bag serialization should not fail")
    }

    /// Serialize all diagnostics to a pretty-printed JSON array string.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.diagnostics)
            .expect("diagnostic bag serialization should not fail")
    }
}

impl IntoIterator for DiagnosticBag {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}

impl<'a> IntoIterator for &'a DiagnosticBag {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_merge() {
        let a = Span::new(0, 5, 10);
        let b = Span::new(0, 8, 15);
        let merged = a.merge(b);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 15);
    }

    #[test]
    fn test_diagnostic_json_roundtrip() {
        let diag = Diagnostic::error("E0001", "test error", Span::new(0, 0, 5))
            .with_label(Label::new(Span::new(0, 0, 5), "here"))
            .with_note("this is a note")
            .with_help("try this instead");

        let json = diag.to_json();
        let parsed: Diagnostic = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, diag);
    }

    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();
        assert!(bag.is_empty());
        assert!(!bag.has_errors());

        bag.add(Diagnostic::warning("W0001", "unused variable", Span::new(0, 0, 1)));
        assert_eq!(bag.len(), 1);
        assert!(!bag.has_errors());

        bag.add(Diagnostic::error("E0001", "bad token", Span::new(0, 2, 3)));
        assert_eq!(bag.len(), 2);
        assert!(bag.has_errors());
    }
}
