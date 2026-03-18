use std::io;

use actix_web::{App, HttpServer};
use crate::routes::{subscribe, health_api};

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_api)
            .service(subscribe)
    })
    .bind("localhost:9090")?
    .run()
    .await
}