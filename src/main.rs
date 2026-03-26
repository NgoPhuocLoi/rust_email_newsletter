use env_logger::Env;
use rust_email_newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = get_configuration().expect("Fail to load config");
    let pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("Fail to connect to DB");
    run(pool).await
}
