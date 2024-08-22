use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use actix_web::{get,post,web,App,Result, HttpResponse,HttpServer,Responder};
use utoipa_swagger_ui::{SwaggerUi, Url};
use serde::{Serialize,Deserialize};

#[derive(ToSchema, Serialize, Deserialize)]
pub struct lginput {
    pub usr: String,
    pub pwd: String,
}


#[derive(ToSchema, Serialize, Deserialize)]
pub struct reqinput {
    pub usr: String,
    pub pwd: String,
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
    web::Json(obj)
    // HttpResponse::Ok().body(rt) 
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
    let recv:reqinput = req_bdy.into_inner();
    let rt = format!("great:{0}, you've request over",recv.usr); //String::from();
    let obj = reqinput { 
        usr: String::from(recv.usr), 
        pwd: String::from(recv.pwd) 
    };
    web::Json(rt)
    // HttpResponse::Ok().body(rt) 
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
    ))]
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
    )
)]
pub struct ApiRsDoc1;
