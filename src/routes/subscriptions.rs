use actix_web::HttpResponse;
use actix_web::web;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

// Serde will attempt to extract to FormData, and if all componensts do not succeed,
// the handler won't even be called and we won't get an OK anyhow
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
