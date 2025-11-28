use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenvy::dotenv().ok(); // Disabled to avoid dependency issues
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("Migrations ran successfully!");
    Ok(())
}
