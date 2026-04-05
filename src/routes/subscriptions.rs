use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    username: String,
    email: String,
}

#[post("subscriptions")]
pub async fn subscribe(form: Form<SubscriptionFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!("Adding new subscription.", %request_id, subscription_email = %form.email, subscription_username = %form.username);

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Executing DB query to save subscription");

    let saved_result = sqlx::query(
        "INSERT INTO subscription (email, username, subscribed_at) VALUES ($1, $2, $3)",
    )
    .bind(&form.email)
    .bind(&form.username)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .instrument(query_span)
    .await;

    match saved_result {
        Ok(_) => {
            tracing::info!("[Request ID: {}] New subscription created!", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "[Request ID: {}] Failed to create subscription: {}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().body("failed")
        }
    }
}
