use crate::Civit;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::api::user::Collection;

use reqwest::header::HeaderMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum CollectionType {
    Image,
    Model,
    Post,
    Article
}

impl std::fmt::Display for CollectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CollectionType::Image => "Image",
            CollectionType::Model => "Model",
            CollectionType::Post => "Post",
            CollectionType::Article => "Article"
        };
        write!(f, "{}", s)
    }
}

impl Civit {
    pub async fn create_collection(&self, collection_name: &str, collection_type: CollectionType, nsfw: bool, description: &str) -> Option<Collection> {       
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
        
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
        
        let response = self.client.post(format!("https://civitai.com/api/trpc/collection.upsert"))
            .headers(headers)
            .json(&json!({
              "json": {
                "name": collection_name,
                "description": description,
                "image": null,
                "nsfw": nsfw,
                "read": null,
                "type": collection_type,
                "mode": null,
                "metadata": { "forcedBrowsingLevel": null },
                "authed": true
              },
              "meta": {
                "values": {
                  "description": ["undefined"],
                  "image": ["undefined"],
                  "read": ["undefined"],
                  "metadata.forcedBrowsingLevel": ["undefined"]
                }
              }
            }))
        .send().await.unwrap().json::<Value>().await.unwrap_or_default();
        
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default().get("json").unwrap_or_default();
        
        match serde_json::from_value::<Collection>(res.clone()) {
            Err(e) => {
                dbg!(e);
                None
            },
            Ok(collection) => Some(collection)
        }
    }
}