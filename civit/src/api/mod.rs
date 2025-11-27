mod images;
mod models;
mod tags;
mod download;

use serde::Deserialize;
pub use images::ImagesOptions;
pub use models::ModelsOptions;
pub use tags::TagsOptions;
pub use download::DownloadOptions;

pub struct Civit {
    api_key: String,
    client: reqwest::Client
}

impl Civit {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        
        Civit {
            api_key: "".to_string(),
            client
        }
    }
    
    pub fn update_api_key(mut self, api_key: impl ToString) -> Self {
        self.api_key = api_key.to_string();
        
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