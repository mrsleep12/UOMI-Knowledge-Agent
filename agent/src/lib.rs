use serde_json::Value;
use ureq;
use uomi_agent::prelude::*;
use crate::utils::{log, fetch_crypto_price, read_knowledge, add_to_history, get_history, query_openai};

#[agent_entry]
pub fn run(input: Value) -> Value {
    log("UOMI Super Final Knowledge Agent started");

    let question = input["question"].as_str().unwrap_or("No question provided");
    add_to_history(question);

    let mut response = read_knowledge(question);

    if response.is_none() {
        if question.to_lowercase().contains("price") || question.to_lowercase().contains("crypto") {
            if let Some(price) = fetch_crypto_price(question) {
                response = Some(format!("The current price is: ${}", price));
            }
        }
    }

    let final_response = match response {
        Some(ans) => ans,
        None => query_openai(question)
    };

    save_output(json!({
        "response": final_response,
        "history": get_history()
    }));

    json!({
        "response": final_response,
        "history": get_history()
    })
}
