
use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::{ListFile, CreateFile};
use crate::{entities::file::model::ShowFile, AppState};
use sqlx::Row;
use uuid::Uuid;


#[get("/chat_file/{chat_id}")]
async fn get_chat_file_list(app_state: web::Data<AppState>, chat_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query("
        SELECT 
            f.id,
            f.name 
        FROM files f
        JOIN chat_file cf ON cf.file_id = f.id
        WHERE cf.chat_id = $1 
        ORDER BY f.created_at ASC
    ")
    .bind(&chat_id.into_inner())
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(file) => HttpResponse::Ok().json(
            file
            .iter()
            .map(|fl| ListFile {
                id: fl.get("id"),
                name: fl.get("name")
            })
            .collect::<Vec<ListFile>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[get("/files/{user_id}")]
async fn get_file_list(app_state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query("SELECT id, name FROM files WHERE user_id = $1 ORDER BY created_at ASC")
    .bind(&user_id.into_inner())
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(file) => HttpResponse::Ok().json(
            file
            .iter()
            .map(|fl| ListFile {
                id: fl.get("id"),
                name: fl.get("name")
            })
            .collect::<Vec<ListFile>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[get("/file/{id}")]
async fn get_file(app_state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query("SELECT * FROM files WHERE id = $1")
    .bind(&id.into_inner())
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(file) => HttpResponse::Ok().json(
            file
            .iter()
            .map(|fl| ShowFile { 
                id: fl.get("id"),
                name: fl.get("name"), 
                file_id: fl.get("file_id")
            })
            .collect::<Vec<ShowFile>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[post("/file/{user_id}")]
async fn create(app_state: web::Data<AppState>, file: web::Json<CreateFile>, user_id: web::Path<Uuid>) -> impl Responder {
    let now = chrono::offset::Utc::now();
    let result: Result<sqlx::postgres::PgRow, sqlx::Error> =  sqlx::query(
        "INSERT INTO files (id, user_id, name, file_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&user_id.into_inner())
    .bind(&file.name)
    .bind(&file.file_id)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    
    match result {
        Ok(_) => HttpResponse::Ok().body("file insert"),
        Err(err) => HttpResponse::InternalServerError().body("Erro ao inserir file: ".to_owned() + &err.to_string())
    }
}

pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file)
    .service(get_file_list)
    .service(get_chat_file_list)
    .service(create);
}