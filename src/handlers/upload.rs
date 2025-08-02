use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;
use pdf_extract::extract_text;

#[derive(Serialize)]
struct UploadResponse {
    text: String,
}

pub async fn upload_pdf_handler(body: Bytes) -> impl IntoResponse {
    // Create a temp file in OS temp dir
    let mut filepath = std::env::temp_dir();
    filepath.push(format!("{}.pdf", Uuid::new_v4()));

    // Write incoming bytes to that file
    if let Err(e) = std::fs::write(&filepath, &body) {
        let msg = format!("failed to write temp file: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
    }

    // Extract text from PDF
    let text = match extract_text(&filepath) {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("failed to extract text: {}", e);
            // cleanup
            let _ = std::fs::remove_file(&filepath);
            return (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
        }
    };

    // cleanup
    let _ = std::fs::remove_file(&filepath);

    let resp = UploadResponse { text };
    (StatusCode::OK, Json(resp)).into_response()
}
