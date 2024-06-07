use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[get("/sys/org/getorg")]
async fn getorg() -> impl Responder {
    let rt = "hi, i'm organization";
    HttpResponse::Ok().body(rt)
}