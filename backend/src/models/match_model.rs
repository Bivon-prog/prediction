use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamStats {
    pub team_id: i32,
    pub team_name: String,
    pub goals_scored_avg: f64,
    pub goals_conceded_avg: f64,
    pub attack_strength: f64,
    pub defense_strength: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prediction {
    pub home_win_prob: f64,
    pub draw_prob: f64,
    pub away_win_prob: f64,
    pub over_2_5_prob: f64,
    pub confidence: f64,
    pub is_premium: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Match {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub fixture_id: i32,
    pub home_team: TeamStats,
    pub away_team: TeamStats,
    pub match_date: DateTime<Utc>,
    pub league: String,
    pub prediction: Option<Prediction>,
    pub created_at: DateTime<Utc>,
}
