use super::model::{GitMessage, GptApi, Message, ResponseData, ResponseUrl, Thread};

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
            .delete(format!("{}threads/{}", self.url, id))
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

    pub async fn get_messages(&self, thread_id: String, limit: i8) -> Option<Vec<GitMessage>> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "{}threads/{}/messages?limit={}",
                self.url, thread_id, limit
            ))
            .headers(get_headers(self.key.to_string()))
            .send()
            .await;

        match res {
            Ok(result) => {
                let body = result.text().await.ok()?; // Retorna `None` se falhar ao obter o corpo da resposta
                let message_response: Result<ResponseData, _> = serde_json::from_str(&body);
                match message_response {
                    Ok(response_data) => {
                        let messages: Vec<GitMessage> = response_data
                            .data
                            .into_iter()
                            .map(|message_data| GitMessage {
                                role: message_data.role,
                                text: message_data
                                    .content
                                    .into_iter()
                                    .find(|content| content.content_type == "text")
                                    .map(|content| content.text.value)
                                    .unwrap_or_default(), // Usa um valor padrão caso o texto não seja encontrado
                            })
                            .collect();
                        Some(messages)
                    }
                    Err(_) => None, // Retorna `None` se a desserialização falhar
                }
            }
            Err(_) => None, // Retorna `None` se a requisição falhar
        }
    }

    pub async fn get_message_to_dall_e(&self, text: String) -> Option<String> {
        let mut header = HeaderMap::new();
        header.insert("Content-Type", format!("application/json").parse().unwrap());
        header.insert(AUTHORIZATION, format!("Bearer {}", self.key.to_string()).parse().unwrap());

        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}images/generations", self.url))
            .headers(header)
            .body(format!(
                r#"{{"model": "dall-e-3","prompt": "{0}", "n": 1, "size": "1024x1024"}}"#,
                text
            ))
            .send()
            .await;

        match res {
            Ok(result) => {
                let body = result.text().await.ok()?;
                let message_response: Result<ResponseUrl, _> = serde_json::from_str(&body);
                match message_response {
                    Ok(response_data) => {
                        let first_url = response_data
                            .data
                            .into_iter()
                            .find_map(|message_data| Some(message_data.url)); // Encontra o primeiro URL disponível
                        first_url
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    pub async fn send_file(&self){}
    
    pub async fn send_img_to_thread(&self){}
}
