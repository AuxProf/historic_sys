mod database {
    pub mod postgres;
}
mod entities;
use actix_web::{dev::Service as _, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
mod middleware;

#[derive(Clone)]
pub struct AppState {
    postgress_cli: Pool<Postgres>,
    jwt: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = database::postgres::start_con().await;

    let jwt_env = std::env::var("JSON_WEB_TOKEN_SECRET").expect("JWT não inserido");
    
    let auth = HttpAuthentication::bearer(validator);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                postgress_cli: _pool.clone(),
                jwt: jwt_env.clone()
            }))
            .configure(entities::client::controller::client_routes)
            .wrap(auth)
            .configure(entities::user::controller::user_routes)
            .configure(entities::message::controller::message_routes)
            .configure(entities::chat::controller::chat_routes)
            .configure(entities::file::controller::file_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
