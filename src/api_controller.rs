use actix_web::{Scope, web, HttpRequest, Responder, HttpResponse};
use crate::handler::handle;
use crate::config::Data;
use std::sync::Arc;
use std::collections::HashMap;

pub fn scope() -> Scope {
    web::scope("/api")
        .service(web::resource("/webhook").to(webhook_incoming))
}

async fn webhook_incoming(req : HttpRequest, body : web::Bytes, query: web::Query<HashMap<String, String>>) -> impl Responder {
    handle(query["secret"].clone(), Arc::new(Data::new()), body.to_vec()).unwrap();
    HttpResponse::Ok().finish()
}