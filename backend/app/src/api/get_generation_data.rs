use axum::{
    Json, Router, extract::Query, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use serde::{Deserialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/get_generation_data/{media_id}", get(get_gen_data))
}

#[derive(Deserialize)]
pub struct DataReq {
    token: String,
}

/// Gets a list of collections with media item id
pub async fn get_gen_data(data: Query<DataReq>, Path(media_id): Path<u32>) -> Response {
    let token = &data.token;
        
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    Json(civit.generation_data(media_id).await).into_response()
}