use super::model::{BaseClient, JWTClaims};
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;
use sqlx::Row;
use uuid::Uuid;

#[post("/login")]
async fn login(app_state: web::Data<AppState>, body: web::Json<BaseClient>) -> impl Responder {
    let client = sqlx::query("SELECT * FROM clients WHERE login =  $1")
        .bind(&body.login)
        .fetch_one(&app_state.postgress_cli)
        .await;

    let result = client
        .iter()
        .map(|cli| BaseClient {
            login: cli.get("login"),
            password: cli.get("password"),
        })
        .collect::<Vec<BaseClient>>();

    match client {
        Ok(_) => {
            let password_valid = bcrypt::verify(body.password.clone(), &result[0].password)
                .expect("Credenciais invalidas");

            if password_valid {
                let claims: JWTClaims = JWTClaims {
                    exp: Local::now()
                        .checked_add_signed(Duration::days(365 * 50))
                        .expect("Erro ao calcular o tempo futuro")
                        .timestamp() as usize,
                    login: result[0].login.clone(),
                };

                let token = jsonwebtoken::encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(&app_state.jwt.as_bytes()),
                )
                .unwrap();
                HttpResponse::Ok().json(json!({"token": token}))
            } else {
                HttpResponse::InternalServerError().body("Credenciais invalidas")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Credenciais invalidas"),
    }
}

#[post("/register")]
async fn create(app_state: web::Data<AppState>, cli: web::Json<BaseClient>) -> impl Responder {
    let senha = hash(&cli.password, DEFAULT_COST).unwrap();
    let result =
        sqlx::query("INSERT INTO clients (id, login, password) VALUES ($1, $2, $3) RETURNING *")
            .bind(Uuid::new_v4())
            .bind(&cli.login)
            .bind(&senha)
            .fetch_one(&app_state.postgress_cli)
            .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario inserido"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir usuario"),
    }
}

pub fn client_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(login).service(create);
}
