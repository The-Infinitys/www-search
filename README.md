# www-search

Rust製のWeb検索クライアント・CLIツールです。GoogleやDuckDuckGoなどの検索エンジンから検索結果を取得し、構造化データやWebページ本文（Markdown形式）として利用できます。

## 特徴
- Google・DuckDuckGoの検索結果取得に対応
- HTMLパースによる柔軟なデータ抽出
- DuckDuckGoは非同期・同期両対応
- CLIからエンジン選択・検索・ページ閲覧（Markdown出力）が可能
- Rust標準の型で結果を返却

## 使い方

### 1. 依存関係
Cargo.toml:
```toml
[dependencies]
www-search = { path = "./www-search" }
```

### 2. CLIの例
```sh
# Google検索（デフォルト）
cargo run -- "Rustとは"

# DuckDuckGoで検索
your_binary --engine duckduckgo "Rustとは"

# 検索結果から番号を選んでWebページ本文をMarkdownで表示
your_binary --engine google "Rust"  # → 検索結果一覧から番号入力

# URLを直接Markdownで閲覧
your_binary --browse https://example.com
```

### 3. ライブラリとしての利用例
```rust
use www_search::{search_google, search_duckduckgo, search_duckduckgo_sync, SearchData};

#[tokio::main]
async fn main() {
    let query = "Rustとは".to_string();
    // Google検索（非同期）
    let google_results = search_google(query.clone()).await.unwrap();
    // DuckDuckGo検索（非同期）
    let ddg_results = search_duckduckgo(query.clone()).await.unwrap();
    // DuckDuckGo検索（同期）
    let ddg_results_sync = search_duckduckgo_sync(query).unwrap();
}
```

### 4. Webページ本文のMarkdown取得
```rust
let md = browse::fetch_and_markdown("https://example.com").await.unwrap();
println!("{}", md);
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
