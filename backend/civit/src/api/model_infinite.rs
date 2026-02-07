use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Civit;
use reqwest::{header::HeaderMap};

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum CheckpointType {
    Trained,
    Merge
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfiniteLoadOptions {
    pub cursor: Option<String>,
    pub period: String,
    pub sort: String,
    pub browsing_level: usize,
    pub base_models: Vec<String>,
    pub model_types: Vec<String>,
    pub early_access: bool,
    pub supports_generation: bool,
    pub from_platform: bool,
    pub is_featured: bool,
    pub checkpoint_type: Option<CheckpointType>,
    pub file_formats: Vec<String>
}

impl Default for ModelInfiniteLoadOptions {
    fn default() -> Self {
        ModelInfiniteLoadOptions {
            cursor: None,
            period: "AllTime".to_string(),
            sort: "Most Downloaded".to_string(),
            browsing_level: 31,
            base_models: vec![],
            model_types: vec![],
            early_access: false,
            supports_generation: false,
            from_platform: false,
            is_featured: false,
            checkpoint_type: None,
            file_formats: vec![]
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
            "period": options.period,
            "periodMode": "published",
            "sort": options.sort,
            "followed": false,
            "types": options.model_types,
            "fileFormats": options.file_formats,
            "pending": false,
            "browsingLevel": options.browsing_level,
            "excludedTagIds": [],
            "tags": [],
            "disablePoi": true,
            "disableMinor": true,
            "authed": true,
          }
        });
           
        let json_params = params["json"].as_object_mut().unwrap();
        
        if options.base_models.len() > 0 {
            json_params.insert("baseModels".to_string(), options.base_models.into());
        }
        
        if options.early_access {
            json_params.insert("earlyAccess".to_string(), true.into());
        }
        
        if options.supports_generation {
            json_params.insert("supportsGeneration".to_string(), true.into());
        }
        
        if options.is_featured {
            json_params.insert("isFeatured".to_string(), true.into());
        }
        
        if options.from_platform {
            json_params.insert("fromPlatform".to_string(), true.into());
        }
        
        match options.checkpoint_type {
            None => (),
            Some(ct) => {
                match ct {
                    CheckpointType::Merge => json_params.insert("checkpointType".to_string(), "Merge".into()),
                    CheckpointType::Trained => json_params.insert("checkpointType".to_string(), "Trained".into())
                };
            }
        }
                
        //println!("{}\n\n", params);
        
        if options.cursor.is_none() {
            params["meta"] = serde_json::from_str(r#"{ "values": { "cursor": ["undefined"] } }"#).unwrap();
        }
                        
        let response = &self.client.get(format!("https://civitai.com/api/trpc/model.getAll?input={}", params.to_string()))
            .headers(headers)
            .send().await.unwrap().text().await.unwrap_or_default();

        
        let response_json: Value = serde_json::from_str(&response).unwrap();
        
        let json_val = response_json.get("result").unwrap_or(&Value::Null).get("data").unwrap_or(&Value::Null).get("json");
        
        if json_val.is_none() {
            return Err(response_json);
        }
        
        let json_val = json_val.unwrap();
                      
        let next_cursor = json_val.get("nextCursor").unwrap_or_default().to_string();
        let items: Vec<ModelResponse> = match serde_json::from_value(json_val.get("items").unwrap_or(&Value::Null).clone()) {
            Ok(d) => d,
            Err(e) => {
                dbg!(e);
                vec![]
            }
        };
        
        Ok((items, next_cursor))
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ModelResponse {
    id: usize,
    name: String,
    #[serde(rename = "type")]
    model_type: String,
    minor: bool,
    #[serde(rename = "sfwOnly")]
    sfw_only: bool,
    poi: bool,
    nsfw: bool,
    #[serde(rename = "nsfwLevel", default)]
    nsfw_level: usize,
    status: String,
    #[serde(rename = "createdAt")]
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastVersionAt")]
    last_version_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "publishedAt")]
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    locked: bool,
    #[serde(rename = "earlyAccessDeadline")]
    early_access_deadline: Option<chrono::DateTime<chrono::Utc>>,
    availability: String,
    tags: Vec<usize>,
    hashes: Vec<String>,
    rank: Rank,
    version: Version,
    #[serde(rename = "userId")]
    user_id: i32,
    user: User,
    images: Vec<Image>,
    #[serde(rename = "canGenerate")]
    can_generate: bool
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Image {
    id: usize,
    #[serde(rename = "userId")]
    user_id: i32,
    name: Option<String>,
    url: String,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    width: usize,
    height: usize,
    hash: String,
    #[serde(rename = "type")]
    asset_type: String,
    minor: bool,
    poi: bool,
    #[serde(rename = "modelVersionId")]
    model_version_id: usize,
    availability: String,
    #[serde(rename = "hasMeta")]
    has_meta: bool,
    #[serde(rename = "hasPositivePrompt")]
    has_positive_prompt: bool,
    #[serde(rename = "onSite")]
    on_site: bool,
    #[serde(rename = "remixOfId")]
    remix_of_id: Option<usize>,
    tags: Vec<usize>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    id: i32,
    username: Option<String>,
    image: Option<String>,
    #[serde(rename = "deletedAt")]
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Rank {
    #[serde(rename = "downloadCount")]
    download_count: usize,
    #[serde(rename = "thumbsUpCount")]
    thumbs_up_count: usize,
    #[serde(rename = "thumbsDownCount")]
    thumbs_down_count: usize,
    #[serde(rename = "commentCount")]
    comment_count: usize,
    #[serde(rename = "collectedCount")]
    collected_count: usize,
    #[serde(rename = "tippedAmountCount")]
    tipped_amount_count: usize,
}

#[derive(Deserialize, Debug, Serialize)]
struct Version {
    id: usize,
    index: usize,
    name: String,
    #[serde(rename = "earlyAccessTimeFrame")]
    early_access_time_frame: usize,
    #[serde(rename = "baseModel")]
    base_model: String,
    #[serde(rename = "baseModelType")]
    base_model_type: String,
    #[serde(rename = "createdAt")]
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "publishedAt")]
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    status: String,
    availability: String,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    description: Option<String>,
    #[serde(rename = "trainedWords")]
    trained_words: Vec<String>,
    #[serde(rename = "vaeId")]
    vae_id: Option<usize>,
    covered: bool
}
