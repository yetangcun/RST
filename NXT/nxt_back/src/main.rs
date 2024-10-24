pub mod rsapi;
pub mod ext;
pub mod bll;
pub mod mdl;

use utoipa_swagger_ui::{SwaggerUi, Url};
use utoipa::{OpenApi, openapi::OpenApiBuilder};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use ext::{swag_ui::*, auth_ext::*};
use rsapi::{
    sysapi::{
        user::{lghdl}
    }
};


#[actix_web::main]
async fn main()->std::io::Result<()> { // println!("Hello, world!");
    println!("web服务127.0.0.1:8086侦听启动!");
    HttpServer::new(move || {
        App::new()
        .wrap(TkAuth)
        .service(web::scope("/no_auth")
           .service(lghdl)
           // .service(reqhdl)
        )
        .service(SwaggerUi::new("/swagger-ui/{_:.*}")
            .urls(vec![
                (
                    Url::new("系统管理",  "/api-docs/openapi1.json"),
                    SysApiDoc::openapi()
                ),
                (
                    Url::new("黑名单管理",  "/api-docs/openapi2.json"),
                    BlkApiDoc::openapi()
                ),
                (
                    Url::new("Asr管理",  "/api-docs/openapi3.json"),
                    AsrApiDoc::openapi()
                ),
                (
                    Url::new("电商管理",  "/api-docs/openapi4.json"),
                    DsApiDoc::openapi()
                )
            ])
        )
    })
    .bind(("127.0.0.1", 8086))?
    .run()
    .await
}
