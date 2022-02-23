use actix_web::{Scope, web, HttpRequest, Responder, HttpResponse};
use crate::handler::handle;
use crate::config::Data;
use std::sync::Arc;
use std::collections::HashMap;
use log::error;
use crate::model::STError;

pub fn scope() -> Scope {
    web::scope("/api")
        .service(web::resource("/webhook").to(webhook_incoming))
}

async fn webhook_incoming(req : HttpRequest, body : web::Bytes, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let resp = handle(query.get("secret"), Arc::new(Data::new()), body.to_vec());
    return match resp {
        Ok(val) => HttpResponse::Ok().finish(),
        Err(err) => {
            error!("{:?}", err);
            return match err {
                STError::Unknown(_) => HttpResponse::InternalServerError().finish(),
                STError::NoSuchProvider => HttpResponse::BadRequest().finish(),
                STError::NoHandler => HttpResponse::BadRequest().finish(),
                STError::SupplierNotFound => HttpResponse::BadRequest().finish(),
                STError::ReceiverNotFound => HttpResponse::BadRequest().finish(),
                STError::HandlerNoResponse => HttpResponse::InternalServerError().finish(),
                STError::GitRefNotFound => HttpResponse::InternalServerError().finish(),
                STError::HandlerErr(_) => HttpResponse::InternalServerError().finish(),
                STError::Error(_) => HttpResponse::InternalServerError().finish(),
                STError::IoError(_) => HttpResponse::InternalServerError().finish(),
                STError::JsonError(_) => HttpResponse::InternalServerError().finish(),
                STError::YamlError(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }

}