use std::io;

use actix_web::{App, HttpServer};
use crate::{configuration::get_configuration, routes::{health_api, subscribe}};

pub async fn run() -> io::Result<()> {
    let setting = get_configuration().expect("Failed to red the configuration file");
    HttpServer::new(|| {
        App::new()
            .service(health_api)
            .service(subscribe)
    })
    .bind(format!("localhost:{}", setting.server_port))?
    .run()
    .await
}