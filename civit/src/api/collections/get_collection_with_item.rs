use crate::Civit;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use super::CollectionType;
use reqwest::header::HeaderMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseInCollection {
    collectionId: usize,
    addedById: usize,
    tagId: Option<usize>,
    collection: ResponseCollection,
    canRemoveItem: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCollection {
    userId: usize,
    read: String
}

impl Civit {
    pub async fn get_collection_with_item(&self, media_id: usize, media_type: CollectionType) -> Vec<ResponseInCollection> {       
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
                
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
        
        let payload = json!({"json":{"imageId":media_id,"type":media_type,"authed":true}});

        let response = self.client.get(format!("https://civitai.com/api/trpc/collection.getUserCollectionItemsByItem?input={payload}"))
            .headers(headers)
        .send().await.unwrap().json::<Value>().await.unwrap_or_default();
                
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default().get("json").unwrap_or_default().to_owned();
        
        let collections: Vec<ResponseInCollection> = serde_json::from_value(res).unwrap();
        
        collections
    }
}