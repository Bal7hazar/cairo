//! Cairo language server. Implements the LSP protocol over stdin/out.

mod semantic_highlighting;

use std::sync::Arc;

use db_utils::Upcast;
use diagnostics::DiagnosticEntry;
use filesystem::db::{AsFilesGroupMut, FilesGroup, FilesGroupEx, PrivRawFileContentQuery};
use filesystem::ids::{FileId, FileLongId};
use filesystem::span::TextPosition;
use parser::db::ParserGroup;
use semantic::test_utils::SemanticDatabaseForTesting;
use semantic_highlighting::token_kind::SemanticTokenKind;
use semantic_highlighting::SemanticTokensTraverser;
use serde_json::Value;
use syntax::node::TypedSyntaxNode;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub type RootDatabase = SemanticDatabaseForTesting;

pub struct Backend {
    pub client: Client,
    // TODO(spapini): Remove this once we support ParallelDatabase.
    pub db_mutex: tokio::sync::Mutex<RootDatabase>,
}
fn from_pos(pos: TextPosition) -> Position {
    Position { line: pos.line as u32, character: pos.col as u32 }
}
impl Backend {
    async fn db(&self) -> tokio::sync::MutexGuard<'_, RootDatabase> {
        self.db_mutex.lock().await
    }
    fn file(&self, db: &RootDatabase, uri: Url) -> FileId {
        let path = uri.to_file_path().expect("Only file URIs are supported.");
        db.intern_file(FileLongId::OnDisk(path))
    }
    async fn publish_diagnostics(
        &self,
        db: tokio::sync::MutexGuard<'_, RootDatabase>,
        uri: Url,
        file: FileId,
    ) {
        let syntax_diagnostics = db.file_syntax_diagnostics(file);
        // TODO(spapini): Add semantic diagnostics.
        // TODO(spapini): Version.
        let mut diags = Vec::new();
        for d in &syntax_diagnostics.0 {
            let location = d.location(db.upcast());
            let start = from_pos(
                location.span.start.position_in_file(db.upcast(), location.file_id).unwrap(),
            );
            let end = from_pos(
                location.span.start.position_in_file(db.upcast(), location.file_id).unwrap(),
            );
            diags.push(Diagnostic {
                range: Range { start, end },
                message: d.format(db.upcast()),
                ..Diagnostic::default()
            });
        }
        self.client.publish_diagnostics(uri, diags, None).await
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: SemanticTokenKind::legend(),
                            token_modifiers: vec![],
                        },
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        ..SemanticTokensOptions::default()
                    }
                    .into(),
                ),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {}

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {}

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let mut db = self.db().await;
        for change in params.changes {
            let file = self.file(&db, change.uri);
            PrivRawFileContentQuery.in_db_mut(db.as_files_group_mut()).invalidate(&file);
        }
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let db = self.db().await;
        let file = self.file(&db, params.text_document.uri.clone());
        self.publish_diagnostics(db, params.text_document.uri, file).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let text =
            if let [TextDocumentContentChangeEvent { text, .. }] = &params.content_changes[..] {
                text
            } else {
                eprintln!("Unexpected format of document change.");
                return;
            };
        let mut db = self.db().await;
        let file = self.file(&db, params.text_document.uri.clone());
        db.override_file_content(file, Some(Arc::new(text.into())));
        self.publish_diagnostics(db, params.text_document.uri, file).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let mut db = self.db().await;
        let file = self.file(&db, params.text_document.uri);
        PrivRawFileContentQuery.in_db_mut(db.as_files_group_mut()).invalidate(&file);
        db.override_file_content(file, None);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut db = self.db().await;
        let file = self.file(&db, params.text_document.uri);
        db.override_file_content(file, None);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let db = self.db().await;
        let file_uri = params.text_document.uri;
        let file = self.file(&db, file_uri.clone());
        let syntax = if let Some(syntax) = db.file_syntax(file) {
            syntax
        } else {
            eprintln!("Semantic analysis failed. File '{file_uri}' does not exist.");
            return Ok(None);
        };

        let node = syntax.as_syntax_node();
        let mut data: Vec<SemanticToken> = Vec::new();
        SemanticTokensTraverser::default().find_semantic_tokens(db.upcast(), &mut data, node);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data })))
    }
}
