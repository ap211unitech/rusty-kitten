use std::{env, time::SystemTime};

use crate::models::{
    booking::{Booking, FullBooking},
    dog::Dog,
    owner::Owner,
};
use chrono::Utc;
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, DateTime},
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

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let res: InsertOneResult = self
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("Failed to create Dog !!");

        Ok(res)
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

    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        let now: SystemTime = Utc::now().into();

        let mut results = self
            .booking
            .aggregate(vec![
                doc! {
                    "$match": {
                        "cancelled": false,
                        "start_time": {
                            "$gte": DateTime::from_system_time(now)
                        }
                    }
                },
                doc! {
                    "$lookup": doc! {
                        "from": "owner",
                        "localField": "owner",
                        "foreignField": "_id",
                        "as": "owner"
                    }
                },
                doc! {
                    "$unwind": doc! {
                        "path": "$owner"
                    }
                },
                doc! {
                    "$lookup": doc! {
                        "from": "dog",
                        "localField": "owner._id",
                        "foreignField": "owner",
                        "as": "dogs"
                    }
                },
            ])
            .await
            .ok()
            .expect("Error getting bookings");

        let mut bookings: Vec<FullBooking> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let booking =
                        from_document(doc).expect("Error converting document to FullBooking");
                    bookings.push(booking);
                }
                Err(err) => panic!("Error getting booking: {}", err),
            }
        }

        Ok(bookings)
    }
}
