use rust_email_newsletter::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
