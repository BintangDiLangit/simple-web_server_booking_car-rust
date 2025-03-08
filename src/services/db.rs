use std::{env, str::FromStr, time::SystemTime};

use crate::models::{
    booking_model::{Booking, FullBooking},
    car_model::Car,
    owner_model::Owner,
};
use chrono::Utc;
use futures_util::StreamExt;
use mongodb::{
    Client, Collection,
    bson::{DateTime, doc, from_document, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

pub struct Database {
    booking: Collection<Booking>,
    car: Collection<Car>,
    owner: Collection<Owner>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => {
                "mongodb://admin:w4xKXLHklhLa4BU7rS@103.245.38.205:27017/?directConnection=true"
                    .to_string()
            }
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("car_driving");

        let booking: Collection<Booking> = db.collection("booking");
        let car: Collection<Car> = db.collection("car");
        let owner: Collection<Owner> = db.collection("owner");

        Database {
            booking,
            car,
            owner,
        }
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let result = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Error creating owner");
        Ok(result)
    }

    pub async fn create_car(&self, car: Car) -> Result<InsertOneResult, Error> {
        let result = self
            .car
            .insert_one(car)
            .await
            .ok()
            .expect("Error creating car");
        Ok(result)
    }

    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Error creating booking");
        Ok(result)
    }

    pub async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id":ObjectId::from_str(booking_id).expect("Failed to parse booking_id")
                },
                doc! {
                    "$set": doc! {
                        "cancelled": true
                    }
                },
            )
            .await
            .ok()
            .expect("Error canceling booking");
        Ok(result)
    }

    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        let now: SystemTime = Utc::now().into();
        let mut results = self
            .booking
            .aggregate(vec![
                doc! {
                    "$match":{
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
                        "from": "car",
                        "localField": "owner._id",
                        "foreignField": "owner",
                        "as": "cars"
                    }
                },
            ])
            .await
            .ok()
            .expect("Error getting full bookings");

        let mut bookings: Vec<FullBooking> = Vec::new();
        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let booking: FullBooking =
                        from_document(doc).expect("Error converting document to full booking");
                    bookings.push(booking)
                }
                Err(err) => panic!("Error getting booking: {}", err),
            }
        }

        Ok(bookings)
    }
}
