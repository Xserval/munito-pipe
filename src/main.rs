use serde_json::json;
use std::env;
mod answer_struct;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Declaring the usage of the struct for the JSON answer
    use answer_struct::JsonAnswer;

    //Initiate the env variable and checking it
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

    //Creating the body of the JSON Request to the API through Munito
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

    //Setting up all the check for errors before reading through the answer
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

    //Finally going through the struct of the JsonAnswer to get all the values we need before printing it
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
