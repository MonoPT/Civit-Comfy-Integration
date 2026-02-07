use axum::{
    Router, extract::Query, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::{Collection, CollectionType};
use serde::Deserialize;

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/update_collections/{media_id}", get(update_collections))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
    add: String,
    remove: String,
    collection_type: CollectionType
}

pub async fn update_collections(data: Query<UserDataReq>, Path(media_id): Path<usize>) -> Response {
    let token = &data.token;
        
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let collections = civit.collections().await;
    
    let add = if data.add.len() > 0 {
        data.add.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()
    } else {
        vec![]
    };
    
    let remove = if data.remove.len() > 0 {
        data.remove.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()
    } else {
        vec![]
    };
        
    let positive_collections = add.iter().map(|c| {
        collections.iter().find(|c2| &c2.id == c).unwrap().clone()
    }).collect::<Vec<Collection>>();
    
    let negative_collections = remove.iter().map(|c| {
        collections.iter().find(|c2| &c2.id == c).unwrap().clone()
    }).collect::<Vec<Collection>>();
    
    let coll_type = data.collection_type.clone();
    
    let res = civit.collection_save_item_by_id(&positive_collections, &negative_collections, media_id, coll_type).await;
    
    match res {
        Ok(_) => "ok".into_response(),
        Err(e) => {
            dbg!(e);
            "err".into_response()
        }
    }
}