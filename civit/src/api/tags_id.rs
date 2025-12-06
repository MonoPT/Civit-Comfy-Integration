use serde::{Deserialize, Serialize};
use serde_json::json;
use std::hash::{Hash, Hasher};


use crate::Civit;
use reqwest::header::HeaderMap;

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum TagsSort {
    MostModels,
    MostImages,
    MostPosts,
    MostArticles,
    MostHidden
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsOptions {
    limit: usize,
    sort: TagsSort
}

impl Default for TagsOptions {
    fn default() -> Self {
        TagsOptions {
            limit: 200,
            sort: TagsSort::MostImages
        }
    }
}

impl TagsOptions {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    pub fn sort(mut self, sort: TagsSort) -> Self {
        self.sort = sort;
        self
    }
    
    fn sort_to_string(&self) -> &str {
        match self.sort {
            TagsSort::MostArticles => "Most Articles",
            TagsSort::MostHidden => "Most Hidden",
            TagsSort::MostPosts => "Most Posts",
            TagsSort::MostImages => "Most Images",
            TagsSort::MostModels => "Most Models",
        } 
    }
}

impl Civit {
    pub async fn tags(&mut self, options: TagsOptions) -> Option<Vec<TagsResponse>>  {                    
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
        
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
                        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
                
        let params = json!({
          "json": {
              "entityType": ["Image"],
              "sort": options.sort_to_string(),
              "unlisted": false,
              "categories": true,
              "limit": options.limit,
              "page": 1,
              "include": ["nsfwLevel"],
              "authed": true
          }
        });
                        
        let response = &self.client.get(format!("https://civitai.com/api/trpc/tag.getAll?input={}", params.to_string()))
            .headers(headers)
            .send().await.unwrap().text().await.unwrap_or_default();

        
        let response_json: Value = serde_json::from_str(&response).unwrap();
        
        let json_val = response_json.get("result").unwrap_or(&Value::Null).get("data").unwrap_or(&Value::Null).get("json");
        
        if json_val.is_none() {
            dbg!(&response_json);
            return None;
        }
        
        let json_val = json_val.unwrap();
        
                
        let items: Vec<TagsResponse> = serde_json::from_value(json_val.get("items").unwrap_or(&Value::Null).clone()).unwrap_or_default();
    
        Some(items)
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TagsResponse {
    id: usize,
    name: String,
    nsfwLevel: usize
}

impl PartialEq for TagsResponse {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TagsResponse {}

impl Hash for TagsResponse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}