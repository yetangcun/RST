use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::{Error,FromRow};
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

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    account: String,
    passwd: String,
}

pub trait SqliteRw: for<'r> sqlx::FromRow<'r, SqliteRow> {}

pub async fn do_search<T:SqliteRw>(sql:&str) -> Result<Vec<T>, Error> { // 
    let pl = _init().await;
    sqlx::query_as::<_,T>(sql)
    .fetch_all(&pl)
    .await
}

pub async fn do_query(sql:&str) -> Result<Vec<User>, Error> { // 
    let pl = _init().await;
    
    sqlx::query_as::<_, User>(sql)
    .fetch_all(&pl)
    .await
}

pub async fn do_opt (sql:&str) -> Result<bool, Error> {
    
    let pl = _init().await;

    sqlx::query(sql)
    .execute(&pl)
    .await;

    Ok(true)
}