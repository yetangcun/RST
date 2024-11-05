use std::io::Error;
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder,Result};
use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use CommonExtensionLib::utils::{secutil, jwtutil};
use crate::mdl::sysmdl::usermdl::{
    lginput,
    usr_page_input,
    usr_permissions
    
};
use crate::mdl::basemdl::{
    resmdl,
    req_pg,
    res_pg
};

use crate::bll::sysbll::{usrbll,menubll,orgbll,permissionbll,settingbll};

const CURR_MD:&str = "/sys";

#[utoipa::path(
    context_path = "/no_auth",
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/user/dologin")]
pub async fn lghdl(req: web::Json<lginput>) -> Result<impl Responder> {
    // HttpResponse::Ok().body(format!("congratulations:{0}, you've logined success!",req.usr))
    let usr = &req.usr;
    let pwd = secutil::md5_hash(&req.pwd);
    let query_pwd = usrbll::dologin(usr).await; // 获取数据库中的密码

    println!("usr:{}, pwd:{}, query_pwd:{}", usr, pwd, query_pwd);

    if pwd != query_pwd { // 匹配失败, 则返回错误
        return Err(actix_web::error::ErrorUnauthorized(query_pwd));
    }
    let tk_str = jwtutil::create_tken();
    Ok(web::Json(tk_str))
}

#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[get("/user/permissions/{id}")]
pub async fn get_permissions(id: web::Path<i32>) -> Result<impl Responder> {
    let data = usr_permissions {
        tk: String::from("token")
    };
    let rs_obj = resmdl::succ(String::from("200"),String::from("succ"),data);   
    Ok(web::Json(rs_obj))
}

#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/user/get_by_pages")]
pub async fn get_by_pages(ipt:web::Json<req_pg<usr_page_input>>) -> Result<impl Responder> {
    let rs = usrbll::get_by_pages(&ipt).await;
    
    let mut pgs = 0;

    if rs.0 % ipt.size == 0 {
        pgs = rs.0 / ipt.size;
    } else {
        pgs = rs.0 / ipt.size + 1;
    }

    let rs_obj = res_pg::succ(rs.0, pgs, rs.1);

    Ok(web::Json(rs_obj))
}


#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[get("/user/{id}")]
pub async fn get_user(id: web::Path<i32>) -> Result<impl Responder> {
    
    let res_obj = lginput {
        usr: String::from("xiaoxiao"),
        pwd: String::from("666666"),
        code: String::from("666666")
    };
    
    Ok(web::Json(res_obj))
}