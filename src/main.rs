use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use std::env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Deserialize)]
    struct Message {
        role: String,
        content: Option<String>,
        reasoning: Option<String>,
    }
    #[derive(Deserialize)]
    struct Choice {
        index: u32,
        message: Message,
        finish_reason: String,
    }
    #[derive(Deserialize)]
    struct Usage {
        prompt_tokens: u32,
        completion_tokens: u32,
        total_tokens: u32,
        prompt_tokens_details: PromptTokensDetails,
    }
    #[derive(Deserialize)]
    struct PromptTokensDetails {
        cached_tokens: u32,
        created_cache_tokens: u32,
    }
    #[derive(Deserialize)]
    struct JsonAnswer {
        id: String,
        object: String,
        created: u64,
        model: String,
        choices: Vec<Choice>,
        usage: Usage,
        service_tier: String,
    }

    dotenvy::dotenv().ok();
    let key = env::var("MUNITO_API_KEY");
    let api_key = match key {
        Ok(val) => {
            println!("Here's the length of the key : {}", val.chars().count());
            val
        }
        Err(e) => {
            eprintln!("Ho no an error {}", e);
            return Err(e.into());
        }
    };
    let body = json!({
      "model": "qwen/qwen3.8-27b",
      "messages": [{"role": "user", "content": "Introduce yourself as a small API Client that is being built to learn rust and using Munito AI"}],
      "max_tokens": 1024
    });
    let client = reqwest::Client::new();
    let request_to_api = client
        .post("https://api.munito.ai/inference/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&body)
        .send()
        .await;
    let response = match request_to_api {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Ho no the request failed because of {}", e);
            return Err(e.into());
        }
    };
    let response_status = response.status();
    let response_body = response.text().await?;
    if response_status.is_success() {
        println!("The request is valid ! The status is {}", response_status);
    } else {
        let message_fail = format!(
            "request failed with {} : {}",
            response_status, response_body
        );
        return Err(message_fail.into());
    }
    let json_answer: JsonAnswer = serde_json::from_str(&response_body)?;
    let choice_in_json = json_answer
        .choices
        .first()
        .ok_or("The request couldn't be checked")?;
    let content_of_json = choice_in_json
        .message
        .content
        .as_ref()
        .ok_or("The request couldn't be checked")?;
    let amount_of_token_consumed = json_answer.usage.total_tokens;
    println!(
        "API answer :{}, cost in token {}",
        content_of_json, amount_of_token_consumed
    );
    Ok(())
}
