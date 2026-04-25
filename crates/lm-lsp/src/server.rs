//! The `tower-lsp` language server implementation.
//!
//! [`LmLanguageServer`] implements the `LanguageServer` trait, handling
//! LSP lifecycle events and dispatching requests to the analysis engine.

use crate::analysis::{self, AnalysisResult, DefEntry, HoverEntry};
use crate::convert::{self, LineIndex};
use std::collections::HashMap;
use std::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

/// Per-document state stored by the server.
struct DocumentState {
    /// The current source text (retained for future incremental analysis).
    _source: String,
    /// Analysis results from the last successful analysis.
    analysis: AnalysisResult,
}

/// The LM language server.
///
/// Stores per-document state and communicates with the editor via the LSP
/// client handle.
pub struct LmLanguageServer {
    /// LSP client for sending notifications (e.g., diagnostics).
    client: Client,
    /// Per-document state, keyed by document URI.
    documents: Mutex<HashMap<Url, DocumentState>>,
}

impl LmLanguageServer {
    /// Create a new language server with the given LSP client handle.
    pub fn new(client: Client) -> Self {
        LmLanguageServer {
            client,
            documents: Mutex::new(HashMap::new()),
        }
    }

    /// Analyze a document and publish diagnostics.
    async fn analyze_and_publish(&self, uri: Url, source: String, version: Option<i32>) {
        let result = analysis::analyze(&source);
        let index = LineIndex::new(&source);

        let lsp_diags: Vec<Diagnostic> = result
            .diagnostics
            .iter()
            .map(|d| convert::convert_diagnostic(d, &index))
            .collect();

        // Store the analysis result
        {
            let mut docs = self.documents.lock().unwrap();
            docs.insert(
                uri.clone(),
                DocumentState {
                    _source: source,
                    analysis: result,
                },
            );
        }

        self.client
            .publish_diagnostics(uri, lsp_diags, version)
            .await;
    }

    /// Look up hover entries for a document.
    fn get_hover_entries(&self, uri: &Url) -> Vec<HoverEntry> {
        let docs = self.documents.lock().unwrap();
        docs.get(uri)
            .map(|state| state.analysis.hover_map.clone())
            .unwrap_or_default()
    }

    /// Look up definition entries for a document.
    fn get_def_entries(&self, uri: &Url) -> Vec<DefEntry> {
        let docs = self.documents.lock().unwrap();
        docs.get(uri)
            .map(|state| state.analysis.def_map.clone())
            .unwrap_or_default()
    }

    /// Look up document symbols for a document.
    fn get_symbols(&self, uri: &Url) -> Vec<DocumentSymbol> {
        let docs = self.documents.lock().unwrap();
        docs.get(uri)
            .map(|state| state.analysis.symbols.clone())
            .unwrap_or_default()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LmLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "lm-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "LM language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let source = params.text_document.text;
        let version = Some(params.text_document.version);
        self.analyze_and_publish(uri, source, version).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = Some(params.text_document.version);
        // We use full sync, so the first change contains the entire document.
        if let Some(change) = params.content_changes.into_iter().next() {
            self.analyze_and_publish(uri, change.text, version).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        // Clear diagnostics and remove stored state
        self.client
            .publish_diagnostics(uri.clone(), vec![], None)
            .await;
        let mut docs = self.documents.lock().unwrap();
        docs.remove(&uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let entries = self.get_hover_entries(uri);

        if let Some(entry) = analysis::find_hover(&entries, pos) {
            Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("```lm\n{}\n```", entry.description),
                }),
                range: Some(entry.range),
            }))
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let entries = self.get_def_entries(uri);

        if let Some(entry) = analysis::find_definition(&entries, pos) {
            Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: uri.clone(),
                range: entry.def_range,
            })))
        } else {
            Ok(None)
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;
        let symbols = self.get_symbols(uri);

        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        }
    }
}
