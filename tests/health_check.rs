use actix_web::{App, test};
use rust_email_newsletter::routes::health_api;

#[actix_web::test]
async fn health_check_works() {
    let app = test::init_service(App::new().service(health_api)).await;

    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
