use actix_web::{get, App, HttpServer, Responder};

mod models;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello world !!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("localhost", 8000))?
        .run()
        .await
}
