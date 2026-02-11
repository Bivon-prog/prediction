use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub mongodb_uri: String,
    pub mongodb_db_name: String,
    pub jwt_secret: String,
    pub api_football_base_url: String,
    pub api_football_key: String,
    pub mpesa_base_url: String,
    pub mpesa_consumer_key: String,
    pub mpesa_consumer_secret: String,
    pub mpesa_shortcode: String,
    pub mpesa_passkey: String,
    pub mpesa_callback_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        Ok(Config {
            app_host: std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            app_port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            mongodb_uri: std::env::var("MONGODB_URI")?,
            mongodb_db_name: std::env::var("MONGODB_DB_NAME")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            api_football_base_url: std::env::var("API_FOOTBALL_BASE_URL")?,
            api_football_key: std::env::var("API_FOOTBALL_KEY")?,
            mpesa_base_url: std::env::var("MPESA_BASE_URL")?,
            mpesa_consumer_key: std::env::var("MPESA_CONSUMER_KEY")?,
            mpesa_consumer_secret: std::env::var("MPESA_CONSUMER_SECRET")?,
            mpesa_shortcode: std::env::var("MPESA_SHORTCODE")?,
            mpesa_passkey: std::env::var("MPESA_PASSKEY")?,
            mpesa_callback_url: std::env::var("MPESA_CALLBACK_URL")?,
        })
    }
}
