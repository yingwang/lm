//! LSP server for the LM programming language.
//!
//! This crate implements the Language Server Protocol using `tower-lsp`,
//! providing diagnostics, hover, go-to-definition, and document symbols
//! for `.lm` files.
//!
//! # Features
//!
//! - **Diagnostics on open/change:** lex, parse, and type-check the file,
//!   then publish `lsp_types::Diagnostic` to the editor.
//! - **Hover:** show the inferred type of a variable, function, or constructor.
//! - **Go to definition:** jump from a usage to its definition site.
//! - **Document symbols:** list all top-level declarations for the outline view.

mod analysis;
mod convert;
mod server;

pub use server::LmLanguageServer;
