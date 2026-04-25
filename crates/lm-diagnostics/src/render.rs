//! Human-readable diagnostic renderer with ANSI color support.
//!
//! Produces output similar to rustc's error format:
//!
//! ```text
//! error[E0001]: unrecognized character `@`
//!  --> test.lm:3:5
//!   |
//! 3 | let @x = 1;
//!   |     ^ unexpected character
//!   |
//!   = help: remove this character
//! ```

use crate::{Diagnostic, Severity};

/// ANSI color codes.
mod ansi {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const BOLD_RED: &str = "\x1b[1;31m";
    pub const BOLD_YELLOW: &str = "\x1b[1;33m";
    pub const BOLD_CYAN: &str = "\x1b[1;36m";
    pub const BOLD_BLUE: &str = "\x1b[1;34m";
}

/// Compute (1-based line, 1-based column) from a byte offset in source text.
fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(source.len());
    let mut line = 1;
    let mut col = 1;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// Get the source line containing a given byte offset. Returns (line_text, line_number_1based, column_1based).
fn get_source_line(source: &str, offset: usize) -> (&str, usize, usize) {
    let (line_num, col) = offset_to_line_col(source, offset);
    let line_text = source.lines().nth(line_num - 1).unwrap_or("");
    (line_text, line_num, col)
}

/// Severity color (with ANSI).
fn severity_color(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => ansi::BOLD_RED,
        Severity::Warning => ansi::BOLD_YELLOW,
        Severity::Info => ansi::BOLD_CYAN,
    }
}

/// The underline character for a severity.
fn underline_char(severity: Severity) -> char {
    match severity {
        Severity::Error => '^',
        Severity::Warning => '^',
        Severity::Info => '-',
    }
}

/// Render a diagnostic with ANSI colors.
pub fn render_diagnostic(diag: &Diagnostic, source: &str, file_name: &str) -> String {
    render_impl(diag, source, file_name, true)
}

/// Render a diagnostic without ANSI colors.
pub fn render_diagnostic_plain(diag: &Diagnostic, source: &str, file_name: &str) -> String {
    render_impl(diag, source, file_name, false)
}

