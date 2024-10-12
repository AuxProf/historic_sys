use actix_web::Error;
use dotext_ed8fc7b::{Docx, MsDoc, Pptx, Xlsx};
use std::{fs, io::Read};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use crate::entities::gpt::model::JsonLineFile;

pub(crate) async fn save_file(mut payload: Multipart) -> Result<JsonLineFile, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let filename = if let Some(filename) = content_disposition.get_filename() { filename.to_string() } 
        else { "default.txt".to_string() };
        
        let dir_path = "./uploads/";
        let hash = uuid::Uuid::new_v4().to_string();
        let filepath = format!("{}/{}_{}", dir_path, sanitize_filename::sanitize(&filename), hash);
        if !Path::new(dir_path).exists() { fs::create_dir_all(dir_path)?; }
        let mut f = File::create(&filepath).await?;
        while let Some(chunk) = field.try_next().await? { f.write_all(&chunk).await?; }
        match extract_file_contents(&filepath) {
            Ok(contents) => return Ok(JsonLineFile {
                title: filename,
                path: filepath,
                content: contents
            }),
            Err(e) => return Err(e),
        }
    }
    Err(actix_web::error::ErrorBadRequest("No file found"))
}

fn extract_file_contents(filepath: &str) -> Result<String, Error> {
    let extension = std::path::Path::new(filepath)
        .extension()
        .and_then(std::ffi::OsStr::to_str);
    let contents = match extension {
        Some("txt") => extract_text_file(filepath),
        Some("pdf") => extract_pdf_file(filepath),
        Some("xlsx") => extract_xlsx_file(filepath),
        Some("docx") => extract_docx_file(filepath),
        Some("pptx") => extract_pptx_file(filepath),
        Some("xls") => extract_xls_file(filepath),
        Some("csv") => extract_csv_file(filepath),
        Some("doc") => extract_doc_file(filepath),
        _ => Err(actix_web::error::ErrorBadRequest("Unsupported file type")),
    }?;
    Ok(contents)
}

fn extract_text_file(filepath: &str) -> Result<String, Error> {
    let contents = fs::read_to_string(&filepath)?;
    Ok(contents)
}

fn extract_pdf_file(filepath: &str) -> Result<String, Error> {
    let bytes = fs::read(filepath).unwrap();
    let contents = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    Ok(contents)
}

fn extract_xlsx_file(filepath: &str) -> Result<String, Error> {
    let mut file = Xlsx::open(&filepath).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}

fn extract_docx_file(filepath: &str) -> Result<String, Error> {
    let mut file = Docx::open(&filepath).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}

fn extract_pptx_file(filepath: &str) -> Result<String, Error> {
    let mut file = Pptx::open(&filepath).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}

// Futuras inplementações
fn extract_doc_file(filepath: &str) -> Result<String, Error> {
    Ok(filepath.to_string())
}

fn extract_xls_file(filepath: &str) -> Result<String, Error> {
    Ok(filepath.to_string())
}

fn extract_csv_file(filepath: &str) -> Result<String, Error> {
    Ok(filepath.to_string())
}