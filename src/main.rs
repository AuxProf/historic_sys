mod database {
    pub mod postgres;
}
mod entities;
mod controllers;

use actix_web::{dev::ServiceRequest, error::Error, web, App, HttpMessage, HttpServer};
use dotenv::dotenv;
use entities::gpt::model::GptApi;
use sqlx::{Pool, Postgres}; 



use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

#[derive(Clone)]
pub struct AppState {
    postgress_cli: Pool<Postgres>,
    jwt: String,
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    
    let jwt_env = std::env::var("JSON_WEB_TOKEN_SECRET").expect("JWT não inserido");


    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_env.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<entities::client::model::JWTClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = database::postgres::start_con().await;
    
    let jwt_env: String = std::env::var("JSON_WEB_TOKEN_SECRET").expect("JWT não inserido");
    let gpt_url: String = std::env::var("GPT_URL").expect("GPT_URL não inserido");
    let gpt_key: String = std::env::var("GPT_KEY").expect("GPT_KEY não inserido");
    let gpt_assistent: String = std::env::var("GPT_ASSISTENT").expect("GPT_KEY não inserido");

    HttpServer::new(move || {
        let bearer_mid = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(web::Data::new(AppState {
                postgress_cli: _pool.clone(),
                jwt: jwt_env.clone(),
            }))
            .app_data(web::Data::new(GptApi {
                url: gpt_url.clone(), 
                key: gpt_key.clone(),
                assistent: gpt_assistent.clone()
            }))
            .configure(entities::client::controller::client_routes)
            .service(
                web::scope("")
                .wrap(bearer_mid)
                .configure(controllers::gpt_controller::gpt_routes)
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
