use civit::{TagsOptions, TagsSort, TagsResponse, Civit};
use std::collections::HashSet;

use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TagsRequest {
    token: String
}

pub fn route() -> Router {
    Router::new()
        .route("/tags_with_id", get(get_tags_list))
}

pub async fn get_tags_list(data: Query<TagsRequest>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    
    let mut sort = vec![];
    let mut tags: HashSet<TagsResponse> = HashSet::new();
    
    sort.push(TagsSort::MostArticles);
    sort.push(TagsSort::MostHidden);
    sort.push(TagsSort::MostImages);
    sort.push(TagsSort::MostModels);
    sort.push(TagsSort::MostPosts);
    
    for sort in sort {
        match civit.tags(TagsOptions::default().sort(sort)).await {
            None => (),
            Some(batch) => {
                for tag in batch {
                    tags.insert(tag);
                }
            },
        }
    }
    
    return (StatusCode::OK, Json(tags)).into_response()
}