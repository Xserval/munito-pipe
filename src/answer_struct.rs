use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    pub reasoning: Option<String>,
}
#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}
#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
#[derive(Deserialize)]
pub struct JsonAnswer {
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub service_tier: Option<String>,
}
