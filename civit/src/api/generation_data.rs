use crate::Civit;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationData {
    #[serde(rename = "type")]
    media_type: String,
    onSite: bool,
    process: String,
    meta: Value,
    resources: Vec<Resource>,
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