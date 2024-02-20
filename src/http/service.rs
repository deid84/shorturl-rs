use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Error, Pool, Postgres};

use super::model::Url;

const MAX_RETRIES: usize = 5;
const SHORTURL_LEN: usize = 4;

pub async fn resolve_short_url(db: &Pool<Postgres>, url: String) -> Result<String, Error> {
    // Check if random string already exists in db
    // match sqlx::query_as!(
    //     Url,
    //     r#"SELECT * FROM url 
    //             WHERE id = $1"#,
    //     url
    // )
    // .fetch_one(db)
    // .await
    // {
    //     Ok(data) => {
    //         // The URL has been found in the database, returning the corresponding long URL
    //         Ok(data.long_url)
    //     }
    //     Err(err) => {
    //         // An error occurred during the database operation, returning the error
    //         Err(err)
    //     }
    // }
    Ok("ok".to_string())
}

pub async fn shorten_url(db: &Pool<Postgres>, url: String) -> Result<String, Error> {
    // Loop for a maximum number of retries
    // for _ in 0..MAX_RETRIES {
    //     let rng = rand::thread_rng();

    //     // Generate a random string for the short URL
    //     let random_string: String = rng
    //         .sample_iter(&Alphanumeric)
    //         .take(SHORTURL_LEN)
    //         .map(char::from)
    //         .collect();

    //     // Try to insert data in the db
    //     match sqlx::query_as!(
    //         Url,
    //         r#"INSERT INTO url (id, long_url) 
    //         VALUES ($1, $2)
    //         RETURNING id, long_url;"#,
    //         random_string,
    //         url
    //     )
    //     .fetch_one(db)
    //     .await
    //     {
    //         Ok(_) => {
    //             // Successful insertion, return the generated random string
    //             return Ok(random_string);
    //         }
    //         Err(err) => {
    //             // Check if the error is due to a unique constraint violation (value already exists)
    //             if let sqlx::Error::Database(db_err) = &err {
    //                 if db_err.is_foreign_key_violation() {
    //                     // Retry inserting with a new random string
    //                     continue;
    //                 }
    //             }
    //             // If it's not a unique constraint violation or if the maximum retries are reached, return the error
    //             return Err(err);
    //         }
    //     }
    // }
    // // If maximum retries are reached without successful insertion, return an error
    // Err(Error::Io(std::io::Error::new(
    //     std::io::ErrorKind::Other,
    //     "Maximum retries reached without successful insertion",
    // )))
    Ok("ok".to_string())
}
