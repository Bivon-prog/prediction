mod config;
mod db;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use routes::{matches::get_todays_matches, payments::{initiate_payment, mpesa_callback}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env().expect("Failed to load config");
    let database = db::init_db(&config.mongodb_uri, &config.mongodb_db_name)
        .await
        .expect("Failed to connect to MongoDB");

    let host = config.app_host.clone();
    let port = config.app_port;

    println!("🚀 BetSure Analytics API running on http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(get_todays_matches)
            .service(initiate_payment)
            .service(mpesa_callback)
    })
    .bind((host, port))?
    .run()
    .await
}
