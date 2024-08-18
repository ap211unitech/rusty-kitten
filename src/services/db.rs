use std::env;

use crate::models::{booking::Booking, dog::Dog, owner::Owner};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

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

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let res = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Failed to create owner !!");

        Ok(res)
    }

    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let res = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Failed to create booking !!");

        Ok(res)
    }

    pub async fn cancel_booking(&self, booking_id: String) -> Result<UpdateResult, Error> {
        let res = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::parse_str(booking_id).expect("Failed to parse booking id !!")
                },
                doc! {
                    "$set":doc!{
                        "cancelled":true,
                    }
                },
            )
            .await
            .ok()
            .expect("Failed to update booking !!");

        Ok(res)
    }
}
