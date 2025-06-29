// src/lib.rs

//! # WWW Search Library for Rust
//!
//! このライブラリは、Google, Bing, DuckDuckGoなどの様々な検索エンジンを使用して、ネット検索を行うことができます。

// searchモジュールを宣言し、その中の関数や型を公開する
pub mod search;

use crate::search::{duckduckgo, google};

/// 検索エンジンの種類を定義するEnum
///
/// - `Google`: Google検索 (デフォルト)
/// - `Bing`: Bing検索
/// - `DuckDuckGo`: DuckDuckGo検索
#[derive(Debug, Clone, Copy, Default)]
pub enum EngineType {
    #[default]
    Google, // default
    DuckDuckGo,
}

/// 検索結果のデータを保持する構造体
///
/// - `title`: 検索結果のタイトル
/// - `url`: 検索結果のURL
/// - `description`: 検索結果の概要 (オプション)
#[derive(Debug, Clone)]
pub struct SearchData {
    pub title: String,       // 必須
    pub url: String,         // 必須
    pub description: String, // オプション
}

/// 指定された検索エンジンとクエリを使用して、ウェブ検索を実行します。
///
/// # 引数
/// - `engine`: 使用する検索エンジンの種類 (`EngineType` enum)。
/// - `query`: 検索クエリ文字列。
///
/// # 戻り値
/// `Result<Vec<SearchData>, String>`:
/// - 成功した場合: 検索結果のリスト (`Vec<SearchData>`)。
/// - 失敗した場合: エラーメッセージ文字列。
///
/// # 例
/// ```ignore
/// use www_search_lib::{www_search, EngineType};
///
/// async fn run_search() {
///     let query = "Rustプログラミング";
///
///     // Google検索を実行
///     match www_search(EngineType::Google, query.to_string()).await {
///         Ok(results) => {
///             println!("Google 検索結果:");
///             for result in results {
///                 println!("  タイトル: {}", result.title);
///                 println!("  URL: {}", result.url);
///                 println!("  説明: {}", result.description);
///             }
///         }
///         Err(e) => eprintln!("Google 検索エラー: {}", e),
///     }
///
///     // DuckDuckGo検索を実行
///     match www_search(EngineType::DuckDuckGo, query.to_string()).await {
///         Ok(results) => {
///             println!("\nBing 検索結果:");
///             for result in results {
///                 println!("  タイトル: {}", result.title);
///                 println!("  URL: {}", result.url);
///                 println!("  説明: {}", result.description);
///             }
///         }
///         Err(e) => eprintln!("Bing 検索エラー: {}", e),
///     }
/// }
/// ```
pub async fn www_search(engine: EngineType, query: String) -> Result<Vec<SearchData>, String> {
    // 選択されたエンジンに基づいて適切な検索関数を呼び出す
    match engine {
        EngineType::Google => google::search_google(query).await,
        EngineType::DuckDuckGo => duckduckgo::search_duckduckgo(query).await,
    }
}
