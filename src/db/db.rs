use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    Client, Database
};

#[derive(Debug)]
pub struct DatabaseInstance {
    pub instance: Database
}

impl DatabaseInstance {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable")
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        return Self {
            instance: db
        }
    }
}
