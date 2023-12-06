#![allow(unused_imports)]
use crate::models::general::llm::{Message, ChatCompletion, APIResponse};
use dotenv::dotenv;
use std::env;
use reqwest::{Client, header::{self, InvalidHeaderValue}, Request, RequestBuilder, Response, Error};


// Box for dynamic sized error
// ownership of type implementing send can be called between threads
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn::std::error::Error + Send>> {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found");
    let api_org = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found");
    let url = "https://api.openai.com/v1/engines/davinci/completions";

    let mut headers = header::HeaderMap::new();

    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", api_key))
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?
    );  
    
    headers.insert("OpenAI-Organization", header::HeaderValue::from_str(&api_org.as_str())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?
    );


    let client: Client = Client::builder()
        .default_headers(headers.clone())
        .build()
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send>)?; // ? is used to return error if any

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    // Constructing the request using RequestBuilder
    let request: RequestBuilder = client
        .post(url)
        .headers(headers.clone())
        .json(&chat_completion);

    // Sending the request and getting the response
    let response = request
    .send()
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

    let api_response: APIResponse = response
        .json::<APIResponse>()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

    Ok(api_response.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "testing 1...2...3".to_string(),
            receiver: "dev".to_string(),
            timestamp: "String".to_string(),
        };

        let messages = vec![message];

        // Assuming call_gpt function does not return anything.
        // If it returns a Result, you should handle it here.
        let res = match call_gpt(messages.clone()).await {
            Ok(response) => response,
            Err(e) => {
                // Handle error, e.g., log it or return early
                eprintln!("Error calling GPT: {}", e);
                return;
            }
        };
        let res: Result<String, Box<dyn::std::error::Error + Send>> = call_gpt(messages.clone()).await;
        if let Ok(res_str) = &res {
            dbg!(res_str);
            assert_eq!(res_str, "testing 1...2...3");
        } else {
           assert!(false, "call_gpt returned an error");
        }
    }
}
