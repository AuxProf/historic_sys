use crate::entities::chat::model::CreateChat;
use crate::entities::user::model::CreateUser;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

///////////////////////
//////USER
////////////////////////////

#[post("/gpt/user")]
async fn create_user(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do service que cria o user
    //TODO: Chamada do GPT


    HttpResponse::Ok().body("Chat criado")
}

#[get("/gpt/user/{id}")]
async fn get_user(
    app_state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do service que puxa a lista de chats
    //TODO: Chamada do service que puxa a lista de files

    HttpResponse::Ok().body("Chat criado")
}

///////////////////////
//////
////////////////////////////

///////////////////////
//////MESSAGE
////////////////////////////

#[post("/gpt/message/txt")]
async fn send_message(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do GPT


    HttpResponse::Ok().body("Chat criado")
}


#[post("/gpt/message/img")]
async fn send_message_img(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do GPT

    HttpResponse::Ok().body("Chat criado")
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
    user: web::Json<CreateUser>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do GPT
    // cria um file no service


    HttpResponse::Ok().body("Chat criado")
}

#[delete("/gpt/file")]
async fn delete_file(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do GPT
    // cria um file no service

    HttpResponse::Ok().body("Chat criado")
}

#[put("/gpt/file")]
async fn atach_file(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do GPT
    // cria um file no service

    HttpResponse::Ok().body("Chat criado")
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
    chat: web::Json<CreateChat>,
) -> impl Responder {
    let now = chrono::offset::Utc::now();

    //TODO: Chamada do GPT
    let thread_id = "".to_string();

    let result =  sqlx::query(
        "INSERT INTO chats (id, user_id, title, thread_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&chat.user_id)
    .bind(&chat.title)
    .bind(&thread_id)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat criado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao inserir chat"),
    }
}

#[delete("/gpt/chat/{thread_id}")]
async fn delete_chat(app_state: web::Data<AppState>, thread_id: web::Path<Uuid>) -> impl Responder {
    //TODO: Chamada do GPT para remover thread

    let result = sqlx::query("DELETE FROM chats WHERE thread_id = $1 RETURNING *")
        .bind(&thread_id.into_inner())
        .fetch_one(&app_state.postgress_cli)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat apagado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao apagar chat"),
    }
}

#[put("/gpt/chat/{thread_id}/refresh")]
async fn refresh_chat(
    app_state: web::Data<AppState>,
    thread_id: web::Path<Uuid>,
) -> impl Responder {
    //TODO: Chamada do GPT para remover thread e criar nova
    let new_thread_id = "".to_string();

    let result = sqlx::query("UPDATE chats SET thread_id = $2 WHERE thread_id = $1 RETURNING *")
        .bind(&thread_id.into_inner())
        .bind(&new_thread_id)
        .fetch_one(&app_state.postgress_cli)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat apagado"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao apagar chat"),
    }
}

#[get("/gpt/chat/{thread_id}")]
async fn get_chat_hist(
    app_state: web::Data<AppState>,
    thread_id: web::Path<String>,
) -> impl Responder {
    //return messages da api gpt com o thread id

    HttpResponse::Ok().body("Chat apagado")
}

///////////////////////
//////
////////////////////////////

pub fn gpt_routes(cfg: &mut web::ServiceConfig) {
    cfg

    //users
        .service(create_user)
        .service(get_user)
        
    //message
        .service(send_message)
        .service(send_message_img)

    //file
        .service(create_file)
        .service(delete_file)
        .service(atach_file)

    //chats
        .service(create_chat)
        .service(delete_chat)
        .service(refresh_chat)
        .service(get_chat_hist);
}
