use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
mod database{pub mod postgres;}


#[get("/")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = database::postgres::start_con().await;
    HttpServer::new(|| {
        App::new()
        .service(index)
    }).bind("127.0.0.1:8000")?.run().await
}
