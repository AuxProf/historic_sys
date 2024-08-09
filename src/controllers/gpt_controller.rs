
use actix_web::{get, put, delete, post, web, HttpResponse, Responder};
use super::model::{TitleChat, CreateChat, GPTInfo};
use crate::AppState;
use sqlx::Row;
use uuid::Uuid;





#[post("/chat")]
async fn create_chat(
    app_state: web::Data<AppState>,
    chat: web::Json<Value>
) -> impl Responder {
    let now = chrono::offset::Utc::now();


    //TODO: Chamada do GPT
    let thread_id = "".to_string();


    let result =  sqlx::query(
        "INSERT INTO chats (id, user_id, title, thread_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&chat.user_id)
    .bind(&chat.title)
    .bind(&thread_id)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat criado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir chat"),
    } 
}

pub fn gpt_routes(cfg: &mut web::ServiceConfig) {
    cfg.scope("/gpt")
    .service(get_chat_list)
    .service(get_gpt_info)
    .service(create);
}