use betsure_backend::{config::Config, db, services::data_ingestion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Starting daily fixture ingestion...");

    let config = Config::from_env()?;
    let database = db::init_db(&config.mongodb_uri, &config.mongodb_db_name).await?;

    println!("📡 Fetching fixtures from API-Football...");
    let matches = data_ingestion::fetch_todays_fixtures(&config).await?;

    println!("💾 Saving {} matches to database...", matches.len());
    data_ingestion::save_matches_to_db(&database, matches).await?;

    println!("✅ Cron job completed successfully!");
    Ok(())
}
