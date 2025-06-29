// src/search/google.rs

use crate::SearchData; // lib.rsからSearchData構造体をインポート
use reqwest; // HTTPクライアントのreqwestをインポート
use scraper::{ElementRef, Html, Selector}; // HTMLパース用のscraperクレートをインポート
// URL解析のためにurlクレートをインポート
use urlencoding; // URLエンコーディングのためにurlencodingをインポート

/// Google検索を実行し、生のHTMLコンテンツを取得し、それを指定された基準でパースします。
///
/// # Arguments
///
/// * `query` - 検索クエリ文字列。
///
/// # Returns
///
/// `Result<Vec<SearchData>, String>`:
/// - 成功した場合: 検索結果のリスト (`Vec<SearchData>`)。
/// - 失敗した場合: エラーメッセージ文字列。
pub async fn search_google(query: String) -> Result<Vec<SearchData>, String> {
    // Google検索のURLを構築
    let url = format!(
        "https://www.google.com/search?q={}",
        urlencoding::encode(&query)
    );

    // reqwest::Clientを使用し、クッキーとリダイレクトを有効化
    let client = match reqwest::ClientBuilder::new()
        .cookie_store(true)
        .user_agent("w3m (w3m/0.5.3+git20230121)")
        .build()
    {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to build reqwest client: {}", e)),
    };

    // クライアントでGETリクエストを送信
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().as_u16().to_string().starts_with("2") {
                match response.text().await {
                    Ok(html) => {
                        // 取得したHTMLをパースし、検索データを抽出
                        let results = parse_data(html);
                        Ok(results)
                    }
                    Err(e) => Err(format!("Failed to get text from Google response: {}", e)),
                }
            } else {
                Err(format!(
                    "Google search request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Failed to send request to Google: {}", e)),
    }
}

/// Google検索結果の生のHTML文字列を `SearchData` のベクターにパースします。
///
/// この関数は、以下の指定された基準に基づいてタイトル、URL、説明を抽出します。
/// - **URL**: `<a>` タグの `ping` 属性から、GoogleのリダイレクトURLを介して実際のURLを取得します。
/// - **タイトル**: `ping` 属性を持つ `<a>` タグの子要素である最初の `<h3>` のテキストコンテンツ。
/// - **説明**: `<a>` タグと同じ `data-ved` 属性を持つ `div` 要素を見つけ、
///   その子要素の2番目の `div`、その中の `div`、さらにその中の `span` のテキストを抽出します。
///   説明内の `<em>` タグは Markdown の `*テキスト*` 形式に変換されます。
///
/// # 重要な注意点:
/// GoogleのHTML構造は頻繁に変更されるため、ここで使用されているセレクタとパースロジックは
/// 非常に特定性が高く、すぐに使えなくなる可能性があります。
/// 本番レベルのパーサーには、より堅牢で適応性のある選択ロジック（例：ヘッドレスブラウザの利用やGoogle Search APIの利用）が推奨されます。
///
/// # Arguments
///
/// * `html_str` - Google検索結果ページの生のHTMLコンテンツ。
///
/// # Returns
///
/// `Vec<SearchData>`: パースされた検索結果を含むベクター。
fn parse_data(html_str: String) -> Vec<SearchData> {
    let document = Html::parse_document(&html_str);
    let mut search_results = Vec::new();

    let root_selector = match Selector::parse("body > div > div > div > div > div > div") {
        Ok(selector) => selector,
        Err(_) => {
            eprintln!("Error parsing selector. Returning empty results.");
            return Vec::new();
        }
    };

    for root_div in document.select(&root_selector) {
        // aタグ取得
        let a_selector = Selector::parse("a").unwrap();
        let a_element = match root_div.select(&a_selector).next() {
            Some(a) => a,
            None => continue,
        };
        // URL: href から /url?q=... のqパラメータを正確に抽出
        let mut url: Option<String> = None;
        if let Some(href) = a_element.value().attr("href") {
            if let Some(q_start) = href.find("q=") {
                let q_and_rest = &href[q_start + 2..];
                let q_value = q_and_rest.split('&').next().unwrap_or("");
                if let Ok(decoded) = urlencoding::decode(q_value) {
                    url = Some(decoded.into_owned());
                }
            }
        }
        // タイトル: a要素の最初のspan子要素のテキスト
        let span_selector = Selector::parse("span").unwrap();
        let title = a_element
            .select(&span_selector)
            .next()
            .map(|s| s.text().collect::<String>());
        // 説明: a要素の親→親のtable要素を探し、行ごとに改行で結合
        let mut description: Option<String> = None;
        if let Some(parent1) = a_element.parent().and_then(ElementRef::wrap) {
            if let Some(parent2) = parent1.parent().and_then(ElementRef::wrap) {
                // table要素を探す
                let table_selector = Selector::parse("table").unwrap();
                if let Some(table) = parent2.select(&table_selector).next() {
                    // trごとにテキストを改行で結合
                    let tr_selector = Selector::parse("tr").unwrap();
                    let mut lines = Vec::new();
                    for tr in table.select(&tr_selector) {
                        let line = tr.text().collect::<Vec<_>>().join("").trim().to_string();
                        if !line.is_empty() {
                            lines.push(line);
                        }
                    }
                    if !lines.is_empty() {
                        description = Some(lines.join("\n"));
                    } else {
                        // trがなければtable全体のテキスト
                        let table_text =
                            table.text().collect::<Vec<_>>().join("").trim().to_string();
                        if !table_text.is_empty() {
                            description = Some(table_text);
                        }
                    }
                }
            }
        }
        // URLとタイトルが取得できた場合、結果に追加
        if let (Some(t), Some(u), Some(description)) = (title, url, description) {
            search_results.push(SearchData {
                title: t,
                url: u,
                description: description,
            });
        }
    }
    search_results
}
