use std::collections::HashSet;
use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get
};

use civit::{TagsResponse, TagsSort, TagsOptions};

use serde::Deserialize;

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/user_data", get(get_user))
}

#[derive(Deserialize)]
pub struct UserDataReq {
    token: String,
}

pub async fn get_user(data: Query<UserDataReq>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let data = civit.user_data().await;
    
    match data {
        None => return (StatusCode::BAD_REQUEST, "Token Invalid").into_response(),
        Some(mut data) => {
            let mut tags: HashSet<TagsResponse> = HashSet::new();
            let mut sort = vec![];
            
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
        
            data.tags = tags;
            
            return (StatusCode::OK, Json(data)).into_response()
        }
    }
}