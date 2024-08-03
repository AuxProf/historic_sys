
use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::{TitleChat, CreateChat, GPTInfo};
use crate::AppState;
use sqlx::Row;
use uuid::Uuid;
use chrono;


#[get("/chat/{user_id}")]
async fn get_chat_list(app_state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query("SELECT id, title FROM chats WHERE user_id = $1 ORDER BY created_at ASC")
    .bind(&user_id.into_inner())
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(chats) => HttpResponse::Ok().json(
            chats
            .iter()
            .map(|chat| TitleChat {
                id: chat.get("id"),
                title: chat.get("title")
            })
            .collect::<Vec<TitleChat>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os chats")
    }
}

#[get("/chat/{id}")]
async fn get_gpt_info(app_state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query("SELECT assistent_id, thread_id FROM chats WHERE id = $1 ORDER BY created_at ASC")
    .bind(&id.into_inner())
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(chats) => HttpResponse::Ok().json(
            chats
            .iter()
            .map(|chat| GPTInfo {
                assistent_id: chat.get("assistent_id"),
                thread_id: chat.get("thread_id")
            })
            .collect::<Vec<GPTInfo>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os chats")
    }
}

#[post("/chat/{user_id}")]
async fn create(app_state: web::Data<AppState>, message: web::Json<CreateChat>, user_id: web::Path<Uuid>) -> impl Responder {
    let now = chrono::offset::Utc::now();
    let result =  sqlx::query(
        "INSERT INTO chats (id, user_id, title, assistent_id, thread_id, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&user_id.into_inner())
    .bind(&message.title)
    .bind(&message.assistent_id)
    .bind(&message.thread_id)
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
    .service(get_gpt_info)
    .service(create);
}