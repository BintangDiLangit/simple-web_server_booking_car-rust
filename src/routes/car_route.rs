use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use crate::{
    models::car_model::{Car, CarRequest},services::db::Database,
};

#[post("/car")]
pub async fn create_car(db: Data<Database>, request: Json<CarRequest>) -> HttpResponse {
    // Convert CarRequest to Car first
    let car = match Car::try_from(request.into_inner()) {
        Ok(c) => c,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    // Then perform the async database operation
    match db.create_car(car).await {
        Ok(created_car) => HttpResponse::Ok().json(created_car),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}