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

use CommonExtensionLib::utils::jwtutil;

pub struct TkAuth;

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
            nxt
        }))
    }
}

pub struct TkAuthHdl<S> {
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
        let tk0 = req.headers().get("Authorization"); // 获取token
        
        let mut tk = String::from("");
        match tk0 {
            None => {
                println!("call fn: {0}", "tk none");
                return Box::pin(
                    ready(
                        Err(ErrorUnauthorized("Invalid tken")) // Err(ErrorUnauthorized("Invalid token"))
                    )
                )
            },
            Some(tk1) => {
                println!("call fn: {0}", "tk some");
                println!("get reqtk: {0}", tk1.to_str().unwrap());
                tk = tk1.to_str().unwrap().to_string();
            }
        }

        println!("get req tk: {0}", tk);

        let check_rs:(bool,String) = jwtutil::verify_tken(&tk); // 校验token

        if false == check_rs.0 {
            println!("call fn: {0}", "tk error");
            return Box::pin(
                ready(
                    Err(ErrorUnauthorized("Invalid token")) // Err(ErrorUnauthorized("Invalid token"))
                )
            )
        }

        let fut = self.nxt.call(req); // token校验通过，继续调用下一个服务
        Box::pin(async move {
            let res = fut.await?;
            println!("call fn end");
            Ok(res)
        })
    }
}