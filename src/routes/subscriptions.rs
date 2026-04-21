use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult};

use crate::domain::{NewSubscriber, SubscriberName};

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
    let subscriber = NewSubscriber {
        email: form.email.clone(),
        name: SubscriberName::parse(form.username.clone()),
    };

    match insert_subscription(&subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("failed"),
    }
}

#[tracing::instrument(name = "Inserting subscription", skip(new_subscriber, pool))]
async fn insert_subscription(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO subscription (email, username, subscribed_at) VALUES ($1, $2, $3)",
    )
    .bind(&new_subscriber.email)
    .bind(&new_subscriber.name.inner_ref())
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(result)
}
