use actix_web::{HttpServer, Error, App};
use crate::config::Config;
use actix_web::dev::{Transform, ServiceRequest, Service, ServiceResponse};
use futures::future::{Ready, ok};
use std::pin::Pin;
use futures::Future;
use futures::task::{Context, Poll};
use http::header::HeaderName;
use http::HeaderValue;

#[actix_web::main]
pub async fn serve() {
    let conf = Config::load();
    let listening_address = format!("{}:{}", conf.address, conf.port);

    HttpServer::new(move || App::new()
        .service(crate::api_controller::scope().wrap(ServerDefaults)))
        .bind(listening_address).unwrap()
        .run()
        .await;
}

//
// ServerDefaults middleware
//

pub struct ServerDefaults;

impl<S, B> Transform<S> for ServerDefaults
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ServerDefaultsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ServerDefaultsMiddleware { service })
    }
}

pub struct ServerDefaultsMiddleware<S> {
    service: S
}

impl<S, B> Service for ServerDefaultsMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(HeaderName::from_static("server"), HeaderValue::from_static("bgsi"));

            Ok(res)
        })
    }
}