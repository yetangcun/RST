pub mod rsapi;
pub mod ext;
pub mod bll;
pub mod mdl;

use utoipa_swagger_ui::{SwaggerUi, Url, oauth};
use utoipa::{
    OpenApi, 
    openapi::{
        OpenApiBuilder
    }
};
use actix_cors::Cors;
use actix_web::{get, post, web, services, App, HttpResponse, HttpServer, Responder, http::header};

use ext::{swag_ui::*, auth_ext::*};
use rsapi::{
    sysapi::{
        user::{lghdl,get_user,get_permissions,get_by_pages,user_opt}
    }
};


#[actix_web::main]
async fn main()->std::io::Result<()> { // println!("Hello, world!");
    let server_url = "127.0.0.1";
    println!("WebAPI服务{server_url}:8086侦听启动!");
    HttpServer::new(move || {
        // let cors = Cors::permissive();
        let _cors = Cors::default()
        // .allow_any_origin()
        // .allow_any_method()
        // .allow_any_header()
        .allowed_origin("http://127.0.0.1:6111")
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])  // 允许指定的HTTP方法 
        .allowed_headers(vec!["Authorization", "Content-Type", "Accept", "Access-Control-Allow-Origin", "Access-Control-Allow-Headers","Access-Control-Allow-Methods"]) // 允许更多CORS相关的请求头 
        // .expose_headers(vec!["Authorization"])
        .expose_headers(vec!["Authorization"])
        // .supports_credentials() // 允许携带认证信息(cookies等)
        .max_age(3600); // 预检请求缓存时间
        // .allow_any_origin()
        // .allowed_origin("http://192.168.30.166")
        // .allowed_origin("*") //.send_wildcard()
        // .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        // .allowed_headers(vec!["Access-Control-Allow-Headers", 
        //     "Authorization", "authorization", "X-Requested-With",
        //     "Content-Type", "content-type", "Origin", "Client-id",
        //     "user-agent", "User-Agent", "Accept", "Referer","referer",
        //     "Nonce", "signature", "Timestamp","AppKey","x-super-properties",
        //     "X-Super-Properties"])
        // .max_age(3600);

        App::new()
        .wrap(_cors)
        .wrap(TkAuth)
        .service(web::scope("/no_auth")
           .service(lghdl) //.service(get_user)
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
        .service( // 系统管理模块
            web::scope("/sys")
            .service(user_opt)
            .service(get_user)
            .service(get_by_pages)
            .service(get_permissions)
        )
        .service( // 黑名单管理模块
            web::scope("/blk")
            .service(get_user)
            .service(get_by_pages)
            .service(get_permissions)
        )
        .service( // AI智能模块
            web::scope("/ai")
            .service(get_user)
            .service(get_by_pages)
            .service(get_permissions)
        )
        .service( // Asr模块
            web::scope("/asr")
            .service(get_user)
            .service(get_by_pages)
            .service(get_permissions)
        )
    })
    .bind((server_url, 8086))?
    .run()
    .await
}
