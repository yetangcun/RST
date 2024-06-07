use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[get("/sys/cfg/getconfig")]
async fn getconfig() -> impl Responder {
    let rt = "hi, i'm config";
    HttpResponse::Ok().body(rt)
}