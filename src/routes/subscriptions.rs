use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use serde::Deserialize;
use sqlx::PgConnection;

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    _username: String,
    _email: String,
}

#[post("subscriptions")]
pub async fn subscribe(
    _form: Form<SubscriptionFormData>,
    _data: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
