use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult};

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
