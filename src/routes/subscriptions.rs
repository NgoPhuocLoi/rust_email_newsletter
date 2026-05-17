use actix_web::{
    HttpResponse, post,
    web::{self, Form},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult};

use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    email_client::EmailClient,
};

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    username: String,
    email: String,
}

impl TryFrom<SubscriptionFormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: SubscriptionFormData) -> Result<Self, Self::Error> {
        let email = SubscriberEmail::parse(value.email)?;
        let name = SubscriberName::parse(value.username)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding subscription",
    skip(form, pool, email_client),
    fields(
        email = %form.email,
        username = %form.username
    )
)]
#[post("subscriptions")]
pub async fn subscribe(
    form: Form<SubscriptionFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let subscriber: NewSubscriber = match form.0.try_into() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscription(&subscriber, &pool).await.is_err() {
        return HttpResponse::InternalServerError().body("failed");
    }

    if email_client
        .send_email(subscriber.email, "Welcome", "<h1>Hello there</h1>")
        .await
        .is_err()
    {
        tracing::error!("Failed to send to confirmation email");
        return HttpResponse::InternalServerError().body("failed");
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(name = "Inserting subscription", skip(new_subscriber, pool))]
async fn insert_subscription(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO subscription (email, username, subscribed_at, status) VALUES ($1, $2, $3, 'confirmed')",
    )
    .bind(new_subscriber.email.as_ref())
    .bind(&new_subscriber.name.as_ref())
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(result)
}
