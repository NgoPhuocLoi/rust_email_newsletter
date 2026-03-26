use std::io;

use crate::{
    configuration::get_configuration,
    routes::{health_api, subscribe},
};
use actix_web::{App, HttpServer, middleware::Logger, web};
use sqlx::PgPool;

pub async fn run(db_connection: PgPool) -> io::Result<()> {
    let connection = web::Data::new(db_connection);
    let setting = get_configuration().expect("Failed to red the configuration file");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_api)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .bind(format!("localhost:{}", setting.server_port))?
    .run()
    .await
}
