use actix_web::{Scope, web, HttpRequest, Responder, HttpResponse};

pub fn scope() -> Scope {
    web::scope("/api")
        .service(web::resource("/webhook").to(webhook_incoming))
}

async fn webhook_incoming(req : HttpRequest, body : web::Bytes) -> impl Responder {
    HttpResponse::Ok().finish()
}