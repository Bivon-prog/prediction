use betsure_backend::{config::Config, db, models::{User, Match, TeamStats, Prediction}};
use chrono::{Utc, Local, Timelike, Datelike};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Seeding database with realistic match data...");

    let config = Config::from_env()?;
    let database = db::init_db(&config.mongodb_uri, &config.mongodb_db_name).await?;

    // Clear existing matches
    let matches_collection = database.collection::<Match>("matches");
    matches_collection.drop().await?;
    println!("🗑️  Cleared existing matches");

    // Create test user
    let test_user = User::new("254712345678".to_string());
    
    let users_collection = database.collection::<User>("users");
    let result = users_collection.insert_one(test_user).await?;

    println!("✅ Created test user with ID: {}", result.inserted_id);
    println!("📱 Phone: 254712345678");

    // Get current local time and create matches for TODAY and TOMORROW
    let now = Local::now();
    let current_hour = now.hour();
    
    // Today's matches - set times after current time
    let today = now.date_naive();
    let today_match1 = if current_hour < 15 {
        today.and_hms_opt(15, 0, 0).unwrap()
    } else {
        today.and_hms_opt(current_hour + 1, 0, 0).unwrap()
    };
    let today_match2 = if current_hour < 17 {
        today.and_hms_opt(17, 30, 0).unwrap()
    } else {
        today.and_hms_opt(current_hour + 2, 30, 0).unwrap()
    };
    let today_match3 = if current_hour < 20 {
        today.and_hms_opt(20, 0, 0).unwrap()
    } else {
        today.and_hms_opt(current_hour + 3, 0, 0).unwrap()
    };
    
    // Tomorrow's matches
    let tomorrow = today.succ_opt().unwrap();
    let tomorrow_match1 = tomorrow.and_hms_opt(15, 0, 0).unwrap();
    let tomorrow_match2 = tomorrow.and_hms_opt(18, 0, 0).unwrap();
    let tomorrow_match3 = tomorrow.and_hms_opt(20, 45, 0).unwrap();

    let creation_time = Utc::now();

    let matches = vec![
        // TODAY - Free matches (Real fixtures for Feb 23, 2026)
        Match {
            id: None,
            fixture_id: 1001,
            home_team: TeamStats {
                team_id: 45,
                team_name: "Everton".to_string(),
                goals_scored_avg: 1.4,
                goals_conceded_avg: 1.6,
                attack_strength: 0.93,
                defense_strength: 1.07,
            },
            away_team: TeamStats {
                team_id: 33,
                team_name: "Manchester United".to_string(),
                goals_scored_avg: 2.1,
                goals_conceded_avg: 1.2,
                attack_strength: 1.4,
                defense_strength: 0.8,
            },
            match_date: today.and_hms_opt(20, 0, 0).unwrap().and_utc(),
            league: "Premier League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 28.0,
                draw_prob: 26.0,
                away_win_prob: 46.0,
                over_2_5_prob: 52.0,
                confidence: 62.0,
                is_premium: false,
            }),
            created_at: creation_time,
        },
        Match {
            id: None,
            fixture_id: 1002,
            home_team: TeamStats {
                team_id: 489,
                team_name: "Fiorentina".to_string(),
                goals_scored_avg: 1.9,
                goals_conceded_avg: 1.2,
                attack_strength: 1.27,
                defense_strength: 0.8,
            },
            away_team: TeamStats {
                team_id: 488,
                team_name: "Pisa".to_string(),
                goals_scored_avg: 1.5,
                goals_conceded_avg: 1.4,
                attack_strength: 1.0,
                defense_strength: 0.93,
            },
            match_date: today.and_hms_opt(18, 30, 0).unwrap().and_utc(),
            league: "Serie A".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 52.0,
                draw_prob: 28.0,
                away_win_prob: 20.0,
                over_2_5_prob: 48.0,
                confidence: 58.0,
                is_premium: false,
            }),
            created_at: creation_time,
        },
        Match {
            id: None,
            fixture_id: 1003,
            home_team: TeamStats {
                team_id: 490,
                team_name: "Bologna".to_string(),
                goals_scored_avg: 1.7,
                goals_conceded_avg: 1.3,
                attack_strength: 1.13,
                defense_strength: 0.87,
            },
            away_team: TeamStats {
                team_id: 491,
                team_name: "Udinese".to_string(),
                goals_scored_avg: 1.4,
                goals_conceded_avg: 1.5,
                attack_strength: 0.93,
                defense_strength: 1.0,
            },
            match_date: today.and_hms_opt(14, 45, 0).unwrap().and_utc(),
            league: "Serie A".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 48.0,
                draw_prob: 30.0,
                away_win_prob: 22.0,
                over_2_5_prob: 45.0,
                confidence: 54.0,
                is_premium: false,
            }),
            created_at: creation_time,
        },
        // TODAY - VIP matches
        Match {
            id: None,
            fixture_id: 1004,
            home_team: TeamStats {
                team_id: 532,
                team_name: "Alavés".to_string(),
                goals_scored_avg: 1.3,
                goals_conceded_avg: 1.6,
                attack_strength: 0.87,
                defense_strength: 1.07,
            },
            away_team: TeamStats {
                team_id: 533,
                team_name: "Girona".to_string(),
                goals_scored_avg: 2.0,
                goals_conceded_avg: 1.1,
                attack_strength: 1.33,
                defense_strength: 0.73,
            },
            match_date: today.and_hms_opt(21, 0, 0).unwrap().and_utc(),
            league: "La Liga".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 0.0,
                draw_prob: 0.0,
                away_win_prob: 0.0,
                over_2_5_prob: 0.0,
                confidence: 76.0,
                is_premium: true,
            }),
            created_at: creation_time,
        },
        // TOMORROW - Free matches (Real fixtures for Feb 24, 2026)
        Match {
            id: None,
            fixture_id: 2001,
            home_team: TeamStats {
                team_id: 487,
                team_name: "Inter Milan".to_string(),
                goals_scored_avg: 2.3,
                goals_conceded_avg: 0.9,
                attack_strength: 1.53,
                defense_strength: 0.6,
            },
            away_team: TeamStats {
                team_id: 600,
                team_name: "Bodo/Glimt".to_string(),
                goals_scored_avg: 1.6,
                goals_conceded_avg: 1.4,
                attack_strength: 1.07,
                defense_strength: 0.93,
            },
            match_date: tomorrow.and_hms_opt(15, 0, 0).unwrap().and_utc(),
            league: "Champions League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 68.0,
                draw_prob: 20.0,
                away_win_prob: 12.0,
                over_2_5_prob: 58.0,
                confidence: 72.0,
                is_premium: false,
            }),
            created_at: creation_time,
        },
        Match {
            id: None,
            fixture_id: 2002,
            home_team: TeamStats {
                team_id: 157,
                team_name: "Bayer Leverkusen".to_string(),
                goals_scored_avg: 2.6,
                goals_conceded_avg: 0.8,
                attack_strength: 1.73,
                defense_strength: 0.53,
            },
            away_team: TeamStats {
                team_id: 601,
                team_name: "Olympiacos".to_string(),
                goals_scored_avg: 1.7,
                goals_conceded_avg: 1.2,
                attack_strength: 1.13,
                defense_strength: 0.8,
            },
            match_date: tomorrow.and_hms_opt(15, 0, 0).unwrap().and_utc(),
            league: "Champions League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 62.0,
                draw_prob: 24.0,
                away_win_prob: 14.0,
                over_2_5_prob: 65.0,
                confidence: 68.0,
                is_premium: false,
            }),
            created_at: creation_time,
        },
        Match {
            id: None,
            fixture_id: 2003,
            home_team: TeamStats {
                team_id: 34,
                team_name: "Newcastle United".to_string(),
                goals_scored_avg: 2.0,
                goals_conceded_avg: 1.1,
                attack_strength: 1.33,
                defense_strength: 0.73,
            },
            away_team: TeamStats {
                team_id: 602,
                team_name: "Qarabag FK".to_string(),
                goals_scored_avg: 1.4,
                goals_conceded_avg: 1.6,
                attack_strength: 0.93,
                defense_strength: 1.07,
            },
            match_date: tomorrow.and_hms_opt(15, 0, 0).unwrap().and_utc(),
            league: "Champions League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 58.0,
                draw_prob: 26.0,
                away_win_prob: 16.0,
                over_2_5_prob: 52.0,
                confidence: 64.0,
                is_premium: false,
            }),
            created_at: creation_time,
        },
        // TOMORROW - VIP matches
        Match {
            id: None,
            fixture_id: 2004,
            home_team: TeamStats {
                team_id: 530,
                team_name: "Atlético Madrid".to_string(),
                goals_scored_avg: 2.1,
                goals_conceded_avg: 1.0,
                attack_strength: 1.4,
                defense_strength: 0.67,
            },
            away_team: TeamStats {
                team_id: 603,
                team_name: "Club Brugge".to_string(),
                goals_scored_avg: 1.6,
                goals_conceded_avg: 1.3,
                attack_strength: 1.07,
                defense_strength: 0.87,
            },
            match_date: tomorrow.and_hms_opt(17, 45, 0).unwrap().and_utc(),
            league: "Champions League".to_string(),
            prediction: Some(Prediction {
                home_win_prob: 0.0,
                draw_prob: 0.0,
                away_win_prob: 0.0,
                over_2_5_prob: 0.0,
                confidence: 80.0,
                is_premium: true,
            }),
            created_at: creation_time,
        },
    ];

    for match_data in matches {
        matches_collection.insert_one(match_data).await?;
    }

    println!("✅ Created 8 REAL matches from actual fixtures:");
    println!("   TODAY ({}): 3 free + 1 VIP predictions", today.format("%Y-%m-%d"));
    println!("     - Everton vs Manchester United (Premier League)");
    println!("     - Fiorentina vs Pisa (Serie A)");
    println!("     - Bologna vs Udinese (Serie A)");
    println!("     - Alavés vs Girona (La Liga) [VIP]");
    println!("   TOMORROW ({}): 3 free + 1 VIP predictions", tomorrow.format("%Y-%m-%d"));
    println!("     - Inter Milan vs Bodo/Glimt (Champions League)");
    println!("     - Bayer Leverkusen vs Olympiacos (Champions League)");
    println!("     - Newcastle United vs Qarabag FK (Champions League)");
    println!("     - Atlético Madrid vs Club Brugge (Champions League) [VIP]");
    println!("\n💡 Use this user_id in your frontend for testing:");
    println!("   {}", result.inserted_id);

    Ok(())
}
