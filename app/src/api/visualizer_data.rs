use std::collections::HashSet;
use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::{TagsResponse, TagsSort, TagsOptions};

use serde::Deserialize;

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/media_data/{creator_id}/{media_id}", get(get_media_data))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
}

pub async fn get_media_data(data: Query<UserDataReq>, Path((creator_id, media_id)): Path<(u32, u32)>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    //println!("{creator_id} / {media_id}");
    let votable_tags = civit.votable_tags(media_id as usize).await;
    let creator = civit.creator(creator_id).await;
    
    
    
    ("OK").into_response()
}