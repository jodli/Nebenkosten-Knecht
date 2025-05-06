use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::Mutex;

mod api_handlers;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let app_config = match config::load_app_config("config.toml") {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Configuration load failed",
            ));
        }
    };

    // Wrap the configuration in a Mutex to make it thread-safe
    let app_data = web::Data::new(Mutex::new(app_config));

    // Start the Actix server
    info!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .app_data(app_data.clone())
            .configure(api_handlers::config_handlers::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
