use sqlx::{Error,FromRow,Row,MySqlPool,query};
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

pub async fn query_list<T:SqlxMysqlMp>(sql:&str) ->Result<Vec<T>,Error>
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

pub async fn query_lst<T>(sql:&str) ->Result<Vec<T>,Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> 
    + Send 
    + Unpin,
{
    let pl = _init().await;
    let mut ts:Vec<T> = Vec::new();
    let rws = sqlx::query_as::<_, T>(sql).fetch_all(&pl).await;

    match rws {
        Ok(rws) => {
            for rw in rws {
                ts.push(rw);
            }
        },
        Err(e) => {
            println!("err: {}", e);  // panic!("{}", e)
            return Err(e);
        }
    }
    Ok(ts)

    // rws
}

pub async fn query_one<T:SqlxMysqlMp>(sql:&str) ->Result<T,Error> // 查询一条记录
{
    let pl = _init().await;
    let rw = sqlx::query(sql).fetch_one(&pl).await.unwrap();
    let itm = T::frm_rw(rw);
    Ok(itm.unwrap())
}

pub async fn query_scalar<T>(sql:&str) -> Result<T,Error> // 查询单值
where 
T: for<'r> sqlx::decode::Decode<'r, sqlx::MySql> 
+ sqlx::Type<sqlx::MySql>
+ Send
+ Unpin
{
    let pl = _init().await;
    sqlx::query_scalar(sql)
        .fetch_one(&pl)
        .await
}

pub async fn query_page<T>(pgsql:&str, sql:&str) -> Result<(i32, Vec<T>), Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> 
    + Send 
    + Unpin
{
    let pl = _init().await;
    let mut ts:Vec<T> = Vec::new();
    let count = sqlx::query_scalar::<_, i32>(pgsql) // 获取总记录数
    .fetch_one(&pl)
    .await
    .unwrap();

    let rws = sqlx::query_as::<_,T>(sql) // 获取记录
    .fetch_all(&pl)
    .await;

    match rws {
        Ok(rws) => {
            for rw in rws {
                ts.push(rw);
            }
        },
        Err(e) => {
            println!("err: {}", e);  // panic!("{}", e)
            return Err(e);
        }
    }

    // Ok(match count {
    //     Ok(count) => (count, ts), // rws.unwrap()
    //     Err(e) => panic!("{}", e)
    // })

    Ok((count, ts))
}

pub async fn do_query<T:DbrowMap<MySqlRow,Error>>(sql:&str) ->Result<Vec<T>,Error> // 查询满足条件的数据列表
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

pub async fn do_opt (sql:&str) -> Result<bool, Error> { // 执行sql语句，返回是否成功
    
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