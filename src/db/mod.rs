use rusqlite::{params, Connection};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tweet {
    pub name     : String,
    pub value    : String
}

pub trait TweetDB{
    fn new() -> Self;
    fn create(&self);
    fn post(&self, tweet: Tweet);
    fn get(&self) -> Vec<Tweet>;
}

pub struct TweetDBSqlite3{
    conn : Connection,
}

impl TweetDB for TweetDBSqlite3{
    fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        TweetDBSqlite3 {conn: conn}
    }

    fn create(&self){
        self.conn.execute(
            "CREATE TABLE tweet (name TEXT NOT NULL, value TEXT NOT NULL)",
            params![],
        ).unwrap();
    }

    fn post(&self, tweet: Tweet){
        self.conn.execute(
            "INSERT INTO tweet (name, value)
                    VALUES (?1, ?2)",
            params![tweet.name, tweet.value],
        ).unwrap();
    }

    fn get(&self) -> Vec<Tweet>{
        let mut stmt = self.conn.prepare("SELECT name, value FROM tweet").unwrap();
        let tweet_maps = stmt.query_map(params![], |row| {
            Ok(Tweet {
                name       : row.get(0)?,
                value      : row.get(1)?
            })
        }).unwrap();
        let mut tweets = Vec::new();
        for tweet in tweet_maps {
            tweets.push(tweet.unwrap());
        }
        tweets   
    }
}