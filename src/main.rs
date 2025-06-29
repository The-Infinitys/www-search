// src/main.rs

use std::io::Write;

// lib.rsから必要な要素をインポート
use www_search::{EngineType, www_search};
// async/awaitのためにtokioを使用

/// メイン関数
///
/// この関数は、`www_search`関数を使用してGoogleとBingの検索を実行し、結果を出力します。
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let query = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        let mut s = String::new();
        print!("query: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut s).ok();
        s.trim().parse().ok().unwrap()
    };

    println!("--- WWW Search Library Example ---");

    // Google検索を実行
    println!("\nSearching with Google for: '{}'", query);
    match www_search(EngineType::Google, query.to_string()).await {
        Ok(results) => {
            println!("Google Search Results:");
            if results.is_empty() {
                println!("  No results found.");
            } else {
                for result in results {
                    println!("  Title: {}", result.title);
                    println!("  URL: {}", result.url);
                    println!("  Description: {}", result.description);
                    println!("  ---");
                }
            }
        }
        Err(e) => eprintln!("Error during Google search: {}", e),
    }

    // DuckDuckGo検索を実行
    println!("\nSearching with DuckDuckGo for: '{}'", query);
    match www_search(EngineType::DuckDuckGo, query.to_string()).await {
        Ok(results) => {
            println!("DuckDuckGo Search Results:");
            if results.is_empty() {
                println!("  No results found.");
            } else {
                for result in results {
                    println!("  Title: {}", result.title);
                    println!("  URL: {}", result.url);
                    println!("  Description: {}", result.description);
                    println!("  ---");
                }
            }
        }
        Err(e) => eprintln!("Error during DuckDuckGo search: {}", e),
    }

    println!("\n--- End of Example ---");
}
