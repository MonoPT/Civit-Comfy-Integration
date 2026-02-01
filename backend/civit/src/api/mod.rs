mod models;
//mod tags;
mod download;
mod user;
mod image_infinite;
mod tags_id;
mod creator;
mod generation_data;
mod collections;
mod tool_list;
mod technique_list;

use serde::Deserialize;
pub use models::{ModelsOptions, Model};
pub use tags_id::{TagsOptions, TagsSort, TagsResponse, VotableTagsResponse};
pub use download::DownloadOptions;
pub use image_infinite::ImagesInfiniteLoadOptions;
pub use creator::Creator;
pub use generation_data::GenerationData;
pub use collections::CollectionType;

pub use crate::api::user::{UserData, Collection};

pub struct Civit {
    api_key: String,
    client: reqwest::Client,
    auth_token: String,
    user_data: Option<UserData>
}

impl Civit {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        
        Civit {
            api_key: "".to_string(),
            client,
            auth_token: "".to_string(),
            user_data: None
        }
    }
    
    pub fn update_api_key(mut self, api_key: impl ToString) -> Self {
        self.api_key = api_key.to_string();
        
        self
    }
    
    pub fn set_auth_token(mut self, token: impl ToString) -> Self {
        self.auth_token = token.to_string().trim().to_owned();
        
        self
    }
}

#[derive(Deserialize, Debug)]
struct CivitResponseMetadata {
    #[serde(rename = "nextCursor")]
    next_cursor: Option<String>,
    #[serde(rename = "nextPage")]
    next_page: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct CivitResponse<T> {
    pub metadata: CivitResponseMetadata,
    pub items: Vec<T>
}

trait ParametersFromOptions {
    fn to_parameters(&self) -> String;
}