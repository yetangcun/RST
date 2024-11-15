use std::{
    future::{ready, Ready, Future},
    task::{Context, Poll},
    pin::Pin
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    // FromRequest,
    // HttpResponse, 
    // web::{Data, ServiceConfig},
    Error, error::ErrorUnauthorized
};
use CommonExtensionLib::utils::jwtutil;

pub struct TkAuth;
impl<S,B> Transform<S, ServiceRequest> for TkAuth
where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
      S::Future: 'static, B: 'static
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
    nxt: S, // The next service to call(下一步要执行调用的服务)
}
type LclBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;
impl<S,B> Service<ServiceRequest> for TkAuthHdl<S>
where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
      S::Future: 'static, B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LclBoxFuture<Result<Self::Response,Self::Error>>;

    forward_ready!(nxt);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let req_pth:&str = req.path();
        let req_method:&str = req.method().as_str();
        println!("req api addr: {0}, {1}", req_pth, req_method);

        // 打印所有headers信息
        // println!("请求头信息:");
        // for (name, value) in req.headers().iter() {
        //    println!("{}: {:?}", name, value);
        // }

        if req_pth.contains("/no_auth/") || 
           req_pth.contains("/swagger-ui/") || 
           req_pth.contains("/api-docs/") 
           // || req_pth.contains("/rsapi/")
           || req_method == "OPTIONS"  // 不需要校验token的情况 
        {
            let rsfut = self.nxt.call(req);
            return Box::pin(async move {
                let res = rsfut.await?; // println!("request end");
                Ok(res)
            });
        }

        let tk0 = req.headers().get("Authorization"); // 获取token
        let tk;
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
                let tk_sr:&str = tk1.to_str().unwrap();
                println!("req tk: {0}", tk_sr);
                tk = tk_sr.to_string();
            }
        }

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
            let res = fut.await?; // println!("call fn end");
            Ok(res)
        })
    }
}