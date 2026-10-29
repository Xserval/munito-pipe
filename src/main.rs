use serde_json::Value;
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

    println!("{}", &response.status());

    let response_text = &response.text().await?;
    let json_value: Value = serde_json::from_str(&response_text)?;
    println!(
        "API answer :{}, cost in token {}",
        json_value["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("ERROR"),
        json_value["usage"]["total_tokens"]
    );
    Ok(())
}
