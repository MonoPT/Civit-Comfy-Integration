mod download_by_id;
mod worker;

pub use worker::download_worker;

use axum::{Router, routing::get};

use axum::{Extension, Json};
use std::sync::Arc;
use crate::{AppState, ModelDownloading};

pub fn route() -> Router {
    Router::new()
        .route("/download_by_id/{model_id}", get(download_by_id::download_by_id))
        .route("/downloads", get(list_downloads))
}

#[derive(Debug)]
pub struct DownloadJob {
    payload: String,
    kind: DownloadKind,
    models_dir: String,
    model_type: String,
    user_token: String,
    cover: String,
    base_model: String,
    model_name: String,
    author_name: String,
    published_at: String,
    based_on_model: String,
    file_name: String
}

#[derive(Debug)]
pub enum DownloadKind {
    ModelId
}

pub async fn list_downloads(state: Extension<Arc<AppState>>) -> Json<Vec<ModelDownloading>> { 
    let status = state.downloads.lock().await;
    
    let status = status.to_vec();
    
    Json(status)
}