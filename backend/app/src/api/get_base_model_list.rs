
use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get
};

use serde::Deserialize;

use civit::{Civit, ImagesInfiniteLoadOptions};

pub fn route() -> Router {
    Router::new()
        .route("/base_models_list", get(base_models_list))
}


#[derive(Deserialize, Debug)]
pub struct DataReq {
    token: String,
}

/// Takes advantage of the error message to get a list of valid base models
pub async fn base_models_list(data: Query<DataReq>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
                
    let base_models = vec!["Some fucking this that is not valid".to_string()];
    
    // Options
    let mut options = ImagesInfiniteLoadOptions::default();
    options.cursor = None;
    options.period = String::from("Month");
    options.types = vec![];
    options.sort = String::from("Most Collected");
    options.browsingLevel = 31;
    options.base_models = base_models;
    options.techniques = vec![];
    options.withMeta = false;
    options.remixesOnly = false;
    options.nonRemixesOnly = false;
    options.fromPlatform = false;
    options.tags = vec![];
    
    //
    let data = civit.load_infinite(options).await;
    
    match data {
        Ok(_data) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal server error").into_response()
        },
        Err(err_r) => {
            let error_message = err_r.get("error").unwrap_or_default().get("json").unwrap_or_default().get("message").unwrap_or_default()
                .to_string();
                        
            let mut parts = error_message.split("|").collect::<Vec<&str>>();
            
            parts.remove(0);
            
            let base_models = parts.iter().map(|part| {
                let p = part.replace("\\", "").replace("\"", "").replace("}n]", "");
                p.trim().to_string()
            }).collect::<Vec<String>>();
                        
            return (StatusCode::OK, Json(base_models)).into_response()
        },
        
    }
}
