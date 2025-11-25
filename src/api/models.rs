use std::collections::HashMap;

use super::{ParametersFromOptions, Civit, CivitResponse};
use crate::filters::{NSFW, Sort, Period};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use chrono::{DateTime, Utc};


#[derive(Debug, Deserialize)]
enum ModelTypes {
    Checkpoint, 
    TextualInversion, 
    Hypernetwork, 
    AestheticGradient, 
    LORA, 
    Controlnet, 
    Poses
}

#[derive(Debug, Deserialize)]
pub enum AllowComercialUse {
    None,
    Image,
    Rent,
    Sell
}

#[derive(Debug, Default, Deserialize)]
pub struct ModelsOptions {
    pub limit: Option<usize>,
    pub page: Option<usize>,
    pub query: String,
    pub tag: Option<String>,
    pub username: Option<String>,
    pub model_types: Option<ModelTypes>,
    pub sort: Option<Sort>,
    pub period: Option<Period>,
    pub favorites: Option<bool>,
    pub hidden: Option<bool>,
    pub primary_file_only: Option<bool>,
    pub allow_no_credit: Option<bool>,
    pub allow_derivatives: Option<bool>,
    pub allow_different_licenses: Option<bool>,
    pub allow_commercial_use: Option<AllowComercialUse>,
    pub nsfw: Option<bool>,
    pub supports_generation: Option<bool>
}



impl ModelsOptions {
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
    
    pub fn tag(mut self, tag: impl ToString) -> Self {
        self.tag = Some(tag.to_string());
        self
    }
    
    pub fn username(mut self, username: impl ToString) -> Self {
        self.username = Some(username.to_string());
        self
    }
    
    pub fn model_type(mut self, model_type: ModelTypes) -> Self {
        self.model_types = Some(model_type);
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
    
    pub fn favorites(mut self, favorites: bool) -> Self {
        self.favorites = Some(favorites);
        self
    }
    
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }
    
    pub fn primary_file_only(mut self, primary_file_only: bool) -> Self {
        self.primary_file_only = Some(primary_file_only);
        self
    }
    
    pub fn allow_no_credit(mut self, allow_no_credit: bool) -> Self {
        self.allow_no_credit = Some(allow_no_credit);
        self
    }
    
    pub fn allow_derivatives(mut self, allow_derivatives: bool) -> Self {
        self.allow_derivatives = Some(allow_derivatives);
        self
    }
    
    pub fn allow_different_licenses(mut self, allow_different_licenses: bool) -> Self {
        self.allow_different_licenses = Some(allow_different_licenses);
        self
    }
    
    pub fn allow_commercial_use(mut self, allow_commercial_use: AllowComercialUse) -> Self {
        self.allow_commercial_use = Some(allow_commercial_use);
        self
    }
    
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }
    
    pub fn supports_generation(mut self, supports_generation: bool) -> Self {
        self.supports_generation = Some(supports_generation);
        self
    }
}

impl ParametersFromOptions for ModelsOptions {
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
        
        match &self.tag{
            None => (),
            Some(v) => {
                params.insert("tag".to_string(), v.to_string());
            }
        }
        
        match &self.username{
            None => (),
            Some(v) => {
                params.insert("username".to_string(), v.to_string());
            }
        }
        
