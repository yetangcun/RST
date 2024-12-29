use utoipa::{
    // ToSchema, IntoParams, 
    OpenApi,
    openapi::{
        OpenApiBuilder, 
        SecurityRequirement,
        security::{ 
            SecurityScheme, 
            HttpBuilder, 
            HttpAuthScheme
        }
    }
};
use utoipa_swagger_ui::{SwaggerUi, Url};
use serde::{Serialize,Deserialize};
use actix_web::{get,post,web,App,Result,HttpResponse,HttpServer,Responder};
use crate::mdl::{
    sysmdl::{
        usermdl
    },
    blkmdl::{
        recordmdl
    }
};
use crate::rsapi::{
    sysapi::{
        user
    },
    blkapi::{
        dial_record
    }
};

// 访问地址：http://127.0.0.1:8086/swagger-ui/index.html 或 http://127.0.0.1:8086/swagger-ui/

#[derive(OpenApi)]
#[openapi(
    paths(  // 对应接口
        user::lghdl, user::get_user, user::get_permissions, user::get_by_pages,
        user::user_opt
    ), 
    components(
        schemas( // 对应mdl请求入参模型
            usermdl::lginput, // , menumdl
        )
    ),
    tags(
        (name = "系统管理", description = "system apis")
    ),
    security(
        (),
        ("bearer_auth" = [])
    )
    // 全局应用安全要求
    ,modifiers(&SecureMiddleware) // 方案1
)]
pub struct SysApiDoc;

// 实现安全方案 方案2
// impl SysApiDoc {
//     fn bearer_auth() -> SecurityScheme {
//         SecurityScheme::Http(
//             HttpBuilder::new()
//                 .scheme(HttpAuthScheme::Bearer)
//                 .bearer_format("JWT")
//                 .build()
//         )
//     }
// }

#[derive(OpenApi)]
#[openapi(
    paths(
        dial_record::get_by_pgs, 
        dial_record::rcd_opt, 
        dial_record::get, 
        dial_record::rcd_del,
        dial_record::rcd_inserts
    ), 
    components(
        schemas(
            recordmdl::dial_page_input,
            recordmdl::dial_rcd_input
        )
    ),
    tags(
        (name = "黑名单管理", description = "blk apis")
    ),
    security(
        (),
        ("bearer_auth" = [])
    )
    // 全局应用安全要求
    ,modifiers(&SecureMiddleware)  //方案1
)]
pub struct BlkApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(), 
    components(
        schemas()
    ),
    tags(
        (name = "ASR管理", description = "asr apis")
    ),
    security(
        (),
        ("bearer_auth" = [])
    )
    // 全局应用安全要求
    ,modifiers(&SecureMiddleware)  //方案1
)]
pub struct AsrApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(), 
    components(
        schemas()
    ),
    tags(
        (name = "电商管理", description = "ds apis")
    ),
    security(
        (),
        ("bearer_auth" = [])
    )
    // 全局应用安全要求
    ,modifiers(&SecureMiddleware)  //方案1
)]
pub struct DsApiDoc;

// 方案1
struct SecureMiddleware;
impl utoipa::Modify for SecureMiddleware {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = &mut openapi.components {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                )
            );
        }
    }
}

