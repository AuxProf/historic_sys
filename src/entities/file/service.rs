use super::model::{CreateFile, File, FileCheck, FileList};
use crate::AppState;
use actix_web::web;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub async fn create(app_state: web::Data<AppState>, file: CreateFile) -> Option<File> {
    let now = chrono::offset::Utc::now();
    let result: Result<sqlx::postgres::PgRow, sqlx::Error> =  sqlx::query(
        "INSERT INTO files (id, user_id, name, file_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&file.user_id)
    .bind(&file.name)
    .bind(&file.file_id)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    match result {
        Ok(f) => Some(File {
            id: f.get("id"),
            user_id: file.user_id,
            name: file.name.clone(),
            file_id: file.file_id.clone(),
            created_at: now.into(),
        }),
        Err(_) => None,
    }
}

pub async fn delete(
    app_state: web::Data<AppState>,
    file_id: &String,
) -> Result<PgRow, sqlx::Error> {
    sqlx::query("DELETE FROM files WHERE file_id = $1 RETURNING *")
        .bind(&file_id)
        .fetch_one(&app_state.postgress_cli)
        .await
}

pub async fn attach_file(
    app_state: web::Data<AppState>,
    thread_id: &String,
    files: Vec<FileCheck>,
) -> Vec<String> {
    let mut file_ids: Vec<String> = Vec::new();

    for file in files {
        if file.check {
            file_ids.push(file.file_id.clone());
        }
    }

    // Verifica se os arquivos já existem na tabela
    let chat_id: Uuid = sqlx::query("SELECT id FROM chats WHERE thread_id = $1 ")
        .bind(&thread_id)
        .fetch_one(&app_state.postgress_cli)
        .await
        .unwrap()
        .get("id");

    let _ = sqlx::query("DELETE FROM chat_file WHERE chat_id = $1")
        .bind(&chat_id)
        .execute(&app_state.postgress_cli)
        .await;

    let _ = sqlx::query("INSERT INTO chat_file (chat_id, file_id) SELECT $1, unnest($2::text[])")
        .bind(&chat_id)
        .bind(&file_ids)
        .execute(&app_state.postgress_cli)
        .await
        .unwrap();

        file_ids
}

pub async fn get_file_list(app_state: web::Data<AppState>, user_id: Uuid) -> Vec<FileList> {
    // Busca todos os registros que correspondem ao chat_id e estão na lista de file_ids
    let result =
        sqlx::query("SELECT name, file_id FROM files WHERE user_id = $1 ORDER BY created_at ASC")
            .bind(&user_id)
            .fetch_all(&app_state.postgress_cli)
            .await
            .unwrap_or_else(|_| vec![]);
    // Extrair os IDs dos chats como Uuid
    let files: Vec<FileList> = result
        .iter()
        .map(|file| FileList {
            file_id: file.get("file_id"),
            name: file.get("name"),
        })
        .collect();

    files
}
