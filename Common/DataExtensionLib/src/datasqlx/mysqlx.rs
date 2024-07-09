use sqlx::{Error,FromRow,Row};
use sqlx::{MySqlPool};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use toml;
use crate::cfg;
use crate::datasqlx::sqlitex;

pub async fn _init() -> MySqlPool {

    // let mut file = File::open("cfg.toml");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents);
    // let cfg: Cfg = toml::from_str(&contents);
    // let db_path = cfg::database::mysql_url;
    let database_url = format!("mysql://root:99999999@localhost:3306/dsweb");
    MySqlPool::connect(&database_url).await.unwrap()

    // 执行一个查询来创建表，这将隐式创建数据库文件x
}

pub trait SqlxMysqlMap {
    fn frm_rw(rw:MySqlRow) -> Result<Self,Error> where Self: Sized;
}

pub async fn do_query<T:SqlxMysqlMap>(sql:&str) ->Result<Vec<T>,Error> where
T: FromRow<'static, MySqlRow> + Send + 'static, 
{
    let pl = _init().await;
    let mut ts = Vec::new();

    sqlx::query(sql).
    map(|rw: sqlx::mysql::MySqlRow|{
        let itm = T::frm_rw(rw).unwrap();
        ts.push(itm);
    }).
    fetch_all(&pl).await?;

    // sqlx::query_as::<_, T>(sql)
    // .fetch_all(&pl)
    // .await;

    Ok(ts)
}

pub async fn do_opt (sql:&str) -> Result<bool, Error> {
    
    let pl = _init().await;

    let rs = sqlx::query(sql)
    .execute(&pl)
    .await;

    match rs {
        Ok(rst) => {
            println!("Query succeeded: {:?}", rst);
            return Ok(true);
        },
        Err(e) => {
            println!("Query failed: {:?}", e);
            return Ok(false);
        }
    }

    Ok(false)
}

// pub async fn get_mx<T>(arrs:&[T]) -> &T {
//     let mut vl = &arrs[0];
//     for itm in arrs {
//         if itm > vl {
//             vl = itm;
//         }
//     }
//     vl
// }