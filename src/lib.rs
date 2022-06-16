use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    let s = {};

    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
