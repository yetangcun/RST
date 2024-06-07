use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[get("/sys/menu/getmenu")]
async fn getmenu() -> impl Responder {
    let rt = "hi, i'm menu";
    HttpResponse::Ok().body(rt)
}