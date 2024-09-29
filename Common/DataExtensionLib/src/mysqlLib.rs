use mysql::{prelude::*, Pool, PooledConn}; // use mysql::*;

// 99999999 | xiaoxiaojun
const conn_str:&str = "mysql://root:99999999@localhost:3306/blackweb";

fn get_conn() -> PooledConn {
    let pool = Pool::new(conn_str).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn
}

pub fn do_insert() {
    let mut conn = get_conn();
    conn.exec_drop("insert into sys_user(Account,Passwd) values(?,?)", ("admin", "123456")).unwrap();
}

pub fn do_update() {
    let mut conn = get_conn();
    conn.exec_drop("update sys_user set Passwd=? where Account=?", ("123456", "admin")).unwrap();
}

pub fn do_del(sql:&str) {
    let mut conn = get_conn();
    conn.exec_drop("delete from sys_user where Account=?", ("admin",)).unwrap();
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

// 类似于接口 trait 只有方法定义不需要实现
pub trait FrmRow: Sized {
    fn from_row(row: mysql::Row) -> Result<Self, mysql::Error>;
}

pub fn query<T:FrmRow>(sql:&str) -> Vec<T> {
    
    let mut conn = get_conn();
    let mut rsts = Vec::new();

    // 严谨处理
    let rows = conn.query(sql);
    
    // match rows {
    //     Ok(rws) => {
    //         for rw in rws {
    //             let itm = T::from_row(rw).unwrap();
    //             rsts.push(itm);
    //         }
    //     },
    //     Err(err) => println!("{:?}",err),
    // };
    if let Ok(rws) = rows {
        for rw in rws {
            let itm = T::from_row(rw).unwrap();
            rsts.push(itm);
        }
    }
    else if let Err(err) = rows { //  
        println!("{:?}",err);
    }
    
    // 简单处理
    // let rws = conn.query(sql).unwrap();
    // for rw in rws {
    //     let itm = T::from_row(rw).unwrap();
    //     reslts.push(itm);
    // }

    rsts
}

// 操作:插入、更新、删除
pub fn opt(sql:&str) -> bool {
    let mut conn = get_conn();
    let reslt = conn.exec_drop(sql, ());

    if reslt.is_ok() {
        let rs = reslt.unwrap();
        println!("{:?}-{sql}",rs);
        return true;
    }

    false
}
