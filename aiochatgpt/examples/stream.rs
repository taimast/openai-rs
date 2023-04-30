use std::error::Error;
use futures::{StreamExt, TryStreamExt};

use chatgpt::OpenAIRequest;

type DynResult<T> = Result<T, Box<dyn Error>>;



#[tokio::main]
async fn main() -> DynResult<()> {
    const OPENAI_API_KEY: &str = "<OPENAI_API_KEY>";
    let client = chatgpt::OpenAIClient::new(OPENAI_API_KEY);
    let mut request = OpenAIRequest::default();
    request.system_message("You are a human".to_string());
    request.user_message("Hello, how are you? Tell more about you".to_string());
    let mut stream = client.new_map_stream(&request).await.unwrap();
    let mut answer = String::new();
    let mut new_char_count = 0;
    while let Some(chunks) = stream.try_next().await.unwrap() {
        for chunk in chunks {
            if let Some(content) = chunk.get_content() {
                answer.push_str(&content);
                if answer.len() - new_char_count > 100 {
                    new_char_count = answer.len();
                    println!("{}", answer);
                }
            }
        }
    }
    println!("{}", answer);
    Ok(())
}