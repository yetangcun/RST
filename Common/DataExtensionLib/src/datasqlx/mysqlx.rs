use sqlx::{Error,FromRow,Row};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use toml;
use crate::cfg;
use crate::datasqlx::sqlitex;

pub async fn _init() -> mysql::MySqlPool {

    // let mut file = File::open("cfg.toml");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents);
    // let cfg: Cfg = toml::from_str(&contents);
    // let db_path = cfg::database::mysql_url;
    let database_url = format!("mysql://root:99999999@localhost:3306/dsweb");
    mysql::MySqlPool::connect(&database_url).await.unwrap()

    // 执行一个查询来创建表，这将隐式创建数据库文件x
}


pub trait MysqlMap {
    fn frm_rw(rw:&MySqlRow) -> Result<Self,Error> where Self: Sized;
}

// impl<T> MysqlMap<T> for sqlitex::User {
//     fn frm_rw(rw:&MySqlRow) -> Result<Self, Error> {
//         Ok(User {
//             id: rw.try_get("id").unwrap(),
//             account: rw.try_get("account").unwrap(),
//             passwd: rw.try_get("passwd").unwrap()
//         })
//     }
// }

pub async fn do_query<T>(sql:&str) ->Result<Vec<T>,Error> where
T: FromRow<'static, MySqlRow> + Send + 'static, 
{
    let pl = _init().await;
    let mut ts = Vec::new();

    sqlx::query_as(sql)
    .fetch_all(&pl)
    .await;

    Ok(ts)
}