// src/search/duckduckgo.rs

use std::str::FromStr;

use reqwest;
use scraper::{Html, Selector};

use crate::SearchData; // lib.rsからSearchData構造体をインポート

/// DuckDuckGo検索を実行し、lite版のHTMLをパースして検索結果を返す
///
/// 実際には、HTTPリクエストを送信し、DuckDuckGoの検索結果ページをパースするロジックがここに入ります。
/// 現在は、指定されたクエリに基づいてダミーの検索結果を返します。
///
/// # 引数
/// - `query`: 検索クエリ文字列。
///
/// # 戻り値
/// `Result<Vec<SearchData>, String>`:
/// - 成功した場合: ダミーの検索結果リスト。
/// - 失敗した場合: 常に成功として空でないリストを返します (エラー処理は将来の実装用)。
pub async fn search_duckduckgo(query: String) -> Result<Vec<SearchData>, String> {
    let url = format!(
        "https://lite.duckduckgo.com/lite/?q={}",
        urlencoding::encode(&query)
    );
    let client = match reqwest::ClientBuilder::new()
        .user_agent("w3m (w3m/0.5.3+git20230121)")
        .build()
    {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to build reqwest client: {}", e)),
    };
    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(t) => t,
            Err(e) => return Err(format!("Failed to get text from DuckDuckGo: {}", e)),
        },
        Err(e) => return Err(format!("Failed to send request to DuckDuckGo: {}", e)),
    };
    // HTMLパース
    let document = Html::parse_document(&html);
    let mut results = Vec::new();
    // 検索結果は form > div.results > a.result-link などの構造
    let result_selector = Selector::parse("a.result-link").unwrap();
    for a in document.select(&result_selector) {
        let title = a.text().collect::<Vec<_>>().join("").trim().to_string();
        let url = format!("https:{}", a.value().attr("href").unwrap_or("").to_string());
        let url = url::Url::from_str(&url).unwrap();
        let url = url
            .query_pairs()
            .find(|(key, _)| key == "uddg")
            .map(|(_, value)| value.into_owned())
            .unwrap_or_else(|| url.to_string());
        // 説明文はaの次の兄弟要素（smallタグ）
        let mut description = String::new();
        let mut next = a.next_sibling();
        while let Some(node) = next {
            if let Some(elem) = scraper::ElementRef::wrap(node) {
                if elem.value().name() == "small" {
                    description = elem.text().collect::<Vec<_>>().join("").trim().to_string();
                    break;
                }
            }
            next = node.next_sibling();
        }
        if !title.is_empty() && !url.is_empty() {
            results.push(SearchData {
                title,
                url,
                description,
            });
        }
    }
    Ok(results)
}
