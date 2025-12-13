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