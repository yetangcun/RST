use std::{
    future::{ready, Ready, Future},
    pin::Pin,
    task::{Context, Poll}
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, 
    web::{Data, ServiceConfig},
    FromRequest,
    HttpResponse,
    Error,
    error::ErrorUnauthorized
};

pub struct TkAuth {
    tk: String
}

impl<S,B> Transform<S, ServiceRequest> for TkAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TkAuthHdl<S>;
    type Future = Ready<Result<Self::Transform,Self::InitError>>;

    fn new_transform(&self, nxt: S) -> Self::Future {
        ready(Ok(TkAuthHdl {
            tk: self.tk.clone(),
            nxt
        }))
    }
}

pub struct TkAuthHdl<S> {
    tk: String,
    nxt: S, // The next service to call
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S,B> Service<ServiceRequest> for TkAuthHdl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response,Self::Error>>;

    forward_ready!(nxt);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("call fn: {0}", req.path());
        let tk = req.headers().get("Authorization").unwrap().to_str().unwrap();

        println!("get req tk: {0}", tk);

        if tk != self.tk {
            println!("call fn: {0}", "tk error");
            return Box::pin(
                ready(
                    Err(ErrorUnauthorized("Invalid token")) // Err(ErrorUnauthorized("Invalid token"))
                )
            )
        }

        let fut = self.nxt.call(req);
        Box::pin(async move {
            let res = fut.await?;
            println!("call fn end");
            Ok(res)
        })
    }
}