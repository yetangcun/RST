use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::{Error,FromRow,Row};
use serde::{Serialize,Deserialize};
// use crate::SqliteRw;

pub async fn _init() -> SqlitePool {

    let db_path = "lcl_sqlt_db.db";

    let pol = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://D:\\{}", db_path))
        .await.unwrap();

    // 执行一个查询来创建表，这将隐式创建数据库文件x
    sqlx::query("CREATE TABLE IF NOT EXISTS sys_user (id INTEGER PRIMARY KEY, account TEXT, passwd TEXT)").execute(&pol).await.unwrap();

    pol
}

// #[derive(Debug,Serialize, Deserialize, FromRow)]
// pub struct User {
//     id: i32,
//     account: String,
//     passwd: String,
// }

// 特性定义
pub trait SqlxSqliteRw:Sized { 
    fn frm_rw(row: SqliteRow) -> Result<Self, Error>;   // Required method
}

pub async fn do_query<T:SqlxSqliteRw>(sql:&str) -> Result<Vec<T>, Error> { // 
    let pl = _init().await;
    
    let mut ts = Vec::new();
    
    let rws = sqlx::query(sql)
    .fetch_all(&pl)
    .await.unwrap();

    for rw in rws {
        let itm = T::frm_rw(rw);
        ts.push(itm.unwrap());
    }

    Ok(ts)
}

pub async fn do_opt (sql:&str) -> Result<bool, Error> {
    
    let pl = _init().await;

    let _ = sqlx::query(sql)
    .execute(&pl)
    .await;

    Ok(true)
}