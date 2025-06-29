use reqwest;
use scraper::{Html, Selector};

/// 指定したURLのWebページ本文を取得し、Markdown形式で返す関数
pub async fn fetch_and_markdown(url: &str) -> Result<String, String> {
    let html = match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(t) => t,
            Err(e) => return Err(format!("Failed to get text: {}", e)),
        },
        Err(e) => return Err(format!("Failed to fetch URL: {}", e)),
    };
    let document = Html::parse_document(&html);
    // 本文抽出: <main> > <article> > <p> などを優先し、なければbody内の<p>を抽出
    let mut markdown = String::new();
    let selectors = [
        "main article p", // 一般的な構造
        "main p",
        "article p",
        "body p",
    ];
    for sel in selectors.iter() {
        if let Ok(selector) = Selector::parse(sel) {
            let mut found = false;
            for p in document.select(&selector) {
                let text = p.text().collect::<Vec<_>>().join("").trim().to_string();
                if !text.is_empty() {
                    markdown.push_str(&format!("\n{}
", text));
                    found = true;
                }
            }
            if found {
                return Ok(markdown.trim().to_string());
            }
        }
    }
    // fallback: body全体のテキスト
    if let Ok(selector) = Selector::parse("body") {
        if let Some(body) = document.select(&selector).next() {
            let text = body.text().collect::<Vec<_>>().join("").trim().to_string();
            if !text.is_empty() {
                return Ok(text);
            }
        }
    }
    Err("No readable content found".to_string())
}
