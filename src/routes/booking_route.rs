use actix_web::{
    HttpResponse, get, post, put,
    web::{Data, Json, Path},
};

use crate::{
    models::booking_model::{Booking, BookingRequest},services::db::Database,
};


#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
    // Convert BookingRequest to Booking first
    let booking: Booking = match Booking::try_from(request.into_inner()) {
        Ok(c) => c,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    // Then perform the async database operation
    match db.create_booking(booking).await {
        Ok(created_booking) => HttpResponse::Ok().json(created_booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/bookings")]
pub async fn get_bookings(db: Data<Database>) -> HttpResponse {
    match db.get_bookings().await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(db: Data<Database>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    match db.cancel_booking(id.as_str()).await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
