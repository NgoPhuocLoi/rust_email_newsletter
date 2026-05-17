use std::time::Duration;

use fake::{Fake, faker::internet::en::SafeEmail};
use rust_email_newsletter::{
    configuration::get_configuration, domain::SubscriberEmail, email_client::EmailClient,
};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{Connection, PgConnection, PgPool, Pool, Postgres};
use wiremock::MockServer;

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

pub async fn get_mock_email_server() -> (MockServer, EmailClient) {
    let mock_server = MockServer::start().await;
    let email_client = EmailClient::new(
        mock_server.uri(),
        SubscriberEmail::parse("sender@gmail.com".to_string()).expect(""),
        Duration::from_secs(2),
        SecretString::from("1234"),
    );
    (mock_server, email_client)
}

pub fn get_random_email() -> String {
    SafeEmail().fake()
}
