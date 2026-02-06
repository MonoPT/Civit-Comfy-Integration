use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Civit;
use reqwest::{Response, header::HeaderMap};

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImagesInfiniteLoadOptions {
    pub cursor: Option<String>,
    pub period: String,
    pub base_models: Vec<String>,
    periodMode: String,
    pub techniques: Vec<usize>,
    pub tools: Vec<usize>,
    pub sort: String,
    pub types: Vec<String>,
    pub withMeta: bool,
    pub fromPlatform: bool,
    hideAutoResources: bool,
    hideManualResources: bool,
    notPublished: bool,
    scheduled: bool,
    hidden: bool,
    pub remixesOnly: bool,
    pub nonRemixesOnly: bool,
    pub requiringMeta: bool,
    useIndex: bool,
    pub browsingLevel: usize,
    include: Vec<String>,
    excludedTagIds: Vec<usize>,
    disablePoi: bool,
    disableMinor: bool,
    pub tags: Vec<usize>
}

impl Default for ImagesInfiniteLoadOptions {
    fn default() -> Self {
        ImagesInfiniteLoadOptions {
            period: "Year".to_string(),
            base_models: vec![],
            periodMode: "published".to_string(),
            sort: "Most Reactions".to_string(),
            types: vec![],
            withMeta: false,
            fromPlatform: false,
            hideAutoResources: false,
            hideManualResources: false,
            notPublished: false,
            scheduled: false,
            hidden: false,
            remixesOnly: false,
            nonRemixesOnly: false,
            requiringMeta: false,
            useIndex: true,
            browsingLevel: 1,
            include: vec![],
            excludedTagIds: vec![],
            disablePoi: true,
            disableMinor: false,
            cursor: None,
            techniques: vec![],
            tags: vec![],
            tools: vec![]
        }
    }
}

impl Civit {
    pub async fn load_infinite(&mut self, options: ImagesInfiniteLoadOptions) -> Result<(Vec<ImageResponse>, String), Value>  {                    
        let mut cookies = std::collections::HashMap::new();
        cookies.insert("__Secure-civitai-token", &self.auth_token);
        
        let cookie_header = cookies.iter().map(|c| format!("{}={}", c.0, c.1)).collect::<Vec<String>>().join(";");
                        
        let mut headers = HeaderMap::new();
        headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_header).unwrap());
                        
        let mut params = json!({
          "json": {
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
            "cursor": options.cursor,
            "authed": true
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
        let mut items: Vec<ImageResponse> = serde_json::from_value(json_val.get("items").unwrap_or(&Value::Null).clone()).unwrap_or_default();
        
        items.iter_mut().for_each(|i| {
            i.img_url = Some(format!("https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/{}/transcode=true,original=true,quality=100/3cde953f-b339-426a-97fa-9b47071c1df6", i.url.clone().unwrap_or_default()));
        });
        
        Ok((items, next_cursor))
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ImageResponse {
    img_url: Option<String>,
    id: usize,
    reactionCount: usize,
    commentCount: usize,
    collectedCount: usize,
    index: usize,
    #[serde(rename = "postId")]
    post_id: usize,
    url: Option<String>,
    #[serde(rename = "aiNsfwLevel")]
    ai_nsfw_level: usize,
    hash: Option<String>,
    width: usize,
    height: usize,
    #[serde(rename = "nsfwLevel")]
    nsfw_level: usize,
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
    #[serde(rename = "stats")]
    stats: Stats,
    meta: serde_json::Value,
    #[serde(rename = "baseModel")]
    base_model: Option<String>,
    #[serde(rename = "modelVersionIds")]
    model_version_ids: Vec<usize>,
    #[serde(rename = "modelVersionId")]
    model_version_id: Option<usize>,
    #[serde(rename = "sortAt")]
    sort_at: Option<String>,
    #[serde(rename = "type")]
    item_type: Option<String>,
    #[serde(rename = "userId")]
    user_id: usize,
    #[serde(rename = "hasMeta")]
    has_meta: bool,
    #[serde(rename = "onSite")]
    on_site: bool,
    #[serde(rename = "postedToId")]
    posted_to_id: Option<usize>,
    #[serde(rename = "combinedNsfwLevel")]
    combined_nsfw_level: usize,
    #[serde(rename = "toolIds")]
    tool_ids: Vec<usize>,
    #[serde(rename = "techniqueIds")]
    technique_ids: Vec<usize>,
    existedAtUnix: usize,
    sortAtUnix: usize,
    tagIds: Vec<usize>,
    tags: Vec<usize>,
    #[serde(default)]
    modelVersionIdsManual: Vec<usize>,
    #[serde(default)]
    minor: bool,
    remixOfId: Option<usize>,
    #[serde(default)]
    hasPositivePrompt: bool,
    availability: Option<String>,
    #[serde(default)]
    poi: bool,
    #[serde(default)]
    acceptableMinor: bool,
    publishedAt: Option<String>,
    user: User
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
