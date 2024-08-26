use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize)]
pub struct Thread {
    pub id: String,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}


#[derive(Clone)]
pub struct GptApi {
    pub url: String,
    pub key: String,
    pub assistent: String
}

#[derive(Serialize,Deserialize)]
pub struct Message {
    pub thread_id: String,
    pub text: String
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
    pub text: MessageContent,
}

#[derive(Serialize, Deserialize)]
pub struct MessageContent {
    pub value: String,
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
    pub url: String
}

