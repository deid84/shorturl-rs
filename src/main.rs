use anyhow::Result;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

mod db;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from the `.env` file
    dotenv().ok();
    // Retrieve the value of the `DATABASE_URL` env variable
    let db_url = dotenv!("DATABASE_URL").to_owned();
    // Connect to Postgres database
    let _db = db::connect(db_url).await?;
    Ok(())
}
