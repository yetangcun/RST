use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use utoipa_swagger_ui::{SwaggerUi, Url};
use serde::{Serialize,Deserialize};
use actix_web::{get,post,web,App,Result,HttpResponse,HttpServer,Responder};

use crate::mdl::{
    sysmdl::{
        usermdl
    }
};

use crate::rsapi::{
    sysapi::{
        user
    }
};


#[derive(OpenApi)]
#[openapi(
    paths(  // 对应接口
        user::lghdl, user::get_user, user::get_permissions, user::get_by_pages
    ), 
    components(
        schemas( // 对应mdl请求入参模型
            usermdl::lginput, // , menumdl
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
            //lginput, reqinput
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
