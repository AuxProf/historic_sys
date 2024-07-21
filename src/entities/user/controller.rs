
use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::{User, CreateUser};
use crate::AppState;
use sqlx::Row;
use uuid::Uuid;


#[get("/user")]
async fn get_all(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query("SELECT * FROM users")
    .fetch_all(&app_state.postgress_cli)
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(
            users
            .iter()
            .map(|user| User {
                id: user.get("id"),
                email: user.get("email")
            })
            .collect::<Vec<User>>()
        ),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao puxar todos os usuarios")
    }
}

#[post("/user")]
async fn create(app_state: web::Data<AppState>, user: web::Json<CreateUser>) -> impl Responder {
    let result =  sqlx::query(
        "INSERT INTO users (id, email) VALUES ($1, $2) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&user.email)
    .fetch_one(&app_state.postgress_cli)
    .await;

    
    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario inserido"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir usuario")
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
    .service(create);
}