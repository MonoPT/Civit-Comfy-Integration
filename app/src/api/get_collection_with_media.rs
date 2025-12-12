use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::CollectionType;
use serde::{Deserialize, Serialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/collection_with_media/{media_id}", get(collection_with_media))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
}

/// Gets a list of collections with media item id
pub async fn collection_with_media(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
        
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    Json(civit.get_collection_with_item(media_id, CollectionType::Image).await).into_response()
}