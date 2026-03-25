use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    username: String,
    email: String,
}

#[post("subscriptions")]
pub async fn subscribe(form: Form<SubscriptionFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    sqlx::query("INSERT INTO subscription (email, username, subscribed_at) VALUES ($1, $2, $3)")
        .bind(&form.email)
        .bind(&form.username)
        .bind(Utc::now())
        .execute(pool.get_ref())
        .await
        .expect("Failed to execute query");
    HttpResponse::Ok().body("OK")
}
