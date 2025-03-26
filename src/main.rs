mod backend;
mod frontend;

use actix_web::{get, App, HttpServer, Responder};
use leptos::*;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello from Actix-web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("0.0.0.0", 52389))?
    .run()
    .await
}
