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
    let sql = format!("select Id, Account, Status from sys_user where Id={}", id);
    let rs = mysqlx::query_row(&sql).await;
    lginput {
        usr: String::from("xiaoxiao"),
        pwd: String::from("666666"),
        code: String::from("666666")
    }
}

pub fn usr_opt () -> bool { true }

pub async fn get_by_pages(ipt:&req_pg<usr_page_input>) -> (i32, Vec<usrs>) {
    let mut whr = String::from("1=1");
    if ipt.params.name != "" {
        whr += &format!(" and Account like '%{}%'", ipt.params.name);
    }
    let data_sql = format!("select * from sys_user where {} limit {} offset {}", whr, ipt.size, (ipt.page - 1) * ipt.size);
    println!("data_sql: {}", &data_sql);
    
    let pg_res = mysqlx::query_page::<usrs>(
        "select count(1) from sys_user", 
        &data_sql)
        .await;

    match pg_res {
        Ok(pg_res) => pg_res,
        Err(e) => {
            println!("err: {}", e.to_string());
            let datas: Vec<usrs> = vec![];
            (0, datas)
        }
    }
}