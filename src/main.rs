use actix_web::{get, web::Data, App, HttpServer, Responder};
use services::db::Database;

mod models;
mod services;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello world !!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    println!("Server started to port 8000");
    HttpServer::new(move || App::new().app_data(db_data.clone()).service(hello))
        .bind(("localhost", 8000))?
        .run()
        .await
}
