use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonLineFile {
    pub title: String,
    pub path: String,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: String,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>
}

#[derive(Clone, Debug)]
pub struct GptApi {
    pub url: String,
    pub key: String,
    pub assistent: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub thread_id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageImage {
    pub thread_id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitMessage {
    pub role: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageData {
    pub id: String,
    pub role: String,
    pub content: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<MessageContent>,
    pub image_url: Option<ImageContent>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageContent {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageContent {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    pub data: Vec<MessageData>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUrl {
    pub data: Vec<ResponseUrlData>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUrlData {
    pub url: String,
}
