use crate::Civit;
use reqwest::header::HeaderMap;
use scraper::Html;
use scraper::Selector;
use serde_json::Value;
use serde::Deserialize;
pub mod collections;

#[derive(Debug, Clone)]
pub struct UserData {
    pub default_locale: String,
    pub color_scheme: String,
    pub user_id: usize,
    pub user_name: String,
    pub username: String,
    pub user_email: String,
    pub image: String,
    pub show_nsfw: bool,
    pub created_at: String,
    pub autoplay_gifs: bool,
    pub collections: Vec<Collection>,
    pub country_code: String,
    pub region_code: usize
}

#[derive(Debug, Deserialize, Clone)]
pub struct Collection {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub read: String,
    #[serde(rename = "userId")]
    pub user_id: usize,
    pub write: String,
    #[serde(rename = "userId")]
    pub image_id: Option<usize>,
    #[serde(rename = "isOwner")]
    pub is_owner: bool,
    pub image: Option<String>,
    pub tags: Vec<String>
}

impl Civit {
    /// Returns user settings, metadata and collection list
    pub async fn user_data(&mut self) -> Option<UserData> {
        match &self.user_data { // Caches user data
            Some(user_data) => Some(user_data.clone()),
            None => {
                let client = &self.client;
                            
                let mut cookies = std::collections::HashMap::new();
                cookies.insert("__Secure-civitai-token", &self.auth_token);
                
                let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
                let mut headers = HeaderMap::new();
                headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
                
                let response = client
                        .get(format!("https://civitai.com/collections"))
                        .headers(headers)
                        .send()
                        .await.unwrap();
                
                let html = response.text().await.unwrap();
                let document = Html::parse_document(&html);
        
                let selector = Selector::parse("script#__NEXT_DATA__").unwrap();
                
                
                for element in document.select(&selector) {
                    let json_str = element.text().collect::<String>();
                    let v: Value = serde_json::from_str(&json_str).unwrap();
                            
                    let user_j = v.get("props").unwrap().get("pageProps").unwrap().get("session").unwrap().get("user").unwrap();
                    
                    let collections = v.get("props").unwrap().get("pageProps").unwrap().get("trpcState").unwrap().get("json").unwrap()
                        .get("queries").unwrap().as_array().unwrap();
                    
                    let collections: Vec<Collection> = match collections.first() {
                        None => vec![],
                        Some(coll) => {
                            let v = coll.get("state").unwrap().get("data").unwrap().to_owned();
                            let col: Vec<Collection> = serde_json::from_value(v).unwrap();
                        
                            col
                        }
                    };
                    
                    let user_data = UserData {
                        default_locale: v.get("defaultLocale").unwrap().to_string().replace("\"", ""),
                        color_scheme: v.get("props").unwrap().get("pageProps").unwrap().get("colorScheme").unwrap().to_string().replace("\"", ""),
                        user_id: user_j.get("id").unwrap().to_string().replace("\"", "").parse::<usize>().unwrap(),
                        user_name: user_j.get("name").unwrap().to_string().replace("\"", ""),
                        username: user_j.get("email").unwrap().to_string().replace("\"", ""),
                        user_email: user_j.get("email").unwrap().to_string().replace("\"", ""),
                        image: user_j.get("image").unwrap().to_string().replace("\"", ""),
                        show_nsfw: user_j.get("showNsfw").unwrap().to_string().replace("\"", "").parse::<bool>().unwrap(),
                        created_at: user_j.get("createdAt").unwrap().to_string().replace("\"", ""),
                        autoplay_gifs: user_j.get("autoplayGifs").unwrap().to_string().parse::<bool>().unwrap(),
                        collections: collections,
                        country_code: v.get("props").unwrap().get("pageProps").unwrap().get("region").unwrap().get("countryCode").unwrap().to_string().replace("\"", ""),
                        region_code: v.get("props").unwrap().get("pageProps").unwrap().get("region").unwrap().get("regionCode").unwrap().to_string().trim().to_string().replace("\"", "").parse().unwrap()
                    };
                    
                    self.user_data = Some(user_data.clone());
                    
                    return Some(user_data);
                }
                
                None
            }
        }
    }
    
    pub async fn collections(&mut self) -> Vec<Collection> {
        self.user_data().await.unwrap().collections
    }
}