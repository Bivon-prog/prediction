use crate::config::Config;
use crate::models::{Match, TeamStats};
use crate::services::prediction_engine::{calculate_team_strengths, predict_match};
use chrono::{DateTime, Utc};
use mongodb::Database;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SportmonksResponse {
    data: Vec<SportmonksFixture>,
}

#[derive(Debug, Deserialize)]
struct SportmonksFixture {
    id: i32,
    starting_at: String,
    participants: Vec<Participant>,
    league: LeagueInfo,
}

#[derive(Debug, Deserialize)]
struct Participant {
    id: i32,
    name: String,
    meta: ParticipantMeta,
}

#[derive(Debug, Deserialize)]
struct ParticipantMeta {
    location: String,
}

#[derive(Debug, Deserialize)]
struct LeagueInfo {
    name: String,
}

pub async fn fetch_todays_fixtures(config: &Config) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // Get today's date
    let today = Utc::now().format("%Y-%m-%d").to_string();
    
    // Sportmonks API endpoint for fixtures
    let url = format!("{}/fixtures?api_token={}&filters=fixtureDate:{}", 
        config.api_football_base_url, 
        config.api_football_key,
        today
    );
    
    println!("📡 Fetching fixtures from Sportmonks API...");
    
    let response = client
        .get(&url)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API returned status {}: {}", status, error_text).into());
    }

    let data = response.json::<SportmonksResponse>().await?;
    
    println!("✅ Received {} fixtures from API", data.data.len());

    let mut matches = Vec::new();

    for fixture in data.data.iter().take(10) { // Limit to 10 matches
        if fixture.participants.len() < 2 {
            continue;
        }

        // Determine home and away teams
        let (home_team, away_team) = if fixture.participants[0].meta.location == "home" {
            (&fixture.participants[0], &fixture.participants[1])
        } else {
            (&fixture.participants[1], &fixture.participants[0])
        };

        // Generate realistic team statistics
        let (home_goals_avg, home_conceded_avg) = generate_team_stats(&home_team.name);
        let (away_goals_avg, away_conceded_avg) = generate_team_stats(&away_team.name);

        let (home_attack, home_defense, away_attack, away_defense) = calculate_team_strengths(
            home_goals_avg,
            away_goals_avg,
            home_conceded_avg,
            away_conceded_avg,
        );

        let home_team_stats = TeamStats {
            team_id: home_team.id,
            team_name: home_team.name.clone(),
            goals_scored_avg: home_goals_avg,
            goals_conceded_avg: home_conceded_avg,
            attack_strength: home_attack,
            defense_strength: home_defense,
        };

        let away_team_stats = TeamStats {
            team_id: away_team.id,
            team_name: away_team.name.clone(),
            goals_scored_avg: away_goals_avg,
            goals_conceded_avg: away_conceded_avg,
            attack_strength: away_attack,
            defense_strength: away_defense,
        };

        let match_date = DateTime::parse_from_rfc3339(&fixture.starting_at)?
            .with_timezone(&Utc);

        let mut match_data = Match {
            id: None,
            fixture_id: fixture.id,
            home_team: home_team_stats,
            away_team: away_team_stats,
            match_date,
            league: fixture.league.name.clone(),
            prediction: None,
            created_at: Utc::now(),
        };

        let prediction = predict_match(&match_data);
        match_data.prediction = Some(prediction);

        matches.push(match_data);
    }

    Ok(matches)
}

// Generate realistic team statistics based on team quality
fn generate_team_stats(team_name: &str) -> (f64, f64) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Top tier teams
    let top_teams = [
        "Manchester City", "Liverpool", "Bayern", "Real Madrid", 
        "Barcelona", "Paris Saint-Germain", "Arsenal", "Inter"
    ];
    
    // Good teams
    let good_teams = [
        "Manchester United", "Chelsea", "Tottenham", "Newcastle",
        "Atletico", "Dortmund", "Milan", "Juventus"
    ];

    // Use team name hash for consistent but varied stats
    let mut hasher = DefaultHasher::new();
    team_name.hash(&mut hasher);
    let hash = hasher.finish();
    let variation = (hash % 100) as f64 / 100.0;

    if top_teams.iter().any(|&t| team_name.contains(t)) {
        (2.3 + (variation * 0.7), 0.7 + (variation * 0.4))
    } else if good_teams.iter().any(|&t| team_name.contains(t)) {
        (1.7 + (variation * 0.6), 1.0 + (variation * 0.5))
    } else {
        (1.2 + (variation * 0.8), 1.1 + (variation * 0.7))
    }
}

pub async fn save_matches_to_db(db: &Database, matches: Vec<Match>) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Match>("matches");
    
    // Clear existing matches first
    collection.drop().await?;
    
    for match_data in matches {
        collection.insert_one(match_data).await?;
    }

    Ok(())
}
