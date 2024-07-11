use crate::mdl::sysmdl::usermdl::{lginput,userQueryInput,userOptInput,userQueryOutput,userQuerySimple,userOptSimplInput};
use crate::mdl::sysmdl::rsmdl;
use crate::bll::sysbll::userbll;
use actix_web::{get,post,web,App,Result, HttpResponse,HttpServer,Responder};
use DataExtensionLib::{mysqlLib, datasqlx};
use chrono::{Local, DateTime};

#[post("/sys/user/dologin")]
pub async fn do_login(req: web::Json<lginput>) -> impl Responder {
    // let rt = String::from(userbll::do_login(req_body.into_inner()).await);
    let rt = String::from(format!("congratulations:{0}, you've logined success!",req.account));
    HttpResponse::Ok().body(rt) // Ok(rt)
}

// #[post("/sys/user/dologin1")]
pub async fn opt(req: web::Json<userOptInput>) -> impl Responder {
    // let rt = String::from(userbll::do_login(req_body.into_inner()).await);
    let _nm = &req.name;
    let rt = String::from("congratulations {_nm}, you've logined success!");
    
    HttpResponse::Ok().body(rt) // Ok(rt)
}

// mysqlLib mysql
// #[delete("/sys/user/del/{id}")]
pub async fn user_del(id: web::Path<i32>) -> impl Responder{
    let sql = format!("delete from sys_user where Id={}",id);
    let rs = mysqlLib::opt(&sql);
    if rs {
        return HttpResponse::Ok().body("删除成功");
    }
    HttpResponse::Ok().body("删除失败") // Ok(rt)
    // HttpResponse::Ok().body(rs)
}

#[post("/sys/user/add")]
pub async fn user_add(req: web::Json<userOptSimplInput>) -> impl Responder {
    let tm = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("add fn: {tm}");
    let sql = format!("insert into sys_user(Id,Account,Passwd,Status,IsDeleted,CreateTime,CreateUserId) values({0},'{1}','{2}',{3},{4},'{5}',{6});",
    req.id,req.account,"fae0b27c451c728867a567e8c1bb4e53",1,0,tm,99999998);
    let rs = mysqlLib::opt(&sql);
    let msg = if rs {
        String::from("添加成功")
    }
    else {
        String::from("添加失败")
    };
    HttpResponse::Ok().body(msg) // Ok(rt)
}

#[post("/sys/user/update")]
pub async fn user_update(req: web::Json<userOptSimplInput>) -> Result<impl Responder> {
    let tm = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("up fn: {tm}");
    let sql = format!("update sys_user set Account='{0}',ModifyTime='{1}',ModifyUserId={2} where Id={3};",req.account,tm,99999998,req.id);
    let rs = mysqlLib::opt(&sql);
    let msg = if rs {
        String::from("更新成功")
    }
    else {
        String::from("更新失败")
    };

    // HttpResponse::Ok().body(msg) 
    let rsml = rsmdl {
        msg: msg,
        succ: true
    };

    Ok(web::Json(rsml))
}

#[get("/sys/user/getuser")]
pub async fn get_user() -> Result<impl Responder> {

    let _rt = "hi, i'm user";

    let _usr = userQueryOutput {
        name:String::from("xiaoxiao"),
        employee_no:String::from("YF00001"),
        role_name:String::from("管理员"),
        org_name:String::from("研究中心"),
        addr:String::from("yetangcun"),
        age:1,
        phone:String::from("15111111111"),
        add_user:String::from("SUPERADMIN"),
        add_time:String::from("2024-06-12 10:10:10"),
        up_user:String::from("ADMIN"),
        up_time:String::from("2024-06-12 10:10:12"),
        state:1
    };

    // let rss:Vec<(i32,String,String)> = mysqlLib::do_query();
    let rss0:Vec<userQuerySimple> = mysqlLib::query::<userQuerySimple>("select Id, Account, Passwd from sys_user");

    Ok(web::Json(rss0))
}

// sqlx sqlite
#[get("/sys/user/get_tusr")]
pub async fn get_usr() -> Result<impl Responder> {

    let rss0:Vec<userQuerySimple> = datasqlx::sqlitex::do_query::<userQuerySimple>("select Id, Account, Passwd from sys_user").await.unwrap();

    Ok(web::Json(rss0))
}

#[post("/sys/user/add_usr")]
pub async fn add_usr(req: web::Json<userQuerySimple>) -> Result<impl Responder> {

    println!("add fn: {0}-{1}-{2}", req.id, req.account, req.passwd);
    let sql = format!("insert into sys_user(Id, Account, Passwd) values('{0}','{1}', '{2}')", req.id, req.account, req.passwd);
    let rs = datasqlx::sqlitex::do_opt(&sql).await.unwrap();

    Ok(web::Json(rs))
}

// sqlx mysql
#[get("/sys/user/search")]
pub async fn search() -> Result<impl Responder> {

    let rs:Vec<userQuerySimple> = datasqlx::mysqlx::exe_query::<userQuerySimple>("select Id,Account,Passwd from sys_user").await.unwrap();

    Ok(web::Json(rs))
}

#[post("/sys/user/opt")]
pub async fn do_opt(req: web::Json<userOptSimplInput>) -> Result<impl Responder> {

    let rs = datasqlx::mysqlx::do_opt("insert into sys_user() values()").await.unwrap();

    Ok(web::Json(rs))
}