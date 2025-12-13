use crate::Civit;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::api::user::Collection;
use super::CollectionType;
use reqwest::header::HeaderMap;

#[derive(Debug, Serialize, Deserialize)]
struct CollectionItem {
    collectionId: usize,
    userId: usize,
    read: String
}

impl Civit {
    pub async fn collection_save_item_by_id(&self, add_collection: &Vec<Collection>, remove_collection: &Vec<Collection>, media_id: usize, media_type: CollectionType) -> String {       
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
                
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
        
        let add_collection = add_collection.iter().map(|c| CollectionItem {
            collectionId: c.id,
            userId: c.user_id,
            read: c.read.to_string()
        }).collect::<Vec<CollectionItem>>();
        
        let remove_collection = remove_collection.iter().map(|collection| collection.id).collect::<Vec<usize>>();
        
        let response = self.client.post(format!("https://civitai.com/api/trpc/collection.saveItem"))
            .headers(headers)
            .json(&json!({
              "json": {
                "imageId": media_id,
                "type": media_type,
                "collections": add_collection,
                "removeFromCollectionIds": remove_collection,
                "authed": true
              }
            }))
        .send().await.unwrap().json::<Value>().await.unwrap_or_default();
        
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default().get("json").unwrap_or_default().get("status");
        
        match res {
            Some(v) => v.to_string(),
            None => "Failed".to_string()
        }
    }
}