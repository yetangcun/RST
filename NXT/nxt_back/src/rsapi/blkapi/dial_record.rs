use std::io::Error;
use DataExtensionLib::clkhouseLib::ClkHouseHdl;
use actix_web::{get,post,options,web,put,App,HttpResponse,HttpServer,Responder,Result};
use utoipa::{ToSchema, IntoParams, OpenApi, openapi::OpenApiBuilder};
use crate::mdl::blkmdl::recordmdl::{
    dial_record,
    dial_page_input,
    dial_rcd_input,
    dial_record_out
};
use crate::mdl::basemdl::{
    resmdl,
    req_pg,
    res_pg
};
use chrono::{Local, DateTime, TimeZone, Utc, NaiveDateTime};

const CURR_MD:&str = "/blk";
// const clk_url:&str = "http://default:xiaoxiao@192.168.30.111:8123/blklogs";
// const clk_url:&str = "http://default:xiaoxiao@192.168.3.101:8123/blklogs";
// const clk_url:&str = "http://192.168.3.101:8123";
const clk_url:&str = "http://192.168.30.111:8123";

// 分页查询
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/rcd/get_by_pgs")]
pub async fn get_by_pgs(req:web::Json<req_pg<dial_page_input>>) -> Result<impl Responder> {
    let clk: ClkHouseHdl = ClkHouseHdl::new(clk_url);
    let sql = "select id,name,age,toString(intime) from my_table";
    let rs = clk.query_page::<dial_record_out>(sql, req.page, req.size).await.unwrap();

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
    
    let clk: ClkHouseHdl = ClkHouseHdl::new(clk_url);
    let sql = format!("select id,name,age,toString(intime) from blklogs.my_table where id = {}", id);
    let rs = clk.query::<dial_record_out>(&sql).await.unwrap();

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

    // let _now = chrono::Local::now().with_timezone(&chrono::FixedOffset::east(8 * 3600)).naive_local().to_string();
    let _now = Local::now()
    .naive_local()
    .format("%Y-%m-%d %H:%M:%S")    // 这个格式是必须的,不然通过toDateTime()转换会变成1970-01-01 00:00:00
    .to_string();

    println!("now: {}", _now);

    let clk: ClkHouseHdl = ClkHouseHdl::new(clk_url);
    let mut sql = String::from(""); // req.id
    if req.id > 0 {
        sql = format!("update my_table set name = '{}', age = {}, intime = toDateTime('{}') where id = {}", req.name, req.age, _now, req.id);
    } else {
        let _id = clk.query::<i32>("select max(id) as id from my_table").await.unwrap();
        let rid = _id + 1;
        sql = format!("insert into my_table (id, name, age, intime) values ({}, '{}', {}, toDateTime('{}'))", rid, req.name, req.age, _now);
    }

    println!("sql: {}", sql);
    let rs = clk.insert::<dial_record>(&sql).await;

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

// 批量写入拨打记录 不适用表中有DateTime字段的情况
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/rcd/inserts")]
pub async fn rcd_inserts(req:web::Json<Vec<dial_rcd_input>>) -> Result<impl Responder> {
    let clk: ClkHouseHdl = ClkHouseHdl::new(clk_url);
    // let mut sql = "insert into my_tb (id,name,age) values";
    let mut rows = Vec::new();
    // 使用time::OffsetDateTime::now_utc()获取当前时间
    // let _now = OffsetDateTime::now_utc();
    // let _now = Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string();
    // let dt = NaiveDateTime::parse_from_str(&_now, "%Y-%m-%d %H:%M:%S").unwrap();
    // let _now =Local::now().naive_local();

    // let _now = Local::now().to_string();
    for r in req.iter() {
        let row = dial_record {
            id: r.id,
            name: r.name.clone(),
            age: r.age
            // intime: _now
        };
        rows.push(row);
    }
    
    // let rs = clk.add_batch::<dial_record>("my_tb", rows).await.unwrap();
    let rs = clk.inserts::<dial_record>("my_tb", rows).await.unwrap();
    Ok(web::Json(rs))
}


// 批量写入拨打记录
#[utoipa::path(
    context_path = CURR_MD,
    responses(
        (status = 200, description = "succ", body = String),
        (status = 400, description = "fail"))
)]
#[post("/rcd/rcd_batch_inserts")]
pub async fn rcd_batch_inserts(req:web::Json<Vec<dial_rcd_input>>) -> Result<impl Responder> {

    let clk: ClkHouseHdl = ClkHouseHdl::new(clk_url);
    
    let _now = Local::now() // let _now = Local::now().naive_local();
    .naive_local()
    .format("%Y-%m-%d %H:%M:%S")    // 这个格式是必须的,不然通过toDateTime()转换会变成1970-01-01 00:00:00
    .to_string();

    let vals: Vec<String> = req.iter().map(|itm| {
        format!("({}, '{}', {}, toDateTime('{}'))",
            itm.id, itm.name, itm.age, _now.clone()
        )
    }).collect();
    
    let sql = format!("insert into my_table (id, name, age, intime) values {}", vals.join(","));
    // println!("sql: {}", sql);

    let rs = clk.batch_insert::<dial_record_out>(&sql).await.unwrap();
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
    let clk: ClkHouseHdl = ClkHouseHdl::new(clk_url);
    let sql = format!("delete from my_tb where id = {}", id);
    let rs = clk.del(&sql).await.unwrap();
    Ok(web::Json(rs))
}
