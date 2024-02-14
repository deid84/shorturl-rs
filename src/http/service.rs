use anyhow::{Result, Error};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Pool, Postgres};

use super::model::Url;

const SHORTURL_LEN: usize = 4;

pub async fn resolve_short_url(db: &Pool<Postgres>, url: String) -> Result<String, Error> {
    // Check if random string already exists in db
    match sqlx::query_as!(
        Url,
        r#"SELECT * FROM url 
                WHERE id = $1"#,
        url
    )
    .fetch_one(db)
    .await
    {
        Ok(data) => {
            // Nothing to do
            Ok(data.long_url)
        }
        Err(err) => {
            Err(err.into())
        }
    }
}

pub async fn shorten_url(db: &Pool<Postgres>, url: String) -> Result<String, Error> {
    let result;
    loop {
        let rng = rand::thread_rng();

        let random_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(SHORTURL_LEN)
            .map(char::from)
            .collect();

        // Check if random string already exists in db
        match sqlx::query_as!(
            Url,
            r#"SELECT * FROM url 
                WHERE id = $1"#,
            random_string
        )
        .fetch_one(db)
        .await
        {
            Ok(_) => {
                // Nothing to do
            }
            Err(_) => {
                // Save url in db and break loop
                match sqlx::query_as!(
                    Url,
                    r#"INSERT INTO url (id, long_url) VALUES($1, $2) RETURNING id, long_url;"#,
                    random_string,
                    url
                )
                .fetch_one(db)
                .await {
                    Ok(data) => result = Ok(data.id),
                    Err(err) => result = Err(err.into())
                };
                break;
            }
        }
    }
    result
}
