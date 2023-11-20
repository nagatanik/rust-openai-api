use futures::stream::StreamExt;
use openai_api_stream_rs::OpenAIStream;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = match env::var("OPEN_AI_KEY") {
        Ok(key) => key,
        Err(_) => panic!("Please set OPEN_AI_KEY environment variable."),
    };

    let openai_stream = OpenAIStream::new(api_key);
    let input = r#"
        {
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": "RustでOpenAIのAPIを使う方法を教えてください。"
                }
            ]
        }
    "#;
    let gpt_stream = openai_stream.gpt_stream(input).await.unwrap();
    let mut gpt_stream = Box::pin(gpt_stream);
    while let Some(response) = gpt_stream.next().await {
        println!("{}", response);
    }
}
