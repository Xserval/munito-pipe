use serde_json::json;
use std::env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let key = env::var("MUNITO_API_KEY");
    let api_key = match key {
        Ok(val) => {
            println!("Here's the length of the key : {}", val.chars().count());
            val
        }
        Err(e) => {
            eprintln!("Ho no an error {}", e);
            String::from("0")
        }
    };
    let body = json!({
      "model": "qwen/qwen3.8-27b",
      "messages": [{"role": "user", "content": "Say hello in one sentence."}],
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

    println!("{}", &response.status());

    let response_text = response.text().await?;
    println!("{}", response_text);
    Ok(())
}
