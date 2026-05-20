use actix_web::{App, test};
use rust_email_newsletter::routes::confirm_subscriptions;

#[actix_web::test]
async fn confirmations_without_token_are_rejected() {
    let app = test::init_service(App::new().service(confirm_subscriptions)).await;
    let req = test::TestRequest::get()
        .uri("/subscriptions/confirm")
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status().as_u16(), 400);
}
