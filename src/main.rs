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
    let port = 52389;
    println!("Server running at http://0.0.0.0:{}", port);
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
