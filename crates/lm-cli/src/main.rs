//! `lmc` — the LM programming language compiler CLI.
//!
//! # Commands
//!
//! - `lmc tokenize <file>` — Lex a `.lm` file and print the token stream.
//! - `lmc parse <file>` — Parse a `.lm` file (not yet implemented).
//! - `lmc check <file>` — Type-check a `.lm` file (not yet implemented).
//! - `lmc run <file>` — Execute a `.lm` file (not yet implemented).

use clap::{Parser, Subcommand, ValueEnum};
use lm_diagnostics::DiagnosticBag;
use lm_lexer::Lexer;
use std::process;

/// The LM programming language compiler.
#[derive(Parser)]
#[command(name = "lmc", version, about = "The LM programming language compiler")]
struct Cli {
    /// Output format for diagnostics and results.
    #[arg(long, default_value = "human", global = true)]
    format: OutputFormat,

    #[command(subcommand)]
    command: Command,
}

/// Available output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Human-readable output with ANSI colors.
    Human,
    /// Machine-readable JSON output.
    Json,
}

/// Compiler subcommands.
#[derive(Subcommand)]
enum Command {
    /// Tokenize a source file and print the token stream.
    Tokenize {
        /// Path to the `.lm` source file.
        file: String,
    },
    /// Parse a source file and print the AST.
    Parse {
        /// Path to the `.lm` source file.
        file: String,
    },
    /// Type-check a source file.
    Check {
        /// Path to the `.lm` source file.
        file: String,
    },
    /// Run a source file.
    Run {
        /// Path to the `.lm` source file.
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Tokenize { file } => cmd_tokenize(&file, cli.format),
        Command::Parse { file } => cmd_placeholder("parse", &file),
        Command::Check { file } => cmd_placeholder("check", &file),
        Command::Run { file } => cmd_placeholder("run", &file),
    }
}

/// Tokenize a source file and print results.
fn cmd_tokenize(path: &str, format: OutputFormat) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: could not read `{}`: {}", path, e);
            process::exit(1);
        }
    };

    let (tokens, diagnostics) = Lexer::new(&source, 0).tokenize();

    // Print diagnostics
    let mut bag = DiagnosticBag::new();
    for d in diagnostics {
        bag.add(d);
    }

    let has_errors = bag.has_errors();

    match format {
        OutputFormat::Human => {
            // Print diagnostics first
            if !bag.is_empty() {
                eprint!("{}", bag.render_all(&source, path));
            }

            // Print token table
            println!("{:<6} {:<14} {:<12} TEXT", "IDX", "KIND", "SPAN");
            println!("{}", "-".repeat(50));
            for (i, tok) in tokens.iter().enumerate() {
                println!(
                    "{:<6} {:<14} {:>4}..{:<6} {:?}",
                    i,
                    format!("{:?}", tok.kind),
                    tok.span.start,
                    tok.span.end,
                    tok.text,
                );
            }

            // Summary
            let error_count = bag.iter().filter(|d| d.severity == lm_diagnostics::Severity::Error).count();
            let warning_count = bag.iter().filter(|d| d.severity == lm_diagnostics::Severity::Warning).count();
            if error_count > 0 || warning_count > 0 {
                eprintln!(
                    "\n{} error(s), {} warning(s)",
                    error_count, warning_count
                );
            }
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "tokens": tokens,
                "diagnostics": bag.into_vec(),
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
    }

    if has_errors {
        process::exit(1);
    }
}

/// Placeholder for not-yet-implemented commands.
fn cmd_placeholder(name: &str, path: &str) {
    // Verify the file exists at least
    if !std::path::Path::new(path).exists() {
        eprintln!("error: could not find `{}`", path);
        process::exit(1);
    }
    eprintln!("lmc {}: not yet implemented", name);
    process::exit(2);
}
