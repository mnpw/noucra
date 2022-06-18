use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    let s = {};

    HttpResponse::Ok()
}
