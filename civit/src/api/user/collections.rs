use crate::Civit;
use serde_json::{Value, json};
use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ModelColl {
    id: usize,
    name: String,
    #[serde(rename = "type")]
    model_type: String,
    nsfw: bool,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    status: String,
    tags: Vec<usize>
}

pub struct ModelsList {
    models: Vec<ModelColl>,
    cursor: String
}

impl Civit {
    pub async fn collection_data(&mut self, collection_id: usize) -> ModelsList {
        let url_params = json!({
          "json": {
            "periodMode": "published",
            "sort": "Highest Rated",
            "collectionId": collection_id,
            "pending": false,
            "browsingLevel": 31,
            "excludedTagIds": [],
            "disablePoi": true,
            "disableMinor": true,
            "cursor": null,
            "authed": true
          },
          "meta": { "values": { "cursor": ["undefined"] } }
        });
        
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
        
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
        
        let response = &self.client.get(format!("https://civitai.com/api/trpc/model.getAll?input={}", url_params.to_string()))
            .headers(headers)
        .send().await.unwrap().text().await.unwrap();
        
        let json_v = serde_json::from_str::<Value>(&response).unwrap();
        
        let items: Vec<ModelColl> = serde_json::from_value(json_v.get("result").unwrap().get("data").unwrap().get("json").unwrap().get("items").unwrap().to_owned()).unwrap();
        let cursor = json_v.get("result").unwrap().get("data").unwrap().get("json").unwrap().get("nextCursor").unwrap().to_string();
        
        ModelsList {
            models: items,
            cursor
        }
    }
}