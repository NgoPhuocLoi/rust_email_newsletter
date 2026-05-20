use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ConfirmationParams {
    subscription_token: String,
}

#[get("subscriptions/confirm")]
async fn confirm_subscriptions(params: web::Query<ConfirmationParams>) -> HttpResponse {
    HttpResponse::Ok().json(params.into_inner())
}
