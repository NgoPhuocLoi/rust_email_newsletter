use actix_web::{HttpResponse, post, web::Form};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct SubscriptionFormData {
    username: String,
    email: String
}

#[post("subscriptions")]
pub async fn subscribe(form: Form<SubscriptionFormData>) -> HttpResponse {
    HttpResponse::Ok().body("OK")
}