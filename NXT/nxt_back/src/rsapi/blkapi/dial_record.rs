use std::io::Error;
use DataExtensionLib::clkhouseLib::ClkHouseHdl;
use actix_web::{get,post,options,web,put,App,HttpResponse,HttpServer,Responder,Result};
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
#[post("/rcd/get_by_pgs")]
pub async fn get_by_pgs(req:web::Json<req_pg<dial_page_input>>) -> Result<impl Responder> {

    // let clk: ClkHouseHdl = ClkHouseHdl::new("http://default:xiaoxiao@192.168.30.111:8123/blklogs");
    
    let clk: ClkHouseHdl = ClkHouseHdl::new("http://default:xiaoxiao@192.168.30.111:8123/blklogs");
    let sql = "select id,name,age,intime from my_table";
    let rs = clk.query_page::<dial_record>(sql, req.page, req.size).await.unwrap();

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
    
    let clk: ClkHouseHdl = ClkHouseHdl::new("http://192.168.30.111:8123");
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
    let clk: ClkHouseHdl = ClkHouseHdl::new("http://192.168.30.111:8123");
    let mut sql = format!("insert into my_table values ({},'{}',{},'{}')", req.id, req.name, req.age, _now);
    if req.id > 0 {
        sql = format!("update my_table set name = '{}', age = {}, intime = '{}' where id = {}", req.name, req.age, _now, req.id);
    }
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


// 批量写入拨打记录
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/rcd/inserts")]
pub async fn rcd_inserts(req:web::Json<Vec<dial_rcd_input>>) -> Result<impl Responder> {
    let clk: ClkHouseHdl = ClkHouseHdl::new("http://");
    let mut sql = "insert into my_table values ";
    let mut rows = Vec::new();
    let _now = chrono::Local::now().naive_local();
    for r in req.iter() {
        let row = dial_record {
            id: r.id,
            name: r.name.clone(),
            age: r.age,
            intime: _now
        };
        rows.push(row);
    }
    let rs = clk.inserts::<dial_record>(sql, rows).await.unwrap();
    Ok(web::Json(rs))
}

// 删除拨打记录
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[put("/rcd/del/{id}")]
pub async fn rcd_del(id: web::Path<u64>) -> Result<impl Responder> {
    let clk: ClkHouseHdl = ClkHouseHdl::new("http://192.168.30.111:8123");
    let sql = format!("delete from my_table where id = {}", id);
    let rs = clk.del(&sql).await.unwrap();
    Ok(web::Json(rs))
}
