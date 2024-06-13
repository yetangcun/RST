use crate::mdl::sysmdl::usermdl::{lginput,userQueryInput,userOptInput,userQueryOutput,userQuerySimple};
use crate::bll::sysbll::userbll;
use actix_web::{get,post,web,App,Result, HttpResponse,HttpServer,Responder};
use DataExtensionLib::{mysqlLib};

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

// #[delete("/sys/user/del/{id}")]
pub async fn user_del(item_id: web::Path<i32>) -> impl Responder{
    let sql = format!("delete from sys_user where Id={}",item_id);
    let rs = mysqlLib::opt(&sql);
    if rs {
        return HttpResponse::Ok().body("删除成功");
    }
       
    HttpResponse::Ok().body("删除失败") // Ok(rt)
    // HttpResponse::Ok().body(rs)
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