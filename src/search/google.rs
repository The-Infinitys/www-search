// src/search/google.rs

use crate::SearchData; // lib.rsからSearchData構造体をインポート
use reqwest; // HTTPクライアントのreqwestをインポート

/// Google検索を実行し、生のHTMLコンテンツを取得します。
///
/// 実際には、取得したHTMLをパースして`SearchData`を抽出する必要があります。
/// 現在はHTMLをフェッチし、ダミーの`SearchData`を返します。
///
/// # 引数
/// - `query`: 検索クエリ文字列。
///
/// # 戻り値
/// `Result<Vec<SearchData>, String>`:
/// - 成功した場合: ダミーの検索結果リスト。
/// - 失敗した場合: エラーメッセージ文字列。
pub async fn search_google(query: String) -> Result<Vec<SearchData>, String> {
    // Google検索のURLを構築
    let url = format!("https://www.google.com/search?q={}", urlencoding::encode(&query));

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

                        // TODO: ここで取得したHTMLをパースし、SearchDataを抽出するロジックを実装

                        // デモンストレーション用のダミーデータ (実際のHTMLから抽出するまではこれを使用)
                        let results = vec![
                            SearchData {
                                title: format!("{} - Google Result 1 (Dummy)", query),
                                url: "https://dummy.example.com/google/1".to_string(),
                                description: "This is a dummy description for Google search result 1.".to_string(),
                            },
                            SearchData {
                                title: format!("{} - Google Result 2 (Dummy)", query),
                                url: "https://dummy.example.com/google/2".to_string(),
                                description: "This is a dummy description for Google search result 2.".to_string(),
                            },
                        ];
                        Ok(results)
                    }
                    Err(e) => Err(format!("Failed to get text from Google response: {}", e)),
                }
            } else {
                // レスポンスが成功ステータスでなかった場合
                Err(format!("Google search request failed with status: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Failed to send request to Google: {}", e)),
    }
}
