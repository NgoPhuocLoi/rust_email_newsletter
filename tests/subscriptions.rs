use actix_web::{App, http::header::ContentType, test, web};
use rust_email_newsletter::{configuration::get_configuration, routes::subscribe};
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool};

#[actix_web::test]
async fn subscribe_return_200_for_a_valid_form_data() {
    let configuration = get_configuration().expect("Fail to load configuration");
    let pool = PgPool::connect(&configuration.db.connection_string().expose_secret())
        .await
        .expect("Fail to connect to DB");
    let app =
        test::init_service(App::new().service(subscribe).app_data(web::Data::new(pool))).await;

    let req = test::TestRequest::post()
        .uri("/subscriptions")
        .insert_header(ContentType::form_url_encoded())
        .set_payload("username=LoiNgo&email=nploi@axonactive.com")
        .to_request();

    let resp = test::call_service(&app, req).await;

    let mut connection =
        PgConnection::connect(&configuration.db.connection_string().expose_secret())
            .await
            .expect("Fail to connect to DB");
    let saved = sqlx::query_as::<_, (String, String)>("SELECT email, username FROM subscription")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to check saved subscription");

    assert!(resp.status().is_success());
    assert_eq!(saved.0, "nploi@axonactive.com");
    assert_eq!(saved.1, "LoiNgo");
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

    let configuration = get_configuration().expect("Fail to load configuration");
    let pool = PgPool::connect(&configuration.db.connection_string().expose_secret())
        .await
        .expect("Fail to connect to DB");
    let app =
        test::init_service(App::new().service(subscribe).app_data(web::Data::new(pool))).await;

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

    let configuration = get_configuration().expect("Fail to load configuration");
    let pool = PgPool::connect(&configuration.db.connection_string().expose_secret())
        .await
        .expect("Fail to connect to DB");
    let app =
        test::init_service(App::new().service(subscribe).app_data(web::Data::new(pool))).await;

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
