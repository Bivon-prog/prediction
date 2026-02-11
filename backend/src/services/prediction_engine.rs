use crate::models::{Match, Prediction};
use statrs::distribution::{Discrete, Poisson};

const LEAGUE_AVG_GOALS: f64 = 2.7; // Premier League average

/// Calculate attack and defense strength for teams
pub fn calculate_team_strengths(
    home_goals_avg: f64,
    away_goals_avg: f64,
    home_conceded_avg: f64,
    away_conceded_avg: f64,
) -> (f64, f64, f64, f64) {
    let home_attack = home_goals_avg / LEAGUE_AVG_GOALS;
    let home_defense = home_conceded_avg / LEAGUE_AVG_GOALS;
    let away_attack = away_goals_avg / LEAGUE_AVG_GOALS;
    let away_defense = away_conceded_avg / LEAGUE_AVG_GOALS;
    
    (home_attack, home_defense, away_attack, away_defense)
}

/// Poisson probability mass function
fn poisson_pmf(lambda: f64, k: u64) -> f64 {
    match Poisson::new(lambda) {
        Ok(dist) => dist.pmf(k),
        Err(_) => 0.0,
    }
}

/// Calculate expected goals using Poisson distribution
pub fn calculate_expected_goals(
    home_attack: f64,
    away_defense: f64,
    away_attack: f64,
    home_defense: f64,
) -> (f64, f64) {
    let home_expected = home_attack * away_defense * LEAGUE_AVG_GOALS;
    let away_expected = away_attack * home_defense * LEAGUE_AVG_GOALS;
    (home_expected, away_expected)
}

/// Generate match prediction using Poisson distribution
pub fn predict_match(match_data: &Match) -> Prediction {
    let home = &match_data.home_team;
    let away = &match_data.away_team;

    let (home_expected, away_expected) = calculate_expected_goals(
        home.attack_strength,
        away.defense_strength,
        away.attack_strength,
        home.defense_strength,
    );

    let mut home_win = 0.0;
    let mut draw = 0.0;
    let mut away_win = 0.0;
    let mut over_2_5 = 0.0;

    // Calculate probabilities for scorelines 0-0 to 5-5
    for home_goals in 0..=5 {
        for away_goals in 0..=5 {
            let prob = poisson_pmf(home_expected, home_goals) * poisson_pmf(away_expected, away_goals);
            
            if home_goals > away_goals {
                home_win += prob;
            } else if home_goals == away_goals {
                draw += prob;
            } else {
                away_win += prob;
            }

            if home_goals + away_goals > 2 {
                over_2_5 += prob;
            }
        }
    }

    // Normalize probabilities
    let total = home_win + draw + away_win;
    home_win /= total;
    draw /= total;
    away_win /= total;

    // Calculate confidence (entropy-based)
    let max_prob = home_win.max(draw).max(away_win);
    let confidence = max_prob * 100.0;

    let is_premium = confidence > 75.0;

    Prediction {
        home_win_prob: (home_win * 100.0).round(),
        draw_prob: (draw * 100.0).round(),
        away_win_prob: (away_win * 100.0).round(),
        over_2_5_prob: (over_2_5 * 100.0).round(),
        confidence: confidence.round(),
        is_premium,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_prediction_engine() {
        let match_data = Match {
            id: None,
            fixture_id: 1,
            home_team: TeamStats {
                team_id: 1,
                team_name: "Man City".to_string(),
                goals_scored_avg: 2.5,
                goals_conceded_avg: 0.8,
                attack_strength: 2.5 / LEAGUE_AVG_GOALS,
                defense_strength: 0.8 / LEAGUE_AVG_GOALS,
            },
            away_team: TeamStats {
                team_id: 2,
                team_name: "Burnley".to_string(),
                goals_scored_avg: 1.0,
                goals_conceded_avg: 2.0,
                attack_strength: 1.0 / LEAGUE_AVG_GOALS,
                defense_strength: 2.0 / LEAGUE_AVG_GOALS,
            },
            match_date: Utc::now(),
            league: "Premier League".to_string(),
            prediction: None,
            created_at: Utc::now(),
        };

        let prediction = predict_match(&match_data);
        assert!(prediction.home_win_prob > prediction.away_win_prob);
        assert!(prediction.confidence > 0.0);
    }
}
