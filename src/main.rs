use rust_email_newsletter::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Fail to load config");
    let connection = PgConnection::connect(&config.db.connection_string())
        .await
        .expect("Fail to connect to DB");
    run(connection).await
}
