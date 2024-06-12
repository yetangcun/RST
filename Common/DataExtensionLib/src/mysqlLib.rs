// use mysql::*;
use mysql::{prelude::*, Pool, PooledConn};

const conn_str:&str = "mysql://root:99999999@localhost:3306/dsweb";

fn get_conn() -> PooledConn {
    let pool = Pool::new(conn_str).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn
}

pub fn do_query() -> Vec<(i32,String,String)> {
    let mut conn = get_conn();
    let rs:Vec<(i32,String,String)> = 
    conn.query_map("select Id, Account, Passwd from sys_user", |(Id, Account, Passwd)|{(Id,Account,Passwd)}).unwrap();

    for (Id,Account,Passwd) in &rs {
        println!("ID: {}, Account: {}, Passwd:{}", Id, Account, Passwd);
    }

    rs
}