use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use actix_web::{get,post,web,App,Result, HttpResponse,HttpServer,Responder};

#[derive(ToSchema)]
pub struct lginput {
    pub usr: String,
    pub pwd: String,
}

#[utoipa::path(
    post,
    path = "/rst/opdoc/tst",
    request_body = lginput,
    responses(
        (status = 200, description = "Successful operation", body = String),
        (status = 400, description = "Invalid input"),
    )
)]
pub async fn tst(req_bdy: web::Json<lginput>) -> lginput {
    println!("{:?}", req_bdy.usr);
    // HttpResponse::Ok().body("Hello world!")
    req_bdy.into_inner()
}

#[derive(OpenApi)]
#[openapi(paths(tst), components(schemas(lginput)))]
pub struct ApiRsDoc;
