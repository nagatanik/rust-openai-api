use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
    created: i64,
    id: String,
    model: String,
    object: String,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
struct Choice {
    finish_reason: String,
    index: i32,
    message: MessageResponse,
}

#[derive(Deserialize, Debug)]
struct MessageResponse {
    content: String,
    role: String,
}

#[derive(Deserialize, Debug)]
struct Usage {
    completion_tokens: i32,
    prompt_tokens: i32,
    total_tokens: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("Please set OPENAI_API_KEY environment variable."),
    };

    let client = Client::new();
    let body = RequestBody {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "Hello, World!".to_string(),
            }
        ],
    };

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    if res.status().is_success() {
        let response_body = res.json::<ApiResponse>().await?;
        println!("{:#?}", response_body);
    } else {
        eprintln!("Error: {:?}", res.status());
    }

    Ok(())
}
