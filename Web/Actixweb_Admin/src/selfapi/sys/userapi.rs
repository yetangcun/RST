use crate::mdl::sysmdl::usermdl::{lginput,userQueryInput,userOptInput,userQueryOutput};
use crate::bll::sysbll::userbll;
use actix_web::{get,post,web,App,Result, HttpResponse,HttpServer,Responder};

#[post("/sys/user/dologin")]
pub async fn do_login(req_body: web::Json<lginput>) -> impl Responder {
    // let rt = String::from(userbll::do_login(req_body.into_inner()).await);
    let rt = String::from("congratulations,you've logined success!");
    HttpResponse::Ok().body(rt)
    // Ok(rt)
}

// #[post("/sys/user/dologin1")]
pub async fn do_login1(req_body: web::Json<lginput>) -> impl Responder {
    // let rt = String::from(userbll::do_login(req_body.into_inner()).await);
    let rt = String::from("congratulations,you've logined success!");
    HttpResponse::Ok().body(rt)
    // Ok(rt)
}

#[get("/sys/user/getuser")]
pub async fn get_user() -> impl Responder {
    let rt = "hi, i'm user";
    HttpResponse::Ok().body(rt)
}