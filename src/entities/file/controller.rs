
use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::{ListFile, CreateFile, File};
use crate::AppState;
use sqlx::Row;
use uuid::Uuid;
use chrono;


#[get("/file/{user_id}")]
async fn get_file_list(app_state: web::Data<AppState>, user_id: Uuid) -> impl Responder {
    let result = sqlx::query("SELECT role, content FROM files WHERE user_id = $1 ORDER BY created_at ASC")
    .bind(id)
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(file) => HttpResponse::Ok().json(
            file
            .iter()
            .map(|fl| Contentfile {
                content: fl.get("content")
            })
            .collect::<Vec<Contentfile>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[get("/file/{user_id}")]
async fn get_file(app_state: web::Data<AppState>, user_id: Uuid) -> impl Responder {
    let result = sqlx::query("SELECT role, content FROM files WHERE $1 ORDER BY created_at ASC")
    .bind(id)
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(file) => HttpResponse::Ok().json(
            file
            .iter()
            .map(|fl| Contentfile {
                content: fl.get("content")
            })
            .collect::<Vec<Contentfile>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[post("/file/{user_id}")]
async fn create(app_state: web::Data<AppState>, file: web::Json<Createfile>, user_id: Uuid) -> impl Responder {
    let now = chrono::offset::Utc::now();
    let result =  sqlx::query(
        "INSERT INTO users (id, user_id, role, content, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&user_id)
    .bind(&file.role)
    .bind(&file.content)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    
    match result {
        Ok(_) => HttpResponse::Ok().body("file insert"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir file")
    }
}

pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chat)
    .service(create);
}