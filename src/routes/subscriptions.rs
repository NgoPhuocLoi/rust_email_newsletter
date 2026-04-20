use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    username: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding subscription",
    skip(form, pool),
    fields(
        email = %form.email,
        username = %form.username
    )
)]
#[post("subscriptions")]
pub async fn subscribe(form: Form<SubscriptionFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.username) {
        tracing::info!("Invlaid username: {}", &form.username);
        return HttpResponse::BadRequest().finish();
    }

    match insert_subscription(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("failed"),
    }
}

#[tracing::instrument(name = "Inserting subscription", skip(form, pool))]
async fn insert_subscription(
    form: &Form<SubscriptionFormData>,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO subscription (email, username, subscribed_at) VALUES ($1, $2, $3)",
    )
    .bind(&form.email)
    .bind(&form.username)
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(result)
}

pub fn is_valid_name(name: &str) -> bool {
    const MAXIMUM_NAME_LENGTH: usize = 256;
    const FORBBIDEN_CHARS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    // Should not be empty
    if name.trim().is_empty() {
        return false;
    }

    // Should be shorter than MAXIMUM_NAME_LENGTH
    if name.graphemes(true).count() > MAXIMUM_NAME_LENGTH {
        return false;
    }

    // Should not contain any forbbiden characters
    if name.chars().any(|c| FORBBIDEN_CHARS.contains(&c)) {
        return false;
    }

    true
}
