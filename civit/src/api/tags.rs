use super::{ParametersFromOptions, Civit, CivitResponse};
use crate::filters::{NSFW, Sort, Period};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct TagsOptions {
    pub limit: Option<usize>,
    pub page: Option<usize>,
    pub query: String,
}

impl TagsOptions {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }
    
    pub fn query(mut self, query: impl ToString) -> Self {
        self.query = query.to_string();
        self
    }
}

impl ParametersFromOptions for TagsOptions {
    fn to_parameters(&self) -> String {
        use std::collections::HashMap;
        
        let mut params: HashMap<String, String> = HashMap::new();
        
        match self.limit {
            None => (),
            Some(v) => {
                params.insert("limit".to_string(), v.to_string());
            }
        }
        
        match self.page{
            None => (),
            Some(v) => {
                params.insert("page".to_string(), v.to_string());
            }
        }
        
        params.insert("query".to_string(), self.query.to_string());
        
        
        let p = params.iter()
            .map(|p| format!("{}={}", p.0.trim(), p.1.trim())).collect::<Vec<String>>()
            .join("&");
        
        p
    }
}

#[derive(Deserialize, Debug)]
pub struct Tags {
    name: String,
    link: String
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub totalItems: usize,
    pub currentPage: usize,
    pub pageSize: usize,
    pub totalPages: usize
}

#[derive(Deserialize, Debug)]
pub struct TagsResponse {
    pub items: Vec<Tags>,
    pub metadata: Metadata
}

impl Civit {    
    pub async fn models_by_tag(&self, options: TagsOptions) -> TagsResponse {
        let client = &self.client;
        
        let parameters =  options.to_parameters();
           
        let response = client
                .get(format!("https://civitai.com/api/v1/tags?{parameters}"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
                .send()
                .await.unwrap().json::<TagsResponse>().await.unwrap();
        
        response
    }
}