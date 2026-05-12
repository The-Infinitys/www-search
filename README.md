# www-search

`www-search` is a Rust-based Web Search client and [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server. It allows you to fetch search results from engines like Google and DuckDuckGo and retrieve web page content in Markdown format.

## Features
- **Search Engines**: Supports Google and DuckDuckGo search.
- **MCP Support**: Can be used as an MCP server with AI clients like Claude Desktop.
- **Markdown Conversion**: Fetches web page content and converts it to readable Markdown.
- **Flexible Data Extraction**: Uses HTML parsing for data extraction without requiring official APIs.
- **Async/Sync Support**: Provides both asynchronous and synchronous interfaces for integration.

## MCP Server Setup

To use `www-search` as an MCP server with Claude Desktop:

1. **Build the project**:
   ```bash
   cargo build --release
   ```

2. **Configure Claude Desktop**:
   Add the following to your `claude_desktop_config.json` (usually located at `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS or `%APPDATA%\Claude\claude_desktop_config.json` on Windows):

   ```json
   {
     "mcpServers": {
       "www-search": {
         "command": "/path/to/www-search/target/release/www-search"
       }
     }
   }
   ```
   *Replace `/path/to/www-search` with the actual absolute path to the project.*

3. **Restart Claude Desktop**.

### Available Tools
- `web_search`: Search the web using Google or DuckDuckGo.
  - Arguments: `query` (string), `engine` (optional: "google", "duckduckgo").
- `fetch_page`: Fetch the content of a web page and convert it to Markdown.
  - Arguments: `url` (string).

## Library Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
www-search = { path = "./www-search" }
```

### Search Example
```rust
use www_search::{www_search, EngineType, SearchData};

#[tokio::main]
async fn main() {
    let query = "Rust programming".to_string();
    
    // Search using Google
    let results = www_search(EngineType::Google, query).await.unwrap();
    
    for result in results {
        println!("Title: {}", result.title);
        println!("URL: {}", result.url);
        println!("Description: {}", result.description);
        println!("---");
    }
}
```

### Fetch Page Example
```rust
use www_search::browse;

#[tokio::main]
async fn main() {
    let url = "https://example.com";
    let markdown = browse::fetch_and_markdown(url).await.unwrap();
    println!("{}", markdown);
}
```

## Data Structure
```rust
pub struct SearchData {
    pub title: String,
    pub url: String,
    pub description: String,
}
```

## Important Notes
- **Scraping Based**: This library relies on HTML scraping. Since search engine HTML structures change frequently, the parsing logic might break.
- **No Official API**: This is not an official API client. Use it responsibly and at your own risk.
- **TLS**: Uses `rustls-tls` to avoid native OpenSSL dependencies.
