// src/search/duckduckgo.rs

use reqwest;
use scraper::{Html, Selector};
use std::fs;
use std::str::FromStr;

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
    println!("Successfully fetched HTML from DuckDuckGo (first 500 chars):");
    println!("{}", &html[0..std::cmp::min(html.len(), 500)]); // HTMLの最初の部分を表示
    println!("...");
    // 取得したHTMLをdata.htmlファイルに書き込み
    if let Err(e) = fs::write("data.html", &html) {
        eprintln!("Failed to write HTML to data.html: {}", e);
    }
    // HTMLパース
    let document = Html::parse_document(&html);
    let mut results = Vec::new();
    // 検索結果は form > div.results > a.result-link などの構造
    let result_selector = Selector::parse("a.result-link").unwrap();
    for a in document.select(&result_selector) {
        let title = a.text().collect::<Vec<_>>().join("").trim().to_string();
        // URL抽出
        let href = a.value().attr("href").unwrap_or("");
        let url = if href.starts_with("//") {
            let abs = format!("https:{}", href);
            if let Ok(parsed) = url::Url::from_str(&abs) {
                parsed
                    .query_pairs()
                    .find(|(k, _)| k == "uddg")
                    .map(|(_, v)| v.into_owned())
                    .unwrap_or(abs)
            } else {
                abs
            }
        } else {
            href.to_string()
        };
        // description: aの親td→親tr→次の兄弟trのtd.result-snippet
        let mut description = String::new();
        if let Some(parent_td) = a.parent().and_then(scraper::ElementRef::wrap) {
            if let Some(parent_tr) = parent_td.parent().and_then(scraper::ElementRef::wrap) {
                let mut next_tr = parent_tr.next_sibling();
                while let Some(node) = next_tr {
                    if let Some(tr_elem) = scraper::ElementRef::wrap(node) {
                        let td_selector = Selector::parse("td.result-snippet").unwrap();
                        if let Some(snippet_td) = tr_elem.select(&td_selector).next() {
                            description = snippet_td
                                .text()
                                .collect::<Vec<_>>()
                                .join("")
                                .trim()
                                .to_string();
                            break;
                        }
                    }
                    next_tr = node.next_sibling();
                }
            }
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
