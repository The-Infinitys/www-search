use async_trait::async_trait;
use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{ServerHandler, McpServerOptions, server_runtime},
    schema::*,
    McpServer,
    ToMcpServerHandler,
    StdioTransport,
};
use serde::Deserialize;
use www_search::{www_search, EngineType, browse};
use std::sync::Arc;
use rust_mcp_transport::TransportOptions;

#[derive(Clone)]
pub struct WwwSearchHandler;

#[derive(Deserialize)]
struct SearchArgs {
    query: String,
    engine: Option<EngineType>,
}

#[derive(Deserialize)]
struct FetchArgs {
    url: String,
}

#[async_trait]
impl ServerHandler for WwwSearchHandler {
    async fn handle_list_tools_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        let web_search = Tool {
            name: "web_search".to_string(),
            description: Some("Search the web using Google or DuckDuckGo. Returns a list of search results with titles, URLs, and descriptions.".to_string()),
            input_schema: serde_json::from_value(serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "engine": { "type": "string", "enum": ["google", "duckduckgo"] }
                },
                "required": ["query"]
            })).unwrap(),
            annotations: None,
            execution: None,
            icons: vec![],
            meta: None,
            output_schema: None,
            title: None,
        };

        let fetch_page = Tool {
            name: "fetch_page".to_string(),
            description: Some("Fetch the content of a web page and convert it to Markdown. Useful for reading the full content of a search result.".to_string()),
            input_schema: serde_json::from_value(serde_json::json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" }
                },
                "required": ["url"]
            })).unwrap(),
            annotations: None,
            execution: None,
            icons: vec![],
            meta: None,
            output_schema: None,
            title: None,
        };

        Ok(ListToolsResult {
            tools: vec![web_search, fetch_page],
            next_cursor: None,
            meta: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, rust_mcp_sdk::schema::schema_utils::CallToolError> {
        match params.name.as_str() {
            "web_search" => {
                let args: SearchArgs = serde_json::from_value(serde_json::Value::Object(params.arguments.unwrap_or_default()))
                    .map_err(|e| rust_mcp_sdk::schema::schema_utils::CallToolError::invalid_arguments("web_search", Some(e.to_string())))?;
                
                let results = match www_search(args.engine.unwrap_or_default(), args.query).await {
                    Ok(r) => r,
                    Err(e) => return Err(rust_mcp_sdk::schema::schema_utils::CallToolError::from_message(e)),
                };
                let content = serde_json::to_string_pretty(&results)
                    .map_err(|e| rust_mcp_sdk::schema::schema_utils::CallToolError::from_message(e.to_string()))?;

                Ok(CallToolResult {
                    content: vec![TextContent::new(content, None, None).into()],
                    is_error: None,
                    meta: None,
                    structured_content: None,
                })
            }
            "fetch_page" => {
                let args: FetchArgs = serde_json::from_value(serde_json::Value::Object(params.arguments.unwrap_or_default()))
                    .map_err(|e| rust_mcp_sdk::schema::schema_utils::CallToolError::invalid_arguments("fetch_page", Some(e.to_string())))?;
                
                let md = match browse::fetch_and_markdown(&args.url).await {
                    Ok(m) => m,
                    Err(e) => return Err(rust_mcp_sdk::schema::schema_utils::CallToolError::from_message(e)),
                };

                Ok(CallToolResult {
                    content: vec![TextContent::new(md, None, None).into()],
                    is_error: None,
                    meta: None,
                    structured_content: None,
                })
            }
            _ => Err(rust_mcp_sdk::schema::schema_utils::CallToolError::unknown_tool(params.name)),
        }
    }
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "www-search-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: None,
            icons: vec![],
            title: None,
            website_url: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        instructions: None,
        meta: None,
        protocol_version: "2025-11-25".to_string(),
    };

    let handler = WwwSearchHandler.to_mcp_server_handler();
    
    let transport = StdioTransport::new(TransportOptions::default()).map_err(|e| {
        rust_mcp_sdk::schema::schema_utils::SdkError::internal_error()
            .with_message(&e.to_string())
    })?;

    let options = McpServerOptions {
        server_details,
        transport,
        handler,
        task_store: None,
        client_task_store: None,
        message_observer: None,
    };

    let server = server_runtime::create_server(options);
    server.start().await?;

    Ok(())
}
