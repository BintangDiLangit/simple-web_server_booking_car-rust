use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use crate::{
    models::owner_model::{Owner, OwnerRequest},
    services::db::Database,
};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    // Convert OwnerRequest to Owner first
    let owner = match Owner::try_from(request.into_inner()) {
        Ok(c) => c,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    // Then perform the async database operation
    match db.create_owner(owner).await {
        Ok(created_owner) => HttpResponse::Ok().json(created_owner),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
