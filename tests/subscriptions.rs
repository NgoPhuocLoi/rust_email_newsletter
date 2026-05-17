use actix_web::{App, http::header::ContentType, test, web};
use rust_email_newsletter::routes::subscribe;
use wiremock::{Mock, ResponseTemplate, matchers::any};

use crate::helpers::get_postgres_pool_and_connection;

mod helpers;

#[actix_web::test]
async fn subscribe_return_200_for_a_valid_form_data() {
    let (postgres_pool, mut connection) = helpers::get_postgres_pool_and_connection().await;

    let subscriber_email = helpers::get_random_email();

    let (mock_server, email_client) = helpers::get_mock_email_server().await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let app = test::init_service(
        App::new()
            .service(subscribe)
            .app_data(web::Data::new(postgres_pool))
            .app_data(web::Data::new(email_client)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/subscriptions")
        .insert_header(ContentType::form_url_encoded())
        .set_payload(format!("username=LoiNgo&email={}", &subscriber_email))
        .to_request();

    let resp = test::call_service(&app, req).await;

    let saved = sqlx::query_as::<_, (String, String, String)>(
        "SELECT email, username, status FROM subscription WHERE email = $1",
    )
    .bind(&subscriber_email)
    .fetch_one(&mut connection)
    .await
    .expect("Failed to check saved subscription");

    assert!(resp.status().is_success());
    assert_eq!(saved.0, subscriber_email);
    assert_eq!(saved.1, "LoiNgo");
    assert_eq!(saved.2, "pending_confirmation")
}

#[actix_web::test]
async fn subscribe_return_400_when_data_is_missing() {
    let app = test::init_service(App::new().service(subscribe)).await;

    let req = test::TestRequest::post()
        .uri("/subscriptions")
        .insert_header(ContentType::form_url_encoded())
        .set_payload("username=LoiNgo")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn subscribe_return_400_when_email_is_invalid() {
    let invalid_emails = vec![
        "not-an-email",
        "@missinglocal.com",
        "missingdomain@",
        "spaces in@email.com",
        "",
    ];

    let (pool, _) = get_postgres_pool_and_connection().await;
    let (_, email_client) = helpers::get_mock_email_server().await;
    let app = test::init_service(
        App::new()
            .service(subscribe)
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(email_client)),
    )
    .await;

    for invalid_email in invalid_emails {
        let payload = format!("username=LoiNgo&email={invalid_email}");
        let req = test::TestRequest::post()
            .uri("/subscriptions")
            .insert_header(ContentType::form_url_encoded())
            .set_payload(payload.clone())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_client_error(),
            "Expected 4xx for email: {invalid_email}"
        );
    }
}

#[actix_web::test]
async fn subscribe_return_400_when_username_is_invalid() {
    let invalid_names = vec![
        "",
        "   ",
        "name/with/slash",
        "name(with)parens",
        "name<with>angles",
        "name{with}braces",
        "name\"with\"quotes",
    ];

    let (pool, _) = get_postgres_pool_and_connection().await;

    let (_, email_client) = helpers::get_mock_email_server().await;
    let app = test::init_service(
        App::new()
            .service(subscribe)
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(email_client)),
    )
    .await;

    for invalid_name in invalid_names {
        let payload = format!("username={invalid_name}&email=valid@example.com");
        let req = test::TestRequest::post()
            .uri("/subscriptions")
            .insert_header(ContentType::form_url_encoded())
            .set_payload(payload.clone())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_client_error(),
            "Expected 4xx for username: {invalid_name}"
        );
    }
}
