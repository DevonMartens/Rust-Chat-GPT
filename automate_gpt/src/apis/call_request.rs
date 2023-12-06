use crate::models::general::llm::{Message, ChatCompletion};
use dotenv::dotenv;
use reqwest::{Client, header, Request, RequestBuilder, Response};
use std::env;

pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found");
    let api_org = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found");
    let url = "https://api.openai.com/v1/engines/davinci/completions";

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    headers.insert("OpenAI-Organization", header::HeaderValue::from_str(&api_org.as_str()).unwrap());

    let client = Client::new();

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    // Constructing the request using RequestBuilder
    let request: RequestBuilder = client
        .post(url)
        .headers(headers)
        .json(&chat_completion);

    // Sending the request and getting the response
    let res_raw: Response = request
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}
