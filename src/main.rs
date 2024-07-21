mod database{pub mod postgres;}
mod traits{pub mod crud;}
mod entities;
mod routes;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use routes::index;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = database::postgres::start_con().await;
    HttpServer::new(|| {
        App::new()
        .service(index)
    }).bind("127.0.0.1:8000")?.run().await
}
