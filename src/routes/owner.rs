use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    models::owner::{Owner, OwnerRequest},
    services::db::Database,
};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> impl Responder {
    match db
        .create_owner(
            Owner::try_from(request.into_inner())
                .expect("Error in converting OwnerRequest to Owner !!"),
        )
        .await
    {
        Ok(owner) => HttpResponse::Ok().json(owner),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
