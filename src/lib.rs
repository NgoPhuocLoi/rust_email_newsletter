use std::io;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    println!("Hello there");
    HttpResponse::Ok().body("OK")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .route("/hello", web::get().to(manual_hello))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
