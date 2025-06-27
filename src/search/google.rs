// src/search/google.rs

use crate::SearchData; // lib.rsからSearchData構造体をインポート
use reqwest; // HTTPクライアントのreqwestをインポート
use scraper::{ElementRef, Html, Selector}; // HTMLパース用のscraperクレートをインポート
use url::Url; // URL解析のためにurlクレートをインポート
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
    // Note: Google's search URL might vary by region or specific features.
    let url = format!(
        "https://www.google.com/search?q={}",
        urlencoding::encode(&query)
    );

    println!("Fetching HTML from: {}", url); // 取得するURLを表示

    // reqwestを使用してHTTP GETリクエストを送信し、HTMLコンテンツを取得
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                // レスポンスが成功した場合
                match response.text().await {
                    Ok(html) => {
                        println!("Successfully fetched HTML from Google (first 500 chars):");
                        println!("{}", &html[0..std::cmp::min(html.len(), 500)]); // HTMLの最初の部分を表示
                        println!("...");

                        // 取得したHTMLをパースし、検索データを抽出
                        let results = parse_data(html);
                        Ok(results)
                    }
                    Err(e) => Err(format!("Failed to get text from Google response: {}", e)),
                }
            } else {
                // レスポンスが成功ステータスでなかった場合
                Err(format!(
                    "Google search request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Failed to send request to Google: {}", e)),
    }
}

/// ElementRefの子ノードを再帰的にトラバースし、テキストコンテンツを収集します。
/// `<em>` タグはMarkdownの `*テキスト*` 形式に変換されます。
///
/// # Arguments
///
/// * `element` - テキストコンテンツを抽出する対象の`ElementRef`。
///
/// # Returns
///
/// `String`: 変換されたテキストコンテンツ。
fn get_text_with_markdown_em(element: &ElementRef) -> String {
    let mut result = String::new();
    for child in element.children() {
        if let Some(text_node) = child.value().as_text() {
            // テキストノードの場合、そのテキストを追加
            result.push_str(text_node.trim());
        } else if let Some(child_element) = ElementRef::wrap(child) {
            // 要素ノードの場合
            if child_element.value().name() == "em" {
                // <em> タグの場合、Markdownの * を追加し、子を再帰的に処理
                result.push('*');
                result.push_str(&get_text_with_markdown_em(&child_element));
                result.push('*');
            } else {
                // その他の要素の場合、その子を再帰的に処理（タグ自体は含めない）
                result.push_str(&get_text_with_markdown_em(&child_element));
            }
        }
    }
    result
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

    // 1. aタグをすべて検索し、pingキーを保有しているものを探す
    let a_ping_selector = match Selector::parse("a[ping]") {
        Ok(selector) => selector,
        Err(_) => {
            eprintln!("Error parsing selector 'a[ping]'. Returning empty results.");
            return Vec::new();
        }
    };

    // 'ping' 属性を持つ各 'a' 要素をイテレート
    for a_element in document.select(&a_ping_selector) {
        let mut title: Option<String> = None;
        let mut url: Option<String> = None;
        let mut description: Option<String> = None;
        let mut data_ved_value: Option<String> = None;

        // URLについて:
        // 1. pingキーを保有しているものを探す
        // 2. pingキーは絶対パス(スラッシュで始まる)の文字列を保有する。これを元のドメインを使用して、URLに変換する
        // 3. 変換により手に入れたURLから、urlパラメータを取得する
        // 4. その取得できた文字列が、URLとなる
        if let Some(ping_path) = a_element.value().attr("ping") {
            let full_google_redirect_url_str = format!("https://www.google.com{}", ping_path);
            if let Ok(parsed_google_url) = Url::parse(&full_google_redirect_url_str) {
                if let Some(q_param) = parsed_google_url.query_pairs().find(|(key, _)| key == "q") {
                    url = Some(q_param.1.to_string());
                }
            }
        }

        // タイトルについて: pingキーを保有するaエレメントの子要素の最初のh3のinnerHTMLがタイトルとなる
        let h3_selector = match Selector::parse("h3") {
            Ok(selector) => selector,
            Err(_) => {
                eprintln!("Error parsing selector 'h3' for title. Skipping current 'a' element.");
                continue; // Skip to the next 'a' element if selector fails
            }
        };
        if let Some(h3_element) = a_element.select(&h3_selector).next() {
            title = Some(h3_element.text().collect::<String>());
        }

        // 説明について: 対応するaエレメントからdata-vedの値を取得する
        if let Some(ved_value) = a_element.value().attr("data-ved") {
            data_ved_value = Some(ved_value.to_string());
        }

        // 説明の残りのステップ
        if let Some(ved_val) = data_ved_value {
            // 2. divエレメントで同じdata-ved属性を保有する要素を探す (ドキュメント全体から)
            let desc_ved_div_selector_str = format!("div[data-ved=\"{}\"]", ved_val);
            let desc_ved_div_selector = match Selector::parse(&desc_ved_div_selector_str) {
                Ok(selector) => selector,
                Err(_) => {
                    eprintln!(
                        "Error parsing selector for description div: '{}'. Skipping description for this item.",
                        desc_ved_div_selector_str
                    );
                    continue; // Skip to the next 'a' element if selector fails
                }
            };

            // ドキュメント全体からこの特定のdivを検索
            if let Some(desc_ved_div) = document.select(&desc_ved_div_selector).next() {
                // 3. その子要素の中から、2番目のdiv>div>spanを探し、そのHTMLが説明となっている
                //    (emなどが混じっているので、逐一markdown記法に変えること)

                // 指定された構造 (2番目のdiv > div > span) に従って手動でトラバース
                // これは非常に具体的であり、Googleの構造変更によって壊れやすいです。
                let mut current_div_children = desc_ved_div.children().filter(|node| {
                    node.value().is_element() && node.value().as_element().unwrap().name() == "div"
                });

                if let Some(second_div_child_node) = current_div_children.nth(1) {
                    if let Some(second_div_child_element) = ElementRef::wrap(second_div_child_node) {
                        let mut inner_div_children = second_div_child_element.children().filter(|node| {
                            node.value().is_element()
                                && node.value().as_element().unwrap().name() == "div"
                        });
                        if let Some(inner_div_child_node) = inner_div_children.next() {
                            if let Some(inner_div_child_element) = ElementRef::wrap(inner_div_child_node) {
                                let mut span_children = inner_div_child_element.children().filter(|node| {
                                    node.value().is_element()
                                        && node.value().as_element().unwrap().name() == "span"
                                });
                                if let Some(span_element_node) = span_children.next() {
                                    if let Some(span_element_ref) = ElementRef::wrap(span_element_node) {
                                        // ヘルパー関数を使用して、<em> タグを Markdown 形式に変換しながらテキストを抽出
                                        description = Some(get_text_with_markdown_em(&span_element_ref));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // URLとタイトルが取得できた場合、結果に追加
        if let (Some(t), Some(u)) = (title, url) {
            search_results.push(SearchData {
                title: t,
                url: u,
                description: description.unwrap_or_else(|| "".to_string()), // 説明がない場合は空文字列
            });
        }
    }

    search_results
}
