use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    models::booking::{Booking, BookingRequest},
    services::db::Database,
};

#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> impl Responder {
    match db
        .create_booking(
            Booking::try_from(request.into_inner())
                .expect("Error converting BookingRequest to Booking !!"),
        )
        .await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[get("/booking")]
pub async fn get_bookings(db: Data<Database>) -> impl Responder {
    match db.get_bookings().await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(db: Data<Database>, path: Path<(String,)>) -> impl Responder {
    let booking_id = path.into_inner().0;

    match db.cancel_booking(booking_id).await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
