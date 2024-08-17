use crate::entities::chat;
use crate::entities::gpt::model::{GptApi, Message};
use crate::entities::user::model::{CreateUser, User};
use crate::entities::{chat::model::CreateChat, user};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

///////////////////////
//////USER
////////////////////////////

#[post("/gpt/user")]
async fn create_user(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let result = user::service::create(app_state, user).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(json!({
            "id": user.id,
            "email": user.email,
        })),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir usuario"),
    }
}

// #[get("/gpt/user/{id}")]
// async fn get_user(app_state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
//     //TODO: Chamada do service que puxa a lista de chats
//     let chats = chat::service::get_chat_list(app_state, id).await;
//     //TODO: Chamada do service que puxa a lista de files

//     HttpResponse::Ok().body("Chat criado")
// }

///////////////////////
//////
////////////////////////////

///////////////////////
//////MESSAGE
////////////////////////////

#[post("/gpt/message/txt")]
async fn send_message(gpt_api: web::Data<GptApi>, message: web::Json<Message>) -> impl Responder {
    gpt_api
        .send_messages_thread(Message {
            thread_id: message.thread_id.clone(),
            text: message.text.clone(),
        })
        .await;

    HttpResponse::Ok().body("Mensagem enviada")
}

// #[post("/gpt/message/img")]
// async fn send_message_img(
//     app_state: web::Data<AppState>,
//     user: web::Json<CreateUser>,
// ) -> impl Responder {
//     let now = chrono::offset::Utc::now();

//     //TODO: Chamada do GPT

//     HttpResponse::Ok().body("Chat criado")
// }

///////////////////////
//////
////////////////////////////

///////////////////////
//////FILE
////////////////////////////

// #[post("/gpt/file")]
// async fn create_file(
//     app_state: web::Data<AppState>,
//     user: web::Json<CreateUser>,
// ) -> impl Responder {
//     let now = chrono::offset::Utc::now();

//     //TODO: Chamada do GPT
//     // cria um file no service

//     HttpResponse::Ok().body("Chat criado")
// }

// #[delete("/gpt/file")]
// async fn delete_file(
//     app_state: web::Data<AppState>,
//     user: web::Json<CreateUser>,
// ) -> impl Responder {
//     let now = chrono::offset::Utc::now();

//     //TODO: Chamada do GPT
//     // cria um file no service

//     HttpResponse::Ok().body("Chat criado")
// }

// #[put("/gpt/file")]
// async fn atach_file(app_state: web::Data<AppState>, user: web::Json<CreateUser>) -> impl Responder {
//     let now = chrono::offset::Utc::now();

//     //TODO: Chamada do GPT
//     // cria um file no service

//     HttpResponse::Ok().body("Chat criado")
// }

///////////////////////
//////
////////////////////////////

///////////////////////
//////CHAT
////////////////////////////

#[post("/gpt/chat")]
async fn create_chat(
    app_state: web::Data<AppState>,
    gpt_api: web::Data<GptApi>,
    chat: web::Json<CreateChat>,
) -> impl Responder {
    let thread_id = gpt_api.create_thread().await;

    let result = chat::service::create(app_state, chat, thread_id).await;

    match result {
        Ok(chat) => HttpResponse::Ok().json(json!({
            "title": chat.title,
            "thread_id": chat.thread_id
        })),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir chat"),
    }
}

#[delete("/gpt/chat/{thread_id}")]
async fn delete_chat(
    app_state: web::Data<AppState>,
    gpt_api: web::Data<GptApi>,
    thread_id: web::Path<Uuid>,
) -> impl Responder {
    let _ = gpt_api.delete_thread(thread_id.to_string()).await;

    let result = chat::service::delete(app_state, thread_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat apagado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao apagar chat"),
    }
}

#[put("/gpt/chat/{thread_id}/refresh")]
async fn refresh_chat(
    app_state: web::Data<AppState>,
    thread_id: web::Path<Uuid>,
    gpt_api: web::Data<GptApi>,
) -> impl Responder {
    let _ = gpt_api.delete_thread(thread_id.to_string()).await;
    let new_thread_id = gpt_api.create_thread().await;
    let result = chat::service::update_thread_id(
        app_state,
        thread_id.into_inner().to_string(),
        new_thread_id,
    )
    .await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Chat apagado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao apagar chat"),
    }
}

#[get("/gpt/chat/{thread_id}")]
async fn get_chat_hist(thread_id: web::Path<String>) -> impl Responder {
    //return messages da api gpt com o thread id

    HttpResponse::Ok().body("Menssagens do Chat")
}

///////////////////////
//////
////////////////////////////

pub fn gpt_routes(cfg: &mut web::ServiceConfig) {
    //users
    // cfg.service(create_user).service(get_user);
    // //message
    // cfg.service(send_message).service(send_message_img);
    // //file
    // cfg.service(create_file)
    //     .service(delete_file)
    //     .service(atach_file);
    //chats
    cfg.service(create_chat)
        .service(delete_chat)
        .service(refresh_chat)
        .service(get_chat_hist);
}
