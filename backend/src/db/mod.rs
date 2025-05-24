use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use diesel::prelude::*;
use std::env;

use crate::DbPool;

// Embed the migrations in the binary
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// Run database migrations
pub fn run_migrations(pool: &DbPool) {
    let mut conn = pool
        .get()
        .expect("Failed to get DB connection for migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");

    info!("Database migrations completed successfully");
}

// Get a connection from the pool
pub fn get_connection(pool: &DbPool) -> PooledConnection<ConnectionManager<SqliteConnection>> {
    pool.get().expect("Failed to get DB connection from pool")
}

// Establish a direct connection to the database for handlers that don't use the connection pool
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "nebenkosten_knecht.db".to_string());
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
