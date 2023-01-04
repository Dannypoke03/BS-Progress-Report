use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Badge {
    pub description: String,
    pub image: String,
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreStats {
    pub total_score: u64,
    pub total_ranked_score: u64,
    pub average_ranked_accuracy: f64,
    pub total_play_count: u64,
    pub ranked_play_count: u64,
    pub replays_watched: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SSUser {
    pub id: String,
    pub name: String,
    pub bio: Option<String>,
    pub profile_picture: String,
    pub country: String,
    pub pp: f32,
    pub rank: i32,
    pub country_rank: i32,
    pub role: Option<String>,
    pub badges: Option<Vec<Badge>>,
    pub histories: String,
    pub score_stats: ScoreStats,
    pub permissions: i32,
    pub banned: bool,
    pub inactive: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SSPlayersResponse {
    pub players: Vec<SSUser>,
}

pub async fn get_users(
    count: i32,
    countries: Option<String>,
) -> Result<Vec<SSUser>, reqwest::Error> {
    let pages = (count as f32 / 50.0).floor() as i32;

    let mut users: Vec<SSUser> = Vec::new();
    let mut page = 0;
    while page <= pages {
        page += 1;
        users.extend(get_users_page(Some(page), countries.clone()).await?);
    }

    Ok(users)
}

pub async fn get_users_page(
    page: Option<i32>,
    countries: Option<String>,
) -> Result<Vec<SSUser>, reqwest::Error> {
    let cur_page = page.unwrap_or(1).to_string();
    let mut params = HashMap::new();
    params.insert("limit", "250");
    params.insert("page", &cur_page);

    if let Some(ref c) = countries {
        params.insert("countries", c);
    }

    let client = reqwest::Client::new();
    let resp = client
        .get("https://scoresaber.com/api/players")
        .query(&params)
        .send()
        .await;
    match resp {
        Ok(r) => {
            let users = r.json::<SSPlayersResponse>().await?;
            let new_users = users.players.clone();
            Ok(new_users)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}
