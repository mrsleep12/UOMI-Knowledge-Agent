use std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;
use ureq;

lazy_static! {
    static ref HISTORY: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn log(msg: &str) { println!("[LOG] {}", msg); }
pub fn add_to_history(question: &str) { HISTORY.lock().unwrap().push(question.to_string()); }
pub fn get_history() -> Vec<String> { HISTORY.lock().unwrap().clone() }

pub fn detect_language(question: &str) -> &str {
    if question.to_lowercase().contains("bahasa") || question.to_lowercase().contains("id") {
        "id"
    } else {
        "en"
    }
}

pub fn read_knowledge(question: &str, lang: &str) -> Option<String> {
    let data = fs::read_to_string("knowledge/uomi_docs.json").ok()?;
    let kb: serde_json::Value = serde_json::from_str(&data).ok()?;
    let q_lower = question.to_lowercase();
    for (key, val) in kb.as_object()? {
        if q_lower.contains(&key.to_lowercase()) {
            return val.get(lang)?.as_str().map(|s| s.to_string());
        }
    }
    None
}

pub fn fetch_crypto_price(question: &str) -> Option<f64> {
    let coin = if question.to_lowercase().contains("eth") { "ethereum" }
               else if question.to_lowercase().contains("btc") { "bitcoin" }
               else { return None; };
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", coin);
    if let Ok(response) = ureq::get(&url).call() {
        if let Ok(json) = response.into_json() {
            if let Some(price_obj) = json.get(coin) {
                return price_obj.get("usd")?.as_f64();
            }
        }
    }
    None
}

pub fn query_openai(question: &str, lang: &str) -> String {
    let secrets = fs::read_to_string("config/secrets.json").unwrap_or_default();
    let key: serde_json::Value = serde_json::from_str(&secrets).unwrap_or_default();
    let api_key = key["openai_api_key"].as_str().unwrap_or("");
    if api_key.is_empty() {
        return format!("Dummy GPT Answer ({}): '{}'", lang, question);
    }
    format!("GPT Answer ({}) for '{}'", lang, question)
}
