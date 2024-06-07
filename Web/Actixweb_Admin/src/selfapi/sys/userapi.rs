use crate::mdl::sysmdl::usermdl::{lginput,userQueryInput,userOptInput,userQueryOutput};
use crate::bll::sysbll::userbll;
use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[post("/sys/user/dologin")]
async fn do_login(req_body: web::Data<lginput>) -> impl Responder {
    let rt = "congratulations,you've logined success!";
    HttpResponse::Ok().body(rt)
}

#[get("/sys/user/getuser")]
async fn get_user() -> impl Responder {
    let rt = "hi, i'm user";
    HttpResponse::Ok().body(rt)
}