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
    model_type: String
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
        _ => "other"
    }.to_string();
    
    let payload = DownloadJob {
        payload: url_payload,
        kind: DownloadKind::ModelId,
        models_dir: String::from("C:/Users/USER/Downloads/Nova pasta"),
        model_type: model_folder,
        user_token: options.token.clone()
    };
    

    let _ = state.tx_downloader.send(payload).await;
    
    format!("Download Model with id {model_id}").into_response()
}