fn render_impl(diag: &Diagnostic, source: &str, file_name: &str, color: bool) -> String {
    let mut out = String::new();

    let sev_color = if color { severity_color(diag.severity) } else { "" };
    let bold = if color { ansi::BOLD } else { "" };
    let blue = if color { ansi::BOLD_BLUE } else { "" };
    let reset = if color { ansi::RESET } else { "" };

    // Header line: error[E0001]: message
    out.push_str(&format!(
        "{sev_color}{severity}[{code}]{reset}: {bold}{message}{reset}\n",
        severity = diag.severity,
        code = diag.code,
        message = diag.message,
    ));

    // Location line:  --> file.lm:3:5
    let (line_text, line_num, col) = get_source_line(source, diag.span.start);
    let line_num_width = format!("{}", line_num).len();

    // Calculate max line number width considering labels too
    let max_line_num = {
        let mut max = line_num;
        for label in &diag.labels {
            let (ln, _) = offset_to_line_col(source, label.span.start);
            if ln > max {
                max = ln;
            }
        }
        max
    };
    let gutter_width = format!("{}", max_line_num).len();

    out.push_str(&format!(
        "{blue}{arrow:>width$}{reset} {file}:{line}:{col}\n",
        arrow = "-->",
        width = gutter_width + 1,
        file = file_name,
        line = line_num,
        col = col,
    ));

    // Empty gutter line
    out.push_str(&format!(
        "{blue}{blank:>width$} |{reset}\n",
        blank = "",
        width = gutter_width,
    ));

    // Source line with primary underline
    out.push_str(&format!(
        "{blue}{num:>width$} |{reset} {text}\n",
        num = line_num,
        width = gutter_width,
        text = line_text,
    ));

    // Underline for primary span
    let span_len = if diag.span.end > diag.span.start {
        diag.span.end - diag.span.start
    } else {
        1
    };
    // Clamp underline length to remaining line width
    let underline_len = span_len.min(line_text.len().saturating_sub(col - 1)).max(1);
    let padding = " ".repeat(col - 1);
    let uc = underline_char(diag.severity);
    let underline: String = std::iter::repeat_n(uc, underline_len).collect();

    // If there's a primary label matching the span, show its message
    let primary_msg = diag
        .labels
        .iter()
        .find(|l| l.span == diag.span)
        .map(|l| format!(" {}", l.message))
        .unwrap_or_default();

    out.push_str(&format!(
        "{blue}{blank:>width$} |{reset} {padding}{sev_color}{underline}{primary_msg}{reset}\n",
        blank = "",
        width = gutter_width,
    ));

    // Additional labels on different spans
    for label in &diag.labels {
        if label.span == diag.span {
            continue; // already shown above
        }
        let (l_text, l_num, l_col) = get_source_line(source, label.span.start);
        let l_span_len = if label.span.end > label.span.start {
            label.span.end - label.span.start
        } else {
            1
        };
        let l_underline_len = l_span_len.min(l_text.len().saturating_sub(l_col - 1)).max(1);
        let l_padding = " ".repeat(l_col - 1);
        let l_underline = "-".repeat(l_underline_len);

        out.push_str(&format!(
            "{blue}{blank:>width$} |{reset}\n",
            blank = "",
            width = gutter_width,
        ));
        out.push_str(&format!(
            "{blue}{num:>width$} |{reset} {text}\n",
            num = l_num,
            width = gutter_width,
            text = l_text,
        ));
        out.push_str(&format!(
            "{blue}{blank:>width$} |{reset} {l_padding}{sev_color}{l_underline} {msg}{reset}\n",
            blank = "",
            width = gutter_width,
            msg = label.message,
        ));
    }

    // Separator
    out.push_str(&format!(
        "{blue}{blank:>width$} |{reset}\n",
        blank = "",
        width = gutter_width,
    ));

    // Notes
    for note in &diag.notes {
        out.push_str(&format!(
            "{blue}{blank:>width$} = {reset}note: {note}\n",
            blank = "",
            width = gutter_width,
        ));
    }

    // Help
    if let Some(help) = &diag.help {
        out.push_str(&format!(
            "{blue}{blank:>width$} = {reset}{sev_color}help{reset}: {help}\n",
            blank = "",
            width = gutter_width,
        ));
    }

    // Quick-fixes
    for fix in &diag.quickfixes {
        out.push_str(&format!(
            "{blue}{blank:>width$} = {reset}quickfix: {desc}\n",
            blank = "",
            width = gutter_width,
            desc = fix.description,
        ));
    }

    // Drop trailing whitespace but keep the final newline structure
    // (callers may join multiple diagnostics)
    let _ = line_num_width; // suppress unused warning
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Label, QuickFix, Span};

    #[test]
    fn test_render_plain_error() {
        let source = "let @x = 1;\nlet y = 2;\n";
        let diag = Diagnostic::error("E0001", "unrecognized character `@`", Span::new(0, 4, 5))
            .with_label(Label::new(Span::new(0, 4, 5), "unexpected character"))
            .with_help("remove this character")
            .with_quickfix(QuickFix::new(
                Span::new(0, 4, 5),
                "",
                "remove the character",
            ));

        let rendered = diag.render_plain(source, "test.lm");
        assert!(rendered.contains("error[E0001]"));
        assert!(rendered.contains("unrecognized character `@`"));
        assert!(rendered.contains("test.lm:1:5"));
        assert!(rendered.contains("let @x = 1;"));
        assert!(rendered.contains("^ unexpected character"));
        assert!(rendered.contains("help: remove this character"));
        assert!(rendered.contains("quickfix: remove the character"));
    }

    #[test]
    fn test_render_plain_warning() {
        let source = "let x = 1;\nlet y = 2;\n";
        let diag = Diagnostic::warning("W0001", "unused variable `x`", Span::new(0, 4, 5))
            .with_label(Label::new(Span::new(0, 4, 5), "defined here"))
            .with_note("prefix with `_` to suppress this warning");

        let rendered = diag.render_plain(source, "test.lm");
        assert!(rendered.contains("warning[W0001]"));
        assert!(rendered.contains("unused variable `x`"));
        assert!(rendered.contains("^ defined here"));
        assert!(rendered.contains("note: prefix with `_` to suppress this warning"));
    }

    #[test]
    fn test_offset_to_line_col() {
        let src = "abc\ndef\nghi";
        assert_eq!(offset_to_line_col(src, 0), (1, 1));
        assert_eq!(offset_to_line_col(src, 3), (1, 4));
        assert_eq!(offset_to_line_col(src, 4), (2, 1));
        assert_eq!(offset_to_line_col(src, 7), (2, 4));
        assert_eq!(offset_to_line_col(src, 8), (3, 1));
    }
}
