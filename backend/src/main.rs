use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod models;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Set up logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");

    // Run database migrations
    db::run_migrations(&pool);

    // Start HTTP server
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    log::info!("Starting server at: {}", bind_address);

    HttpServer::new(move || {
        // Configure CORS for local development
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            // Register API routes
            .configure(handlers::property_unit::configure)
            .configure(handlers::tenant::configure)
            .configure(handlers::meter::configure)
            .configure(handlers::meter_reading::configure)
            .configure(handlers::cost::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
