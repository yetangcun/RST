use DataExtensionLib::{mysqlLib, pgsqlLib, mssqlLib, mongodbLib, clkhouseLib, datasqlx::{mysqlx}};
use crate::mdl::sysmdl::usermdl::{
    usrs,
    lginput,
    usr_page_input,
    usr_permissions
};

use crate::mdl::basemdl::{
    resmdl,
    req_pg,
    res_pg
};

pub async fn dologin(usr:&str) -> String {
    let sql = format!("select Passwd from sys_user where Account='{}'", usr);
    let pwd = mysqlx::query_scalar(&sql).await;
    match pwd {
        Ok(pwd) => pwd,
        Err(e) => {
            println!("err: {}", e);  // panic!("{}", e)
            e.to_string()
        }
    }
}

pub fn get_permissions (uid:i32) -> String {
    String::from("get_permissions")
}

pub fn get_user(id:i32) -> lginput {
    lginput {
        usr: String::from("xiaoxiao"),
        pwd: String::from("666666"),
        code: String::from("666666")
    }
}

pub fn get_by_pages(ipt:&req_pg<usr_page_input>) -> String {
    String::from("get_by_pages")
}