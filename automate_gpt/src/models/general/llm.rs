use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize )]

pub struct Message {
    pub role: String,
    pub content: String,
    pub receiver: String,
    pub timestamp: String,
}