use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use civit::{Creator, VotableTagsResponse, GenerationData};

use serde::{Deserialize, Serialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/media_data/{creator_id}/{media_id}", get(get_media_data))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct MediaDataResponse {
    generation_data: GenerationData,
    creator: Option<Creator>,
    votable_tags: Vec<VotableTagsResponse>
}

pub async fn get_media_data(data: Query<UserDataReq>, Path((creator_id, media_id)): Path<(u32, u32)>) -> Response {
    let token = &data.token;
    
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let generation_data = match civit.generation_data(media_id).await {
        Some(d) => d,
        None => {
            return(StatusCode::BAD_REQUEST, "Could not get generation data").into_response();
        }
    };
    
    let votable_tags = civit.votable_tags(media_id as usize).await;
    let creator = civit.creator(creator_id).await;
    
    
    return (StatusCode::OK, Json(
        MediaDataResponse {
            generation_data: generation_data,
            creator: creator,
            votable_tags: votable_tags
        }
    )).into_response()
}