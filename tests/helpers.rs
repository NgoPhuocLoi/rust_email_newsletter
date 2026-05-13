use rust_email_newsletter::configuration::get_configuration;
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool, Pool, Postgres};

pub async fn get_postgres_pool_and_connection() -> (Pool<Postgres>, PgConnection) {
    let configuration = get_configuration().expect("Fail to load configuration");
    let pool = PgPool::connect(&configuration.db.connection_string().expose_secret())
        .await
        .expect("Fail to connect to DB");
    let connection: PgConnection =
        PgConnection::connect(&configuration.db.connection_string().expose_secret())
            .await
            .expect("Fail to connect to DB");
    (pool, connection)
}
