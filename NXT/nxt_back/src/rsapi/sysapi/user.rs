use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use crate::mdl::sysmdl::usermdl::{
    lginput
};

#[utoipa::path(
    context_path = "/no_auth",
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/user/dologin")]
pub async fn lghdl(req: web::Json<lginput>) -> impl Responder {
    HttpResponse::Ok().body(format!("congratulations:{0}, you've logined success!",req.usr))
}