//! Conversion utilities between LM internal types and LSP types.
//!
//! Handles byte-offset spans to LSP line/column positions, and
//! LM diagnostics to LSP diagnostics.

use lm_diagnostics::{Diagnostic as LmDiagnostic, Severity as LmSeverity, Span};
use tower_lsp::lsp_types::{
    Diagnostic as LspDiagnostic, DiagnosticSeverity, NumberOrString, Position, Range,
};

/// A pre-computed line index for fast byte-offset to line/column conversion.
///
/// The index stores the byte offset of the start of each line.
#[derive(Debug, Clone)]
pub struct LineIndex {
    /// Source text used for UTF-16 column conversion required by LSP.
    source: String,
    /// Byte offsets of the start of each line (line 0 starts at offset 0).
    line_starts: Vec<usize>,
    /// Total length of the source in bytes.
    len: usize,
}

impl LineIndex {
    /// Build a line index from source text.
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0usize];
        for (i, ch) in source.bytes().enumerate() {
            if ch == b'\n' {
                line_starts.push(i + 1);
            }
        }
        LineIndex {
            source: source.to_string(),
            line_starts,
            len: source.len(),
        }
    }

    /// Convert a byte offset to an LSP `Position` (line, character).
    ///
    /// Both line and character are zero-based. If the offset is past the end
    /// of the file, it is clamped to the last valid position.
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let mut offset = offset.min(self.len);
        while offset > 0 && !self.source.is_char_boundary(offset) {
            offset -= 1;
        }
        // Binary search for the line containing this offset.
        let line = match self.line_starts.binary_search(&offset) {
            Ok(exact) => exact,
            Err(next) => next.saturating_sub(1),
        };
        let line_start = self.line_starts[line];
        let col = self.source[line_start..offset].encode_utf16().count();
        Position::new(line as u32, col as u32)
    }

    /// Convert an LM `Span` to an LSP `Range`.
    pub fn span_to_range(&self, span: Span) -> Range {
        let start = self.offset_to_position(span.start);
        let end = self.offset_to_position(span.end);
        Range::new(start, end)
    }
}

/// Convert an LM severity to an LSP diagnostic severity.
pub fn convert_severity(severity: LmSeverity) -> DiagnosticSeverity {
    match severity {
        LmSeverity::Error => DiagnosticSeverity::ERROR,
        LmSeverity::Warning => DiagnosticSeverity::WARNING,
        LmSeverity::Info => DiagnosticSeverity::INFORMATION,
    }
}

/// Convert an LM diagnostic to an LSP diagnostic.
pub fn convert_diagnostic(diag: &LmDiagnostic, index: &LineIndex) -> LspDiagnostic {
    let range = index.span_to_range(diag.span);
    let severity = Some(convert_severity(diag.severity));
    let code = Some(NumberOrString::String(diag.code.0.clone()));
    let message = if let Some(help) = &diag.help {
        format!("{}\n\nhelp: {}", diag.message, help)
    } else {
        diag.message.clone()
    };
    LspDiagnostic {
        range,
        severity,
        code,
        source: Some("lm".to_string()),
        message,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_index_single_line() {
        let idx = LineIndex::new("hello");
        assert_eq!(idx.offset_to_position(0), Position::new(0, 0));
        assert_eq!(idx.offset_to_position(3), Position::new(0, 3));
        assert_eq!(idx.offset_to_position(5), Position::new(0, 5));
    }

    #[test]
    fn test_line_index_utf16_columns() {
        let idx = LineIndex::new("a😀b");
        assert_eq!(idx.offset_to_position(1), Position::new(0, 1));
        assert_eq!(idx.offset_to_position(5), Position::new(0, 3));
        assert_eq!(idx.offset_to_position(6), Position::new(0, 4));
    }

    #[test]
    fn test_line_index_multiple_lines() {
        let idx = LineIndex::new("abc\ndef\nghi");
        // Line 0: bytes 0..3 ("abc")
        // Line 1: bytes 4..7 ("def")
        // Line 2: bytes 8..10 ("ghi")
        assert_eq!(idx.offset_to_position(0), Position::new(0, 0));
        assert_eq!(idx.offset_to_position(2), Position::new(0, 2));
        assert_eq!(idx.offset_to_position(4), Position::new(1, 0));
        assert_eq!(idx.offset_to_position(6), Position::new(1, 2));
        assert_eq!(idx.offset_to_position(8), Position::new(2, 0));
        assert_eq!(idx.offset_to_position(10), Position::new(2, 2));
    }

    #[test]
    fn test_line_index_offset_at_newline() {
        let idx = LineIndex::new("ab\ncd\n");
        // offset 2 = 'b' on line 0? No, offset 2 is '\n' char.
        // Line 0: offsets 0,1,2(\n) -> line_starts[0]=0
        // Line 1: offsets 3,4,5(\n) -> line_starts[1]=3
        // Line 2: offset 6          -> line_starts[2]=6
        assert_eq!(idx.offset_to_position(2), Position::new(0, 2));
        assert_eq!(idx.offset_to_position(3), Position::new(1, 0));
        assert_eq!(idx.offset_to_position(5), Position::new(1, 2));
        assert_eq!(idx.offset_to_position(6), Position::new(2, 0));
    }

    #[test]
    fn test_line_index_empty() {
        let idx = LineIndex::new("");
        assert_eq!(idx.offset_to_position(0), Position::new(0, 0));
    }

    #[test]
    fn test_line_index_clamp() {
        let idx = LineIndex::new("hello");
        // Past end => clamped
        assert_eq!(idx.offset_to_position(100), Position::new(0, 5));
    }

    #[test]
    fn test_convert_diagnostic() {
        let span = Span::new(0, 4, 8);
        let diag = lm_diagnostics::Diagnostic::error("E0201", "undefined variable `x`", span);
        let source = "let x = 42;\nlet y = x;";
        let idx = LineIndex::new(source);
        let lsp_diag = convert_diagnostic(&diag, &idx);

        assert_eq!(lsp_diag.range.start, Position::new(0, 4));
        assert_eq!(lsp_diag.range.end, Position::new(0, 8));
        assert_eq!(lsp_diag.severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(lsp_diag.source, Some("lm".to_string()));
        assert!(lsp_diag.message.contains("undefined variable"));
    }

    #[test]
    fn test_convert_severity() {
        assert_eq!(
            convert_severity(LmSeverity::Error),
            DiagnosticSeverity::ERROR
        );
        assert_eq!(
            convert_severity(LmSeverity::Warning),
            DiagnosticSeverity::WARNING
        );
        assert_eq!(
            convert_severity(LmSeverity::Info),
            DiagnosticSeverity::INFORMATION
        );
    }

    #[test]
    fn test_span_to_range() {
        let source = "fn foo() {\n  42\n}";
        let idx = LineIndex::new(source);
        let span = Span::new(0, 12, 14); // "42" on line 1
        let range = idx.span_to_range(span);
        assert_eq!(range.start, Position::new(1, 1));
        assert_eq!(range.end, Position::new(1, 3));
    }
}
