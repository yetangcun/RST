use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use utoipa_swagger_ui::{SwaggerUi, Url};
use serde::{Serialize,Deserialize};
use crate::selfapi::{sys::userapi};
use actix_web::{get,post,web,App,Result,HttpResponse,HttpServer,Responder};

#[derive(ToSchema, Serialize, Deserialize)]
pub struct lginput {
    pub usr: String,
    pub pwd: String
}


#[derive(ToSchema, Serialize, Deserialize)]
pub struct reqinput {
    pub usr: String,
    pub pwd: String
}

#[utoipa::path( // context_path = "/rsapi",
    context_path = "/rsapi",
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail")
    )
)]
#[post("/lghdl")]
pub async fn lghdl(req_bdy: web::Json<lginput>) -> impl Responder {
    println!("{:?}", req_bdy.usr);
    let recv:lginput = req_bdy.into_inner();
    let rt = String::from(format!("congratulations:{0}, you've logined success",recv.usr));
    let obj = reqinput { 
        usr: String::from(recv.usr), 
        pwd: String::from(recv.pwd) 
    };
    web::Json(obj) // HttpResponse::Ok().body(rt) 
}

#[utoipa::path(
    context_path = "/rsapi",
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail")
    )
)]
#[post("/reqhdl")]
pub(super) async fn reqhdl(req_bdy: web::Json<reqinput>) -> impl Responder {
    println!("{:?}", req_bdy.usr);
    let recv = req_bdy.into_inner();
    let rt = format!("great:{}, you've request over", recv.usr); //String::from();
    let obj = reqinput { 
        usr: String::from(recv.usr), 
        pwd: String::from(recv.pwd) 
    };
    web::Json(rt) // HttpResponse::Ok().body(rt) 
}

#[derive(OpenApi)]
#[openapi(
    paths(
        lghdl,
        reqhdl
    ), 
    components(
        schemas(
            lginput,
            reqinput
        )
    ),
    tags(
        (name = "系统管理", description = "systemmanager apis")
    )
)]
pub struct ApiRsDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        lghdl,
        reqhdl
    ), 
    components(
        schemas(
            lginput,
            reqinput
        )
    ),
    tags(
        (name = "业务管理", description = "bllmanager apis")
    )
)]
pub struct ApiRsDoc1;

#[derive(OpenApi)]
#[openapi(
    paths(
        userapi::get_user
    ), 
    components(
        schemas()
    ),
    tags(
        (name = "系统管理Apis", description = "sysmanager apis")
    )
)]
pub struct ApiRsDoc2;
