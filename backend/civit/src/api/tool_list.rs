use crate::Civit;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use reqwest::header::HeaderMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    alias: Option<String>,
    #[serde(rename = "bannerUrl")]
    banner_url: Option<String>,
    company: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    description: Option<String>,
    domain: Option<String>,
    icon: Option<String>,
    id: usize,
    name: String,
    priority: Option<usize>,
    #[serde(default)]
    supported: bool,
    #[serde(rename = "type")]
    tool_type: String
}

impl Civit {
    pub async fn get_tools_list(&self) -> Vec<Tool> {       
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
                
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
        
        let payload = json!({"json":{"include":["unlisted"],"sort":"AZ","cursor":null,"authed":true},"meta":{"values":{"cursor":["undefined"]}}});

        let response = self.client.get(format!("https://civitai.com/api/trpc/tool.getAll?input={payload}"))
            .headers(headers)
            .send().await.unwrap().json::<Value>().await.unwrap_or_default();
                
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default()
            .get("json").unwrap_or_default()
            .get("items").unwrap_or_default()
            .to_owned();
        
        let tools = serde_json::from_value::<Vec<Tool>>(res).unwrap_or_default();
        
        tools
    }
}