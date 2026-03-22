use std::io;

use crate::{
    configuration::get_configuration,
    routes::{health_api, subscribe},
};
use actix_web::{App, HttpServer, web};
use sqlx::PgConnection;

pub async fn run(db_connection: PgConnection) -> io::Result<()> {
    let connection = web::Data::new(db_connection);
    let setting = get_configuration().expect("Failed to red the configuration file");
    HttpServer::new(move || {
        App::new()
            .service(health_api)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .bind(format!("localhost:{}", setting.server_port))?
    .run()
    .await
}
