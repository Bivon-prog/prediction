use crate::models::{Match, User};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserQuery {
    user_id: String,
}

#[get("/matches/today")]
pub async fn get_todays_matches(
    db: web::Data<Database>,
    query: web::Query<UserQuery>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&query.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(doc! {"error": "Invalid user_id"}),
    };

    // Check if user is VIP
    let users_collection = db.collection::<User>("users");
    let user = match users_collection.find_one(doc! {"_id": user_id}).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::NotFound().json(doc! {"error": "User not found"}),
        Err(_) => return HttpResponse::InternalServerError().json(doc! {"error": "Database error"}),
    };

    let is_vip = user.is_vip_active();

    // Fetch today's matches (temporarily get all matches for debugging)
    let matches_collection = db.collection::<Match>("matches");
    
    let filter = doc! {}; // Get all matches

    let mut cursor = match matches_collection.find(filter).await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json(doc! {"error": "Database error"}),
    };

    let mut matches = Vec::new();
    use futures::stream::StreamExt;
    
    while let Some(result) = cursor.next().await {
        match result {
            Ok(mut match_data) => {
                // Blur premium predictions for non-VIP users
                if let Some(ref mut prediction) = match_data.prediction {
                    if prediction.is_premium && !is_vip {
                        prediction.home_win_prob = 0.0;
                        prediction.draw_prob = 0.0;
                        prediction.away_win_prob = 0.0;
                        prediction.over_2_5_prob = 0.0;
                    }
                }
                matches.push(match_data);
            }
            Err(_) => continue,
        }
    }

    HttpResponse::Ok().json(matches)
}
