use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use utoipa_swagger_ui::{SwaggerUi, Url};
use serde::{Serialize,Deserialize};
use actix_web::{get,post,web,App,Result,HttpResponse,HttpServer,Responder};

// use crate::selfapi::{sys::userapi};


#[derive(OpenApi)]
#[openapi(
    paths(  // 对应接口
        // lghdl,
        // reqhdl
    ), 
    components(
        schemas( // 对应mdl
            //lginput,
            //reqinput
        )
    ),
    tags(
        (name = "系统管理", description = "system apis")
    )
)]
pub struct SysApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(), 
    components(
        schemas(
            //lginput,
            //reqinput
        )
    ),
    tags(
        (name = "黑名单管理", description = "blk apis")
    )
)]
pub struct BlkApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        // userapi::get_user
    ), 
    components(
        schemas()
    ),
    tags(
        (name = "ASR管理", description = "asr apis")
    )
)]
pub struct AsrApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        // userapi::get_user
    ), 
    components(
        schemas()
    ),
    tags(
        (name = "电商管理", description = "ds apis")
    )
)]
pub struct DsApiDoc;
