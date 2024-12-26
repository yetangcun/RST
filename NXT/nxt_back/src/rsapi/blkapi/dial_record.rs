use std::io::Error;
use DataExtensionLib::clkhouseLib::ClkHouseClient;
use actix_web::{get,post,options,web,App,HttpResponse,HttpServer,Responder,Result};
use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use crate::mdl::blkmdl::recordmdl::{
    dial_record,
    dial_page_input,
    dial_rcd_input
};
use crate::mdl::basemdl::{
    resmdl,
    req_pg,
    res_pg
};

const CURR_MD:&str = "/blk";

// 分页查询
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/rcd/get_by_pages")]
pub async fn get_by_pages(req:web::Json<req_pg<dial_page_input>>) -> Result<impl Responder> {

    let clk: ClkHouseClient = ClkHouseClient::new("http://192.168.30.111:8123");
    let sql = "select * from my_table";
    let rs = clk.query_page::<dial_record>(sql, 1, 20).await.unwrap();

    let mut pgs = 0;

    if rs.0 % req.size == 0 {
        pgs = rs.0 / req.size;
    } else {
        pgs = rs.0 / req.size + 1;
    }

    let rs_obj = res_pg::succ(rs.0, pgs, rs.1);

    Ok(web::Json(rs_obj))
}

// 获取单个记录
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[get("/rcd/{id}")]
pub async fn get(id: web::Path<u64>) -> Result<impl Responder> {
    
    let clk: ClkHouseClient = ClkHouseClient::new("http://192.168.30.111:8123");
    let sql = format!("select * from my_table where id = {}", id);
    let rs = clk.query::<dial_record>(&sql).await.unwrap();

    Ok(web::Json(rs))
}

// 添加拨打记录
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/rcd/opt")]
pub async fn rcd_opt(req:web::Json<dial_rcd_input>) -> Result<impl Responder> {

    let _now = chrono::Local::now().naive_local();
    let clk: ClkHouseClient = ClkHouseClient::new("http://192.168.30.111:8123");
    let sql = format!("insert into my_table values ({},'{}',{},'{}')", req.id, req.name, req.age, _now);
    let rs = clk.insert::<dial_record>(&sql);
    
    match rs {
        Ok(_) => {
            let rs_obj = resmdl::succ(String::from("200"),String::from("succ"), true);
            Ok(web::Json(rs_obj))
        },
        Err(e) => {
            let rs_obj = resmdl::fail(String::from("400"),String::from("fail"), false);
            Ok(web::Json(rs_obj))
        }
    }
}