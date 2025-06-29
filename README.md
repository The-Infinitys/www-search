# www-search

Rust製のWeb検索クライアントライブラリです。Google、DuckDuckGo、Bingなどの検索エンジンから検索結果を取得し、構造化データとして利用できます。

## 特徴
- Google、DuckDuckGo、Bingの検索結果取得に対応
- HTMLパースによる柔軟なデータ抽出
- 非同期・同期両対応（DuckDuckGo）
- Rust標準の型で結果を返却

## 使い方

### 1. 依存関係
Cargo.toml:
```toml
[dependencies]
www-search = { path = "./www-search" }
```

### 2. 検索の実行例
```rust
use www_search::{search_google, search_duckduckgo, SearchData};

#[tokio::main]
async fn main() {
    let query = "Rustとは".to_string();
    // Google検索（非同期）
    let google_results = search_google(query.clone()).await.unwrap();
    for result in google_results {
        println!("{}\n{}\n{}\n---", result.title, result.url, result.description);
    }
    // DuckDuckGo検索（非同期）
    let ddg_results = search_duckduckgo(query.clone()).await.unwrap();
    // DuckDuckGo検索（同期）
    let ddg_results_sync = www_search::search_duckduckgo_sync(query).unwrap();
}
```

## 検索結果データ構造
```rust
pub struct SearchData {
    pub title: String,
    pub url: String,
    pub description: String,
}
```

## 注意事項
- Google等のHTML構造は頻繁に変化するため、パースロジックが動作しなくなる場合があります。
- 本ライブラリは公式APIではなく、HTMLスクレイピングによるものです。
- 利用は自己責任でお願いします。

## ライセンス
MIT License
