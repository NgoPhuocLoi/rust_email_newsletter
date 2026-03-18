use actix_web::{App, http::header::ContentType, test};
use rust_email_newsletter::routes::subscribe;

#[actix_web::test]
async fn subscribe_return_200_for_a_valid_form_data() {
    let app = test::init_service(App::new().service(subscribe)).await;

    let req = test::TestRequest::post().uri("/subscriptions")
        .insert_header(ContentType::form_url_encoded())
        .set_payload("username=LoiNgo&email=nploi@axonactive.com")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn subscribe_return_400_when_data_is_missing() {
    let app = test::init_service(App::new().service(subscribe)).await;

    let req = test::TestRequest::post().uri("/subscriptions")
        .insert_header(ContentType::form_url_encoded())
        .set_payload("username=LoiNgo")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_client_error());
}