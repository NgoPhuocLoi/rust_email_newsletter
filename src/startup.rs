use std::io;

use crate::{
    configuration::get_configuration,
    email_client::EmailClient,
    routes::{health_api, subscribe},
};
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub async fn run(db_connection: PgPool, email_client: EmailClient) -> io::Result<()> {
    let connection = web::Data::new(db_connection);
    let email_client = web::Data::new(email_client);
    let setting = get_configuration().expect("Failed to red the configuration file");
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_api)
            .service(subscribe)
            .app_data(connection.clone())
            .app_data(email_client.clone())
    })
    .bind(format!(
        "{}:{}",
        setting.application.server_host, setting.application.server_port
    ))?
    .run()
    .await
}
