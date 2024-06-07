use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};

#[get("/sys/role/getrole")]
async fn getrole() -> impl Responder {
    let rt = "hi, i'm role";
    HttpResponse::Ok().body(rt)
}