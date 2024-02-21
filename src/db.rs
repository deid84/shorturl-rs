use redis::{Client, Connection};

pub async fn connect(db_url: String) -> anyhow::Result<Connection> { 
    let client = Client::open("redis://localhost:6379/")?;
    let con = client.get_connection()?;
    println!("Succesfully connected to Redis");
    Ok(con)
}
