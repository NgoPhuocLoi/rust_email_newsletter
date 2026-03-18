use std::io;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Form}};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OkResponse {
    message: String
}

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    username: String,
    email: String
}

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    println!("Hello there");
    HttpResponse::Ok().body("OK")
}

#[post("subscriptions")]
pub async fn subscribe(form: Form<SubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok().json( OkResponse {
        message: format!("username: {}, email: {}", form.username, form.email)
    })
}

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
    .bind("localhost:9090")?
    .run()
    .await
}
