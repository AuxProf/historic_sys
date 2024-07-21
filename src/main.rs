mod database {
    pub mod postgres;
}
mod traits {
    pub mod crud;
}
mod entities;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::index;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    postgress_cli: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = database::postgres::start_con().await;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                postgress_cli: _pool.clone(),
            }))
            .configure(entities::user::controller::user_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
