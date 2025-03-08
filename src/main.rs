mod models;
mod routes;
mod services;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::Data};
use routes::{
    booking_route::{cancel_booking, create_booking, get_bookings},
    car_route::create_car,
    owner_route::create_owner,
};
use services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Youtube")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data: Data<_> = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_booking)
            .service(create_car)
            .service(create_owner)
            .service(get_bookings)
            .service(cancel_booking)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}
