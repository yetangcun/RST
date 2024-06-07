// fn main() {
//     println!("Hello, world!");
// }
mod bll;
mod mdl;
mod selfapi;
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use selfapi::sys::userapi::{do_login,get_user};
// use mdl::sysmdl::usermdl::{lginput};

// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;
// use utoipa::ToSchema;

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

// #[derive(OpenApi)]
// #[openapi(paths(do_login,get_user), components(schemas(lginput)))]
// struct ApiDoc;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    
    // let openapi = ApiDoc::openapi();

    println!("服务127.0.0.1:8080启动侦听!");
    HttpServer::new(move || {
        App::new()
            .service(hllo)
            .service(outputs)
            .service(get_user)
            .service(do_login)
            .route("/hey", web::get().to(manual_hllo))
            // .service(
            //     SwaggerUi::new("/swagger-ui/{_:.*}")
            //         .url("/api-docs/openapi.json", openapi.clone()),
            // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}