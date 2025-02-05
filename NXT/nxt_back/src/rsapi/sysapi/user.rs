use std::io::Error;
use actix_web::{get,post,options,web,App,HttpResponse,HttpServer,Responder,Result};
use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use CommonExtensionLib::utils::{secutil, jwtutil};
use crate::mdl::sysmdl::usermdl::{
    lginput,
    lgoutput,
    usr_page_input,
    usr_permissions,
    permissions_input
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
    let usr = &req.usr;
    let pwd = secutil::md5_hash(&req.pwd);
    let login_res = usrbll::dologin(usr).await; // 获取数据库中的密码

    println!("usr:{}, pwd:{}, query_pwd:{}", usr, pwd, login_res.1);

    if pwd != login_res.1 { // 匹配失败, 则返回错误
        let err_mdl = resmdl::fail(
            400.to_string(), 
            String::from("账号或密码错误"), 
            lgoutput { tken: String::from(""), uid: String::from("") }
        );
        return Ok(web::Json(err_mdl)); // actix_web::error::ErrorUnauthorized("账号或密码错误")
    }

    let res_mdl = resmdl::succ(
        200.to_string(), 
        String::from("success"), 
        lgoutput { tken: jwtutil::create_tken(), uid: login_res.0 } 
    );
    
    Ok(web::Json(res_mdl))
}

#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = usr_permissions),
        (status = 400, description = "fail"))
)]
#[get("/user/permissions/{id}")]
pub async fn get_permissions(id: web::Path<String>) -> Result<impl Responder> {
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
    // ,security(
    //     ("bearer_auth" = [])
    // )
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

#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/user/opt")]
pub async fn user_opt(req: web::Json<lginput>) -> Result<impl Responder> {
    let rs = usrbll::usr_opt(&req).await;
    
    if rs {
        let rs_obj = resmdl::succ(200.to_string(), String::from(""), rs);
        Ok(web::Json(rs_obj))
    }
    else {
        let rs_obj = resmdl::fail(400.to_string(), String::from(""), rs);
        Ok(web::Json(rs_obj))
    }
}