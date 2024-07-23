
use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::{Chat, TitleChat, CreateChat};
use crate::AppState;
use sqlx::Row;
use uuid::Uuid;
use chrono;


#[get("/chat/{user_id}")]
async fn get_chat_list(app_state: web::Data<AppState>, user_id: Uuid) -> impl Responder {
    let result = sqlx::query("SELECT id, title FROM chats WHERE user_id = $1 ORDER BY created_at ASC")
    .bind(&user_id)
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(chats) => HttpResponse::Ok().json(
            chats
            .iter()
            .map(|chat| ContentMessage {
                id: chat.get("id"),
                title: chat.get("title")
            })
            .collect::<Vec<ContentMessage>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os chats")
    }
}

#[post("/chat/{user_id}")]
async fn create(app_state: web::Data<AppState>, message: web::Json<CreateChat>, user_id: Uuid) -> impl Responder {
    let result =  sqlx::query(
        "INSERT INTO users (id, user_id, title, created_at) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&user_id)
    .bind(&message.title)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    
    match result {
        Ok(_) => HttpResponse::Ok().body("Chat insert"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir message")
    }
}

pub fn chat_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chat_list)
    .service(create);
}