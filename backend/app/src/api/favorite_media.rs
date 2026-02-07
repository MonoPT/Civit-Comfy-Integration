use axum::{
    Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::CollectionType;
use serde::{Deserialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/favorite_media/{media_id}", get(favorite_media))
        .route("/unfavorite_media/{media_id}", get(unfavorite_media))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
    collection_type: CollectionType
}

const FAVORITE_COLLECTIONS_NAME: &str = "comfyui_civit_favorites";
const FAVORITE_MODELS_NAME: &str = "Liked Models";

pub async fn favorite_media(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
        
    let mut civit = Civit::new()
        .set_auth_token(token);
    
    let collections = civit.collections().await;
    
    let collection = match &data.collection_type {
        CollectionType::Image => find_target_collection(CollectionType::Image, &collections, FAVORITE_COLLECTIONS_NAME, &civit).await,
        CollectionType::Model => find_target_collection(CollectionType::Model, &collections, FAVORITE_MODELS_NAME, &civit).await,
        _ => todo!()
    };
    
    let collection = match collection {
        Ok(c) => c,
        Err(e) => {
            dbg!(&e);
            return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response();
        }
    };
    
    let res = civit.collection_save_item_by_id(&vec![collection], &vec![], media_id, data.collection_type.clone()).await;
    (res).into_response()
}

pub async fn unfavorite_media(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
        
    let mut civit = Civit::new()
        .set_auth_token(token);
    
    let collections = civit.collections().await;
    
    let collection = match &data.collection_type {
        CollectionType::Image => find_target_collection(CollectionType::Image, &collections, FAVORITE_COLLECTIONS_NAME, &civit).await,
        CollectionType::Model => find_target_collection(CollectionType::Model, &collections, FAVORITE_MODELS_NAME, &civit).await,
        _ => todo!()
    };
    
    let collection = match collection {
        Ok(c) => c,
        Err(e) => {
            dbg!(&e);
            return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response();
        }
    };
        
    let res = civit.collection_save_item_by_id(&vec![], &vec![collection], media_id, data.collection_type.clone()).await;
    (res).into_response()
}

use civit::Collection;

async fn find_target_collection(collection_type: CollectionType, collections: &Vec<Collection>, name: &str, civit: &Civit) -> Result<Collection, String> {
    match collection_type {
        CollectionType::Image => {
            match collections.iter().find(|coll| coll.name == name) {
                Some(coll) => Ok(coll.clone()),
                None => {
                    let collection = civit.create_collection(name, CollectionType::Image, true, "My favorite media from ComfyUI - Civit explorer").await;
                    
                    match collection {
                        Some(c) => Ok(c),
                        None => return Err("Could not create collection".to_string()),
                    }
                }
            }
        },
        CollectionType::Model => {
            match collections.iter().find(|coll| coll.name == name) {
                Some(coll) => Ok(coll.clone()),
                None => {
                    let collection = civit.create_collection(name, CollectionType::Model, true, "My favorite models from ComfyUI - Civit explorer").await;
                    
                    match collection {
                        Some(c) => Ok(c),
                        None => Err("Could not create collection".to_string()),
                    }
                }
            }
        }
        _ => todo!()
    }
}