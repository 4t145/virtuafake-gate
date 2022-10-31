use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body("关注851181")
}