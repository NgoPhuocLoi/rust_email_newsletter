use rust_email_newsletter::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust_news_letter".into(), "info".into());
    init_subscriber(subscriber);
    let config = get_configuration().expect("Fail to load config");
    let pool = PgPool::connect(&config.db.connection_string().expose_secret())
        .await
        .expect("Fail to connect to DB");
    run(pool).await
}
