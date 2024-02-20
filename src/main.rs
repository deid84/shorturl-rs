use anyhow::Result;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

mod db;
mod http;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from the `.env` file
    dotenv().ok();
    // Retrieve the value of the `DATABASE_URL` env variable
    let db_url = dotenv!("REDIS_URL").to_owned();
    // Connect to Postgres database
    let db = db::connect(db_url).await?;
    // Start the http server
    // http::serve(db).await?;
    Ok(())
}
