use actix_web::{App, http::header::ContentType, test};
use rust_email_newsletter::{configuration::get_configuration, routes::subscribe};
use sqlx::{Connection, PgConnection};

#[actix_web::test]
async fn subscribe_return_200_for_a_valid_form_data() {
    let app = test::init_service(App::new().service(subscribe)).await;
    let configuration = get_configuration().expect("Fail to load configuration");

    let req = test::TestRequest::post()
        .uri("/subscriptions")
        .insert_header(ContentType::form_url_encoded())
        .set_payload("username=LoiNgo&email=nploi@axonactive.com")
        .to_request();

    let resp = test::call_service(&app, req).await;

    let mut connection = PgConnection::connect(&configuration.db.connection_string())
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
