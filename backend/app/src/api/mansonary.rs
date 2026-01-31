
use axum::{
    Json, Router, extract::Query, http::StatusCode, response::{IntoResponse, Response}, routing::get
};

use serde::Deserialize;

use civit::{Civit, ImagesInfiniteLoadOptions};

pub fn route() -> Router {
    Router::new()
        .route("/infinite_images", get(infinite_images))
}


#[derive(Deserialize, Debug)]
pub struct InfiniteImagesDataReq {
    token: String,
    period: String,
    sort: String,
    #[serde(rename = "baseModel")]
    base_model: String,
    #[serde(rename = "mediaType")]
    media_type: String,
    cursor: String,
    #[serde(rename = "browsingLevel")]
    browsing_level: usize,
    techniques: String,
    #[serde(rename = "requiringMeta")]
    requiring_meta: bool,
    #[serde(rename = "madeOnsite")]
    made_onsite: bool,
    #[serde(rename = "originalsOnly")]
    originals_only: bool,   
    #[serde(rename = "remixesOnly")]
    remixes_only: bool,
    tags: String
}


pub async fn infinite_images(data: Query<InfiniteImagesDataReq>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
            
    let cursor = match data.cursor.as_str() {
        "null" => None,
        _ => Some(data.cursor.clone())
    };
    
    let medias = data.media_type.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.to_lowercase().to_string()).collect::<Vec<String>>();
    let base_models = data.base_model.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.to_string()).collect::<Vec<String>>();
    let techniques = data.techniques.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let tags = data.tags.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    
    // Options
    let mut options = ImagesInfiniteLoadOptions::default();
    options.cursor = cursor;
    options.period = data.period.clone();
    options.types = medias;
    options.sort = data.sort.clone();
    options.browsingLevel = data.browsing_level;
    options.base_models = base_models;
    options.techniques = techniques;
    options.withMeta = data.requiring_meta;
    options.remixesOnly = data.remixes_only;
    options.nonRemixesOnly = data.originals_only;
    options.fromPlatform = data.made_onsite;
    options.tags = tags;
    
    //
    let data = civit.load_infinite(options).await;
    
    match data {
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal server error").into_response(),
        Some(data) => {
            return (StatusCode::OK, Json(data)).into_response()
        }
    }
}
