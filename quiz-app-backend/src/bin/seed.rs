use quiz_app_backend::{config, seeder};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from config
    let database_url = config::get_database_url();

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run seeder
    seeder::seed_database(&pool).await?;

    Ok(())
}
