// src/main.rs

use std::io::Write;
use www_search::{EngineType, www_search};
mod browse;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut engine = EngineType::Google;
    let mut query = String::new();
    let mut url_to_browse = None;

    // 引数パース: --engine, --browse, 検索クエリ
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--engine" => {
                if i + 1 < args.len() {
                    engine = match args[i + 1].to_lowercase().as_str() {
                        // "google" => EngineType::Google,
                        "duckduckgo" => EngineType::DuckDuckGo,
                        _ => EngineType::Google,
                    };
                    i += 1;
                }
            }
            "--browse" => {
                if i + 1 < args.len() {
                    url_to_browse = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {
                query.push_str(&args[i]);
                query.push(' ');
            }
        }
        i += 1;
    }
    query = query.trim().to_string();
    if query.is_empty() && url_to_browse.is_none() {
        print!("query: ");
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        query = s.trim().to_string();
    }

    if let Some(url) = url_to_browse {
        println!("\n--- Browse Mode ---");
        match browse::fetch_and_markdown(&url).await {
            Ok(md) => println!("\n# Page Content (Markdown)\n\n{}", md),
            Err(e) => eprintln!("Failed to browse: {}", e),
        }
        return;
    }

    println!("--- WWW Search Library ---");
    println!("\nSearching with {:?} for: '{}'", engine, query);
    match www_search(engine, query.clone()).await {
        Ok(results) => {
            if results.is_empty() {
                println!("  No results found.");
                return;
            }
            for (i, result) in results.iter().enumerate() {
                println!("[{}] {}\n    {}\n    {}\n---", i + 1, result.title, result.url, result.description);
            }
            print!("\nSelect result number to browse (or Enter to skip): ");
            std::io::stdout().flush().unwrap();
            let mut sel = String::new();
            std::io::stdin().read_line(&mut sel).ok();
            if let Ok(idx) = sel.trim().parse::<usize>() {
                if idx > 0 && idx <= results.len() {
                    let url = &results[idx - 1].url;
                    println!("\n--- Browsing: {} ---", url);
                    match browse::fetch_and_markdown(url).await {
                        Ok(md) => println!("\n# Page Content (Markdown)\n\n{}", md),
                        Err(e) => eprintln!("Failed to browse: {}", e),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error during search: {}", e),
    }
}
