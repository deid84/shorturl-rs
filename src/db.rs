use anyhow::{Context, Result};
use redis::{Client, Connection};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

/// Create a new `PgPoolOptions` instance and set the
/// maximum number of connections in the connection pool to 50.
// pub async fn connect(db_url: String) -> Result<Pool<Postgres>> {
//     let db = PgPoolOptions::new()
//         .max_connections(50)
//         .connect(&db_url)
//         .await
//         .context("Error: unable to connect to database!")?;

//     println!("Successfully connected to database.");

//     Ok(db)
// }

pub async fn connect(db_url: String) -> anyhow::Result<Connection> { 
    let client = Client::open("redis://localhost:6379")?;
    let con = client.get_connection()?;
    Ok(con)
}
