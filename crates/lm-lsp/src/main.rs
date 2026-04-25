//! Binary entry point for the LM language server.
//!
//! Starts the LSP server on stdin/stdout, communicating with the editor
//! via the Language Server Protocol.

use lm_lsp::LmLanguageServer;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(LmLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
