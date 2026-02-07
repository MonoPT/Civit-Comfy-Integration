use axum::{
    Json, Router, extract::Query, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::CollectionType;
use serde::{Deserialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/collection_with_media/{media_id}", get(collection_with_media))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
    collection_type: CollectionType
}

/// Gets a list of collections with media item id
pub async fn collection_with_media(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
        
    let coll_type = &data.collection_type;
    
    let civit = Civit::new()
        .set_auth_token(token);
    
    Json(civit.get_collection_with_item(media_id, coll_type).await).into_response()
}