use serde::{Serialize};

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