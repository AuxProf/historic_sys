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
