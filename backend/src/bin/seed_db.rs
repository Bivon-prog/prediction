use betsure_backend::{config::Config, db, models::{User, Match, TeamStats, Prediction}};
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Seeding database with test data...");

    let config = Config::from_env()?;
    let database = db::init_db(&config.mongodb_uri, &config.mongodb_db_name).await?;

    // Create test user
    let test_user = User::new("254712345678".to_string());
    
    let users_collection = database.collection::<User>("users");
    let result = users_collection.insert_one(test_user).await?;

    println!("✅ Created test user with ID: {}", result.inserted_id);
    println!("📱 Phone: 254712345678");

    // Create sample matches for today
    let matches_collection = database.collection::<Match>("matches");
    
    let now = Utc::now();
    let today_15_00 = now + Duration::hours(3);
    let today_17_30 = now + Duration::hours(5);
    let today_20_00 = now + Duration::hours(8);

    let matches = vec![
        Match {
            id: None,
            fixture_id: 1001,
            home_team: TeamStats {
                team_id: 33,
                team_name: "Manchester United".to_string(),
                goals_scored_avg: 2.1,
                goals_conceded_avg: 1.2,
                attack_strength: 1.4,
                defense_strength: 0.8,
            },
            away_team: TeamStats {
                team_id: 34,
                team_name: "Newcastle United".to_string(),
                goals_scored_avg: 1.8,
                goals_conceded_avg: 1.4,
                attack_strength: 1.2,
                defense_strength: 0.93,
            },
            match_date: today_15_00,
            league: "Premier League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 52.0,
                draw_prob: 26.0,
                away_win_prob: 22.0,
                over_2_5_prob: 58.0,
                confidence: 68.0,
                is_premium: false,
            }),
            created_at: now,
        },
        Match {
            id: None,
            fixture_id: 1002,
            home_team: TeamStats {
                team_id: 40,
                team_name: "Liverpool".to_string(),
                goals_scored_avg: 2.5,
                goals_conceded_avg: 0.9,
                attack_strength: 1.67,
                defense_strength: 0.6,
            },
            away_team: TeamStats {
                team_id: 50,
                team_name: "Manchester City".to_string(),
                goals_scored_avg: 2.8,
                goals_conceded_avg: 0.8,
                attack_strength: 1.87,
                defense_strength: 0.53,
            },
            match_date: today_17_30,
            league: "Premier League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 0.0,
                draw_prob: 0.0,
                away_win_prob: 0.0,
                over_2_5_prob: 0.0,
                confidence: 82.0,
                is_premium: true,
            }),
            created_at: now,
        },
        Match {
            id: None,
            fixture_id: 1003,
            home_team: TeamStats {
                team_id: 529,
                team_name: "Barcelona".to_string(),
                goals_scored_avg: 2.3,
                goals_conceded_avg: 1.0,
                attack_strength: 1.53,
                defense_strength: 0.67,
            },
            away_team: TeamStats {
                team_id: 530,
                team_name: "Atletico Madrid".to_string(),
                goals_scored_avg: 1.7,
                goals_conceded_avg: 1.1,
                attack_strength: 1.13,
                defense_strength: 0.73,
            },
            match_date: today_20_00,
            league: "La Liga".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 0.0,
                draw_prob: 0.0,
                away_win_prob: 0.0,
                over_2_5_prob: 0.0,
                confidence: 78.0,
                is_premium: true,
            }),
            created_at: now,
        },
        Match {
            id: None,
            fixture_id: 1004,
            home_team: TeamStats {
                team_id: 157,
                team_name: "Bayern Munich".to_string(),
                goals_scored_avg: 2.9,
                goals_conceded_avg: 0.7,
                attack_strength: 1.93,
                defense_strength: 0.47,
            },
            away_team: TeamStats {
                team_id: 165,
                team_name: "Borussia Dortmund".to_string(),
                goals_scored_avg: 2.2,
                goals_conceded_avg: 1.3,
                attack_strength: 1.47,
                defense_strength: 0.87,
            },
            match_date: today_17_30,
            league: "Bundesliga".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 48.0,
                draw_prob: 28.0,
                away_win_prob: 24.0,
                over_2_5_prob: 65.0,
                confidence: 55.0,
                is_premium: false,
            }),
            created_at: now,
        },
        Match {
            id: None,
            fixture_id: 1005,
            home_team: TeamStats {
                team_id: 489,
                team_name: "AC Milan".to_string(),
                goals_scored_avg: 1.9,
                goals_conceded_avg: 1.2,
                attack_strength: 1.27,
                defense_strength: 0.8,
            },
            away_team: TeamStats {
                team_id: 487,
                team_name: "Inter Milan".to_string(),
                goals_scored_avg: 2.1,
                goals_conceded_avg: 0.9,
                attack_strength: 1.4,
                defense_strength: 0.6,
            },
            match_date: today_20_00,
            league: "Serie A".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 38.0,
                draw_prob: 32.0,
                away_win_prob: 30.0,
                over_2_5_prob: 52.0,
                confidence: 45.0,
                is_premium: false,
            }),
            created_at: now,
        },
    ];

    for match_data in matches {
        matches_collection.insert_one(match_data).await?;
    }

    println!("✅ Created 5 sample matches for today");
    println!("   - 3 free predictions (confidence <60%)");
    println!("   - 2 premium predictions (confidence >75%)");
    println!("\n💡 Use this user_id in your frontend for testing:");
    println!("   {}", result.inserted_id);

    Ok(())
}
