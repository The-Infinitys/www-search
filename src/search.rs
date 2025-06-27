// src/search.rs

//! このモジュールは、各種検索エンジンモジュールを再エクスポートします。

// googleモジュールを宣言し、その中の関数を公開する
pub mod google;
// bingモジュールを宣言し、その中の関数を公開する
pub mod bing;
// duckduckgoモジュールを宣言し、その中の関数を公開する
pub mod duckduckgo;

// 必要に応じて、各検索エンジンの共通ヘルパー関数や共通エラー処理などをここに定義できます。

