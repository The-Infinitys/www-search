// src/search/bing.rs

use crate::SearchData; // lib.rsからSearchData構造体をインポート

/// Bing検索を実行するダミー関数です。
///
/// 実際には、HTTPリクエストを送信し、Bingの検索結果ページをパースするロジックがここに入ります。
/// 現在は、指定されたクエリに基づいてダミーの検索結果を返します。
///
/// # 引数
/// - `query`: 検索クエリ文字列。
///
/// # 戻り値
/// `Result<Vec<SearchData>, String>`:
/// - 成功した場合: ダミーの検索結果リスト。
/// - 失敗した場合: 常に成功として空でないリストを返します (エラー処理は将来の実装用)。
pub async fn search_bing(query: String) -> Result<Vec<SearchData>, String> {
    // 実際のBing検索API呼び出しやWebスクレイピングのロジックがここに入る
    // 例: reqwest::get("https://www.bing.com/search?q={query}").await?.text().await?

    // デモンストレーション用のダミーデータ
    let results = vec![
        SearchData {
            title: format!("{} - Bing 検索結果 A", query),
            url: "https://example.com/bing/resultA".to_string(),
            description: "これはBing検索結果のダミー説明Aです。".to_string(),
        },
        SearchData {
            title: format!("{} - Bing 検索結果 B", query),
            url: "https://example.com/bing/resultB".to_string(),
            description: "これはBing検索結果のダミー説明Bです。".to_string(),
        },
    ];

    Ok(results)
}
