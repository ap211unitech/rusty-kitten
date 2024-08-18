use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    models::dog::{Dog, DogRequest},
    services::db::Database,
};

#[post("/dog")]
pub async fn create_dog(db: Data<Database>, request: Json<DogRequest>) -> impl Responder {
    match db
        .create_dog(
            Dog::try_from(request.into_inner()).expect("Error converting DogRequest to Dog !!"),
        )
        .await
    {
        Ok(dog) => HttpResponse::Ok().json(dog),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
