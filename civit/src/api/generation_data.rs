use crate::Civit;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationData {
    #[serde(rename = "type")]
    media_type: Option<String>,
    onSite: bool,
    process: Option<String>,
    meta: Value,
    resources: Vec<Resource>,
    tools: Vec<Tool>,
    techniques: Vec<Technique>,
    remixOfId: Option<usize>,
    external: Value,
    canRemix: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Technique { 
    id: usize,
    name: String,
    notes: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tool { 
    id: usize,
    name: String,
    icon: Option<String>,
    domain: Option<String>,
    priority: Option<String>,
    notes: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    imageId: usize,
    modelVersionId: usize,
    strength: Option<f32>,
    modelId: usize,
    modelName: String,
    modelType: String,
    versionId: usize,
    versionName: String,
    baseModel: String
}

impl Civit {
    pub async fn generation_data(&self, media_id: u32) -> Option<GenerationData> {
        let client = &self.client;
        
        let response = client
                .get(format!("https://civitai.com/api/trpc/image.getGenerationData?input={{\"json\":{{\"id\":{media_id},\"authed\":true}}}}"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
                .send()
                .await.unwrap().json::<Value>().await.unwrap_or_default();
        
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default().get("json").unwrap_or_default();
        
        return match serde_json::from_value::<GenerationData>(res.clone()) {
            Err(e) => {
                println!("{e}");
                None
            },
            Ok(data) => Some(data)
        };
    }
}