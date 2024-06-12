use crate::mdl::sysmdl::usermdl::{lginput,userQueryInput,userOptInput,userQueryOutput};
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
    let nm = &req.name;
    let rt = String::from("congratulations {nm}, you've logined success!");
    HttpResponse::Ok().body(rt) // Ok(rt)
}

#[get("/sys/user/getuser")]
pub async fn get_user() -> Result<impl Responder> {
    let rt = "hi, i'm user";

    let usr = userQueryOutput {
        name:String::from("xiaoxiao"),
        employeeNo:String::from("YF00001"),
        roleName:String::from("管理员"),
        orgName:String::from("研究中心"),
        addr:String::from("yetangcun"),
        age:1,
        phone:String::from("15111111111"),
        addUser:String::from("SUPERADMIN"),
        addTime:String::from("2024-06-12 10:10:10"),
        upUser:String::from("ADMIN"),
        upTime:String::from("2024-06-12 10:10:12"),
        state:1
    };

    let rss:Vec<(i32,String,String)> = mysqlLib::do_query();

    Ok(web::Json(rss))
}