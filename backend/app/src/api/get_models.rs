
use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get
};

use serde::Deserialize;

use civit::{Civit, ModelInfiniteLoadOptions};

pub fn route() -> Router {
    Router::new()
        .route("/models", get(load_models_inf))
}


#[derive(Deserialize, Debug)]
pub struct ModelInfiniteLoadDataReq {
    token: String,
    cursor: Option<String>,
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
    
    /*let medias = data.media_type.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.trim().to_lowercase().to_string()).collect::<Vec<String>>();
    let base_models = data.base_model.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.trim().to_string()).collect::<Vec<String>>();
    let techniques = data.techniques.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let tags = data.tags.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let tools = data.tools.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    */
    
    // Options
    let mut options = ModelInfiniteLoadOptions::default();
    options.cursor = cursor;
    
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
