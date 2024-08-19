use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use actix_web::{get,post,web,App,Result, HttpResponse,HttpServer,Responder};
use utoipa_swagger_ui::{SwaggerUi, Url};
use serde::{Serialize,Deserialize};

#[derive(ToSchema, Serialize, Deserialize)]
pub struct lginput {
    pub usr: String,
    pub pwd: String,
}

#[utoipa::path(
    context_path = "/rsapi",
    responses(
        (status = 200, description = "Successful operation", body = String),
        (status = 400, description = "Invalid input"),
    )
)]
#[post("/rsapi/lghdl")]
pub(super) async fn lghdl(req_bdy: web::Json<lginput>) -> impl Responder {
    println!("{:?}", req_bdy.usr);
    let recv:lginput = req_bdy.into_inner();
    let rt = String::from(format!("congratulations:{0}, you've logined success!",recv.usr));
    HttpResponse::Ok().body(rt) 
}

#[utoipa::path(
    context_path = "/rsapi",
    responses(
        (status = 200, description = "Successful operation", body = String),
        (status = 400, description = "Invalid input"),
    )
)]
#[post("/rsapi/reqhdl")]
pub(super) async fn reqhdl(req_bdy: web::Json<lginput>) -> impl Responder {
    println!("{:?}", req_bdy.usr);
    let recv:lginput = req_bdy.into_inner();
    let rt = String::from(format!("great:{0}, you've request over!",recv.usr));
    HttpResponse::Ok().body(rt) 
}

#[derive(OpenApi)]
#[openapi(paths(lghdl), components(schemas(lginput)))]
pub struct ApiRsDoc;

#[derive(OpenApi)]
#[openapi(paths(reqhdl), components(schemas(lginput)))]
pub struct ApiRsDoc1;
