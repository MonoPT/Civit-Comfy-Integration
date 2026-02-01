use crate::Civit;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use reqwest::header::HeaderMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Technique {
    id: usize,
    name: String,
    #[serde(rename = "type")]
    tool_type: String
}

impl Civit {
    pub async fn get_techniques_list(&self) -> Vec<Technique> {       
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
                
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
        
        let payload = json!({"json":{"authed":true}});

        let response = self.client.get(format!("https://civitai.com/api/trpc/technique.getAll?input={payload}"))
            .headers(headers)
            .send().await.unwrap().json::<Value>().await.unwrap_or_default();
                
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default()
            .get("json").unwrap_or_default()
            .to_owned();
        
        let tools = serde_json::from_value::<Vec<Technique>>(res).unwrap_or_default();
        
        tools
    }
}