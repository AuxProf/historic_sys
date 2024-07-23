
use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::{ContentMessage, Message, CreateMessage};
use crate::AppState;
use sqlx::Row;
use uuid::Uuid;
use chrono;


#[get("/message/{chat_id}")]
async fn get_chat(app_state: web::Data<AppState>, chat_id: Uuid) -> impl Responder {
    let result = sqlx::query("SELECT role, content FROM messages WHERE chat_id = $1 ORDER BY created_at ASC")
    .bind(&id)
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(message) => HttpResponse::Ok().json(
            message
            .iter()
            .map(|mess| ContentMessage {
                content: mess.get("content")
            })
            .collect::<Vec<ContentMessage>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[post("/message/{chat_id}")]
async fn create(app_state: web::Data<AppState>, message: web::Json<CreateMessage>, chat_id: Uuid) -> impl Responder {
    let now = chrono::offset::Utc::now();
    let result =  sqlx::query(
        "INSERT INTO users (id, chat_id, role, content, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&chat_id)
    .bind(&message.role)
    .bind(&message.content)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    
    match result {
        Ok(_) => HttpResponse::Ok().body("Message insert"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir message")
    }
}

pub fn message_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chat)
    .service(create);
}