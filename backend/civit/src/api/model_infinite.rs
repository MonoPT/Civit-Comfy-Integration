use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Civit;
use reqwest::{header::HeaderMap};

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfiniteLoadOptions {
    pub cursor: Option<String>,
}

impl Default for ModelInfiniteLoadOptions {
    fn default() -> Self {
        ModelInfiniteLoadOptions {
            cursor: None
        }
    }
}

impl Civit {
    pub async fn load_model_infinite(&mut self, options: ModelInfiniteLoadOptions) -> Result<(Vec<ModelResponse>, String), Value>  {                    
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
        
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
                        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
                        
        let mut params = json!({
          "json": {
            "cursor": options.cursor,
            /* 
            "period": options.period,
            "periodMode": "published",
            "sort": options.sort,
            "types": options.types,
            "withMeta": options.withMeta,
            "baseModels": options.base_models,
            "techniques": options.techniques,
            "tools": options.tools,
            "fromPlatform": options.fromPlatform,
            "hideAutoResources": false,
            "hideManualResources": false,
            "notPublished": false,
            "scheduled": false,
            "hidden": false,
            "remixesOnly": options.remixesOnly,
            "nonRemixesOnly": options.nonRemixesOnly,
            "requiringMeta": options.requiringMeta,
            "useIndex": true,
            "browsingLevel": options.browsingLevel, // 31 - include nsfw ; 1 - exclude nsfw
            "include": [],
            "tags" : options.tags,
            "excludedTagIds": [],
            "disablePoi": true,
            "disableMinor": true,
            
            "authed": true*/
          }
        });
           
        //println!("{}\n\n", params);
        
        if options.cursor.is_none() {
            params["meta"] = serde_json::from_str(r#"{ "values": { "cursor": ["undefined"] } }"#).unwrap();
        }
                        
        let response = &self.client.get(format!("https://civitai.com/api/trpc/image.getInfinite?input={}", params.to_string()))
            .headers(headers)
            .send().await.unwrap().text().await.unwrap_or_default();

        
        let response_json: Value = serde_json::from_str(&response).unwrap();
        
        let json_val = response_json.get("result").unwrap_or(&Value::Null).get("data").unwrap_or(&Value::Null).get("json");
        
        if json_val.is_none() {
            return Err(response_json);
        }
        
        let json_val = json_val.unwrap();
        
        
        
        let next_cursor = json_val.get("nextCursor").unwrap_or_default().to_string();
        let items: Vec<ModelResponse> = serde_json::from_value(json_val.get("items").unwrap_or(&Value::Null).clone()).unwrap_or_default();
        
        Ok((items, next_cursor))
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ModelResponse {
    
}

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    id: usize,
    username: Option<String>,
    image: Option<String>,
    deletedAt: Option<String>,
    profilePicture: Option<String>
}

#[derive(Deserialize, Debug, Serialize)]
struct Stats {
    #[serde(rename = "likeCountAllTime")]
    like: usize,
    #[serde(rename = "laughCountAllTime")]
    laugh: usize,
    #[serde(rename = "heartCountAllTime")]
    heart: usize,
    #[serde(rename = "cryCountAllTime")]
    cry: usize,
    #[serde(rename = "commentCountAllTime")]
    comment: usize,
    #[serde(rename = "collectedCountAllTime")]
    collected: usize,
    #[serde(rename = "tippedAmountCountAllTime")]
    tipped_amount_count: usize,
    #[serde(rename = "dislikeCountAllTime")]
    disliked: usize,
    #[serde(rename = "viewCountAllTime")]
    view: usize,
}
