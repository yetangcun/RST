use crate::mdl::sysmdl::usermdl;
use crate::bll::sysbll::userbll;
use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[post("/user/dologin")]
async fn dologin(req_body: String) -> impl Responder {
    let rt = "login";
    HttpResponse::Ok().body(rt)
}

#[get("/user/getuser")]
async fn getuser() -> impl Responder {
    let rt = "hi, i'm user";
    HttpResponse::Ok().body(rt)
}