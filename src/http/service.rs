use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Pool, Postgres};

use super::model::Url;

const SHORTURL_LEN: usize = 4;

pub async fn shorten_url(db: &Pool<Postgres>, url: String) -> Result<()> {
    let rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(SHORTURL_LEN)
        .map(char::from)
        .collect();

    // Check if random string already exists in db
    let query = sqlx::query_as!(
        Url,
        r#"SELECT * FROM url 
         WHERE id = $1"#,
        random_string
    )
    .fetch_one(db)
    .await?;
    Ok(())
}