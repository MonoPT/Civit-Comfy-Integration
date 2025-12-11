use crate::Civit;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Creator {
    id: usize,
    image: Option<String>,
    username: String,
    muted: bool,
    bannedAt: Option<String>,
    deletedAt: Option<String>,
    #[serde(default)]
    createdAt: String,
    publicSettings: Value,
    excludeFromLeaderboards: bool,
    links: Vec<CreatorLink>,
    stats: CreatorStats,
    rank: Option<Rank>,
    profilePicture: profilePicture
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rank {
    leaderboardCosmetic: String,
    leaderboardId: String,
    leaderboardRank: usize,
    leaderboardTitle: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatorLink  {
    url: String,
    #[serde(rename = "type")]
    link_type: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatorStats {
    downloadCountAllTime: usize,
    thumbsUpCountAllTime: usize,
    followerCountAllTime: usize,
    reactionCountAllTime: usize,
    uploadCountAllTime: usize,
    generationCountAllTime: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct profilePicture {
    id: usize,
    name: String,
    url: String,
    nsfwLevel: usize,
    hash: String,
    userId: usize,
    ingestion: String,
    #[serde(rename = "type")]
    media_type: String,
    width: usize,
    height: usize
}

impl Civit {
    pub async fn creator(&self, creator_id: u32) -> Option<Creator> {
        let client = &self.client;
        
        
           
        let response = client
                .get(format!("https://civitai.com/api/trpc/user.getCreator?input={{\"json\":{{\"id\":{creator_id},\"authed\":true}}}}"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", &self.api_key))
                .send()
                .await.unwrap().json::<Value>().await.unwrap_or_default();
        
        let res = response.get("result").unwrap_or_default().get("data").unwrap_or_default().get("json").unwrap_or_default();
        
        return match serde_json::from_value::<Creator>(res.clone()) {
            Err(e) => {
                dbg!(res);
                println!("{e}");
                None
            },
            Ok(creator) => Some(creator)
        };
    }
}