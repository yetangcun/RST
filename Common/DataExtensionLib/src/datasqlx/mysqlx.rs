use sqlx::{Error,FromRow,Row};
use sqlx::{MySqlPool};
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use toml;
use crate::{cfg, DbrowMap};

pub async fn _init() -> MySqlPool {

    // let mut file = File::open("cfg.toml");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents);
    // let cfg: Cfg = toml::from_str(&contents);
    // let db_path = cfg::database::mysql_url;
    
    // let db_url = format!("mysql://root:99999999@localhost:3306/dsweb");
    // MySqlPool::connect(&db_url).await.unwrap()
    
    let db_url = "mysql://root:99999999@localhost:3306/dsweb";
    MySqlPool::connect(db_url).await.unwrap()

    // 执行一个查询来创建表，这将隐式创建数据库文件x
}

// 特性定义
pub trait SqlxMysqlMp: Sized {
    fn frm_rw(rw:MySqlRow) -> Result<Self,Error> where Self: Sized;
}

pub async fn exe_query<T:SqlxMysqlMp>(sql:&str) ->Result<Vec<T>,Error>
{
    let pl = _init().await;
    let mut ts = Vec::new();

    let rws = sqlx::query(sql).fetch_all(&pl).await.unwrap();

    for rw in rws {
        let itm = T::frm_rw(rw);
        ts.push(itm.unwrap());
    }

    Ok(ts)
}

pub async fn do_query<T:DbrowMap<MySqlRow,Error>>(sql:&str) ->Result<Vec<T>,Error>
{
    let pl = _init().await;
    let mut ts = Vec::new();

    let rws = sqlx::query(sql).fetch_all(&pl).await.unwrap();

    for rw in rws {
        let itm = T::frm_rw(rw);
        ts.push(itm.unwrap());
    }

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