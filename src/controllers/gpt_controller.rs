use crate::entities::file::model::CreateFile;
use crate::entities::gpt::model::{GptApi, Message, MessageImage};
use crate::entities::user::model::CreateUser;
use crate::entities::{chat, file};
use crate::entities::{chat::model::CreateChat, user};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

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

#[get("/gpt/user/{email}")]
async fn get_user(app_state: web::Data<AppState>, email: web::Path<String>) -> impl Responder {
    let op_user = user::service::get_user(app_state.clone(), email).await;

    match op_user {
        Some(user) => {
            let chats = chat::service::get_chat_list(app_state.clone(), user.id.into()).await;
            let files = file::service::get_file_list(app_state.clone(), user.id.into()).await;

            // TODO: Chamada do service que puxa a lista de arquivos (files) pode ser adicionada aqui.

            HttpResponse::Ok().json(json!({
                "id": user.id,
                "chats": chats,
                "files": files
            }))
        }
        None => HttpResponse::InternalServerError().body("Usuário não encontrado"),
    }
}

///////////////////////
//////
////////////////////////////

///////////////////////
//////MESSAGE
////////////////////////////

#[post("/gpt/message/txt")]
async fn send_message(gpt_api: web::Data<GptApi>, message: web::Json<Message>) -> impl Responder {
    let result = gpt_api
        .send_messages_thread(Message {
            thread_id: message.thread_id.clone(),
            text: message.text.clone(),
        })
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Mensagem enviada"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao enviar mensagem"),
    }
}

#[post("/gpt/message/img")]
async fn send_message_img(
    gpt_api: web::Data<GptApi>,
    message: web::Json<Message>,
) -> impl Responder {
    let result = gpt_api.get_message_to_dall_e(message.text.clone()).await;
    match result {
        Some(url) => {
            let _ = gpt_api
                .send_image_hist_thread(message.into_inner(), &url)
                .await;
            HttpResponse::Ok().json(json!({
                "text": &url
            }))
        }
        None => HttpResponse::InternalServerError().body("Erro ao enviar instrução"),
    }
}
#[post("/gpt/message/img/send")]
async fn send_img(gpt_api: web::Data<GptApi>, message: web::Json<MessageImage>) -> impl Responder {
    gpt_api
        .send_img_to_thread(MessageImage {
            thread_id: message.thread_id.clone(),
            url: message.url.clone(),
        })
        .await;

    HttpResponse::Ok().body("Mensagem enviada")
}

///////////////////////
//////
////////////////////////////

///////////////////////
//////FILE
////////////////////////////

#[post("/gpt/file")]
async fn create_file(
    app_state: web::Data<AppState>,
    file: web::Json<CreateFile>,
) -> impl Responder {
    let file = file::service::create(app_state, file).await;
    match file {
        Some(file) => HttpResponse::Ok().json(json!(file)),
        None => HttpResponse::InternalServerError().body("Erro ao enviar Arquivo"),
    }
}

#[delete("/gpt/file/{file_id}")]
async fn delete_file(
    gpt_api: web::Data<GptApi>,
    app_state: web::Data<AppState>,
    file_id: web::Path<String>,
) -> impl Responder {
    let id = file_id.into_inner();
    let _ = gpt_api.delete_file(&id).await;
    let file = file::service::delete(app_state, &id).await;

    match file {
        Ok(_) => HttpResponse::Ok().body("Arquivo deletado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao deletar Arquivo"),
    }
}

#[put("/gpt/file/")]
async fn atach_file(
    app_state: web::Data<AppState>,
    gpt_api: web::Data<GptApi>,
    file_chat: web::Json<file::model::FileChat>,
) -> impl Responder {
    let t_id = file_chat.thread_id.clone();
    let result = file::service::attach_file(
        app_state,
        &t_id,
        file_chat.files.clone(),
    )
    .await;
    let _ = gpt_api.update_file_attachments(&t_id, result).await;

    HttpResponse::Ok().body("Arquivos conectados")
}

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
    thread_id: web::Path<String>,
) -> impl Responder {
    let thread_id = thread_id.into_inner();

    let result = chat::service::delete(app_state, &thread_id).await;
    let _ = gpt_api.delete_thread(&thread_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat apagado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao apagar chat"),
    }
}

#[put("/gpt/chat/refresh/{thread_id}")]
async fn refresh_chat(
    app_state: web::Data<AppState>,
    thread_id: web::Path<String>,
    gpt_api: web::Data<GptApi>,
) -> impl Responder {
    let thread_id = thread_id.into_inner();
    let _ = gpt_api.delete_thread(&thread_id).await;
    let new_thread_id = gpt_api.create_thread().await;
    let result = chat::service::update_thread_id(app_state, thread_id, new_thread_id).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Chat apagado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao apagar chat"),
    }
}

#[get("/gpt/chat/{thread_id}")]
async fn get_chat_hist(gpt_api: web::Data<GptApi>, thread_id: web::Path<String>) -> impl Responder {
    let messages = gpt_api.get_messages(thread_id.into_inner(), 80).await;
    HttpResponse::Ok().json(messages)
}

#[get("/gpt/chat/last/{thread_id}")]
async fn get_chat_last(gpt_api: web::Data<GptApi>, thread_id: web::Path<String>) -> impl Responder {
    let messages = gpt_api.get_messages(thread_id.into_inner(), 1).await;
    HttpResponse::Ok().json(messages)
}

///////////////////////
//////
////////////////////////////

///////////////////////
//////GPT
////////////////////////////

#[get("/gpt/key")]
async fn get_key(
    gpt_api: web::Data<GptApi>,
) -> impl Responder {
    HttpResponse::Ok().json(json!({"key":gpt_api.key}))
}

///////////////////////
//////
////////////////////////////
/// 
pub fn gpt_routes(cfg: &mut web::ServiceConfig) {
    //users
    cfg.service(create_user).service(get_user);
    //message
    cfg.service(send_message)
        .service(send_message_img)
        .service(send_img);
    // //file
    cfg.service(create_file)
        .service(delete_file)
        .service(atach_file);
    //chats
    cfg.service(create_chat)
        .service(delete_chat)
        .service(refresh_chat)
        .service(get_chat_hist)
        .service(get_chat_last);
    //gpt
    cfg.service(get_key);
}
