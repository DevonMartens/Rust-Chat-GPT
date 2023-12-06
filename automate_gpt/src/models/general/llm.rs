use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize )]

pub struct Message {
    pub role: String,
    pub content: String,
    pub receiver: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize )]

pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Deserialize, Debug )]

pub struct APIMessage {
    pub content: String,
}


#[derive(Deserialize, Debug )]

pub struct APIChoice {
    pub message: APIMessage,
}


#[derive(Deserialize, Debug )]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}