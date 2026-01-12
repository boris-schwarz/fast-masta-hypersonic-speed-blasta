// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

fn get_highlight_index(length: usize) -> usize {
    match length {
        1 => 0,
        2..=5 => 1,
        6..=9 => 2,
        10..=13 => 3,
        _ => 4,
    }
}

fn add_word_anchor(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let length = chars.len();
    let n = get_highlight_index(length);

    let before: String = chars[..n].iter().collect();
    let highlight = chars[n];
    let after: String = chars[n + 1..].iter().collect();

    format!(
        "<span class=\"before\">{}</span><span class=\"highlight\">{}</span><span class=\"after\">{}</span>",
        before, highlight, after
    )
}

#[tauri::command]
fn parse_text(text: &str) -> Vec<String> {
    text.split_whitespace()
        .flat_map(|word| word.split_inclusive('-'))
        .flat_map(|word| word.split_inclusive('/'))
        .map(|word| add_word_anchor(word))
        .collect()
}

#[tauri::command]
fn extract_pdf_text(bytes: Vec<u8>) -> Result<String, String> {
    pdf_extract::extract_text_from_mem(&bytes)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn extract_pdf_text_from_path(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    pdf_extract::extract_text_from_mem(&bytes)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn fetch_url(url: &str) -> Result<String, String> {
    use scraper::{Html, Selector};

    let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let html = response.text().map_err(|e| e.to_string())?;

    let document = Html::parse_document(&html);

    let body_selector =
        Selector::parse("body").map_err(|e| format!("Invalid body selector: {e}"))?;
    let exclude_selector =
        Selector::parse("nav, #navbar, .navbar, script, style, noscript, [role='navigation']")
            .map_err(|e| format!("Invalid exclude selector: {e}"))?;

    let text: String = if let Some(body) = document.select(&body_selector).next() {
        let exclude_ids: std::collections::HashSet<_> = body
            .select(&exclude_selector)
            .flat_map(|el| std::iter::once(el.id()).chain(el.descendants().map(|node| node.id())))
            .collect();

        body.descendants()
            .filter_map(|node| {
                if exclude_ids.contains(&node.id()) {
                    return None;
                }
                node.value().as_text().map(|t| t.text.trim())
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        let exclude_ids: std::collections::HashSet<_> = document
            .select(&exclude_selector)
            .flat_map(|el| std::iter::once(el.id()).chain(el.descendants().map(|node| node.id())))
            .collect();

        document
            .root_element()
            .descendants()
            .filter_map(|node| {
                if exclude_ids.contains(&node.id()) {
                    return None;
                }
                node.value().as_text().map(|t| t.text.trim())
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    };

    Ok(text)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            parse_text,
            extract_pdf_text,
            extract_pdf_text_from_path,
            fetch_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
