use super::model::{GptApi, Message, Thread};

use reqwest::header::{HeaderMap, AUTHORIZATION};

fn get_headers(token: String) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    header.insert("OpenAI-Beta", "assistants=v2".parse().unwrap());
    header
}

impl GptApi {
    pub async fn create_thread(&self) -> String {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}threads", self.url))
            .headers(get_headers(self.key.to_string()))
            .send()
            .await;

        match res {
            Ok(result) => {
                let body = result.text().await;
                match body {
                    Ok(body_text) => {
                        let thread_response: Result<Thread, _> = serde_json::from_str(&body_text);
                        match thread_response {
                            Ok(thread) => thread.id,
                            Err(_) => "".to_string(), // Caso a desserialização falhe
                        }
                    }
                    Err(_) => "".to_string(), // Caso a leitura do corpo da resposta falhe
                }
            }
            Err(_) => "".to_string(), // Caso a solicitação falhe
        }
    }

    pub async fn delete_thread(&self, id: String) -> Result<reqwest::Response, reqwest::Error> {
        
        let client = reqwest::Client::new();
        let res = client
            .delete(format!("{}threads/{}", self.url,id))
            .headers(get_headers(self.key.to_string()))
            .send()
            .await;

        res
    }

    pub async fn send_messages_thread(&self, message: Message) {
        let client = reqwest::Client::new();
        let _ = client
            .post(format!(
                "{}threads/{}/messages",
                self.url, message.thread_id
            ))
            .headers(get_headers(self.key.to_string()))
            .body(format!(
                r#"{{"role": "user","content": "{0}"}}"#,
                message.text
            ))
            .send()
            .await;

        let _ = client
            .post(format!("{}threads/{}/runs", self.url, message.thread_id))
            .headers(get_headers(self.key.to_string()))
            .body(format!(r#"{{"assistant_id": "{0}"}}"#, self.assistent))
            .send()
            .await;
    }
}
