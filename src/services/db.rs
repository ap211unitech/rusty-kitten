use std::env;

use crate::models::{booking::Booking, dog::Dog, owner::Owner};
use mongodb::{Client, Collection};

pub struct Database {
    booking: Collection<Booking>,
    dog: Collection<Dog>,
    owner: Collection<Owner>,
}

impl Database {
    pub async fn init() -> Self {
        let mongo_uri = match env::var("MONGO_URI") {
            Ok(uri) => uri.to_string(),
            Err(_) => panic!("MONGO_URI not found !!",),
        };

        println!("Connected to database !!");

        let client = Client::with_uri_str(mongo_uri).await.unwrap();
        let db = client.database("rusty-kennel");

        Database {
            booking: db.collection("booking"),
            dog: db.collection("dog"),
            owner: db.collection("owner"),
        }
    }
}
