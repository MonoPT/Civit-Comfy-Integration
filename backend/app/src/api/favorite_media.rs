use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::CollectionType;
use serde::{Deserialize, Serialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/favorite_media/{media_id}", get(favorite_media))
        .route("/unfavorite_media/{media_id}", get(unfavorite_media))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
}

pub async fn favorite_media(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
    
    let favorite_collection_name = "comfyui_civit_favorites";
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let collections = civit.collections().await;
    
    
    let collection = match collections.iter().find(|coll| coll.name == favorite_collection_name) {
        Some(coll) => coll.clone(),
        None => {
            let collection = civit.create_collection(favorite_collection_name, CollectionType::Image, true, "My favorite media from ComfyUI - Civit explorer").await;
            
            match collection {
                Some(c) => c,
                None => return (StatusCode::INTERNAL_SERVER_ERROR, "Could not create collection").into_response(),
            }
        },
    };
    
    let res = civit.collection_save_item_by_id(&vec![collection], &vec![], media_id, CollectionType::Image).await;
    (res).into_response()
}

pub async fn unfavorite_media(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
    
    let favorite_collection_name = "comfyui_civit_favorites";
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let collections = civit.collections().await;
    
    
    let collection = match collections.iter().find(|coll| coll.name == favorite_collection_name) {
        Some(coll) => coll.clone(),
        None => {
            let collection = civit.create_collection(favorite_collection_name, CollectionType::Image, true, "My favorite media from ComfyUI - Civit explorer").await;
            
            match collection {
                Some(c) => c,
                None => return (StatusCode::INTERNAL_SERVER_ERROR, "Could not create collection").into_response(),
            }
        },
    };
    
    let res = civit.collection_save_item_by_id(&vec![], &vec![collection], media_id, CollectionType::Image).await;
    (res).into_response()
}