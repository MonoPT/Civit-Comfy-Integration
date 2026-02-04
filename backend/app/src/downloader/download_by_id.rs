use super::{DownloadJob, DownloadKind};

use axum::{extract::{Path, Query}, response::{IntoResponse, Response}, Extension};
use std::sync::Arc;
use crate::AppState;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ModelOptions {
    format: Option<String>,
    size: Option<String>,
    fp: Option<String>,
    token: String,
    model_type: String,
    cover: String,
    base_model: String,
    model_name: String,
    author_name: String,
    published_at: String,
    based_on_model: String,
    file_name: String,
    stats: String
}

pub async fn download_by_id(Path(model_id): Path<usize>, state: Extension<Arc<AppState>>, options: Query<ModelOptions>) -> Response {
    let mut url_payload = format!("{model_id}?");
    
    if options.format.is_some() {
        url_payload = format!("{url_payload}&format={}", options.format.clone().unwrap())
    }
    
    if options.size.is_some() {
        url_payload = format!("{url_payload}&size={}", options.size.clone().unwrap())
    }
    
    if options.fp.is_some() {
        url_payload = format!("{url_payload}&fp={}", options.fp.clone().unwrap())
    }
    
    if url_payload.ends_with("?") {
        url_payload = url_payload.replace("?", "");
    }
    
    let model_folder = match options.model_type.to_lowercase().as_str() {
        "checkpoint" => "checkpoints",
        "textualinversion" => "embeddings",
        "hypernetwork" => "hypernetworks",
        "aestheticgradient" => "aesthetics",
        "lora" => "loras",
        "controlnet" => "controlnet",
        "poses" => "poses",
        _ => &options.model_type
    }.to_string();
    
    let models_dir = format!("{}/models", state.comfy_path);
    
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    
    let file = File::create("/workspace/models_dir.log").await;
    let mut file = file.unwrap();
    
    let _ = file.write_all(format!("Models dir: {}", models_dir).as_bytes()).await;
    
    let payload = DownloadJob {
        payload: url_payload,
        kind: DownloadKind::ModelId,
        models_dir: models_dir,
        model_type: model_folder,
        user_token: options.token.clone(),
        cover: options.cover.clone(),
        base_model: options.base_model.clone(),
        model_name: options.model_name.clone(),
        author_name: options.author_name.clone(),
        published_at: options.published_at.clone(),
        based_on_model: options.based_on_model.clone(),
        file_name: options.file_name.clone(),
        stats: options.stats.clone()
    };
    
    println!("Downloading model to {}/{}", payload.models_dir, payload.model_type);

    let _ = state.tx_downloader.send(payload).await;
    
    format!("Download Model with id {model_id}").into_response()
}