use actix_web::{HttpResponse, get};


#[get("/health_check")]
pub async fn health_api() -> HttpResponse {
    println!("Hello there");
    HttpResponse::Ok().body("OK")
}
