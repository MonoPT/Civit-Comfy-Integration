use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::CollectionType;
use serde::{Deserialize, Serialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/get_collections", get(get_collections))
        .route("/get_collections_by_media", get(get_collections_by_media_type))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
}

/// Gets a list of collections with media item id
pub async fn get_collections(data: Query<UserDataReq>) -> Response {
    let token = &data.token;
        
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    Json(civit.collections().await).into_response()
}

#[derive(Deserialize)]
pub struct UserDataReq2 {
    token: String,
    media_type: civit::CollectionType
}

/// Gets a list of collections with media item id
pub async fn get_collections_by_media_type(data: Query<UserDataReq2>) -> Response {
    let token = &data.token;
    let media_type = &data.media_type;
        
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
        
    Json(civit.get_collection_by_media_type(media_type).await).into_response()
}