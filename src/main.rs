// Make clippy print to standard error so it doesn't interfere with
// the lsp interactions with stdio.
#![allow(clippy::print_stderr)]

mod cache;
mod handler;
mod lsp;
mod parsers;
mod tracer;
mod trees;
mod workspace;
use crate::handler::Handler;

use clap::Parser;
use lsp_server::{Connection, Message, RequestId};
use lsp_types::{
    ClientCapabilities, CompletionOptions, DiagnosticOptions, DiagnosticServerCapabilities,
    DocumentLinkOptions, FoldingRangeProviderCapability, GotoDefinitionResponse, InitializeParams,
    OneOf, SemanticTokenModifier, SemanticTokenType, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions,
};
use std::error::Error;
use tracer::Level;
use tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Log level for print statements.
    #[arg(short, long, default_value_t = Level::info)]
    log_level: Level,
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let args = Args::parse();
    tracer::Tracer::init(args.log_level)?;

    // Note that  we must have our logging only write out to stderr.
    info!("Starting MOOS-IvP LSP server");

    // Create the connection to the client.
    // TODO: Need to add support for --port and --pipe
    let (connection, io_threads) = Connection::stdio();

    // Run the server
    let (id, params) = match connection.initialize_start() {
        Ok((id, params)) => (id, params),
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    let initialization_params: InitializeParams = serde_json::from_value(params).unwrap();
    // TODO: Do we want to customize the ServerCapabilities based on the ClientCapabilities
    //let _client_capabilities: ClientCapabilities = initialization_params.capabilities.clone();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).

    /*
    {
        /// For tokens that represent a comment.
        Comment = 0,
        /// For tokens that represent a language keyword.
        Keyword,
        /// For identifiers that declare or reference a local or global variable.
        Variable,
        /// For tokens that represent a string literal.
        String,
        /// For tokens that represent a number literal.
        Number,
        /// For identifiers that declare a macro.
        Macro,
        /// For tokens that represent an operator
        Operator,
    }
    */

    let server_capabilities = ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        semantic_tokens_provider: Some(
            SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    // TODO: These should get moved to where we define the
                    // SemanticToken enum
                    token_types: vec![
                        SemanticTokenType::COMMENT,
                        SemanticTokenType::KEYWORD,
                        SemanticTokenType::VARIABLE,
                        SemanticTokenType::STRING,
                        SemanticTokenType::NUMBER,
                        SemanticTokenType::MACRO,
                        SemanticTokenType::OPERATOR,
                        SemanticTokenType::TYPE,
                        SemanticTokenType::NAMESPACE,
                        SemanticTokenType::STRUCT,
                    ],
                    token_modifiers: vec![
                        SemanticTokenModifier::DEPRECATED,
                        SemanticTokenModifier::DECLARATION,
                        SemanticTokenModifier::DOCUMENTATION,
                    ],
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                ..Default::default()
            }
            .into(),
        ),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            inter_file_dependencies: false,
            workspace_diagnostics: true,
            ..Default::default()
        })),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        // TODO: Get Inline value provider working
        //inline_value_provider: Some(OneOf::Left(true)),
        inlay_hint_provider: Some(OneOf::Left(true)),
        document_link_provider: Some(DocumentLinkOptions {
            resolve_provider: Some(false),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        document_formatting_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec!["#".to_string()]),
            ..Default::default()
        }),
        ..Default::default()
    };

    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let initialize_data = serde_json::json!({
        "capabilities": server_capabilities,
        "serverInfo": {
            "name": NAME,
            "version": VERSION
        }
    });

    match connection.initialize_finish(id, initialize_data) {
        Ok(()) => {}
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    }

    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    info!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: InitializeParams,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let client_capabilities: ClientCapabilities = params.capabilities.clone();
    debug!(
        "Connected to client: {}",
        serde_json::to_string_pretty(&client_capabilities).unwrap_or(String::new())
    );

    let mut handler = Handler::new(connection, params);
    return handler.run(); // Blocks until there are no messages left
}
