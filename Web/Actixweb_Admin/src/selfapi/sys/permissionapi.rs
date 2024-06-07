use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[get("/sys/getpermission")]
async fn getpermission() -> impl Responder {
    let rt = "hi, i'm permission";
    HttpResponse::Ok().body(rt)
}