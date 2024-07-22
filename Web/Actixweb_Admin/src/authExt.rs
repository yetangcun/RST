use actix_web::{web, App, HttpResponse, HttpServer, middleware, Error};
use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use std::convert::Infallible;

/// 自定义中间件验证token
pub struct AuthTkenValidator() {
    pub token: String,
}

impl<S,B> middleware::Middleware<S> for AuthTkenValidator where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    // S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthTkenValidatorMiddleware<S>;

    fn transform(&self, service: S) -> Self::Transform {
        AuthTkenValidatorMiddleware {
            service,
            token: self.token.clone(),
        }
    }
}

pub struct AuthTkenValidatorMiddleware<S> 
// where 
// S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static, 
// B: 'static 
{
    service: S,
    token: String,
}

impl<S, B> Service<ServiceRequest> for AuthTkenValidatorMiddleware<S> 
where 
    S: Service<Request = ServiceRequest, Response=ServiceResponse<B>, Error=Error>, 
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    // type Future = S::Future;
    type Future = actix_service::BoxFuture<'_, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization")
        .and_then(|tk| tk.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").to_string());

        if let Some(t) = token {
            if t == self.token {
                return Box::pin(self.service.call(req));
            }
        }

        Box::pin(async move {
            Err(HttpResponse::Unauthorized()
                .body("Invalid or missing token"))
                .into_response(req.into_response())
        })

        println!("AuthTkenValidatorMiddleware::call");
}