        match &self.model_types{
            None => (),
            Some(v) => {
                
                let m_type = match v {
                  ModelTypes::AestheticGradient => "AestheticGradient",
                  ModelTypes::Checkpoint => "Checkpoint",
                  ModelTypes::TextualInversion => "TextualInversion",
                  ModelTypes::Hypernetwork => "Hypernetwork",
                  ModelTypes::LORA => "LORA",
                  ModelTypes::Controlnet => "Controlnet",
                  ModelTypes::Poses => "Poses",
                };
                
                params.insert("username".to_string(), m_type.to_string());
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
        
        match &self.favorites{
            None => (),
            Some(v) => {
                params.insert("favorites".to_string(), v.to_string());
            }
        }
        
        match &self.hidden {
            None => (),
            Some(v) => {
                params.insert("hidden".to_string(), v.to_string());
            }
        }
        
        match &self.primary_file_only {
            None => (),
            Some(v) => {
                params.insert("primaryFileOnly".to_string(), v.to_string());
            }
        }
        
        match &self.allow_no_credit {
            None => (),
            Some(v) => {
                params.insert("allowNoCredit".to_string(), v.to_string());
            }
        }
        
        match &self.allow_derivatives {
            None => (),
            Some(v) => {
                params.insert("allowDerivatives".to_string(), v.to_string());
            }
        }
        
        match &self.allow_different_licenses {
            None => (),
            Some(v) => {
                params.insert("allowDifferentLicenses".to_string(), v.to_string());
            }
        }
        
        match &self.allow_commercial_use {
            None => (),
            Some(v) => {
                
                let commercial_use = match v {
                    AllowComercialUse::Image => "Image",
                    AllowComercialUse::None => "None",
                    AllowComercialUse::Rent => "Rent",
                    AllowComercialUse::Sell => "Sell",
                };
                
                params.insert("allowCommercialUse".to_string(), commercial_use.to_string());
            }
        }
        
        match &self.nsfw {
            None => (),
            Some(v) => {
                params.insert("nsfw".to_string(), v.to_string());
            }
        }
        
        match &self.supports_generation {
            None => (),
            Some(v) => {
                params.insert("supportsGeneration".to_string(), v.to_string());
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
    #[serde(rename = "downloadCount")]
    download: usize,
    #[serde(rename = "thumbsUpCount")]
    thumbs_up: usize,
    #[serde(rename = "thumbsDownCount")]
    thumbs_down: usize,
    #[serde(rename = "commentCount")]
    comments: usize,
    #[serde(rename = "tippedAmountCount")]
    tipped_amount: usize
}

#[derive(Deserialize, Debug)]
struct Creator {
    username: String,
    image: String
}

#[derive(Deserialize, Debug)]
struct ModelVersionStats {
    #[serde(rename = "downloadCount")]
    downloads: usize,
    #[serde(rename = "thumbsUpCount")]
    thumbs_up: usize,
    #[serde(rename = "thumbsDownCount")]
    thumbs_down: usize
}

#[derive(Deserialize, Debug)]
struct ModelVersionMetadata {
    format: Option<String>,
    size: Option<String>,
    fp: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ModelVersionFiles {
    id: usize,
    #[serde(rename = "sizeKB")]
    size_kb: f64,
    name: String,
    #[serde(rename = "type")]
    model_type: String,
    #[serde(rename = "pickleScanResult")]
    pickle_scan_result: String,
    #[serde(rename = "pickleScanMessage")]
    pickle_scan_message: String,
    #[serde(rename = "virusScanResult")]
    virus_scan_result: String,
    #[serde(rename = "virusScanMessage")]
    virus_scan_message: Option<String>,
    #[serde(rename = "scannedAt")]
    scanned_at: Option<DateTime<Utc>>,
    metadata: ModelVersionMetadata,
    hashes: HashMap<String, String>,
    #[serde(rename = "downloadUrl")]
    download_url: String,
    primary: Option<bool>
}

#[derive(Deserialize, Debug)]
struct ModelImage {
    id: usize,
    url: String,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    width: usize,
    height: usize,
    hash: String,
    #[serde(rename = "type")]
    image_type: String,
    minor: bool,
    poi: bool,
    hasMeta: bool,
    #[serde(rename = "hasPositivePrompt")]
    has_positive_prompt: bool,
    #[serde(rename = "onSite")]
    on_site: bool,
}

#[derive(Deserialize, Debug)]
struct modelVersion {
    id: usize,
    index: usize,
    name: String,
    baseModel: String,
    #[serde(rename = "baseModelType")]
    base_model_type: String,
    #[serde(rename = "publishedAt")]
    publishedAt: DateTime<Utc>,
    availability: String,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    description: Option<String>,
    #[serde(rename = "trainedWords")]
    trained_words: Vec<String>,
    stats: ModelVersionStats,
    #[serde(rename = "supportsGeneration")]
    supports_generation: bool,
    files: Vec<ModelVersionFiles>,
    images: Vec<ModelImage>,
    #[serde(rename = "downloadUrl")]
    download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ModelsResponse {
    id: usize,
    name: String,
    description: String,
    #[serde(rename = "allowNoCredit")]
    allow_no_credit: bool,
    #[serde(rename = "allowCommercialUse")]
    allow_commercial_use: String,
    #[serde(rename = "allowDerivatives")]
    allow_derivatives: bool,
    #[serde(rename = "allowDifferentLicense")]
    allow_different_license: bool,
    #[serde(rename = "type")]
    model_type: ModelTypes,
    minor: bool,
    #[serde(rename = "sfwOnly")]
    sfw_only: bool,
    poi: bool,
    nsfw: bool,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    availability: String,
    #[serde(rename = "userId")]
    user_id: usize,
    cosmetic: Option<String>,
    #[serde(rename = "supportsGeneration")]
    supports_generation: bool,
    stats: Stats,
    creator: Creator,
    tags: Vec<String>,
    #[serde(rename = "modelVersions")]
    model_versions: Vec<modelVersion>
}

impl Civit {    
    pub async fn models(&self, options: ModelsOptions) -> CivitResponse<ModelsResponse> {
        let client = &self.client;
        
        let parameters =  options.to_parameters();
            
        let response = client
                .get(format!("https://civitai.com/api/v1/models?{parameters}"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
                .send()
                .await.unwrap().json::<CivitResponse<ModelsResponse>>().await.unwrap();
        
        response
    }
}