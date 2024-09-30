// fn main() {
//     println!("Hello, world!");
// }
mod bll;
mod mdl;
mod opdoc;
mod auth_util;
pub mod selfapi;
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use selfapi::sys::userapi::{
    opt,
    do_login,
    get_user,
    user_del,
    user_add,
    user_update,
    get_usr,
    add_usr,
    search,
    do_opt
};

use opdoc::{lghdl,reqhdl,ApiRsDoc,ApiRsDoc1,ApiRsDoc2};
use utoipa::{OpenApi, openapi::OpenApiBuilder};
use utoipa_swagger_ui::{SwaggerUi, Url};

// use mdl::sysmdl::usermdl::{lginput};
// use utoipa_swagger_ui::SwaggerUi;
// use utoipa::ToSchema;
// use utoipa::OpenApi;

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
    println!("服务127.0.0.1:8080启动侦听!");
    HttpServer::new(move || {
        App::new()
            .wrap(auth_util::TkAuth)
            // .service(
            //     actix_web::web::resource("/")
            //     .wrap(authExt::auth_tk)
            //     .to(|| async { "UnAuthorized!" })
            // )
            // .service(hllo)
            // .service(outputs)
            .service(web::scope("/rsapi")
               .service(lghdl)
               .service(reqhdl)
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                .urls(vec![
                    (
                        Url::new("系统管理",  "/api-docs/openapi1.json"),
                        ApiRsDoc::openapi()
                    ),
                    (
                        Url::new("业务管理", "/api-docs/openapi2.json"),
                        ApiRsDoc1::openapi()
                    ),
                    (
                        Url::new("系统API", "/api-docs/openapi3.json"),
                        ApiRsDoc2::openapi()
                    )
                ])
            )
            // .service(
            //     SwaggerUi::new("/swagger-ui/{_:.*}")
            //         .url("/api-docs/openapi.json", openapi.clone()),
            // )
            .service(do_login)
            .service(get_user)
            .service(user_add)
            .service(get_usr)
            .service(add_usr)
            .service(user_update)
            .service(search)
            .service(do_opt)
            .route("/sys/user/del/{id}", web::delete().to(user_del))
            // .route("/sys/user/dologin", web::post().to(do_login))
            .route("/sys/user/opt", web::post().to(opt))
            .route("/hey", web::get().to(manual_hllo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}