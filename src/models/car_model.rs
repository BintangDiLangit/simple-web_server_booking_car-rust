use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub name: Option<String>,
    pub car_type: Option<String>,
    pub cc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarRequest {
    pub owner: String,
    pub name: Option<String>,
    pub car_type: Option<String>,
    pub cc: Option<String>,
}

impl TryFrom<CarRequest> for Car {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: CarRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner"),
            name: item.name,
            car_type: item.car_type,
            cc: item.cc,
        })
    }
}
