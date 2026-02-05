use axum::{
    Json, Router, extract::Query, response::{IntoResponse, Response}, routing::get
};

use serde::{Deserialize};

use civit::{Civit, CollectionType};

pub fn route() -> Router {
    Router::new()
        .route("/create_collection", get(create_collection))
}

#[derive(Deserialize)]
pub struct DataReq {
    token: String,
    collection_name: String,
    collection_type: CollectionType,
    nsfw: bool,
    description: Option<String>
}

/// Gets a list of collections with media item id
pub async fn create_collection(data: Query<DataReq>) -> Response {
    let token = &data.token;
        
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let desc = data.description.clone().unwrap_or_default();
    let tp = data.collection_type.clone();
    
    Json(civit.create_collection(&data.collection_name, tp, data.nsfw, &desc).await).into_response()
}
