use super::{ParametersFromOptions, Civit, CivitResponse};
use crate::filters::{NSFW, Sort, Period};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct ImagesOptions {
    pub limit: Option<usize>,
    pub post_id: Option<usize>,
    pub model_id: Option<usize>,
    pub model_version_id: Option<usize>,
    pub username: Option<String>,
    pub nsfw: Option<bool>,
    pub sort: Option<Sort>,
    pub period: Option<Period>,
    pub page: Option<usize>
}

impl ImagesOptions {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn post_id(mut self, id: usize) -> Self {
        self.post_id = Some(id);
        self
    }
    
    pub fn model_id(mut self, id: usize) -> Self {
        self.model_id = Some(id);
        self
    }
    
    pub fn model_version_id(mut self, version_id: usize) -> Self {
        self.model_version_id = Some(version_id);
        self
    }
    
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    
    pub fn nsfw(mut self, nsfw_option: bool) -> Self {
        self.nsfw = Some(nsfw_option);
        self
    }
    
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }
    
    pub fn period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }
    
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }
}

impl ParametersFromOptions for ImagesOptions {
    fn to_parameters(&self) -> String {
        use std::collections::HashMap;
        
        let mut params: HashMap<String, String> = HashMap::new();
        
        match self.limit {
            None => (),
            Some(v) => {
                params.insert("limit".to_string(), v.to_string());
            }
        }
        
        match self.post_id {
            None => (),
            Some(v) => {
                params.insert("postId".to_string(), v.to_string());
            }
        }
        
        match self.model_id {
            None => (),
            Some(v) => {
                params.insert("modelId".to_string(), v.to_string());
            }
        }
        
        match self.model_version_id {
            None => (),
            Some(v) => {
                params.insert("modelVersionId".to_string(), v.to_string());
            }
        }
        
        match &self.username {
            None => (),
            Some(v) => {
                params.insert("username".to_string(), v.clone());
            }
        }
        
        match &self.nsfw {
            None => (),
            Some(v) => {
                
                /*let variant = match v {
                    NSFW::None => "None",
                    NSFW::Soft => "Soft",
                    NSFW::Mature => "Mature",
                    NSFW::X => "X",
                };*/
                
                params.insert("nsfw".to_string(), v.to_string());
            }
        }
        
        match &self.sort {
            None => (),
            Some(v) => {
                
                let variant = match v {
                    Sort::MostComments => "Most Comments",
                    Sort::MostReactions => "Most Reactions",
                    Sort::Newest => "Newest"
                };
                
                params.insert("sort".to_string(), variant.to_string());
            }
        }
        
        match &self.period {
            None => (),
            Some(v) => {
                
                let variant = match v {
                    Period::AllTime => "AllTime",
                    Period::Year => "Year",
                    Period::Month => "Month",
                    Period::Week => "Week",
                    Period::Day => "Day",
                };
                
                params.insert("period".to_string(), variant.to_string());
            }
        }
        
        match self.page {
            None => (),
            Some(v) => {
                params.insert("page".to_string(), v.to_string());
            }
        }
        
        let p = params.iter()
            .map(|p| format!("{}={}", p.0.trim(), p.1.trim())).collect::<Vec<String>>()
            .join("&");
        
        p
    }
}

#[derive(Deserialize, Debug)]
struct Stats {
    #[serde(rename = "cryCount")]
    cry: usize,
    #[serde(rename = "laughCount")]
    laugh: usize,
    #[serde(rename = "likeCount")]
    like: usize,
    #[serde(rename = "dislikeCount")]
    dislike: usize,
    #[serde(rename = "heartCount")]
    heart: usize,
    #[serde(rename = "commentCount")]
    comments: usize,
}

#[derive(Deserialize, Debug)]
pub struct ImageResponse {
    id: usize,
    url: String,
    hash: String,
    width: usize,
    height: usize,
    nsfw: bool,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: NSFW,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    #[serde(rename = "postId")]
    post_id: usize,
    stats: Stats,
    meta: serde_json::Value,
    username: String,
    #[serde(rename = "baseModel")]
    base_model: String,
    #[serde(rename = "modelVersionIds")]
    model_version_ids: Vec<usize>
}

impl Civit {    
    pub async fn images(&self, options: ImagesOptions) -> CivitResponse<ImageResponse> {
        let client = &self.client;
        
        let parameters =  options.to_parameters();
            
        let response = client
                .get(format!("https://civitai.com/api/v1/images?{parameters}"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
                .send()
                .await.unwrap().json::<CivitResponse<ImageResponse>>().await.unwrap();
        
        response
    }
}