
use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get
};

use serde::Deserialize;

use civit::{
    Civit, 
    ModelInfiniteLoadOptions,
    CheckpointType
};

pub fn route() -> Router {
    Router::new()
        .route("/models", get(load_models_inf))
}


#[derive(Deserialize, Debug)]
pub struct ModelInfiniteLoadDataReq {
    token: String,
    cursor: Option<String>,
    //
    period: String,
    sort: String,
    #[serde(rename="browsingLevel")]
    browsing_level: usize,
    #[serde(rename="baseModel")]
    base_models: String,
    #[serde(rename="types")]
    model_types: String,
    #[serde(rename="earlyAccess")]
    early_access: bool,
    #[serde(rename="supportsGeneration")]
    supports_generation: bool,
    #[serde(rename="fromPlatform")]
    from_platform: bool,
    #[serde(rename="isFeatured")]
    is_featured: bool,
    #[serde(rename="checkpointType")]
    checkpoint_type: String,
    #[serde(rename="fileFormat")]
    file_formats: String,
}


pub async fn load_models_inf(data: Query<ModelInfiniteLoadDataReq>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .set_auth_token(token);
            
    let cursor = match &data.cursor {
        None => None,
        Some(cursor) => match cursor.as_str() {
            "null" => None,
            _ => Some(cursor.to_owned())
        }
    };
    
    let base_models = data.base_models.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.trim().to_string()).collect::<Vec<String>>();
    let model_types = data.model_types.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.trim().to_string()).collect::<Vec<String>>();
    let file_formats = data.file_formats.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.trim().to_string()).collect::<Vec<String>>();

        
        
    let checkpoint_type = match data.checkpoint_type.as_str() {
        "Trained" => Some(CheckpointType::Trained),
        "Merge" => Some(CheckpointType::Merge),
        _ => None
    };
    
    // Options
    let mut options = ModelInfiniteLoadOptions::default();
    options.cursor = cursor;
    options.period = data.period.clone();
    options.sort = data.sort.clone();
    options.browsing_level = data.browsing_level.clone();
    options.base_models = base_models;
    options.model_types = model_types;
    options.supports_generation = data.supports_generation;
    options.early_access = data.early_access;
    options.from_platform = data.from_platform;
    options.is_featured = data.is_featured;
    options.checkpoint_type = checkpoint_type;
    options.file_formats = file_formats;
    
    //
    let data = civit.load_model_infinite(options).await;
    
    match data {
        Err(err_val) => {
            dbg!(&err_val);
            return (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal server error").into_response()
        },
        Ok(data) => {
            return (StatusCode::OK, Json(data)).into_response()
        }
    }
}
