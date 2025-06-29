// src/main.rs

// lib.rsから必要な要素をインポート
use www_search::{www_search, EngineType};
 // async/awaitのためにtokioを使用

/// メイン関数
///
/// この関数は、`www_search`関数を使用してGoogleとBingの検索を実行し、結果を出力します。
#[tokio::main]
async fn main() {
    let query = "Rustとは";

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

    // Bing検索を実行
    println!("\nSearching with Bing for: '{}'", query);
    match www_search(EngineType::Bing, query.to_string()).await {
        Ok(results) => {
            println!("Bing Search Results:");
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
        Err(e) => eprintln!("Error during Bing search: {}", e),
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

