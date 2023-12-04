use crate::models::general::llm::{Message};
use crate::dotenv::dotenv;
use reqwest::{Client, header};
use std::env;

// send message to llm (large language model)
pub async fn call_gpt(message: Vec<Message>) {
    dotenv().ok();
    // get API key
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found");
    //get 
    let api_org = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found");
    // get url
    let url = "https:://api.openai.com/v1/engines/davinci/completions";

    let mut headers = header::HeaderMap::new();
    // api header
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    // org header
    headers.insert(
        "OpenAI-Organization", 
        header::HeaderValue::from_str(api_org.as_str()).unwrap()
    );
    // create client        
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
}
