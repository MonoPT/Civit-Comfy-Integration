mod images;
mod models;

use serde::Deserialize;
pub use images::ImagesOptions;
pub use models::ModelsOptions;

pub struct Civit {
    api_key: String,
    client: reqwest::Client
}

impl Civit {
    pub fn new(api_key: impl ToString) -> Self {
        let client = reqwest::Client::new();
        
        Civit {
            api_key: api_key.to_string(),
            client
        }
    }
}

#[derive(Deserialize, Debug)]
struct CivitResponseMetadata {
    #[serde(rename = "nextCursor")]
    next_cursor: String,
    #[serde(rename = "nextPage")]
    next_page: String
}

#[derive(Deserialize, Debug)]
pub struct CivitResponse<T> {
    metadata: CivitResponseMetadata,
    items: Vec<T>
}

trait ParametersFromOptions {
    fn to_parameters(&self) -> String;
}