// fn main() {
//     println!("Hello, world!");
// }
mod bll;
mod mdl;
mod selfapi;
use actix_web::{get,post,web,App, HttpResponse,HttpServer,Responder};
use selfapi::sys::userapi;

#[get("/")]
async fn hllo() -> impl Responder {
    HttpResponse::Ok().body("Hllo world!")
}

#[post("/outputs")]
async fn outputs(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hllo() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("服务127.0.0.1:8080启动侦听!");
    HttpServer::new(|| {
        App::new()
            .service(hllo)
            .service(outputs)
            .service(userapi::getuser)
            .route("/hey", web::get().to(manual_hllo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}